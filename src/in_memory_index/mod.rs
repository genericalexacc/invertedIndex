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
pub struct DocStat {
    doc_len: i64,
}

#[derive(Debug)]
pub struct Stats {
    doc_count: i64,
    average_len: f64,
    doc_stats: HashMap<String, DocStat>,
}

#[derive(Debug)]
pub struct InMemoryDocumentIndex {
    data: Arc<Mutex<IndexMap>>,
    stats: Arc<Mutex<Stats>>,
}

#[derive(Clone)]
pub struct NewDoc {
    pub doc_id: String,
    pub text: String,
}

#[macro_export]
macro_rules! unlock_mutex {
    ($a:expr) => {{
        let index_lock = $a.try_lock();
        if index_lock.is_err() {
            bail!("Can't get index mutex");
        }
        index_lock.unwrap()
    }};
}

// Concrete implementation of inverted index
impl DocumentIndex for InMemoryDocumentIndex {
    fn new() -> Self {
        Self {
            data: Arc::from(Mutex::from(HashMap::new())),
            stats: Arc::from(Mutex::from(Stats {
                doc_count: 0,
                average_len: 0.0,
                doc_stats: HashMap::new(),
            })),
        }
    }

    fn from_single_document(doc: NewDoc) -> (IndexMap, DocStat) {
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

        (
            data,
            DocStat {
                doc_len: doc.text.split_whitespace().count() as i64,
            },
        )
    }

    fn add_multiple_documents(&self, docs: Vec<NewDoc>) {
        docs.par_iter().for_each(|doc| {
            let (result, doc_stats) = Self::from_single_document(doc.clone());
            let dict = self.data.clone();
            let stats = self.stats.clone();
            Self::add_from_index_static((dict, stats), (result, doc_stats, doc.clone()));
            // Error swallowed here
        });
    }

    fn add_single_document(&self, doc: NewDoc) -> JoinHandle<Result<()>> {
        let dict = self.data.clone();
        let stats = self.stats.clone();

        thread::spawn(|| {
            let (index_map, doc_stat) = Self::from_single_document(doc.clone());
            Self::add_from_index_static((dict, stats), (index_map, doc_stat, doc))
        })
    }

    fn add_from_index_static(
        data: (Arc<Mutex<IndexMap>>, Arc<Mutex<Stats>>),
        other: (IndexMap, DocStat, NewDoc),
    ) -> Result<()> {
        let mut index = unlock_mutex!(data.0);
        let mut stats = unlock_mutex!(data.1);

        for (token, docs) in other.0 {
            index
                .entry(token.to_string())
                .or_insert_with(HashMap::new)
                .extend(docs)
        }

        update_document_stats(&mut stats, other.1, other.2);

        Ok(())
    }

    fn query(&self, query_text: String) -> Result<Vec<DocId>> {
        let index = unlock_mutex!(self.data);

        let occurences_per_doc: Vec<DocOccurence> = calc_occurences_per_doc(&index, query_text);

        Ok(std::collections::BinaryHeap::from(occurences_per_doc)
            .into_sorted_vec()
            .iter()
            .map(|a| a.doc_id.clone())
            .collect::<Vec<DocId>>())
    }

    fn query_okapi(&self, query_text_og: &str) -> Result<Vec<DocId>> {
        let index = unlock_mutex!(self.data);

        let query_text = query_text_og.to_lowercase();
        let occurences_per_doc = calc_occurences_per_doc(&index, query_text.clone());

        drop(index);

        let mut results = Vec::new();
        for doc_occurence in occurences_per_doc {
            let score = self.bm25(query_text.as_str(), &doc_occurence.doc_id)?;
            results.push((doc_occurence.clone(), score))
        }

        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        Ok(results.iter().map(|v| v.0.doc_id.clone()).collect())
    }

