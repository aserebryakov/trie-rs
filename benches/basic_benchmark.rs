#![feature(test)]
extern crate gtrie;
extern crate test;

use test::Bencher;
use std::collections::HashMap;


fn generate_keys() -> Vec<String> {
    let mut keys = Vec::new();

    for i in 1..9 {
        for j in 1..9 {
            for k in 1..9 {
                for l in 1..9 {
                    keys.push(format!("{}{}{}{}", i, j, k, l));
                }
            }
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
    let mismatching = String::from("0999");
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
    let mismatching = String::from("9099");
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
    let mismatching = String::from("9909");
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
    let mismatching = String::from("9990");
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
        h.insert(key.clone(), key.clone());
    }

    b.iter(|| for key in &keys {
        assert_eq!(h.contains_key(key), true);
    })
}


#[bench]
fn hash_map_massive_mismatch_on_0(b: &mut Bencher) {
    let mut h = HashMap::new();
    let mismatching = String::from("0999");
    let keys = generate_keys();

    for key in &keys {
        h.insert(key.clone(), key.clone());
    }

    b.iter(|| for _ in 0..keys.len() {
        assert_eq!(h.contains_key(&mismatching), false);
    })
}


#[bench]
fn hash_map_massive_mismatch_on_0_one_symbol_key(b: &mut Bencher) {
    let mut h = HashMap::new();
    let mismatching = String::from("0");
    let keys = generate_keys();

    for key in &keys {
        h.insert(key.clone(), key.clone());
    }

    b.iter(|| for _ in 0..keys.len() {
        assert_eq!(h.contains_key(&mismatching), false);
    })
}
