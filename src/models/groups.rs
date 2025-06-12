use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime};

use crate::models::permissions::{PermissionModel, PermissionSerialize};


/// Estrutura para representação das ações do grupo de permissões.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actions {
    pub read: bool,
    pub write: bool,
    pub delete: bool,
}


/// Estrutura para manipulação no banco de dados.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupModel {
    pub _id: ObjectId,
    pub name: String,
    pub permissions: Vec<PermissionModel>,
    pub actions: Actions,
    pub created_at: DateTime,
}


/// Estrutura para serialização dos dados via API Rest.
#[derive(Debug, Clone, Deserialize)]
pub struct GroupSerialize {
    #[serde(serialize_with="bson::serde_with::serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub name: String,
    pub permissions: Vec<PermissionSerialize>,
    pub actions: Actions,
    #[serde(serialize_with="bson::serde_with::serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
}
