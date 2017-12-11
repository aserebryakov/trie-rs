extern crate trie;


#[cfg(test)]
mod tests {
    use trie::Trie;

    #[test]
    fn new_trie_is_is_empty() {
        assert_eq!(Trie::<char, String>::new().is_empty(), true);
    }


    #[test]
    fn add_word_to_trie() {
        let mut t = Trie::new();
        t.add("test".chars(), String::from("test"));

        assert_eq!(t.is_empty(), false);
    }


    #[test]
    fn has_key_test() {
        let mut t = Trie::new();
        let test = "test".chars();
        let tes = "tes".chars();
        let notintest = "notintest".chars();

        t.add(test.clone(), String::from("test"));

        assert_eq!(t.is_empty(), false);
        assert_eq!(t.has_key(test), true);
        assert_eq!(t.has_key(tes), false);
        assert_eq!(t.has_key(notintest), false);
    }


    #[test]
    fn has_key_sub_path_test() {
        let mut t = Trie::new();
        let test = "test".chars();
        let tes = "tes".chars();
        let notintest = "notintest".chars();

        t.add(test.clone(), String::from("test"));
        t.add(tes.clone(), String::from("tes"));

        assert_eq!(t.is_empty(), false);
        assert_eq!(t.has_key(test), true);
        assert_eq!(t.has_key(tes), true);
        assert_eq!(t.has_key(notintest), false);
    }


    #[test]
    fn clear_test() {
        let mut t = Trie::new();
        let data = "test".chars();

        t.add(data.clone(), String::from("test"));

        assert_eq!(t.is_empty(), false);
        assert_eq!(t.has_key(data.clone()), true);

        t.clear();

        assert_eq!(t.is_empty(), true);
        assert_eq!(t.has_key(data), false);
    }
}
