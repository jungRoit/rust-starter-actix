use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NewUser {
    #[validate(required)]
    #[validate(length(min = 3, message = "First name is must be at least 3 characters long."))]
    pub first_name: Option<String>,

    #[validate(required)]
    #[validate(email(message = "Email must be a valid email address."))]
    pub email: Option<String>,

    pub last_name: Option<String>,

    #[validate(required)]
    #[validate(length(min = 8, max = 35, message = "Password must be 8-35 characters long."))]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EmailQuery {
    #[validate(required)]
    #[validate(email(message = "Email must be a valid email address."))]
    pub email: Option<String>,
}
