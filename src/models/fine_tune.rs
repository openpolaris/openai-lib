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
pub struct FineTune {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "updated_at")]
    pub updated_at: i32,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "fine_tuned_model", deserialize_with = "Option::deserialize")]
    pub fine_tuned_model: Option<String>,
    #[serde(rename = "organization_id")]
    pub organization_id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "hyperparams")]
    pub hyperparams: serde_json::Value,
    #[serde(rename = "training_files")]
    pub training_files: Vec<crate::models::OpenAiFile>,
    #[serde(rename = "validation_files")]
    pub validation_files: Vec<crate::models::OpenAiFile>,
    #[serde(rename = "result_files")]
    pub result_files: Vec<crate::models::OpenAiFile>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::FineTuneEvent>>,
}

impl FineTune {
    pub fn new(id: String, object: String, created_at: i32, updated_at: i32, model: String, fine_tuned_model: Option<String>, organization_id: String, status: String, hyperparams: serde_json::Value, training_files: Vec<crate::models::OpenAiFile>, validation_files: Vec<crate::models::OpenAiFile>, result_files: Vec<crate::models::OpenAiFile>) -> FineTune {
        FineTune {
            id,
            object,
            created_at,
            updated_at,
            model,
            fine_tuned_model,
            organization_id,
            status,
            hyperparams,
            training_files,
            validation_files,
            result_files,
            events: None,
        }
    }
}


