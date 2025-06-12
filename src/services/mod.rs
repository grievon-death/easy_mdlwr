pub mod users;

use core::panic;

use log::{debug, info, error};
use mongodb::options::IndexOptions;
use mongodb::{
    Client,
    Collection,
    Database,
    IndexModel,
    bson::doc
};

use crate::settings::Settings;
use crate::models::{
    users::{UserModel, UserSerialize},
    permissions::{PermissionModel, PermissionSerialize},
    groups::{GroupModel, GroupSerialize},
    micro_services::{MicroServiceModel, MicroServiceSerialize},
    relationship::{UsersGroup, MicroServicePermission}
};


/// Esturura com as coleções de dados a serem usadas no serviço.
pub struct MongoService {
    pub user_model: Collection<UserModel>,
    pub user_serialize: Collection<UserSerialize>,
    pub permissions_model: Collection<PermissionModel>,
    pub permissions_serialize: Collection<PermissionSerialize>,
    pub groups_model: Collection<GroupModel>,
    pub groups_serialize: Collection<GroupSerialize>,
    pub micro_services_model: Collection<MicroServiceModel>,
    pub micro_services_serializer: Collection<MicroServiceSerialize>,
    pub users_groups: Collection<UsersGroup>,
    pub micro_services_permission: Collection<MicroServicePermission>,
    db: Database,
} impl MongoService {
    pub async fn new() -> Self {
        let settings = Settings::load();
        let client = Client::with_uri_str(settings.mongo_uri)
            .await
            .unwrap();
        let db = client.database(&settings.mongo_db);

        // Coleção de dados para os usuários.   
        let users = "users";
        // Coleção de dados para as permissões.
        let permissions = "permissions";
        // Coleção de dados para os grupos.
        let groups = "groups";
        // Coleção para armazenamento das rotas dos micro serviços.
        let micro_services= "micro_services";
        // Coleção para relacionamento de grupos e usuários.
        let users_groups = "users_groups";
        // Coleção para relacionamento de micro serviços e permissões.
        let micro_service_permission = "micro_service_permission";

        let user_model: Collection<UserModel> = db.collection(users);
        let user_serialize: Collection<UserSerialize> = db.collection(users);
        let permissions_model: Collection<PermissionModel> = db.collection(permissions);
        let permissions_serialize: Collection<PermissionSerialize> = db.collection(permissions);
        let groups_model: Collection<GroupModel> = db.collection(groups);
        let groups_serialize: Collection<GroupSerialize> = db.collection(groups);
        let micro_services_model: Collection<MicroServiceModel> = db.collection(micro_services);
        let micro_services_serializer: Collection<MicroServiceSerialize> = db.collection(micro_services);
        let users_groups: Collection<UsersGroup> = db.collection(users_groups);
        let micro_services_permission: Collection<MicroServicePermission> = db.collection(micro_service_permission);

        MongoService{
            user_model,
            user_serialize,
            permissions_model,
            permissions_serialize,
            groups_model,
            groups_serialize,
            micro_services_model,
            micro_services_serializer,
            users_groups,
            micro_services_permission,
            db,
        }
    }

    // Cria as coleções de dados e seus índices.
    pub async fn migrate(&self) {
        let mut collections: Vec<&str> = Vec::new();

        // Captura as os nomes das colections a serem migradas.
        collections.push(self.user_model.name());
        collections.push(self.permissions_model.name());
        collections.push(self.groups_model.name());
        collections.push(self.micro_services_model.name());
        collections.push(self.users_groups.name());
        collections.push(self.micro_services_permission.name());

        debug!("Verifying if collections already exists.");
        // Captura os nomes das coleções existentes.
        let collection_names = match self.db
            .list_collection_names()
            .await{
                Ok(names) => names,
                Err(e) => {
                    error!("Can not verify collections.");
                    panic!("Cause: {}", e);
                }
            };

        for name in collection_names.iter() {
            // Valida se a coleção já existe, se não passa pra próxima.
            if collections.iter().any(| coll | coll == name) {
                debug!("Collection {} already exists!", name);
                continue;
            }

            // Tenta criar a coleção de dados.
            match self.db
                .create_collection(name)
                .await{
                    Ok(_) => info!("Collection {} has been created!", name),
                    Err(e) => {
                        error!("Can not create collection {}.", name);
                        panic!("Cause: {}", e);
                    }
                };
        }

        // Opção para criar campos unique.
        let unique_opt = IndexOptions::builder()
            .unique(true)
            .build();

        // Cria os índices das coleções.
        let user_username_idx = IndexModel::builder().keys(doc!{
            "username": 1,
        }).options(unique_opt.clone()).build();
        // Coleção de usuários
        let user_login_idx = IndexModel::builder().keys(doc!{
            "username": 1,
            "is_active": -1,
        }).build();
        let user_loged_in_idx = IndexModel::builder().keys(doc!{
            "username": 1,
            "token": 1,
        }).build();
        let user_is_super_user_idx = IndexModel::builder().keys(doc!{
            "username": 1,
            "is_superuser": -1,
        }).build();
        let user_rest_filter_idx = IndexModel::builder().keys(doc!{
            "is_active": 1,
            "is_super_user": 1,
            "created_at": -1,
        }).build();

        match self.user_model
            .create_indexes(vec![
                user_username_idx,
                user_login_idx,
                user_loged_in_idx,
                user_is_super_user_idx, 
                user_rest_filter_idx,
            ])
            .await {
                Ok(_) => info!("Created indexes for user collection!"),
                Err(e) => error!("Can not create index for users collection.\nCause: {}", e),
            };

        // Coleção de permissões.
        let permission_idx = IndexModel::builder().keys(doc!{
            "name": 1
        }).options(unique_opt.clone()).build();

        match self.permissions_model
            .create_index(permission_idx)
            .await {
                Ok(_) => info!("Created indexes for permission collection!"),
                Err(e) => error!("Can not create index for permissions collection.\nCause: {}", e),
            };

        // Coleção de grupos.
        let groups_name_idx = IndexModel::builder().keys(doc!{
            "name": 1
        }).options(unique_opt.clone()).build();
        let groups_permission_idx = IndexModel::builder().keys(doc!{
            "name": 1,
            "permissions": 1,
        }).build();

        match self.groups_model
            .create_indexes(vec![groups_name_idx, groups_permission_idx])
            .await {
                Ok(_) => info!("Created indexes for gorups collection!"),
                Err(e) => error!("Can not create index for groups collection.\nCause: {}", e),
            };

        // Coleção de micro serviços.
        let micro_services_idx = IndexModel::builder().keys(doc!{
            "name": 1,
        }).options(unique_opt.clone()).build();

        match self.micro_services_model
            .create_index(micro_services_idx)
            .await {
                Ok(_) => info!("Created indexes for micro_services collection!"),
                Err(e) => error!("Can not create index for micro_services collection.\nCause: {}", e),
            };

        // Coleções de relacionamento.
        let users_group_user_idx = IndexModel::builder().keys(doc!{
            "user": 1,
        }).build();
        let user_groups_group_idx = IndexModel::builder().keys(doc!{
            "group": 1,
        }).build();

        match self.users_groups
            .create_indexes(vec![users_group_user_idx, user_groups_group_idx])
            .await {
                Ok(_) => info!("Create indexes for user_groups realationship"),
                Err(e) => error!("Can not create indexes for user_groups relationship.\nCause: {}", e),
            };

        let micro_service_permission_mc_idx = IndexModel::builder().keys(doc!{
            "micro_service": 1,
        }).build();
        let micro_service_permission_unq_idx = IndexModel::builder().keys(doc!{
            "micro_service": 1,
            "permission": 1,
        }).options(unique_opt.clone()).build();

        match self.micro_services_permission
            .create_indexes(vec![micro_service_permission_mc_idx, micro_service_permission_unq_idx])
            .await {
                Ok(_) => info!("Created indexes for micro_services_permission relationship!"),
                Err(e) => error!("Can not create indexes for micro_services_permission relationship.\nCause: {}", e),
            };
    }
}
