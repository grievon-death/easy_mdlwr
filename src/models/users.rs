use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime};


/// Objeto para manipulação de dados no banco de dados.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserModel {
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_mame: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_superuser: bool,
    pub token: Option<String>,
    pub created_at: DateTime,
    pub last_login: Option<DateTime>,
}


/// Objeto para serialização dos dados via API Rest.
#[derive(Debug, Clone, Deserialize)]
pub struct UserSerialize {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_superuser: bool,
    pub token: Option<String>,
    #[serde(serialize_with = "bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(serialize_with = "bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    pub last_login: Option<DateTime>,
}


/// Estrutura para serialização do token de login
#[derive(Debug, Serialize)]
pub struct Login {
    pub token: String,
}
