mod controllers;
mod query_route;
mod responses;

use actix_web::{get, post, web, web::Bytes, Responder};
use controllers::{create_index_result, insert_index_result, insert_url};
use query_route::get_into_global_index;
use responses::{err_query, success_insert, success_query};

use super::DocumentIndex;

#[get("/search/{index}/{query}")]
async fn search_index(params: web::Path<(String, String)>) -> impl Responder {
    let index = &params.0;
    let query = &params.1;

    match get_into_global_index(query, index) {
        Ok(found_documents) => success_query(found_documents),
        Err(err) => err_query(err),
    }
}

#[post("/insert/{index}/{document}")]
async fn insert_index(params: web::Path<(String, String)>, bytes: Bytes) -> impl Responder {
    let join_result = insert_index_result(params.0.as_str(), params.1.as_str(), bytes).await;
    match join_result {
        Ok(()) => success_insert("Inserted".to_string()),
        Err(err) => err_query(err),
    }
}

#[post("/insert_url/{index}")]
async fn insert_url_index(index_name: web::Path<String>, bytes: Bytes) -> impl Responder {
    match insert_url(index_name.as_str(), bytes).await {
        Ok(_) => success_insert("Inserted".to_string()),
        Err(err) => err_query(err),
    }
}

#[post("/create/{index}")]
async fn create_index(params: web::Path<String>) -> impl Responder {
    let result_insert = create_index_result(params.as_str());
    match result_insert.await {
        Ok(()) => success_insert("Index erased and a new one was created.".to_string()),
        Err(_) => success_insert("Index created.".to_string()),
    }
}
