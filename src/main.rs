use dotenv::dotenv;
use actix_web::{middleware, App, HttpServer};
use std::env;

#[actix_rt::main]
async fn main()-> std::io::Result<()> {
    dotenv().ok();

    let server_url = env::var("SERVER_URL").expect("Server URL is not set in the ENV");

    env_logger::init();

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
    })
    .bind(server_url)?
    .run()
    .await
}
