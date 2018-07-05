// MIT License
//
// Copyright (c) 2017 Alexander Serebryakov
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! # Trie
//!
//! Trie is the library that implements the [trie](https://en.wikipedia.org/wiki/Trie).
//!
//! Trie is a generic data structure, written `Trie<T, U>` where `T` is node key type and `U` is a
//! value type.
//!
//! # Motivation
//!
//! Trie may be faster than other data structures in some cases.
//!
//! For example, `Trie` may be used as a replacement for `std::HashMap` in case of a dictionary
//! where the number of words in dictionary is significantly less than number of different words in
//! the input and matching probability is low.
//!
//! ## Important
//!
//! Search performance is highly dependent on the data stored in `Trie` and may be
//! as significantly faster than `std::HashMap` as significantly slower.
//!
//! # Usage
//!
//! ```rust
//! use gtrie::Trie;
//!
//! let mut t = Trie::new();
//!
//! t.insert("this".chars(), 1);
//! t.insert("trie".chars(), 2);
//! t.insert("contains".chars(), 3);
//! t.insert("a".chars(), 4);
//! t.insert("number".chars(), 5);
//! t.insert("of".chars(), 6);
//! t.insert("words".chars(), 7);
//!
//! assert_eq!(t.contains_key("number".chars()), true);
//! assert_eq!(t.contains_key("not_existing_key".chars()), false);
//! assert_eq!(t.get_value("words".chars()), Some(7));
//! assert_eq!(t.get_value("none".chars()), None);
//! ```

mod trie_node;
use std::clone::Clone;
use std::cmp::{Eq, Ord};
use trie_node::TrieNode;

/// Prefix tree object
pub struct Trie<T, U> {
    /// Root of the prefix tree
    nodes: Vec<TrieNode<T>>,
    values: Vec<U>,
}

impl<T: Eq + Ord + Clone, U: Clone> Trie<T, U> {
    /// Creates a new `Trie` object
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let t = Trie::<char, String>::new();
    /// ```
    pub fn new() -> Trie<T, U> {
        Trie {
            nodes: Vec::<TrieNode<T>>::new(),
            values: Vec::<U>::new(),
        }
    }

    /// Checks that trie is empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let t = Trie::<char, f64>::new();
    /// assert_eq!(t.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Adds a new key to the trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    ///
    /// t.insert(data, 42);
    /// assert_eq!(t.is_empty(), false);
    /// ```
    pub fn insert<V: Iterator<Item = T>>(&mut self, key: V, value: U) {
        let mut node_id = 0usize;

        if self.is_empty() {
            node_id = self.create_new_node();
        }

        for c in key {
            if let Some(id) = self.nodes[node_id].find(&c) {
                node_id = id;
            } else {
                let new_node_id = self.create_new_node();
                self.nodes[node_id].insert(&c, new_node_id);
                node_id = new_node_id;
            }
        }

        let value_id = match self.nodes[node_id].get_value() {
            Some(id) => {
                self.values[id] = value;
                id
            }
            None => {
                self.values.push(value);
                self.values.len() - 1
            }
        };

        self.nodes[node_id].set_value(value_id);
    }

    /// Clears the trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    ///
    /// t.insert(data, String::from("test"));
    /// t.clear();
    /// assert_eq!(t.is_empty(), true);
    /// ```
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.values.clear();
    }

    /// Looks for the key in trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    /// let another_data = "notintest".chars();
    ///
    /// t.insert(data.clone(), 42);
    ///
    /// assert_eq!(t.is_empty(), false);
    /// assert_eq!(t.contains_key(data), true);
    /// assert_eq!(t.contains_key(another_data), false);
    /// ```
    pub fn contains_key<V: Iterator<Item = T>>(&self, key: V) -> bool {
        if self.values.is_empty() && self.nodes.is_empty() {
            return false;
        }

        match self.find_node(key) {
            Some(node_id) => {
                if self.nodes[node_id].may_be_leaf() {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Gets the value from the tree by key
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    /// let another_data = "notintest".chars();
    ///
    /// t.insert(data.clone(), 42);
    ///
    /// assert_eq!(t.get_value(data), Some(42));
    /// assert_eq!(t.get_value(another_data), None);
    /// ```
    pub fn get_value<V: Iterator<Item = T>>(&self, key: V) -> Option<U> {
        match self.find_node(key) {
            // TODO: Properly handle the probable panic
            Some(node_id) => Some(self.values[self.nodes[node_id].get_value().unwrap()].clone()),
            None => None,
        }
    }

    /// Sets the value pointed by a key
    ///
    /// # Example
    ///
    /// ```rust
    /// use gtrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".chars();
    /// let another_data = "notintest".chars();
    ///
    /// t.insert(data.clone(), 42);
    ///
    /// assert_eq!(t.get_value(data.clone()), Some(42));
    /// assert_eq!(t.set_value(data.clone(), 43), Ok(()));
    /// assert_eq!(t.get_value(data), Some(43));
    /// assert_eq!(t.set_value(another_data, 39), Err(()));
    /// ```
    pub fn set_value<V: Iterator<Item = T>>(&mut self, key: V, value: U) -> Result<(), ()> {
        match self.find_node(key) {
            Some(node_id) => {
                let value_id = self.nodes[node_id].get_value().unwrap();
                self.values[value_id] = value;
                Ok(())
            }
            None => Err(()),
        }
    }

    /// Finds the node in the trie by the key
    ///
    /// Internal API
    fn find_node<V: Iterator<Item = T>>(&self, key: V) -> Option<usize> {
        if self.nodes.is_empty() {
            return None;
        }

        let mut node_id = 0usize;

        for c in key {
            match self.nodes[node_id].find(&c) {
                Some(child_id) => node_id = child_id,
                None => return None,
            }
        }

        Some(node_id)
    }

    /// Creates a new node and returns the node id
    ///
    /// Internal API
    fn create_new_node(&mut self) -> usize {
        self.nodes.push(TrieNode::new(None));
        self.nodes.len() - 1
    }
}
