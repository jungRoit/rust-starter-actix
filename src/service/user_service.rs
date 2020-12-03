use bson::Document;
use futures::stream::StreamExt;
use mongodb::{error::Error, results::InsertOneResult, Database};

use crate::dao::generic_dao;
use crate::entity;
use crate::entity::user::User;

#[derive(Clone)]
pub struct UserService {
    connection: Database,
}

const COLLECTION_NAME: &str = "User";

impl UserService {
    pub fn new(connection: Database) -> UserService {
        UserService { connection }
    }
    pub async fn add_user(&self, user: &User) -> Result<InsertOneResult, Error> {
        let document: Document = entity::user::deserialize(user);
        return generic_dao::add(self.connection.clone(), COLLECTION_NAME, document).await;
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let mut cursor = generic_dao::get_all(self.connection.clone(), COLLECTION_NAME).await;
        let mut data: Vec<User> = Vec::new();

        while let Some(result) = cursor.next().await {
            data.push(entity::user::serialize(result?));
        }

        Ok(data)
    }
}
