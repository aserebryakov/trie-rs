use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Eq;
use std::clone::Clone;
use std::default::Default;


pub struct TrieNode<T: Eq + Clone + Default> {
    pub key: T,
    pub children: Vec<Rc<RefCell<TrieNode<T>>>>,
}


impl<T: Eq + Clone + Default> TrieNode<T> {
    pub fn new(key: T) -> TrieNode<T> {
        TrieNode {
            key,
            children: Vec::<Rc<RefCell<TrieNode<T>>>>::new(),
        }
    }


    pub fn find(&self, key: &T) -> Option<Rc<RefCell<TrieNode<T>>>> {
        for child in &self.children {
            if (*child).borrow().key == *key {
                return Some(child.clone());
            }
        }

        None
    }


    pub fn add(&mut self, key: &T) -> Rc<RefCell<TrieNode<T>>> {
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
