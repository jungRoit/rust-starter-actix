use bson::doc;
use bson::oid::ObjectId;
use bson::Document;
use futures::stream::StreamExt;
use mongodb::{error::Error, results::InsertOneResult, Database};
use std::env;

use crate::dao::generic_dao;
use crate::entity::user::{NewUser, User};
use crate::utils::password;

#[derive(Clone)]
pub struct UserService {
    connection: Database,
}

const COLLECTION_NAME: &str = "User";

impl UserService {
    pub fn new(connection: Database) -> UserService {
        UserService { connection }
    }

    pub async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, Error> {
        let result = generic_dao::find_by_id(self.connection.clone(), COLLECTION_NAME, id).await?;

        match result {
            Some(user) => {
                return Ok(bson::from_document(user)?);
            }
            None => return Ok(None),
        }
    }

    pub async fn check_email_taken(&self, email: &String) -> bool {
        let mut cursor = generic_dao::filter(
            self.connection.clone(),
            COLLECTION_NAME,
            doc! {
                "email": email
            },
        )
        .await;

        while let Some(_) = cursor.next().await {
            return true;
        }

        return false;
    }

    pub async fn check_username_taken(&self, username: &String) -> bool {
        let mut cursor = generic_dao::filter(
            self.connection.clone(),
            COLLECTION_NAME,
            doc! {
                "username": username
            },
        )
        .await;

        while let Some(_) = cursor.next().await {
            return true;
        }

        return false;
    }

    pub async fn add_user(&self, new_user: &NewUser) -> Result<InsertOneResult, Error> {
        let mut user = (*new_user).clone();
        user.password = password::hash(&new_user.password, &env::var("PASSWORD_SALT").unwrap());
        let document: Document = bson::to_document(&user)?;
        return generic_dao::add(self.connection.clone(), COLLECTION_NAME, document).await;
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let mut cursor = generic_dao::get_all(self.connection.clone(), COLLECTION_NAME).await;
        let mut data: Vec<User> = Vec::new();
        while let Some(result) = cursor.next().await {
            data.push(bson::from_document(result?)?);
        }

        Ok(data)
    }
}
