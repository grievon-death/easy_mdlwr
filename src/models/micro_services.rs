use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime};


/// Objeto para manipulação dos cadastros de microserviços para manipulação no banco de dados.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MicroServiceModel {
    pub _id: ObjectId,
    pub host: String,
    pub base_route: String,
    pub created_at: DateTime,
}


/// Objeto de serialização do microserviços na API Rest.
#[derive(Debug, Clone, Deserialize)]
pub struct MicroServiceSerialize {
    #[serde(serialize_with="bson::serde_with::serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub host: String,
    pub base_route: String,
    #[serde(serialize_with="bson::serde_with::serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
}
