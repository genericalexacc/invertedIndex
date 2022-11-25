use super::super::in_memory_index::document_index::DocumentIndex;
use super::super::utils::ResultT;
use super::super::GLOBAL_INDEX_MAP;
use crate::in_memory_index::DocId;
use crate::InMemoryDocumentIndex;
use std::collections::HashMap;
use std::sync::MutexGuard;

pub fn get_into_global_index(query: &str, index: &str) -> ResultT<Vec<DocId>> {
    match GLOBAL_INDEX_MAP.lock() {
        Ok(map) => {
            let result = query_index_handle_option(map, index, query)?;
            Ok(result)
        }
        Err(e) => Err(Box::from(format!("{}", e))),
    }
}

fn query_index_handle_option(
    map: MutexGuard<HashMap<String, Box<InMemoryDocumentIndex>>>,
    index: &str,
    query: &str,
) -> ResultT<Vec<DocId>> {
    match query_index(map, index, query) {
        Some(results) => Ok(results),
        None => Err(Box::from("Index not found")),
    }
}

fn query_index(
    map: MutexGuard<HashMap<String, Box<InMemoryDocumentIndex>>>,
    index: &str,
    query: &str,
) -> Option<Vec<DocId>> {
    let index = map.get(index)?;
    Some(index.query(query.to_string()))
}
