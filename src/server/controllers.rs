use super::DocumentIndex;
use crate::in_memory_index::NewDoc;
use crate::utils::get_url_content;
use crate::InMemoryDocumentIndex;
use crate::GLOBAL_INDEX_MAP;
use actix_web::web::Bytes;

use anyhow::{bail, Result};
use serde::Serialize;

pub async fn insert_index_result(
    index_name: &str,
    document_name: &str,
    bytes: Bytes,
) -> Result<()> {
    let text = String::from_utf8(bytes.to_vec())?;
    let index_lock = GLOBAL_INDEX_MAP.try_lock();

    if index_lock.is_err() {
        bail!("Can't get index mutex");
    }

    let index = index_lock.unwrap();

    index
        .get(&index_name.to_string())
        .unwrap()
        .add_single_document(NewDoc {
            doc_id: document_name.to_string(),
            text,
        })
        .join();

    Ok(())
}

#[derive(Serialize)]
struct DocId {
    url: Option<String>,
    name: String,
}

pub async fn insert_url(index_name: &str, bytes: Bytes) -> Result<()> {
    let url = String::from_utf8(bytes.to_vec())?;
    let text = get_url_content(&url).await?;

    let index_lock = GLOBAL_INDEX_MAP.try_lock();

    if index_lock.is_err() {
        bail!("Can't get index mutex");
    }

    let index = index_lock.unwrap();

    let doc_id_text = serde_json::to_string(&DocId {
        url: Some(url),
        name: text.0,
    })?;

    index
        .get(index_name)
        .unwrap()
        .add_single_document(NewDoc {
            doc_id: doc_id_text,
            text: text.1,
        })
        .join();

    Ok(())
}

pub async fn create_index_result(index_name: &str) -> Result<()> {
    let index_lock = GLOBAL_INDEX_MAP.try_lock();

    if index_lock.is_err() {
        bail!("Can't get index mutex");
    }

    let mut index = index_lock.unwrap();

    index.insert(
        index_name.to_string(),
        Box::from(InMemoryDocumentIndex::new()),
    );

    Ok(())
}
