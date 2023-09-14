pub mod collection;
pub mod doc;
use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::{get, post},
    Router,
};
use collection::{create_collection, delete_collection, get_all_docs_data_from_collection};
use doc::{delete_doc, get_all_from_doc, insert_field_in_doc, insert_into_collection};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/create_collection", post(create_collection))
        .route("/insert_doc", post(insert_into_collection))
        .route("/get_doc_by_id", get(get_all_from_doc))
        .route("/add_field_in_doc", post(insert_field_in_doc))
        .route("/delete_collection", post(delete_collection))
        .route("/delete_doc_by_id", post(delete_doc))
        .route(
            "/get_all_docs_in_collection",
            get(get_all_docs_data_from_collection),
        )
        .layer(cors);

    axum::Server::bind(&"0.0.0.0:3690".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
