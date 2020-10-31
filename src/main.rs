use dotenv::dotenv;
use actix_web::{middleware, App, HttpServer};
use std::env;
mod db;

#[actix_rt::main]
async fn main()-> std::io::Result<()> {
    dotenv().ok();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL Environment variable is not set.");
    db::create_connection();
    env::set_var("RUST_LOG", "actix_web=debug, actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
    })
    .bind(server_url)?
    .run()
    .await
}
