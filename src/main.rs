use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use std::env;

use service::user_service::UserService;
use service_manager::ServiceManager;

mod controllers;
mod dao;
mod db;
mod entity;
mod service;
mod service_manager;
mod utils;

struct AppState {
    service_manager: ServiceManager,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let server_url = env::var("HOST").expect("SERVER_URL Environment variable is not set.");
    let database = db::create_connection().await;

    HttpServer::new(move || {
        let user_service_worker = UserService::new(database.clone());
        let service_manager = ServiceManager::new(user_service_worker);
        App::new()
            .wrap(middleware::Logger::default())
            .data(AppState { service_manager })
            .configure(controllers::user_controller::init)
    })
    .bind(server_url)?
    .run()
    .await
}
