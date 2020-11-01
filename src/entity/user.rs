use serde::{Deserialize, Serialize};
use bson::{Document, doc, Bson};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub name: String,
  pub email: String,
  pub password: String
}

pub fn deserialize(user: &User) -> Document {
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

pub fn serialize(document: Document) -> User {
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