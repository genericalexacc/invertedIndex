use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use anyhow::{bail, Result};

mod doc_occurence;
pub mod document_index;
mod stop_words;

use doc_occurence::DocOccurence;
use document_index::DocumentIndex;

pub type DocId = String;
pub type IndexMap = HashMap<String, HashMap<String, usize>>;

#[derive(Debug)]
pub struct InMemoryDocumentIndex {
    data: Arc<Mutex<IndexMap>>,
}

#[derive(Clone)]
pub struct NewDoc {
    pub doc_id: String,
    pub text: String,
}

// Concrete implementation of inverted index
impl DocumentIndex for InMemoryDocumentIndex {
    fn new() -> Self {
        Self {
            data: Arc::from(Mutex::from(HashMap::new())),
        }
    }

    fn from_single_document(doc: NewDoc) -> IndexMap {
        let data: IndexMap = doc
            .text
            .replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), "")
            .to_lowercase()
            .split_whitespace()
            .fold(HashMap::new(), |mut hmap, word| {
                let entry_word = hmap.entry(word.to_string()).or_insert_with(HashMap::new);
                let entry_doc = entry_word.entry(doc.doc_id.to_owned()).or_insert(0);
                *entry_doc += 1;
                hmap
            });

        data
    }

    fn add_multiple_documents(&self, docs: Vec<NewDoc>) {
        docs.par_iter().for_each(|doc| {
            let doc = doc.clone();
            let result = Self::from_single_document(doc);

            let dict = self.data.clone();
            Self::add_from_index_static(dict, result); // Error swallowed here
        });
    }

    fn add_single_document(&self, doc: NewDoc) -> JoinHandle<Result<()>> {
        let dict = self.data.clone();

        thread::spawn(|| {
            let result = Self::from_single_document(doc);
            Self::add_from_index_static(dict, result)
        })
    }

    fn add_from_index_static(dict: Arc<Mutex<IndexMap>>, other: IndexMap) -> Result<()> {
        let s = dict;
        let index_lock = s.try_lock();

        if index_lock.is_err() {
            bail!("Can't get index mutex");
        }

        let mut index = index_lock.unwrap();

        for (token, docs) in other {
            index
                .entry(token.to_string())
                .or_insert_with(HashMap::new)
                .extend(docs)
        }

        Ok(())
    }

    fn query(&self, query_text: String) -> Result<Vec<DocId>> {
        let index_arc = self.data.clone();
        let index_mutex_result = index_arc.try_lock();

        if index_mutex_result.is_err() {
            bail!("Can't get index mutex");
        }

        let index = index_mutex_result.unwrap();

        type UHashMapWithRef<'a> = HashMap<&'a String, usize>;

        fn fold_result_word_occurences<'a>(
            index: &'a IndexMap,
            mut acc: UHashMapWithRef<'a>,
            query_item: &str,
        ) -> UHashMapWithRef<'a> {
            if let Some(values) = index.get(query_item) {
                for (doc_id, count_occurences) in values.iter() {
                    let entry = acc.entry(doc_id).or_insert(0);
                    *entry += count_occurences;
                }
            }
            acc
        }

        let occurences_per_doc: Vec<DocOccurence> = query_text
            .to_lowercase()
            .split_whitespace()
            .filter(|word| !stop_words::STOP_WORDS_SET.contains(*word))
            .fold(HashMap::new(), |acc, query_text| {
                fold_result_word_occurences(&index, acc, query_text)
            })
            .iter()
            .map(|(doc_id, incidence_count)| DocOccurence {
                doc_id: (*doc_id).clone(),
                incidence_count: *incidence_count as i64,
            })
            .collect();

        Ok(std::collections::BinaryHeap::from(occurences_per_doc)
            .into_sorted_vec()
            .iter()
            .map(|a| a.doc_id.clone())
            .collect::<Vec<DocId>>())
    }
}

#[cfg(test)]
mod tests {
    use super::{document_index::DocumentIndex, InMemoryDocumentIndex, NewDoc};
    use std::fs;
    use std::time::Instant;

    #[test]
    fn index_document() {
        let index = InMemoryDocumentIndex::new();
        let handle = index.add_single_document(NewDoc {
            doc_id: "test".to_string(),
            text: "test text.".to_string(),
        });
        let _ = handle.join();

        for doc_id in index.data.try_lock().unwrap().get("test").unwrap().keys() {
            assert_eq!(*doc_id, "test".to_string());
        }

        for doc_id in index.data.try_lock().unwrap().get("test").unwrap().keys() {
            assert_eq!(*doc_id, "test".to_string());
        }
    }

