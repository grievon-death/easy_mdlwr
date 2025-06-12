use bson::de;
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
    pub async fn get_one(&self, username: &String) -> Option<UserModel> {
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
