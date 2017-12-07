extern crate trie;


#[cfg(test)]
mod tests {
    use trie::Trie;

    #[test]
    fn new_trie_is_is_empty() {
        assert_eq!(Trie::<char>::new().is_empty(), true);
    }


    #[test]
    fn add_word_to_trie() {
        let mut t = Trie::new();
        let data: Vec<char> = "test".chars().collect();
        t.add(&data[..]);

        assert_eq!(t.is_empty(), false);
    }


    #[test]
    fn has_key_test() {
        let mut t = Trie::new();
        let data: Vec<char> = "test".chars().collect();
        let another_data: Vec<char> = "notintest".chars().collect();

        t.add(&data[..]);

        assert_eq!(t.is_empty(), false);
        assert_eq!(t.has_key(&data[..]), true);
        assert_eq!(t.has_key(&another_data[..]), false);
    }


    #[test]
    fn clear_test() {
        let mut t = Trie::new();
        let data: Vec<char> = "test".chars().collect();

        t.add(&data[..]);

        assert_eq!(t.is_empty(), false);
        assert_eq!(t.has_key(&data[..]), true);

        t.clear();

        assert_eq!(t.is_empty(), true);
        assert_eq!(t.has_key(&data[..]), false);
    }
}
