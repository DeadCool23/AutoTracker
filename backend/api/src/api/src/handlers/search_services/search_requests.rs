use models::Document;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct SearchByFIORequest {
    #[schema(example = "name")]
    pub name: Option<String>,
    #[schema(example = "surname")]
    pub surname: Option<String>,
    #[schema(example = "lastname")]
    pub lastname: Option<String>,
}

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct SearchByPassportRequest {
    #[schema(example = json!({ 
        "serial": "1111", 
        "number": "111111" 
    }))]
    pub passport: Document,
}

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct SearchByGosNumRequest {
    #[schema(example = "А*23**99")]
    pub gos_num: String,
}

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct SearchByDateRequest {
    #[schema(example = "01.01.2025")]
    pub date: String,
}
