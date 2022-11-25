use log::trace;

pub type ResultT<T> = Result<T, Box<dyn std::error::Error>>;

use crate::in_memory_index::NewDoc;
pub fn get_files_for_test() -> Vec<NewDoc> {
    use std::fs;
    let paths = fs::read_dir("./Input/").unwrap();

    trace!("Got paths for test files");

    paths
        .filter_map(|x| x.ok())
        .map(|path| path.path().into_os_string().into_string().unwrap())
        .filter(|f| f.contains(".txt"))
        .map(|f_name| NewDoc {
            doc_id: f_name.clone(),
            text: fs::read_to_string(&f_name).unwrap(),
        })
        .collect()
}

pub fn testing_index() {
    trace!("Starting the building of a test index.");
    use super::DocumentIndex;
    use super::InMemoryDocumentIndex;
    use super::GLOBAL_INDEX_MAP;
    let mut index = GLOBAL_INDEX_MAP.lock().unwrap();

    let files = get_files_for_test();

    index.insert("test".to_string(), Box::from(InMemoryDocumentIndex::new()));

    trace!("Created test index");

    index
        .get(&"test".to_string())
        .unwrap()
        .add_multiple_documents(files);

    trace!("Added files to test index");
}
