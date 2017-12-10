use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Eq;
use std::clone::Clone;


pub struct TrieNode<T: Eq + Clone> {
    pub key: Option<T>,
    pub children: Vec<Rc<RefCell<TrieNode<T>>>>,
}


impl<T: Eq + Clone> TrieNode<T> {
    pub fn new(key: Option<T>) -> TrieNode<T> {
        TrieNode {
            key,
            children: Vec::<Rc<RefCell<TrieNode<T>>>>::new(),
        }
    }


    pub fn find(&self, key: &T) -> Option<Rc<RefCell<TrieNode<T>>>> {
        for child in &self.children {
            if let Some(ref k) = (*child).borrow().key {
                if *k == *key {
                    return Some(child.clone());
                }
            }
        }

        None
    }


    pub fn add(&mut self, key: &T) -> Rc<RefCell<TrieNode<T>>> {
        match self.find(key) {
            None => {
                let new_node = Rc::new(RefCell::new(TrieNode::new(Some(key.clone()))));
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
