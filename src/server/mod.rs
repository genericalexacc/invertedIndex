mod query_route;
use crate::in_memory_index::DocId;
use query_route::get_into_global_index;

use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
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
