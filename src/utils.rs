use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetaData {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "prompts")]
    pub prompts: Vec<TemplatePrompt>,
}

impl MetaData {
    pub fn new(version: String, prompts: Vec<TemplatePrompt>) -> MetaData {
        MetaData { version, prompts }
    }
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

impl TemplatePrompt {
    pub fn new(r#type: String, field_key: String, prompt: String) -> TemplatePrompt {
        TemplatePrompt {
            r#type,
            field_key,
            prompt,
            enums: None,
            default: None,
            is_required: None,
            enable_path_exist_validator: None,
            callbacks: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Callback {
    #[serde(rename = "follow_up_prompts")]
    pub follow_up_prompts: Vec<FollowupPromot>,
    #[serde(rename = "condition")]
    pub condition: String,
}

impl Callback {
    pub fn new(follow_up_prompts: Vec<FollowupPromot>, condition: String) -> Callback {
        Callback {
            follow_up_prompts,
            condition,
        }
    }
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

impl FollowupPromot {
    pub fn new(r#type: String, field_key: String, prompt: String) -> FollowupPromot {
        FollowupPromot {
            r#type,
            field_key,
            prompt,
            enums: None,
            default: None,
            is_required: None,
            enable_path_exist_validator: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RenderedProject {
    #[serde(rename = "file_name")]
    pub file_name: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "parent_dir")]
    pub parent_dir: String,
}

impl RenderedProject {
    pub fn new(file_name: String, content: String, parent_dir: String) -> RenderedProject {
        RenderedProject {
            file_name,
            content,
            parent_dir,
        }
    }
}
