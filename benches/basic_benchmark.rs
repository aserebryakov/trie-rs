#![feature(test)]
extern crate gtrie;
extern crate test;

use test::Bencher;
use std::collections::HashMap;


fn generate_keys() -> Vec<String> {
    let mut keys = Vec::new();

    for i in 1..42 {
        for j in 1..42 {
            keys.push(format!("the_{}_key_{}", i, j));
        }
    }

    keys
}


#[bench]
fn trie_match(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();

    t.insert("test".chars(), String::from("test"));

    b.iter(|| {
        assert_eq!(t.contains("test".chars()), true);
    })
}


#[bench]
fn trie_mismatch(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();

    t.insert("test".chars(), String::from("test"));

    b.iter(|| {
        assert_eq!(t.contains("tst".chars()), false);
    })
}


#[bench]
fn hash_map_match(b: &mut Bencher) {
    let mut h = HashMap::new();
    let key = String::from("test");

    h.insert(key.clone(), true);

    b.iter(|| { h.get(&key); })
}


#[bench]
fn hash_map_mismatch(b: &mut Bencher) {
    let mut h = HashMap::new();
    let key = String::from("test");
    let notkey = String::from("tst");

    h.insert(key, true);

    b.iter(|| { h.get(&notkey); })
}


#[bench]
fn trie_massive_match(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();
    let keys = generate_keys();

    for key in &keys {
        t.insert(key.chars(), key.clone());
    }

    b.iter(|| for key in &keys {
        assert_eq!(t.contains(key.chars()), true);
    })
}


#[bench]
fn trie_massive_mismatch_on_0(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();
    let mismatching = format!("0he_{}", 21);
    let keys = generate_keys();

    for key in &keys {
        t.insert(key.chars(), key.clone());
    }

    b.iter(|| for _ in 0..keys.len() {
        assert_eq!(t.contains(mismatching.chars()), false);
    })
}


#[bench]
fn trie_massive_mismatch_on_1(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();
    let mismatching = format!("t0e_{}", 21);
    let keys = generate_keys();

    for key in &keys {
        t.insert(key.chars(), key.clone());
    }

    b.iter(|| for _ in 0..keys.len() {
        assert_eq!(t.contains(mismatching.chars()), false);
    })
}


#[bench]
fn trie_massive_mismatch_on_2(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();
    let mismatching = format!("th0_{}", 21);
    let keys = generate_keys();

    for key in &keys {
        t.insert(key.chars(), key.clone());
    }

    b.iter(|| for _ in 0..keys.len() {
        assert_eq!(t.contains(mismatching.chars()), false);
    })
}


#[bench]
fn trie_massive_mismatch_on_3(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();
    let mismatching = format!("the0{}", 21);
    let keys = generate_keys();

    for key in &keys {
        t.insert(key.chars(), key.clone());
    }

    b.iter(|| for _ in 0..keys.len() {
        assert_eq!(t.contains(mismatching.chars()), false);
    })
}


#[bench]
fn hash_map_massive_match(b: &mut Bencher) {
    let mut h = HashMap::new();
    let keys = generate_keys();

    for key in &keys {
        h.insert(key.clone(), true);
    }

    b.iter(|| for key in &keys {
        assert_eq!(h.contains_key(key), true);
    })
}


#[bench]
fn hash_map_massive_mismatch_on_0(b: &mut Bencher) {
    let mut h = HashMap::new();
    let mismatching = format!("0he_{}", 21);
    let keys = generate_keys();

    for key in &keys {
        h.insert(key.clone(), true);
    }

    b.iter(|| for _ in 0..keys.len() {
        assert_eq!(h.contains_key(&mismatching), false);
    })
}


#[bench]
fn hash_map_massive_mismatch_on_0_one_symbol_key(b: &mut Bencher) {
    let mut h = HashMap::new();
    let mismatching = format!("{}", 1);
    let keys = generate_keys();

    for key in &keys {
        h.insert(key.clone(), true);
    }

    b.iter(|| for _ in 0..keys.len() {
        assert_eq!(h.contains_key(&mismatching), false);
    })
}
