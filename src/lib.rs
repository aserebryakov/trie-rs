//! # Trie
//!
//! Trie is the library that implements the [trie](https://en.wikipedia.org/wiki/Trie).
//!
//! Trie is a generic data structure, written `Trie<T>`.

mod trie_node;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Eq;
use std::clone::Clone;
use trie_node::TrieNode;


/// Prefix tree object
pub struct Trie<T: Eq + Clone, U: Clone> {
    /// Root of the prefix tree
    root: Rc<RefCell<TrieNode<T, U>>>,
}


impl<T: Eq + Clone, U: Clone> Trie<T, U> {
    /// Creates a new `Trie` object
    ///
    /// # Example
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let t = Trie::<char, String>::new();
    /// ```
    pub fn new() -> Trie<T, U> {
        Trie { root: Rc::new(RefCell::new(TrieNode::new(None, None))) }
    }


    /// Checks that trie is empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let t = Trie::<char, f64>::new();
    /// assert_eq!(t.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.borrow().children.is_empty()
    }


    /// Adds a new key to the trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    ///
    /// t.add(data, 42);
    /// assert_eq!(t.is_empty(), false);
    /// ```
    pub fn add<V: Iterator<Item = T>>(&mut self, key: V, value: U) {
        let mut node = self.root.clone();
        for c in key {
            let next_node = (*node).borrow_mut().add(&c);
            node = next_node;
        }

        (*node).borrow_mut().set_value(value);
    }


    /// Clears the trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    ///
    /// t.add(data, String::from("test"));
    /// t.clear();
    /// assert_eq!(t.is_empty(), true);
    /// ```
    pub fn clear(&mut self) {
        (*self.root).borrow_mut().children.clear();
    }


    /// Looks for the key in trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    /// let another_data = "notintest".chars();
    ///
    /// t.add(data.clone(), 42);
    ///
    /// assert_eq!(t.is_empty(), false);
    /// assert_eq!(t.has_key(data), true);
    /// assert_eq!(t.has_key(another_data), false);
    /// ```
    pub fn has_key<V: Iterator<Item = T>>(&self, key: V) -> bool {
        match self.find_node(key) {
            Some(node) => {
                if node.borrow().may_be_leaf() {
                    true
                } else {
                    false
                }
            },
            None => false
        }
    }


    /// Gets the value from the tree by key
    ///
    /// # Example
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    /// let another_data = "notintest".chars();
    ///
    /// t.add(data.clone(), 42);
    ///
    /// assert_eq!(t.get_value(data), Some(42));
    /// assert_eq!(t.get_value(another_data), None);
    /// ```
    pub fn get_value<V: Iterator<Item = T>>(&self, key: V) -> Option<U> {
        match self.find_node(key) {
            Some(node) => {
                node.borrow().get_value()
            },
            None => None
        }
    }



    /// Finds the node in by the key
    ///
    /// Internal API
    fn find_node<V: Iterator<Item = T>>(&self, key: V) -> Option<Rc<RefCell<TrieNode<T, U>>>> {
        let mut node = self.root.clone();

        for c in key {
            let mut _next_node = node.clone();

            match node.borrow().find(&c) {
                Some(child) => _next_node = child,
                None => return None,
            }

            node = _next_node;
        }

        Some(node.clone())
    }
}
