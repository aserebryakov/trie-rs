#![feature(test)]
extern crate trie;
extern crate test;

use test::Bencher;
use std::collections::HashMap;


#[bench]
fn trie_match_bench(b: &mut Bencher) {
    let mut t = trie::Trie::new();

    t.add("test".chars(), String::from("test"));

    b.iter(|| {
        assert_eq!(t.has_key("test".chars()), true);
    })
}


#[bench]
fn trie_mismatch_bench(b: &mut Bencher) {
    let mut t = trie::Trie::new();

    t.add("test".chars(), String::from("test"));

    b.iter(|| {
        assert_eq!(t.has_key("tst".chars()), false);
    })
}


#[bench]
fn hash_map_match_bench(b: &mut Bencher) {
    let mut h = HashMap::new();
    let key = String::from("test");

    h.insert(key.clone(), true);

    b.iter(|| {
        h.get(&key);
    })
}


#[bench]
fn hash_map_mismatch_bench(b: &mut Bencher) {
    let mut h = HashMap::new();
    let key = String::from("test");
    let notkey = String::from("tst");

    h.insert(key, true);

    b.iter(|| {
        h.get(&notkey);
    })
}


#[bench]
fn trie_massive_match_bench(b: &mut Bencher) {
    let mut t = trie::Trie::new();

    for i in 1..100 {
        let key = format!("the_key_{}", i);
        t.add(key.chars(), String::from("test"));
    }

    b.iter(|| {
        for i in 1..100 {
            let key = format!("the_key_{}", i);
            assert_eq!(t.has_key(key.chars()), true);
        }
    })
}


#[bench]
fn trie_massive_mismatch_bench(b: &mut Bencher) {
    let mut t = trie::Trie::new();

    for i in 1..100 {
        let key = format!("the_key_{}", i);
        t.add(key.chars(), String::from("test"));
    }

    b.iter(|| {
        for _ in 1..100 {
            let key = "tttt".chars();
            assert_eq!(t.has_key(key), false);
        }
    })
}


#[bench]
fn vector_massive_match_bench(b: &mut Bencher) {
    let mut v = Vec::new();

    for i in 1..100 {
        v.push(String::from("the_key_") + i.to_string().as_str());
    }

    b.iter(|| {
        for i in 1..100 {
            let key = String::from("the_key_") + i.to_string().as_str();

            for k in &v {
                if *k == key {
                    break;
                }
            }
        }
    })
}


#[bench]
fn hash_map_massive_match_bench(b: &mut Bencher) {
    let mut h = HashMap::new();

    for i in 1..100 {
        h.insert(String::from("the_key_") + i.to_string().as_str(), true);
    }

    b.iter(|| {
        for i in 1..100 {
            let key = String::from("the_key_") + i.to_string().as_str();
            h.get(&key);
        }
    })
}


#[bench]
fn hash_map_massive_mismatch_bench(b: &mut Bencher) {
    let mut h = HashMap::new();

    for i in 1..100 {
        h.insert(String::from("the_key_") + i.to_string().as_str(), true);
    }

    b.iter(|| {
        for _ in 1..100 {
            let key = String::from("tttt");
            h.get(&key);
        }
    })
}
