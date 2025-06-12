
use std::str;
use std::io::Read;
use std::iter::Iterator;
use std::collections::BTreeMap;

use log::error;
use sha2::{Sha512, Digest, Sha384};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};


use crate::models::users::UserModel;
use crate::settings::Settings;


/// Encripta uma senha.
pub fn hash_password(password: &str) -> Option<String>{
    // Instância um objeto de hash.
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let result = hasher
        .finalize();
    let result: Result<Vec<_>, _> = result
        .bytes()
        .collect();

    match result {
        Ok(value) =>{
            match str::from_utf8(&value) {
                Ok(data) => Some(data.to_owned()),
                Err(e) => {
                    error!("Can not make hash from password. Cause: {}", e);
                    None
                }
            }
        },
        Err(e) => {
            error!("Can not make hash from password. Cause: {}", e);
            None
        }
    }
}


/// Valida se a senha está correta.
pub fn is_valid_password(password: &str, hash: &str) -> bool {
    let checker = match hash_password(password) {
        Some(value) => value,
        None => {
            return false;
        }
    };

    if checker != hash {
        return false;
    }

    true
}


/// Gera um token JWT.
pub fn generate_jtw(user: &UserModel) -> Option<String> {
    // Captura informações de configuração.
    let settings = Settings::load();
    // Transforma o chave em bytes.
    let secret: &[u8] = &settings.jwt_secret_key.into_bytes();
    // Gera uma chave Hmac.
    let key: Hmac<Sha384> = match Hmac::new_from_slice(secret) {
        Ok(value) => value,
        Err(e) => {
            error!("Can not generate JWT. Cause: {}", e);
            return None;
        },
    };
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let mut claims = BTreeMap::new();
    claims.insert("username", user.username.clone());
    claims.insert("email", user.email.clone());
    let token = match Token::new(header, claims).sign_with_key(&key) {
        Ok(value) => value,
        Err(e) => {
            error!("Can not generate JWT. Cause: {}", e);
            return None;
        },
    };
    let token = token.as_str();

    Some(token.to_owned())
}


/// Valida e desencripta o token.
pub fn decode_jtw(token: String) -> Option<BTreeMap<String, String>>{
    // Captura informações de configuração.
    let settings = Settings::load();
    // Transforma o chave em bytes.
    let secret: &[u8] = &settings.jwt_secret_key.into_bytes();
    // Gera a chave Hmac.
    let key: Hmac<Sha384> = match Hmac::new_from_slice(secret) {
        Ok(value) => value,
        Err(e) => {
            error!("Can not decode JTW. Cause: {}", e);
            return None;
        }
    };
    let token: Token<Header, BTreeMap<String, String>, _> = match token.verify_with_key(&key) {
        Ok(value) => value,
        Err(e) => {
            error!("Can not decode JTW. Cause: {}", e);
            return None;
        }
    };
    let header = token.header();
    let claims = token.claims().clone();

    assert_eq!(header.algorithm, AlgorithmType::Hs384);

    Some(claims)
}
