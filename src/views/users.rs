use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse};
use log::{error, warn, debug};
use bson::oid::ObjectId;

use crate::models::users::Login;
use crate::services::{
    MongoService,
    users::UserService,
};
use crate::views::payloads::LoginPayload;
use crate::tools::hasher;


/// Rota para excução do login dos usuários.
#[post("/login/")]
pub async fn login(payloads: web::Json<LoginPayload>) -> HttpResponse {
    // Inicia o serviço de consultas de usuário.
    let service = UserService::new(MongoService::new().await);
    // Captura, se existir, o usuário no banco de dados.
    let user = match service.get_by_username(&payloads.username).await{
        Some(data) => data,
        None => {
            warn!("User {} not found in database!", &payloads.username);
            return HttpResponse::NotFound()
                .json("Invalid username or password.");
        }
    };

    // Valida a senha do usuário.
    if !hasher::is_valid_password(&payloads.password, &user.password) {
        return HttpResponse::Unauthorized()
            .json("Invalid username or password.");
    }

    // Tenta gerar o token para o usuário.
    let token = match hasher::generate_jtw(&user) {
        Some(tk) => {
            debug!("Generate access token for user {}.", &user.username);
            tk
        },
        None => {
            error!("Can not generate token for user {}.", &user.username);
            return HttpResponse::InternalServerError()
                .json("Can not generate access token!");
        }
    };

    // Linka o token no usuário para identificações futuras.
    service.set_token(&user.username, &token).await;

    HttpResponse::Ok().json(Login{token})
}


/// Rota para capturar um único usuário.
#[get("/{user_id}/")]
pub async fn get(path: web::Path<(String, )>) -> HttpResponse {
    let loopkup = &path.into_inner().0;
    let service = UserService::new(MongoService::new().await);
    let user_id  = match ObjectId::parse_str(loopkup) {
        Ok(id) => id,
        Err(e) => {
            error!("Can not parse user ID {}, cause: {}", loopkup, e);
            return HttpResponse::BadRequest()
                .json("Invalid lookup content!");
        }
    };

    match service.get_by_id(&user_id).await {
        Some(user) => {
            debug!("Get user {} in lookup query.", &user.username);
            HttpResponse::Ok()
                .json(user)
        },
        None => {
            warn!("Not found user by ID {} on data base.", &user_id);
            HttpResponse::NotFound()
                .json("User not found.")
        }
    }
}
