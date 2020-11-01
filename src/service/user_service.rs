use serde::{Deserialize, Serialize};
use bson::{Document, doc, Bson};
// use bson::ordered::OrderedDocument;
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{error::Error, results::InsertOneResult, Database};

use crate::dao::generic_dao;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub name: String,
  pub email: String,
  pub password: String
}

#[derive(Clone)]
pub struct UserService {
  connection: Database,
}



fn user_to_document(user: &User) -> Document {
  let User {
    name,email,password
  } = user;
  doc! {
    "name":name,
    "email":email,
    "password":password
  }
}

fn build_user(
  name: String,
  email: String,
  password: String
) -> User {
  User {
    name,
    email,
    password
  }
}

fn document_to_user(document: Document) -> User {
  let mut _name = "".to_string();
  let mut _email = "".to_string();
  let mut _password = "".to_string();

  if let Some(&Bson::String(ref name)) = document.get("name") {
    _name = name.to_string();
  }

  if let Some(&Bson::String(ref email)) = document.get("email") {
    _email = email.to_string();
  }

  if let Some(&Bson::String(ref password)) = document.get("password") {
    _password = password.to_string();
  }

 return build_user(_name,_email,_password);

}

const COLLECTION_NAME: &str = "User";

impl UserService {
  pub fn new(connection: Database) -> UserService {
    UserService {connection}
  }
  pub fn add_user(&self, user: &User) -> Result<InsertOneResult, Error> {
    println!("reached controller");
    let document: Document = user_to_document(user);
    return generic_dao::add(self.connection.clone(),COLLECTION_NAME, document);
  }

  pub fn get_users(&self) -> Result<Vec<User>, Error> {
    let cursor = generic_dao::get_all(self.connection.clone(),COLLECTION_NAME);
    let mut data: Vec<User> = Vec::new();

    for result in cursor {
        if let Ok(item) = result {
            data.push(document_to_user(item))
        }
    }

    Ok(data)
  }

}
