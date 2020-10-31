use dotenv::dotenv;
use actix_web::{App, HttpServer};
use std::env;
mod db;

#[actix_rt::main]
async fn main()-> std::io::Result<()> {
    dotenv().ok();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL Environment variable is not set.");
    let database = db::create_connection();

    let user_collection = database.collection("User");
    
    HttpServer::new(move || {
        App::new()
        .data(AppState {})
    })
    .bind(server_url)?
    .run()
    .await
}
