
pub trait TrieInterface {
    fn get(&self, key: Vec<u8>) -> (Vec<u8>, bool);

    fn put(&mut self, key: Vec<u8>, value: Vec<u8>);

    fn del(&mut self, key: Vec<u8>, value: Vec<u8>) -> bool;
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
}
