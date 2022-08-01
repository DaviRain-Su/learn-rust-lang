
pub trait TrieInterface {
    fn get(&self, key: Vec<u8>) -> (Vec<u8>, bool);

    fn put(&mut self, key: Vec<u8>, value: Vec<u8>);

    fn del(&mut self, key: Vec<u8>, value: Vec<u8>) -> bool;

    // compute the merkle root hash for varifying data integrity
    fn hash(&self) -> Vec<u8>;
}

#[derive(Debug)]
struct Trie;

impl Trie {
    fn new() -> Self {
        Self
    }
}

impl TrieInterface for Trie {
    fn get(&self, key: Vec<u8>) -> (Vec<u8>, bool) {
        todo!()
    }

    fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        todo!()
    }

    fn del(&mut self, key: Vec<u8>, value: Vec<u8>) -> bool {
        todo!()
    }

    fn hash(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_not_exist_key() {
        let trie = Trie::new(); 
        let (_, found) = trie.get(b"notexist".to_vec());
        assert_eq!(false, found, "should get nothing if key does not exist");
    }

    #[test]
    fn test_get_exist_key() {
        let mut trie = Trie::new();
        trie.put(vec![1, 2, 3, 4], b"hello".to_vec());
        let (val, found) = trie.get(vec![1, 2, 3, 4]);
        assert_eq!(true, found, "should get value if key exist");
        assert_eq!(val, b"hello".to_vec(), "should get value if key");
    }

    #[test]
    fn test_get_update_value() {
        let mut trie = Trie::new();
        trie.put(vec![1, 2, 3, 4], b"hello".to_vec());
        trie.put(vec![1, 2, 3, 4], b"world".to_vec());
        let (val, found) = trie.get(vec![1, 2, 3, 4]);
        assert_eq!(true, found, "should get value if key exist");
        assert_eq!(val, b"world".to_vec(), "should get value if key");
    }

    #[test]
    fn test_data_integrity_diff() {
        let mut trie = Trie::new();
        let hash_0 = trie.hash();

        trie.put(vec![1, 2, 3, 4], b"hello".to_vec());
        let hash_1 = trie.hash();

        trie.put(vec![1, 2, 3, 4], b"world".to_vec());
        let hash_2 = trie.hash();

        trie.put(vec![1, 2, 3, 4], b"trie".to_vec());
        let hash_3 = trie.hash();

        assert_eq!(hash_0, hash_1, "should get a different hash if a new key-value pair was added or updated");

        assert_eq!(hash_1, hash_2, "should get a different hash if a new key-value pair was added or updated");

        assert_eq!(hash_2, hash_3, "should get a different hash if a new key-value pair was added or updated");

        
    }

    #[test]
    fn test_data_integrity_same() {
        let mut trie_1 = Trie::new();
    
        trie_1.put(vec![1, 2, 3, 4], b"hello".to_vec());
        trie_1.put(vec![1, 2, 3, 4], b"world".to_vec());


        let mut trie_2 = Trie::new();
    
        trie_2.put(vec![1, 2, 3, 4], b"hello".to_vec());
        trie_2.put(vec![1, 2, 3, 4], b"world".to_vec());

        assert_eq!(trie_1.hash(), trie_2.hash(), "should get the same if two have the identicial key-value pairs");
    }
}
