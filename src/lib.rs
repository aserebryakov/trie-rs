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
use std::default::Default;
use trie_node::TrieNode;


/// Prefix tree object
pub struct Trie<T: Eq + Clone + Default> {
    /// Root of the prefix tree
    root: Rc<RefCell<TrieNode<T>>>,
}


impl<T: Eq + Clone + Default> Trie<T> {
    /// Creates a new `Trie` object
    ///
    /// # Example
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let t = Trie::<char>::new();
    /// ```
    pub fn new() -> Trie<T> {
        Trie { root: Rc::new(RefCell::new(TrieNode::new(T::default()))) }
    }


    /// Checks that trie is empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let t = Trie::<char>::new();
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
    /// let data: Vec<char> = "test".chars().collect();
    ///
    /// t.add(&data[..]);
    /// assert_eq!(t.is_empty(), false);
    /// ```
    pub fn add(&mut self, key: &[T]) {
        let mut node = self.root.clone();
        for c in key {
            let next_node = (*node).borrow_mut().add(&c);
            node = next_node;
        }
    }


    /// Clears the trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use trie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data: Vec<char> = "test".chars().collect();
    ///
    /// t.add(&data[..]);
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
    /// let data: Vec<char> = "test".chars().collect();
    /// let another_data: Vec<char> = "notintest".chars().collect();
    ///
    /// t.add(&data[..]);
    ///
    /// assert_eq!(t.is_empty(), false);
    /// assert_eq!(t.has_key(&data[..]), true);
    /// assert_eq!(t.has_key(&another_data[..]), false);
    /// ```
    pub fn has_key(&self, key: &[T]) -> bool {
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
