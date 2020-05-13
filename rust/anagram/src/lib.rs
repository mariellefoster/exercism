use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_hm = word_to_hash(word.to_string());
    possible_anagrams.iter().filter_map(|ana_word| {
        let var = word_to_hash(ana_word.to_string());
        if word_hm == var && word.to_lowercase() != ana_word.to_lowercase() {
            Some(*ana_word)
        } else {
            None
        }
    }).collect()
}

fn word_to_hash(word: String) -> HashMap<char, usize> {
    let mut hm = HashMap::new();
    for chr in word.to_lowercase().chars() {
        match hm.get(&chr) {
            Some(count) => hm.insert(chr, count + 1),
            None => hm.insert(chr, 1),
        };
    }
    hm
}
