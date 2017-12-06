pub mod trie {
    use std::rc::Rc;
    use std::cell::RefCell;


    pub struct Trie {
        root : TrieNode,
    }


    struct TrieNode {
        key : char,
        val : u64,
        children : Vec<Rc<RefCell<TrieNode>>>,
    }


    impl Trie {
        pub fn new() -> Trie {
            Trie {
                root : TrieNode {
                    key : '\0',
                    val : 0u64,
                    children : Vec::<Rc<RefCell<TrieNode>>>::new(),
                }
            }
        }


        pub fn empty(&self) -> bool {
            self.root.children.len() == 0
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
}
