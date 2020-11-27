use bson::Document;
use mongodb::{error::Error, results::InsertOneResult, Database};

pub fn get_all(connection: Database, collection_name: &str) -> mongodb::Cursor {
    return connection
        .collection(collection_name)
        .find(None, None)
        .unwrap();
}

pub fn add(
    connection: Database,
    collection_name: &str,
    document: Document,
) -> Result<InsertOneResult, Error> {
    return connection
        .collection(collection_name)
        .insert_one(document, None);
}
