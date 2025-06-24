use bson::de;
use bson::oid::ObjectId;
use log::{debug, info, error};
use mongodb::bson::{Document, doc};
use futures_util::stream::TryStreamExt;

use crate::services::MongoService;
use crate::models::users::{UserModel, UserSerialize};

pub struct UserService{
    service: MongoService,
} impl UserService {
    pub fn new(service: MongoService) -> Self {
        UserService {
            service,
        }
    }

    /// Captura um usuário pelo username.
    pub async fn get_by_username(&self, username: &String) -> Option<UserModel> {
        let data = self.service
            .user_model
            .find_one(doc!{"username": username})
            .await;

        match data {
            Ok(value) => {
                debug!("Get user {} in database.", username);
                value
            },
            Err(e) => {
                error!("Can not filter {} in users, cause {}", username, e);
                None
            }
        }
    }

    /// Captura o usuário pelo ID
    pub async fn get_by_id(&self, id: &ObjectId) -> Option<UserSerialize> {
        let data = self.service
            .user_serialize
            .find_one(doc!{"_id": id})
            .await;

        match data {
            Ok(user) => {
                debug!("Try to get user {} in database.", id);
                user
            },
            Err(e) => {
                error!("Can not filter {} in users, cause {}.", id, e);
                None
            }
        }
    }

    /// Altera o token na coleção de usuários.
    pub async fn set_token(&self, username: &String, token: &String) {
        let query = doc!{
            "username": username,
        };
        let update = doc!{
            "$set": {
                "token": token,
            }
        };

        match self.service
            .user_model
            .update_one(query, update)
            .await {
                Ok(_) => debug!("Updated user {}", username),
                Err(e) => error!("Can not update user {}, cause {}", username, e),
            };
    }
}
