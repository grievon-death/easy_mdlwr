use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime};


/// Objeto para manipulação no banco de dados.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionModel {
    pub _id: ObjectId,
    pub name: String,
    pub created_at: DateTime,
}


/// Estrutura para serialização de dados na API Rest.
#[derive(Debug, Clone, Deserialize)]
pub struct PermissionSerialize {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub name: String,
    #[serde(serialize_with = "bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
}
