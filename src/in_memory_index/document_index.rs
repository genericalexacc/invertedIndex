use crate::in_memory_index::{DocId, IndexMap, NewDoc};

use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

// Interface for indexer.
pub trait DocumentIndex {
    fn new() -> Self;
    fn from_single_document(doc: NewDoc) -> IndexMap;
    fn add_single_document(&self, doc: NewDoc) -> JoinHandle<Result<()>>;
    fn add_multiple_documents(&self, docs: Vec<NewDoc>);
    fn add_from_index_static(dict: Arc<Mutex<IndexMap>>, other: IndexMap) -> Result<()>;
    fn query(&self, query_text: String) -> Result<Vec<DocId>>;
}
