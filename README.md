# GTrie

[![Build Status](https://travis-ci.org/aserebryakov/trie-rs.svg?branch=master)](https://travis-ci.org/aserebryakov/trie-rs)

Trie is the library that implements the [trie](https://en.wikipedia.org/wiki/Trie).

Trie is a generic data structure, written `Trie<T, U>` where `T` is node key type and `U` is a
value type.


# Motivation

Trie may be faster than other data structures in some cases.

For example, `Trie` may be used as a replacement for `std::HashMap` in case of a dictionary where
the number of words in dictionary is significantly less than number of different words in the
input and matching probability is low.


# Usage

```rust
use gtrie::Trie;

let mut t = Trie::new();

t.insert("this".chars(), 1);
t.insert("trie".chars(), 2);
t.insert("contains".chars(), 3);
t.insert("a".chars(), 4);
t.insert("number".chars(), 5);
t.insert("of".chars(), 6);
t.insert("words".chars(), 7);

assert_eq!(t.contains_key("number".chars()), true);
assert_eq!(t.contains_key("not_existing_key".chars()), false);
assert_eq!(t.get_value("words".chars()), Some(7));
assert_eq!(t.get_value("none".chars()), None);
```

# Benchmarks

Benchmark `std::HashMap<String, String>` vs `gtrie::Trie` shows that `Trie` is
faster in the case of key mismatch but significantly slower in the case of
matching key.

```
$ cargo bench
test hash_map_massive_match                        ... bench:     157,555 ns/iter (+/- 15,801)
test hash_map_massive_mismatch_on_0                ... bench:      95,770 ns/iter (+/- 5,632)
test hash_map_massive_mismatch_on_0_one_symbol_key ... bench:      97,157 ns/iter (+/- 5,428)
test hash_map_match                                ... bench:          24 ns/iter (+/- 1)
test hash_map_mismatch                             ... bench:          21 ns/iter (+/- 1)
test trie_massive_match                            ... bench:     332,543 ns/iter (+/- 15,031)
test trie_massive_mismatch_on_0                    ... bench:      54,408 ns/iter (+/- 4,148)
test trie_massive_mismatch_on_1                    ... bench:      54,255 ns/iter (+/- 4,052)
test trie_massive_mismatch_on_2                    ... bench:      53,679 ns/iter (+/- 4,907)
test trie_massive_mismatch_on_3                    ... bench:      54,131 ns/iter (+/- 3,305)
test trie_match                                    ... bench:          42 ns/iter (+/- 2)
test trie_mismatch                                 ... bench:          17 ns/iter (+/- 0)
```

## Important

Search performance is highly dependent on the data stored in `Trie` and may be
as significantly faster than `std::HashMap` as significantly slower.


# Contribution

Source code and issues are hosted on GitHub:

    https://github.com/aserebryakov/trie-rs


# License

[MIT License](https://opensource.org/licenses/MIT)


# Changelog

#### 0.4.0

#### 0.3.0

* Significantly improved performance of the key mismatch case
* API is updated to be closer to `std::HashMap`

#### 0.2.1

* Benchmarks are improved

#### 0.2.0

* API is updated to be closer to `std::HashMap`

#### 0.1.1

* Basic trie implentation
