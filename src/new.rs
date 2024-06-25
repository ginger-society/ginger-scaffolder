use inquire::{
    validator::{MinLengthValidator, Validation},
    Confirm, Select, Text,
};

use serde::Serialize;
use std::{collections::HashMap, fmt, fs, path::Path, process::exit};
use std::{fs::File, io::Write};

use crate::utils::{self, MetaData, RenderedProject, TemplatePrompt};

#[derive(Debug, Serialize)]
enum ContextValue {
    Boolean(bool),
    String(String),
}

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
    let template_path = String::from(format!(
        "https://raw.githubusercontent.com/{}/main/metadata.json",
        repo
    ));

    let context = fetch_metadata_and_process(&template_path);
    create_new_project(&context, template_path)
}

fn get_root_project_folder(_context: &HashMap<String, ContextValue>) -> String {
    match _context.get("root_dir") {
        Some(v) => match v {
            ContextValue::String(s) => s.to_string(),
            _ => String::from("."),
        },
        None => String::from("."),
    }
}

fn write_rendered_files(files: Vec<RenderedProject>, root_folder: String) {
    for f in files.iter() {
        let parent_dir_path = format!("{}{}", root_folder, f.parent_dir,);

        match fs::create_dir_all(&parent_dir_path) {
            Ok(_) => {}
            Err(e) => {
                println!("Unable to create directory : {:?}", e)
            }
        }

        match File::create(format!("{}{}", parent_dir_path, f.file_name)) {
            Ok(mut file) => match file.write_all(f.content.as_bytes()) {
                Ok(_) => println!("Written {}", f.file_name),
                Err(_) => {
                    println!("Failed to write {}", f.file_name);
                    exit(1);
                }
            },
            Err(_) => {
                println!("Unable to create files , please check if the root directory you provided exist and you have the required permission to do so.");
                exit(1);
            }
        }
    }
}

#[tokio::main]
async fn create_new_project(context: &HashMap<String, ContextValue>, path: String) {
    println!("Creating project now. Context is {:?}", context)
}
