use git2::Repository;
use inquire::{
    validator::{MinLengthValidator, Validation},
    Confirm, Select, Text,
};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use tera::{Context, Tera};
use walkdir::WalkDir;

use std::{collections::HashMap, path::Path, process::exit};

use crate::utils::{get_root_project_folder, is_binary, ContextValue, MetaData, TemplatePrompt};

fn ask_questions(
    questions: Vec<TemplatePrompt>,
    context_values: &mut HashMap<String, ContextValue>,
) -> &HashMap<String, ContextValue> {
    for prompt in questions.iter() {
        if prompt.r#type == "string" {
            let mut text_prompt = Text::new(&prompt.prompt);
            if prompt.default.is_some() {
                text_prompt = text_prompt.with_default(&prompt.default.as_ref().unwrap())
            }

            if prompt.is_required.is_some() && prompt.is_required.unwrap().eq(&true) {
                text_prompt = text_prompt.with_validator(MinLengthValidator::new(1));
            }

            let path_exist_validator = move |input: &str| {
                if input.eq(".") {
                    return Ok(Validation::Valid);
                }
                match Path::new(input).is_dir() {
                    false => Ok(Validation::Valid),
                    true => Ok(Validation::Invalid("The folder already exist".into())),
                }
            };

            if prompt.enable_path_exist_validator.is_some()
                && prompt.enable_path_exist_validator.unwrap().eq(&true)
            {
                text_prompt = text_prompt.with_validator(path_exist_validator);
            }
            match text_prompt.prompt() {
                Ok(a) => {
                    context_values.insert(prompt.field_key.clone(), ContextValue::String(a));
                }
                Err(_) => {
                    println!("You cancelled");
                    exit(1);
                }
            };
        } else if prompt.r#type == "bool" {
            let mut confirm_prompt = Confirm::new(&prompt.prompt);
            if prompt.default.is_some() {
                confirm_prompt =
                    confirm_prompt.with_default(prompt.default.as_ref().unwrap().eq("TRUE"));
            }

            match confirm_prompt.prompt() {
                Ok(a) => {
                    context_values.insert(prompt.field_key.clone(), ContextValue::Boolean(a));

                    match &prompt.callbacks {
                        Some(callbacks) => {
                            for callback in callbacks.iter() {
                                let mut follow_ups = vec![];

                                if a && callback.condition.eq("TRUE") {
                                    for ques in callback.follow_up_prompts.iter() {
                                        follow_ups.push(TemplatePrompt {
                                            r#type: ques.r#type.clone(),
                                            field_key: ques.field_key.clone(),
                                            prompt: ques.prompt.clone(),
                                            enums: ques.enums.clone(),
                                            default: ques.default.clone(),
                                            is_required: ques.is_required.clone(),
                                            callbacks: Some(vec![]),
                                            enable_path_exist_validator: ques
                                                .enable_path_exist_validator
                                                .clone(),
                                        });
                                    }

                                    ask_questions(follow_ups, context_values);
                                } else if !a && callback.condition.eq("FALSE") {
                                    for ques in callback.follow_up_prompts.iter() {
                                        follow_ups.push(TemplatePrompt {
                                            r#type: ques.r#type.clone(),
                                            field_key: ques.field_key.clone(),
                                            prompt: ques.prompt.clone(),
                                            enums: ques.enums.clone(),
                                            default: ques.default.clone(),
                                            is_required: ques.is_required.clone(),
                                            callbacks: Some(vec![]),
                                            enable_path_exist_validator: ques
                                                .enable_path_exist_validator
                                                .clone(),
                                        });
                                    }

                                    ask_questions(follow_ups, context_values);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Err(_) => {
                    println!("You cancelled");
                    exit(1);
                }
            };
        } else if prompt.r#type == "enum" {
            match Select::new(&prompt.prompt, prompt.enums.clone().unwrap()).prompt() {
                Ok(selected_value) => {
                    context_values.insert(
                        prompt.field_key.clone(),
                        ContextValue::String(selected_value),
                    );
                }
                Err(_) => {}
            }
        } else {
            println!("Invalid meta data found. Please contact support!")
        }
    }
    return context_values;
}

#[tokio::main]
async fn fetch_metadata_and_process(path: &String) -> HashMap<String, ContextValue> {
    let client = reqwest::Client::new();
    let response = client.get(path).send().await.unwrap();

    if response.status().is_success() {
        let meta_data: MetaData = response.json().await.unwrap();
        let mut context_values = HashMap::new();

        let questions = meta_data.prompts;

        ask_questions(questions, &mut context_values);
        context_values
    } else {
        println!("Unable to get the metadata for this template");
        exit(1)
    }
}

pub fn new_project(repo: String) {
    let template_path = format!(
        "https://raw.githubusercontent.com/{}/main/metadata.json",
        repo
    );
    let context = fetch_metadata_and_process(&template_path);
    create_new_project(&context, repo);
}

fn render_repo(repo_path: PathBuf, context: &HashMap<String, ContextValue>) {
    let mut tera = match Tera::new(&format!("{}/**/*", repo_path.to_str().unwrap())) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let root_dir = get_root_project_folder(context);

    for entry in WalkDir::new(&repo_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative_path = path.strip_prefix(&repo_path).unwrap();
        if relative_path.starts_with(".git")
            || (relative_path == Path::new("metadata.json") || relative_path == Path::new(""))
        {
            continue;
        }

        if path.is_file() {
            if is_binary(path) {
                println!("{:?} (binary file)", relative_path);
            } else {
                println!("{:?} (text file)", relative_path);

                let mut tera_context = Context::new();
                for (key, value) in context.iter() {
                    tera_context.insert(key, value);
                }

                let template_content = fs::read_to_string(path).expect("Failed to read file");
                let rendered_content = tera
                    .render_str(&template_content, &tera_context)
                    .expect("Failed to render template");

                let output_path = Path::new(&root_dir).join(relative_path);
                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent).expect("Failed to create directories");
                }
                let mut output_file = File::create(output_path).expect("Failed to create file");
                output_file
                    .write_all(rendered_content.as_bytes())
                    .expect("Failed to write to file");
            }
        } else {
            println!("{:?} (directory)", relative_path);
        }
    }
}

#[tokio::main]
async fn create_new_project(context: &HashMap<String, ContextValue>, repo: String) {
    println!("Creating project now. Context is {:?}", context);

    let home_dir = env::var("HOME").expect("Failed to get home directory");
    let mut repo_path = PathBuf::from(home_dir);
    repo_path.push("Documents/repos/");
    let repo_name = repo.split('/').last().unwrap_or("repo");
    repo_path.push(repo_name); // Add the repo name to the path

    // Ensure the directory exists
    std::fs::create_dir_all(&repo_path).expect("Failed to create directories");

    let url = format!("https://github.com/{}", repo);
    match Repository::clone(&url, &repo_path) {
        Ok(_) => render_repo(repo_path, context),
        Err(e) => match e.code() {
            git2::ErrorCode::Exists => render_repo(repo_path, context),
            _ => {
                println!("Unable to clone template. Exiting!");
                exit(1)
            }
        },
    };
}
