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
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{Eq, Ord};
use std::clone::Clone;


pub struct TrieNode<T, U> {
    pub value: Option<U>,
    pub children: Vec<(T, Rc<RefCell<TrieNode<T, U>>>)>,
}


impl<T: Eq + Ord + Clone, U: Clone> TrieNode<T, U> {
    pub fn new(value: Option<U>) -> TrieNode<T, U> {
        TrieNode {
            value,
            children: Vec::<(T, Rc<RefCell<TrieNode<T, U>>>)>::new(),
        }
    }


    pub fn find(&self, key: &T) -> Option<Rc<RefCell<TrieNode<T, U>>>> {
        if let Ok(idx) = self.children.binary_search_by(|x| x.0.cmp(key)) {
            return Some(self.children[idx].1.clone());
        }

        None
    }


    pub fn insert(&mut self, key: &T) -> Rc<RefCell<TrieNode<T, U>>> {
        match self.find(key) {
            None => {
                let new_node = Rc::new(RefCell::new(TrieNode::new(None)));
                self.children.push((key.clone(), new_node.clone()));
                self.children.sort_by(|a, b| a.0.cmp(&b.0));
                new_node
            }
            Some(node) => node.clone(),
        }
    }


    pub fn set_value(&mut self, value: U) {
        self.value = Some(value);
    }


    pub fn get_value(&self) -> Option<U> {
        self.value.clone()
    }


    pub fn may_be_leaf(&self) -> bool {
        self.value.is_some()
    }
}
