#![feature(test)]
extern crate trie;
extern crate test;


use test::Bencher;


#[cfg(bench)]
mod benchmarks {
    #[bench]
    fn basic_benchmark(b: &mut Bencher) {
    }
}
