use mongodb::bson::oid::ObjectId;


/// Estrutura para relação entre usuários e grupos.
/// Deve ser usada apenas para relacionar o usuário a um grupo de permissões.
#[derive(Debug, Clone)]
pub struct UsersGroup {
    pub user: ObjectId,
    pub group: ObjectId,
}


/// Estrutura para relação entre micro serviços e permissões.
/// Deve ser usada apenas para relacionar o serviço com a permissão.
#[derive(Debug, Clone)]
pub struct MicroServicePermission {
    pub micro_service: ObjectId,
    pub permission: ObjectId,
}
