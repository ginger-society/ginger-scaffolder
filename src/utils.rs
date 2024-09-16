use serde_json::Value;
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    process::exit,
};

use inquire::Select;
use serde::{Deserialize, Serialize};
use MetadataService::{apis::default_api::metadata_get_all_templates, get_configuration};

use crate::new::new_project;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TemplateDetails {
    pub description: String,
    pub short_name: String,
    pub url: String,
}

impl fmt::Display for TemplateDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_name)
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RepoMetaData {
    version: String,
    pub templates: Vec<TemplateDetails>,
}

#[derive(Debug, Serialize)]
pub enum ContextValue {
    Boolean(bool),
    String(String),
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetaData {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "prompts")]
    pub prompts: Vec<TemplatePrompt>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TemplatePrompt {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "field_key")]
    pub field_key: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "enums", skip_serializing_if = "Option::is_none")]
    pub enums: Option<Vec<String>>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(rename = "is_required", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(
        rename = "enable_path_exist_validator",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_path_exist_validator: Option<bool>,
    #[serde(rename = "callbacks", skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<Vec<Callback>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Callback {
    #[serde(rename = "follow_up_prompts")]
    pub follow_up_prompts: Vec<FollowupPromot>,
    #[serde(rename = "condition")]
    pub condition: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FollowupPromot {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "field_key")]
    pub field_key: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "enums", skip_serializing_if = "Option::is_none")]
    pub enums: Option<Vec<String>>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(rename = "is_required", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(
        rename = "enable_path_exist_validator",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_path_exist_validator: Option<bool>,
}

pub fn is_binary(file_path: &Path) -> bool {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return false, // If the file can't be opened, treat it as non-binary for simplicity
    };
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 8000];
    match reader.read(&mut buffer) {
        Ok(bytes_read) => buffer[..bytes_read].contains(&0),
        Err(_) => false, // If there's an error reading the file, treat it as non-binary for simplicity
    }
}

pub fn get_root_project_folder(_context: &HashMap<String, ContextValue>) -> String {
    match _context.get("root_dir") {
        Some(v) => match v {
            ContextValue::String(s) => s.to_string(),
            _ => String::from("."),
        },
        None => String::from("."),
    }
}

pub fn extract_username_and_repo(git_url: &str) -> Option<(String, String)> {
    let re = regex::Regex::new(r"^https://github.com/([^/]+)/([^/]+)$").unwrap();
    if let Some(captures) = re.captures(git_url) {
        let username = captures.get(1)?.as_str().to_string();
        let repo_name = captures.get(2)?.as_str().to_string();
        Some((username, repo_name))
    } else {
        None
    }
}
#[tokio::main]
pub async fn fetch_all_available_templates() {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            println!("Failed to locate home directory. Exiting.");
            exit(1);
        }
    };
    // Construct the path to the auth.json file
    let auth_file_path: PathBuf = [home_dir.to_str().unwrap(), ".ginger-society", "auth.json"]
        .iter()
        .collect();

    // Read the token from the file
    let mut file = match File::open(&auth_file_path) {
        Ok(f) => f,
        Err(_) => {
            println!("Failed to open {}. Exiting.", auth_file_path.display());
            exit(1);
        }
    };
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        println!("Failed to read the auth.json file. Exiting.");
        exit(1);
    }

    let json: Value = match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(_) => {
            println!("Failed to parse auth.json as JSON. Exiting.");
            exit(1);
        }
    };

    let token = match json.get("API_TOKEN").and_then(|v| v.as_str()) {
        Some(t) => t.to_string(),
        None => {
            println!("API_TOKEN not found in auth.json. Exiting.");
            exit(1);
        }
    };

    let metadata_config = get_configuration(Some(token));

    match metadata_get_all_templates(&metadata_config).await {
        Ok(resp) => {
            let mut template_options: Vec<TemplateOption> = Vec::new();

            for template in resp {
                let option = TemplateOption {
                    short_name: template.short_name,
                    description: template.description,
                    repo_link: template.repo_link,
                    identifier: template.identifier,
                    id: template.id,
                };
                template_options.push(option);
            }

            match Select::new("Please select the template?", template_options).prompt() {
                Ok(choice) => {
                    new_project(choice.repo_link).await;
                }
                Err(err) => {
                    println!("{:?}", err);
                    println!("You cancelled :(. Existing!");
                    exit(1);
                }
            };
        }
        Err(_) => {
            println!("Error getting the templates")
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateOption {
    pub short_name: String,
    pub description: String,
    pub repo_link: String,
    pub identifier: String,
    pub id: i64,
}

impl fmt::Display for TemplateOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TemplateOption {{\n  Short Name: {},\n  Description: {},\n  Identifier: {}\n}}",
            self.short_name, self.description, self.identifier
        )
    }
}
