use crate::in_memory_index::DocId;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DocOccurence {
    pub doc_id: DocId,
    pub incidence_count: i64,
}

impl Ord for DocOccurence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.incidence_count.cmp(&self.incidence_count)
    }
}

impl PartialOrd for DocOccurence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::in_memory_index::DocOccurence;
    use std::cmp::Ordering;

    #[test]
    fn doc_occurence_comparaison() {
        let smaller = DocOccurence {
            doc_id: "b".to_string(),
            incidence_count: 1,
        };

        let bigger = DocOccurence {
            doc_id: "c".to_string(),
            incidence_count: 3,
        };

        let equal_to_smaller = DocOccurence {
            doc_id: "b".to_string(),
            incidence_count: 1,
        };

        assert!(smaller.cmp(&bigger) == Ordering::Greater);
        assert!(smaller.cmp(&equal_to_smaller) == Ordering::Equal);
    }
}
