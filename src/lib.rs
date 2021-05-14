use std::collections::HashMap;

pub struct Document {
    id: String,
    text: String,
}

impl Document {
    pub fn new(id: String, text: String) -> Self {
        Self { id, text }
    }
}

pub type InvertedIndex = HashMap<String, Vec<String>>;

pub fn index(document: Document) -> InvertedIndex {
    let mut index = HashMap::new();

    for word in document.text.split(" ") {
        index.insert(word.into(), vec![document.id.clone()]);
    }

    index
}

pub fn merge(mut index1: InvertedIndex, index2: InvertedIndex) -> InvertedIndex {
    for (k, mut v) in index2 {
        if index1.contains_key(&k) {
            let ids = index1.get_mut(&k).unwrap();
            ids.append(&mut v);
        } else {
            index1.insert(k.into(), v.into());
        }
    }

    index1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_inverted_index() {
        let document = Document::new("doc1".into(), "Hello, world!".into());
        let index1 = index(document);

        let mut expected = HashMap::new();
        expected.insert("Hello,".into(), vec!["doc1".into()]);
        expected.insert("world!".into(), vec!["doc1".into()]);

        assert_eq!(index1, expected);
    }

    #[test]
    fn simple_merge() {
        let doc1 = Document::new("doc1".into(), "Hello,".into());
        let doc2 = Document::new("doc2".into(), "world!".into());
        let index1 = index(doc1);
        let index2 = index(doc2);

        let merged = merge(index1, index2);

        let mut expected = HashMap::new();
        expected.insert("Hello,".into(), vec!["doc1".into()]);
        expected.insert("world!".into(), vec!["doc2".into()]);

        assert_eq!(merged, expected);
    }

    #[test]
    fn multi_merge() {
        let doc1 = Document::new("doc1".into(), "Hello,".into());
        let doc2 = Document::new("doc2".into(), "world!".into());
        let doc3 = Document::new("doc3".into(), "Hello, world!".into());
        let index1 = index(doc1);
        let index2 = index(doc2);
        let index3 = index(doc3);

        let merged = merge(index1, index2);
        let merged = merge(merged, index3);

        let mut expected = HashMap::new();
        expected.insert("Hello,".into(), vec!["doc1".into(), "doc3".into()]);
        expected.insert("world!".into(), vec!["doc2".into(), "doc3".into()]);

        assert_eq!(merged, expected);
    }
}
