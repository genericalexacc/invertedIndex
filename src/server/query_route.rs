use super::super::in_memory_index::document_index::DocumentIndex;
use super::super::GLOBAL_INDEX_MAP;
use crate::in_memory_index::DocId;
use crate::InMemoryDocumentIndex;
use std::collections::HashMap;
use std::sync::MutexGuard;

use anyhow::{bail, Context, Result};

pub fn get_into_global_index(query: &str, index: &str) -> Result<Vec<DocId>> {
    match GLOBAL_INDEX_MAP.try_lock() {
        Ok(map) => {
            let result = query_index(map, index, query)?;
            Ok(result)
        }
        Err(_) => bail!("Can't get index mutex"),
    }
}

fn query_index(
    map: MutexGuard<HashMap<String, Box<InMemoryDocumentIndex>>>,
    index: &str,
    query: &str,
) -> Result<Vec<DocId>> {
    let index = map.get(index).context("Can't get index")?;
    let res = index.query_okapi(&query.to_string())?;
    Ok(res)
}
