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
pub struct CreateTranslationResponse {
    #[serde(rename = "text")]
    pub text: String,
}

impl CreateTranslationResponse {
    pub fn new(text: String) -> CreateTranslationResponse {
        CreateTranslationResponse {
            text,
        }
    }
}

