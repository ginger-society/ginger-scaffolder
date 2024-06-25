use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use serde::{Deserialize, Serialize};

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
