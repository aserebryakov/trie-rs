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


        pub fn find(&self, key: &char) -> Option<Rc<RefCell<TrieNode>>> {
            for child in &self.children {
                if (*child).borrow().key == *key {
                    return Some(child.clone());
                }
            }

            None
        }


        pub fn add(&mut self, key: &char) -> Rc<RefCell<TrieNode>> {
            match self.find(key) {
                None => {
                    let new_node = Rc::new(RefCell::new(TrieNode::new(key.clone())));
                    self.children.push(new_node.clone());
                    new_node
                }
                Some(node) => node.clone(),
            }
        }


        pub fn is_leaf(&self) -> bool {
            self.children.is_empty()
        }
    }


    pub struct Trie {
        root: Rc<RefCell<TrieNode>>,
    }


    impl Trie {
        pub fn new() -> Trie {
            Trie { root: Rc::new(RefCell::new(TrieNode::new('\0'))) }
        }


        pub fn empty(&self) -> bool {
            self.root.borrow().children.is_empty()
        }


        pub fn add(&mut self, key: &[char]) {
            let mut node = self.root.clone();
            for c in key {
                let next_node = (*node).borrow_mut().add(&c);
                node = next_node;
            }
        }


        pub fn clear(&mut self) {
            (*self.root).borrow_mut().children.clear();
        }


        pub fn has_key(&self, key: &[char]) -> bool {
            let mut node = self.root.clone();
            for c in key {
                let mut _next_node = node.clone();

                match node.borrow().find(c) {
                    Some(child) => _next_node = child,
                    None => return false,
                }

                node = _next_node;
            }

            if node.borrow().is_leaf() { true } else { false }
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
