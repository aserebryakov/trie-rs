pub mod trie {
    use std::rc::Rc;
    use std::cell::RefCell;


    struct TrieNode {
        key: char,
        children: Vec<Rc<RefCell<TrieNode>>>,
    }


    impl TrieNode {
        pub fn new(key: char) -> TrieNode {
            TrieNode {
                key,
                children: Vec::<Rc<RefCell<TrieNode>>>::new(),
            }
        }


        pub fn find(key: char) -> Option<Rc<RefCell<TrieNode>>> {
            None
        }


        pub fn add(key: char) {}
    }


    pub struct Trie {
        root: TrieNode,
    }


    impl Trie {
        pub fn new() -> Trie {
            Trie { root: TrieNode::new('\0') }
        }


        pub fn empty(&self) -> bool {
            self.root.children.len() == 0
        }


        pub fn add(&mut self, key: &[char]) {
            for c in key {}
        }


        pub fn clear(&mut self) {}


        pub fn has_key(&self, key: &[char]) -> bool {
            false
        }
    }
}



#[cfg(test)]
mod tests {
    use trie;

    #[test]
    fn new_trie_is_empty() {
        assert_eq!(trie::Trie::new().empty(), true);
    }


    #[test]
    fn add_word_to_trie() {
        let mut t = trie::Trie::new();
        let data: Vec<char> = "test".chars().collect();
        t.add(&data[..]);

        assert_eq!(t.empty(), false);
    }


    #[test]
    fn has_key_test() {
        let mut t = trie::Trie::new();
        let data: Vec<char> = "test".chars().collect();
        let another_data: Vec<char> = "nottest".chars().collect();

        t.add(&data[..]);

        assert_eq!(t.empty(), false);
        assert_eq!(t.has_key(&data[..]), true);
        assert_eq!(t.has_key(&another_data[..]), false);
    }


    #[test]
    fn clear_test() {
        let mut t = trie::Trie::new();
        let data: Vec<char> = "test".chars().collect();

        t.add(&data[..]);

        assert_eq!(t.empty(), false);
        assert_eq!(t.has_key(&data[..]), true);

        t.clear();

        assert_eq!(t.empty(), true);
        assert_eq!(t.has_key(&data[..]), false);
    }
}
