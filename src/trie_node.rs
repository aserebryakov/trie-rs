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
use std::cmp::Eq;
use std::clone::Clone;


pub struct TrieNode<T, U> {
    pub key: Option<T>,
    pub value: Option<U>,
    pub children: Vec<Rc<RefCell<TrieNode<T, U>>>>,
}


impl<T: Eq + Clone, U: Clone> TrieNode<T, U> {
    pub fn new(key: Option<T>, value: Option<U>) -> TrieNode<T, U> {
        TrieNode {
            key,
            value,
            children: Vec::<Rc<RefCell<TrieNode<T, U>>>>::new(),
        }
    }


    pub fn find(&self, key: &T) -> Option<Rc<RefCell<TrieNode<T, U>>>> {
        for child in &self.children {
            if let Some(ref k) = (*child).borrow().key {
                if *k == *key {
                    return Some(child.clone());
                }
            }
        }

        None
    }


    pub fn add(&mut self, key: &T) -> Rc<RefCell<TrieNode<T, U>>> {
        match self.find(key) {
            None => {
                let new_node = Rc::new(RefCell::new(TrieNode::new(Some(key.clone()), None)));
                self.children.push(new_node.clone());
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
