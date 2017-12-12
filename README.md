# Trie

[![Build Status](https://travis-ci.org/aserebryakov/trie-rs.svg?branch=master)](https://travis-ci.org/aserebryakov/trie-rs)

Trie is the library that implements the [trie](https://en.wikipedia.org/wiki/Trie).

Trie is a generic data structure, written `Trie<T, U>` where `T` is node key type and `U` is a
value type.

# Motivation

Trie may be faster than other data structures in some cases.

For example, `Trie` may be used as a replacement for `HashMap` in case of a dictionary where
the number of words in dictionary is significantly less than number of different words in the
input.

# Usage

```rust
use trie::Trie;

let mut t = Trie::new();

t.add("this".chars(), 1);
t.add("trie".chars(), 2);
t.add("contains".chars(), 3);
t.add("a".chars(), 4);
t.add("number".chars(), 5);
t.add("of".chars(), 6);
t.add("words".chars(), 7);

assert_eq!(t.has_key("number".chars()), true);
assert_eq!(t.has_key("not_existing_key".chars()), false);
assert_eq!(t.get_value("words".chars()), Some(7));
assert_eq!(t.get_value("none".chars()), None);
```
