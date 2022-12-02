use crate::in_memory_index::DocId;
use actix_web::{http::header::ContentType, HttpResponse};
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

pub fn success_insert(x: String) -> actix_web::HttpResponse {
    let body = serde_json::to_string(&x).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

pub fn success_query(x: Vec<DocId>) -> actix_web::HttpResponse {
    let response = ResponseSearchIndex::new(200, Some(x), None);
    let body = serde_json::to_string(&response).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

pub fn err_query(x: anyhow::Error) -> actix_web::HttpResponse {
    let response = ResponseSearchIndex::new(500, None, Some(format!("{}", x)));
    let body = serde_json::to_string(&response).unwrap();

    HttpResponse::NotFound()
        .content_type(ContentType::json())
        .body(body)
}
