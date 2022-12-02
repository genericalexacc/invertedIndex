#[macro_use]
extern crate lazy_static;

use actix_files as fs;
use actix_web::{App, HttpServer};

mod in_memory_index;
mod logging;
mod server;
mod utils;

pub use in_memory_index::document_index::DocumentIndex;
use in_memory_index::InMemoryDocumentIndex;

use std::collections::HashMap;
use std::sync::Mutex;

use server::*;

use utils::*;

lazy_static! {
    pub static ref GLOBAL_INDEX_MAP: Mutex<HashMap<String, Box<InMemoryDocumentIndex>>> =
        Mutex::from(HashMap::new());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging::init();
    testing_index();

    HttpServer::new(|| {
        App::new()
            // .default_service(web::resource("").route(web::get().to(react_index)))
            .service(search_index)
            .service(insert_index)
            .service(create_index)
            .service(insert_url_index)
            .service(fs::Files::new("/dashboard", "./static").show_files_listing())
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
