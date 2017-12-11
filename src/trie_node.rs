use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Eq;
use std::clone::Clone;


pub struct TrieNode<T: Eq + Clone, U: Clone> {
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
