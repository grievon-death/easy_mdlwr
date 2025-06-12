use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse};
use log::{error, debug};

use crate::models::users::Login;
use crate::services::{
    MongoService,
    users::UserService,
};
use crate::views::payloads::LoginPayload;
use crate::tools::hasher;


#[post("/login/")]
pub async fn login(payloads: web::Json<LoginPayload>) -> HttpResponse {
    let service = UserService::new(MongoService::new().await);
    let user = match service.get_one(&payloads.username).await{
        Some(data) => data,
        None => {
            return HttpResponse::NotFound().json("Invalid username or password.");
        }
    };

    if !hasher::is_valid_password(&payloads.password, &user.password) {
        return HttpResponse::Unauthorized().json("Invalid username or password.");
    }

    let token = String::from("adoijsaoidjoaisjdoij");
    service.set_token(&payloads.username, &token).await;

    HttpResponse::Ok().json(Login{token})
}
