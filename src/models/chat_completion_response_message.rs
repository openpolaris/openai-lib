/*
 * OpenAI API
 *
 * APIs for sampling from and fine-tuning language models
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChatCompletionResponseMessage {
    /// The role of the author of this message.
    #[serde(rename = "role")]
    pub role: Role,
    /// The contents of the message
    #[serde(rename = "content")]
    pub content: String,
}

impl ChatCompletionResponseMessage {
    pub fn new(role: Role, content: String) -> ChatCompletionResponseMessage {
        ChatCompletionResponseMessage {
            role,
            content,
        }
    }
}

/// The role of the author of this message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl Default for Role {
    fn default() -> Role {
        Self::System
    }
}

