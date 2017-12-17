#![feature(test)]
extern crate gtrie;
extern crate test;

use test::Bencher;
use std::collections::HashMap;


#[bench]
fn trie_match_bench(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();

    t.insert("test".chars(), String::from("test"));

    b.iter(|| {
        assert_eq!(t.contains("test".chars()), true);
    })
}


#[bench]
fn trie_mismatch_bench(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();

    t.insert("test".chars(), String::from("test"));

    b.iter(|| {
        assert_eq!(t.contains("tst".chars()), false);
    })
}


#[bench]
fn hash_map_match_bench(b: &mut Bencher) {
    let mut h = HashMap::new();
    let key = String::from("test");

    h.insert(key.clone(), true);

    b.iter(|| { h.get(&key); })
}


#[bench]
fn hash_map_mismatch_bench(b: &mut Bencher) {
    let mut h = HashMap::new();
    let key = String::from("test");
    let notkey = String::from("tst");

    h.insert(key, true);

    b.iter(|| { h.get(&notkey); })
}


#[bench]
fn trie_massive_match_bench(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();

    for i in 1..100 {
        let key = format!("the_{}_key_{}", i, i);
        t.insert(key.chars(), String::from("test"));
    }

    b.iter(|| for i in 1..100 {
        let key = format!("the_{}_key_{}", i, i);
        assert_eq!(t.contains(key.chars()), true);
    })
}


#[bench]
fn trie_massive_mismatch_bench(b: &mut Bencher) {
    let mut t = gtrie::Trie::new();

    for i in 1..100 {
        let key = format!("the_key_{}", i);
        t.insert(key.chars(), String::from("test"));
    }

    b.iter(|| for _ in 1..100 {
        let key = "the_key0".chars();
        assert_eq!(t.contains(key), false);
    })
}


#[bench]
fn vector_massive_match_bench(b: &mut Bencher) {
    let mut v = Vec::new();

    for i in 1..100 {
        v.push(String::from("the_key_") + i.to_string().as_str());
    }

    b.iter(|| for i in 1..100 {
        let key = String::from("the_key_") + i.to_string().as_str();

        for k in &v {
            if *k == key {
                break;
            }
        }
    })
}


#[bench]
fn hash_map_massive_match_bench(b: &mut Bencher) {
    let mut h = HashMap::new();

    for i in 1..100 {
        let key = format!("the_{}_key_{}", i, i);
        h.insert(key, true);
    }

    b.iter(|| for i in 1..100 {
        let key = format!("the_{}_key_{}", i, i);
        h.get(&key);
    })
}


#[bench]
fn hash_map_massive_mismatch_bench(b: &mut Bencher) {
    let mut h = HashMap::new();

    for i in 1..100 {
        h.insert(String::from("the_key_") + i.to_string().as_str(), true);
    }

    b.iter(|| for _ in 1..100 {
        let key = String::from("tttt");
        h.get(&key);
    })
}
