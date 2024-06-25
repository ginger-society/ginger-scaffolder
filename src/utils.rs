use serde::{Deserialize, Serialize};

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
