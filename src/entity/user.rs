use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 3, message = "Name is must be at least 3 characters long."))]
    pub name: String,

    #[validate(email(message = "Email must be a valid email address."))]
    pub email: String,

    #[validate(length(min = 3, max = 20, message = "Username must be 3-20 characters long."))]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long."))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EmailQuery {
    #[validate(email(message = "Email must be a valid email address."))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UsernameQuery {
    #[validate(length(min = 3, max = 20, message = "Username must be 3-20 characters long."))]
    pub username: String,
}
