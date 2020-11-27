use bson::Document;
// use bson::ordered::OrderedDocument;
// use mongodb::results::{DeleteResult, UpdateResult};
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
    pub fn add_user(&self, user: &User) -> Result<InsertOneResult, Error> {
        let document: Document = entity::user::deserialize(user);
        return generic_dao::add(self.connection.clone(), COLLECTION_NAME, document);
    }

    pub fn get_users(&self) -> Result<Vec<User>, Error> {
        let cursor = generic_dao::get_all(self.connection.clone(), COLLECTION_NAME);
        let mut data: Vec<User> = Vec::new();

        for result in cursor {
            if let Ok(item) = result {
                data.push(entity::user::serialize(item))
            }
        }

        Ok(data)
    }
}
