use std::env;

use log::warn;


/// Estrutura que abrigará as configurações do programa.
pub struct Settings {
    pub mongo_uri: String,
    pub mongo_db: String,
} impl Settings {
    pub fn load() -> Self{
        let mongo_uri = match env::var("MONGO_URI") {
            Ok(value) => value,
            Err(_) => {
                let uri = "mongodb://127.0.0.1:27017";
                warn!("Empty var `MONGO_URI`, default value {}", uri);

                uri.to_string()
            }
        };
        let mongo_db = match env::var("MONGO_DB") {
            Ok(value) => value,
            Err(_) => {
                let db = "ease_mdlwr";
                warn!("Empty var `MONGO_DB`, default value {}", db);

                db.to_string()
            }
        };

        Settings {
            mongo_uri,
            mongo_db,
        }
    }
}
