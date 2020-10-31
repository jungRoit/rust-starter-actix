use mongodb::{options::ClientOptions, Client};
use std::env;

pub fn create_connection() -> mongodb::Database {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL Environment Variable is not set");
  let client_options = ClientOptions::parse(&database_url).unwrap();
  let client = Client::with_options(client_options).unwrap();

  
  let database_name = env::var("DB_NAME").expect("DB_NAME Environment Variable is not set");
  println!("database {}",database_name);
  let database = client.database(&database_name);
  return database;
}