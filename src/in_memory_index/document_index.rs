use crate::in_memory_index::{DocId, DocStat, IndexMap, NewDoc, Stats};

use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

// Interface for indexer.
pub trait DocumentIndex {
    fn new() -> Self;
    fn from_single_document(doc: NewDoc) -> (IndexMap, super::DocStat);
    fn add_single_document(&self, doc: NewDoc) -> JoinHandle<Result<()>>;
    fn add_multiple_documents(&self, docs: Vec<NewDoc>);
    fn add_from_index_static(
        dict: (Arc<Mutex<IndexMap>>, Arc<Mutex<Stats>>),
        other: (IndexMap, DocStat, NewDoc),
    ) -> Result<()>;
    fn query(&self, query_text: String) -> Result<Vec<DocId>>;
    fn query_okapi(&self, query_text: &str) -> Result<Vec<DocId>>;
    fn bm25(&self, query: &str, document: &str) -> Result<f64>;
}