    #[test]
    fn merge_indexes() {
        let index_a_struct = InMemoryDocumentIndex::new();
        {
            let index_arc = index_a_struct.data.clone();
            let mut index_a = index_arc.try_lock().unwrap();

            let mut entry_c = std::collections::HashMap::new();
            entry_c.insert("test".to_string(), 1);

            index_a.insert("c".to_string(), entry_c);
        }

        let mut entry_b = std::collections::HashMap::new();
        entry_b.insert("testb".to_string(), 1);

        let mut b = std::collections::HashMap::new();
        b.insert("b".to_string(), entry_b);

        InMemoryDocumentIndex::add_from_index_static(index_a_struct.data.clone(), b);

        let index_arc = index_a_struct.data;
        let index_a = index_arc.try_lock().unwrap();

        for doc_id in index_a.get("c").unwrap().keys() {
            assert_eq!(*doc_id, "test".to_string());
        }

        for doc_id in index_a.get("b").unwrap().keys() {
            assert_eq!(*doc_id, "testb".to_string());
        }
    }

    #[test]
    fn query_test_pure_char() {
        let index = InMemoryDocumentIndex::new();

        let h1 = index.add_single_document(NewDoc {
            doc_id: "test".to_string(),
            text: "c ".to_string().repeat(32),
        });

        let h2 = index.add_single_document(NewDoc {
            doc_id: "testb".to_string(),
            text: "b ".to_string().repeat(32),
        });

        let _ = h1.join();
        let _ = h2.join();

        let occurences = index.query("c".to_string());
        assert_eq!(occurences[0], "test".to_string());

        let occurences = index.query("b".to_string());
        assert_eq!(occurences[0], "testb".to_string());
    }

    #[test]
    fn query_test_ranking() {
        let index = InMemoryDocumentIndex::new();

        let h1 = index.add_single_document(NewDoc {
            doc_id: "test".to_string(),
            text: "c ".to_string().repeat(32) + "b",
        });

        let h2 = index.add_single_document(NewDoc {
            doc_id: "testb".to_string(),
            text: "b ".to_string().repeat(32) + "c",
        });

        let _ = h1.join();
        let _ = h2.join();

        let occurences = index.query("c".to_string());
        assert_eq!(occurences[0], "test".to_string());

        let occurences = index.query("b".to_string());
        assert_eq!(occurences[0], "testb".to_string());
    }

    fn get_files_for_test() -> Vec<NewDoc> {
        let paths = fs::read_dir("./Input/").unwrap();

        paths
            .filter_map(|x| x.ok())
            .map(|path| path.path().into_os_string().into_string().unwrap())
            .filter(|f| f.contains(".md"))
            .map(|f_name| NewDoc {
                doc_id: f_name.clone(),
                text: fs::read_to_string(&f_name).unwrap(),
            })
            .collect()
    }

    fn wrap_timer(lambda: Box<dyn FnOnce()>) {
        let now = Instant::now();

        lambda();

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }

    #[test]
    fn dict_from_files() -> Result<(), Box<dyn std::error::Error>> {
        let file_contents = get_files_for_test();

        let index = InMemoryDocumentIndex::new();
        wrap_timer(Box::from(move || {
            let mut vec_threads = Vec::new();
            file_contents.iter().for_each(|doc| {
                vec_threads.push(index.add_single_document(doc.clone()));
            });

            for thread in vec_threads {
                let _ = thread.join();
            }
        }));

        Ok(())
    }

    #[test]
    fn parallel_add() -> Result<(), Box<dyn std::error::Error>> {
        let file_contents = get_files_for_test();

        let index = InMemoryDocumentIndex::new();
        wrap_timer(Box::from(move || {
            index.add_multiple_documents(file_contents);
        }));

        Ok(())
    }

    #[test]
    fn query_timer() -> Result<(), Box<dyn std::error::Error>> {
        let file_contents = get_files_for_test();

        let index = InMemoryDocumentIndex::new();
        index.add_multiple_documents(file_contents);

        wrap_timer(Box::from(move || {
            index.query("Aloha".to_string());
        }));

        Ok(())
    }
}
