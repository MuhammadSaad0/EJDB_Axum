use axum::{debug_handler, Json};
use ejdb::{
    bson,
    bson::{oid::ObjectId, ordered::OrderedDocument},
    query::{Q, QH},
    Database,
};
use serde::Deserialize;
use serde_json::Value;
#[derive(Deserialize, Debug)]
pub struct InsertStruct {
    collection_name: String,
    field_name: String,
    field_value: Value,
}

pub async fn insert_into_collection(Json(data): Json<InsertStruct>) -> Json<String> {
    let db = Database::open("ejdb_axum.db").unwrap();
    let coll = db.collection(data.collection_name).unwrap();
    let field_name = data.field_name;
    let field_value = data.field_value;
    let doc = bson! {
        field_name => field_value
    };
    Json(coll.save(&doc).unwrap().to_string())
}

#[derive(Deserialize, Debug)]
pub struct GetAllStruct {
    collection_name: String,
    doc_id: String,
}

#[debug_handler]
pub async fn get_all_from_doc(Json(data): Json<GetAllStruct>) -> Json<Vec<OrderedDocument>> {
    let db = Database::open("ejdb_axum.db").unwrap();
    let coll = db.collection(data.collection_name).unwrap();
    let result = coll
        .query(Q.field("_id").eq(data.doc_id), QH.empty())
        .find()
        .unwrap();
    let mut ret_vec: Vec<OrderedDocument> = Vec::new();
    for (_x, i) in result.enumerate() {
        let x = i.unwrap();
        ret_vec.push(x);
    }
    Json(ret_vec)
}

#[derive(Deserialize, Debug)]
pub struct InsertFieldInDocStruct {
    collection_name: String,
    doc_id: String,
    field_name: String,
    field_value: Value,
}
pub async fn insert_field_in_doc(Json(data): Json<InsertFieldInDocStruct>) -> Json<String> {
    let db = Database::open("ejdb_axum.db").unwrap();
    let coll = db.collection(data.collection_name).unwrap();
    let _result = coll
        .query(
            Q.field("_id")
                .eq(data.doc_id)
                .set(data.field_name, data.field_value),
            QH.empty(),
        )
        .update()
        .unwrap();
    Json("Field added to doc".to_owned())
}

#[derive(Deserialize, Debug)]
pub struct DeleteDocInput {
    collection_name: String,
    doc_id: String,
}
#[debug_handler]
pub async fn delete_doc(Json(data): Json<DeleteDocInput>) -> Json<String> {
    let db = Database::open("ejdb_axum.db").unwrap();
    let coll = db.collection(data.collection_name).unwrap();
    let q = Q.field("_id").eq(data.doc_id).drop_all();
    coll.query(q, QH.empty()).update().unwrap();

    Json("Document dropped!".to_owned())
}

// Json structure should be like this:
// {
//     "collection_name": "Users",
//     "doc_id": "65019caf8526205200000000",
//     "fields_to_insert": {
//       "Height": 185,
//       "Color": "Brown",
//       "Hand": "Right"
//     }

//   }
#[debug_handler]
pub async fn insert_many_fields_in_doc(Json(data): Json<Value>) {
    let db = Database::open("ejdb_axum.db").unwrap();
    let coll = db
        .collection(data["collection_name"].as_str().unwrap())
        .unwrap();
    let fields_to_insert = bson! {data["fields_to_insert"].clone()};

    let _result = coll
        .query(
            Q.field("_id")
                .eq(data["doc_id"].clone())
                .set_many(fields_to_insert.as_document().unwrap().clone()),
            QH.empty(),
        )
        .update()
        .unwrap();
}