    fn bm25(&self, query: &str, doc_id: &str) -> Result<f64> {
        let index = unlock_mutex!(self.data);
        let stats = unlock_mutex!(self.stats);

        let mut scores = HashMap::new();
        for term in parse_query(query) {
            let tfs: HashMap<&String, i64> =
                vec![term].iter().fold(HashMap::new(), |acc, query_text| {
                    fold_result_word_occurences(&index, acc, query_text)
                });

            let tf = get_tf(&index, term.to_string(), &doc_id.to_string()) as f64;
            let idf =
                10_f64.log(stats.doc_count as f64 / tfs.values().map(|v| *v as f64).sum::<f64>());

            let doc_len = if let Some(doc_stats) = stats.doc_stats.get(doc_id) {
                doc_stats.doc_len
            } else {
                0
            };

            let k1 = 1.2;
            let b = 0.75;
            let score = idf * (tf * (k1 + 1.0))
                / (tf + k1 * (1.0 - b + b * (doc_len as f64 / stats.average_len)));

            scores.insert(term, score);
        }

        Ok(scores.values().sum())
    }
}

fn update_document_stats(stats: &mut Stats, doc_stat: DocStat, new_doc: NewDoc) {
    stats.average_len = (stats.average_len * stats.doc_count as f64 + doc_stat.doc_len as f64)
        / (stats.doc_count + 1) as f64;
    stats.doc_count += 1;
    stats.doc_stats.insert(
        new_doc.doc_id,
        DocStat {
            doc_len: doc_stat.doc_len,
        },
    );
}

fn calc_occurences_per_doc(
    index: &HashMap<String, HashMap<String, usize>>,
    query_text: String,
) -> Vec<DocOccurence> {
    query_text
        .to_lowercase()
        .split_whitespace()
        .filter(|word| !stop_words::STOP_WORDS_SET.contains(*word))
        .fold(HashMap::new(), |acc, query_text| {
            fold_result_word_occurences(index, acc, query_text)
        })
        .iter()
        .map(|(doc_id, incidence_count)| DocOccurence {
            doc_id: (*doc_id).clone(),
            incidence_count: *incidence_count as i64,
        })
        .collect()
}

fn get_tf(index: &HashMap<String, HashMap<String, usize>>, term: String, doc_id: &String) -> i32 {
    let doc_occurences = index.get(&term);
    match doc_occurences {
        Some(docs) => match docs.get(doc_id) {
            Some(count) => *count as i32,
            None => 0,
        },
        None => 0,
    }
}

type UHashMapWithRef<'a> = HashMap<&'a String, i64>;

fn fold_result_word_occurences<'a>(
    index: &'a IndexMap,
    mut acc: UHashMapWithRef<'a>,
    query_item: &str,
) -> UHashMapWithRef<'a> {
    if let Some(values) = index.get(query_item) {
        for (doc_id, count_occurences) in values.iter() {
            let entry = acc.entry(doc_id).or_insert(0);
            *entry += *count_occurences as i64;
        }
    }
    acc
}

fn parse_query(query: &str) -> Vec<&str> {
    let terms: Vec<&str> = query.split_whitespace().collect();
    terms
}

#[cfg(test)]
mod tests {
    use super::{document_index::DocumentIndex, DocStat, InMemoryDocumentIndex, NewDoc};
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

        let res = InMemoryDocumentIndex::add_from_index_static(
            (index_a_struct.data.clone(), index_a_struct.stats.clone()),
            (
                b,
                DocStat { doc_len: 0 },
                NewDoc {
                    doc_id: "b".to_string(),
                    text: "testb".to_string(),
                },
            ),
        );

        if res.is_err() {
            panic!("Couldn't add file")
        }

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
    fn query_test_pure_char() -> anyhow::Result<()> {
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

        let occurences = index.query("c".to_string())?;
        assert_eq!(occurences[0], "test".to_string());

        let occurences = index.query("b".to_string())?;
        assert_eq!(occurences[0], "testb".to_string());

        Ok(())
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
