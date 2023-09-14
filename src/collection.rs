use axum::{debug_handler, Json};
use ejdb::{
    bson,
    bson::{oid::ObjectId, ordered::OrderedDocument},
    query::{Q, QH},
    Database,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct CollectionInfo {
    collection_name: String,
}

#[debug_handler]
pub async fn create_collection(Json(data): Json<CollectionInfo>) -> Json<String> {
    let db = Database::open("ejdb_axum.db").unwrap();
    let _coll = db.collection(data.collection_name).unwrap();

    Json("Collection Created".to_owned())
}

#[derive(Deserialize, Debug)]
pub struct DeleteCollectionInput {
    collection_name: String,
    delete_all_data: bool,
}

pub async fn delete_collection(Json(data): Json<DeleteCollectionInput>) -> Json<String> {
    let db = Database::open("ejdb_axum.db").unwrap();
    db.drop_collection(data.collection_name, data.delete_all_data)
        .unwrap();
    Json("Collection dropped!".to_owned())
}

#[derive(Deserialize, Debug)]
pub struct GetAllDocsReqStruct {
    collection_name: String,
}

pub async fn get_all_docs_data_from_collection(
    Json(data): Json<GetAllDocsReqStruct>,
) -> Json<Vec<OrderedDocument>> {
    let db = Database::open("ejdb_axum.db").unwrap();
    let coll = db.collection(data.collection_name).unwrap();
    let result = coll.query(Q.empty(), QH.empty()).find().unwrap();
    let mut ret_vec: Vec<OrderedDocument> = Vec::new();
    for (_x, document) in result.enumerate() {
        ret_vec.push(document.unwrap());
    }
    Json(ret_vec)
}
