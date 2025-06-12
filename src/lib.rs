mod models;
mod settings;
mod services;
mod views;
mod tools;

use self::services::MongoService;


/// Inicia o log do serviço de consumo
/// conforme a variável de ambiente LOGLEVEL.
/// Senão, inicia com nível debug.
pub fn init_service_log() {
    // Importa a função que captura o nível de log.
    use env_logger::Env;

    // Captura 
    let level = Env::new()
        .filter_or("LOGLEVEL", "debug");
    // Inicia o log do serviço.
    env_logger::init_from_env(level);
}


/// Inicia o banco de dados do serviço.
async fn init_database() {
    // Instância o serviço do mongo,
    let service = MongoService::new().await;
    // Migra as coleções de dados.
    service.migrate().await;
}
