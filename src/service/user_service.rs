use bson::Document;
use futures::stream::StreamExt;
use mongodb::{error::Error, results::InsertOneResult, Database};

use crate::dao::generic_dao;
use crate::entity::user::{NewUser, User};

#[derive(Clone)]
pub struct UserService {
    connection: Database,
}

const COLLECTION_NAME: &str = "User";

impl UserService {
    pub fn new(connection: Database) -> UserService {
        UserService { connection }
    }

    pub async fn add_user(&self, user: &NewUser) -> Result<InsertOneResult, Error> {
        let document: Document = bson::to_document(user)?;
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
