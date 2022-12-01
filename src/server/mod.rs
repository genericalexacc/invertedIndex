mod query_route;
use super::InMemoryDocumentIndex;
use crate::in_memory_index::DocId;
use query_route::get_into_global_index;

use super::DocumentIndex;
use super::GLOBAL_INDEX_MAP;
use crate::in_memory_index::NewDoc;

use actix_web::{get, http::header::ContentType, post, web, web::Bytes, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseSearchIndex {
    status: i16,
    response: Option<Vec<DocId>>,
    error: Option<String>,
}

impl ResponseSearchIndex {
    fn new(status: i16, response: Option<Vec<DocId>>, error: Option<String>) -> Self {
        Self {
            status,
            response,
            error,
        }
    }
}

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
    let index_name = &params.0;
    let document_name = &params.1;

    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => {
            let index = GLOBAL_INDEX_MAP.lock().unwrap();

            let join_result = index
                .get(&index_name.to_string())
                .unwrap()
                .add_single_document(NewDoc {
                    doc_id: document_name.clone(),
                    text,
                })
                .join();

            match join_result {
                Ok(_) => success_insert("Inserted".to_string()),
                Err(_) => err_query(Box::from("error".to_string())),
            }
        }
        Err(err) => err_query(Box::from(err)),
    }
}

#[post("/create/{index}")]
async fn create_index(params: web::Path<String>) -> impl Responder {
    let index_name = &params;

    let mut index = GLOBAL_INDEX_MAP.lock().unwrap();

    let result_insert = index.insert(
        index_name.to_string(),
        Box::from(InMemoryDocumentIndex::new()),
    );

    match result_insert {
        Some(_) => success_insert("Index erased and a new one was created.".to_string()),
        None => success_insert("Index created.".to_string()),
    }
}

fn success_insert(x: String) -> actix_web::HttpResponse {
    let body = serde_json::to_string(&x).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

fn success_query(x: Vec<DocId>) -> actix_web::HttpResponse {
    let response = ResponseSearchIndex::new(200, Some(x), None);
    let body = serde_json::to_string(&response).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

fn err_query(x: Box<dyn std::error::Error>) -> actix_web::HttpResponse {
    let response = ResponseSearchIndex::new(500, None, Some(format!("{}", x)));
    let body = serde_json::to_string(&response).unwrap();

    HttpResponse::NotFound()
        .content_type(ContentType::json())
        .body(body)
}
