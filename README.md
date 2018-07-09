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
significantly faster in the case of key mismatch but significantly slower in the case of
matching key.

```
$ cargo bench
test hash_map_massive_match                        ... bench:     150,127 ns/iter (+/- 12,986)
test hash_map_massive_mismatch_on_0                ... bench:      93,246 ns/iter (+/- 5,108)
test hash_map_massive_mismatch_on_0_one_symbol_key ... bench:      93,706 ns/iter (+/- 5,908)
test hash_map_match                                ... bench:          24 ns/iter (+/- 3)
test hash_map_mismatch                             ... bench:          20 ns/iter (+/- 0)
test trie_massive_match                            ... bench:     231,343 ns/iter (+/- 4,940)
test trie_massive_mismatch_on_0                    ... bench:      28,743 ns/iter (+/- 8,401)
test trie_massive_mismatch_on_1                    ... bench:      28,734 ns/iter (+/- 1,839)
test trie_massive_mismatch_on_2                    ... bench:      28,760 ns/iter (+/- 2,582)
test trie_massive_mismatch_on_3                    ... bench:      28,829 ns/iter (+/- 2,504)
test trie_match                                    ... bench:          10 ns/iter (+/- 1)
test trie_mismatch                                 ... bench:           5 ns/iter (+/- 0)
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

* Significant performance improvement due to switch to data oriented model

#### 0.3.0

* Significantly improved performance of the key mismatch case
* API is updated to be closer to `std::HashMap`

#### 0.2.1

* Benchmarks are improved

#### 0.2.0

* API is updated to be closer to `std::HashMap`

#### 0.1.1

* Basic trie implentation
