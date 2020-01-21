use std::collections::HashSet;
use std::iter::FromIterator;

fn sort_lowercase_word(word: &str) -> String {
    let sorted_word = word.to_lowercase();
    let mut sorted_word = sorted_word.chars().collect::<Vec<char>>();
    sorted_word.sort();
    let sorted_word = sorted_word.iter().collect::<String>();
    sorted_word
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_lowercase_word = sort_lowercase_word(word);

    let anagrams = possible_anagrams.iter().filter(|&item| {
        item.to_lowercase() != lowercase_word && sort_lowercase_word(item) == sorted_lowercase_word
    });

    HashSet::from_iter(anagrams.cloned())
}

//////

mod test {
    use super::*;

    #[test]
    fn test_sort_lowercase_word() {
        println!("{}", sort_lowercase_word("ΑΒΓ")); // αβγ ?? interesting
        assert_eq!(sort_lowercase_word("ΑΒΓ"), sort_lowercase_word("γβα"));
    }
}
