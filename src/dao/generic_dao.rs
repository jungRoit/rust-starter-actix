use bson::doc;
use bson::oid::ObjectId;
use bson::Document;
use mongodb::{error::Error, results::InsertOneResult, Database};

pub async fn get_all(connection: Database, collection_name: &str) -> mongodb::Cursor {
    return connection
        .collection(collection_name)
        .find(None, None)
        .await
        .unwrap();
}

pub async fn find_by_id(
    connection: Database,
    collection_name: &str,
    id: &ObjectId,
) -> Result<Option<Document>, Error> {
    return connection
        .collection(collection_name)
        .find_one(
            doc! {
                "_id": id
            },
            None,
        )
        .await;
}

pub async fn add(
    connection: Database,
    collection_name: &str,
    document: Document,
) -> Result<InsertOneResult, Error> {
    return connection
        .collection(collection_name)
        .insert_one(document, None)
        .await;
}
