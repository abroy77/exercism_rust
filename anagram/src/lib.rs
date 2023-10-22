use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut success_set: HashSet<&str> = HashSet::with_capacity(possible_anagrams.len());
    // convert word to lowercase lazily
    let lowercase_word = lazy_to_lowercase(word);
    // make a target hashmap with the count for each unique letter in lowercase_word
    let target_map = to_hashmap(&lowercase_word);
    // loop through the possible anagrams
    for candidate in possible_anagrams {
        let lowercase_candidate = lazy_to_lowercase(&candidate);
        if lowercase_candidate == lowercase_word { // ignore if the words are the same
        } else if target_map == to_hashmap(&lowercase_candidate) {
            success_set.insert(&candidate); // add to set if hashmaps are eq
        }
    }
    success_set
}

fn to_hashmap(s: &str) -> HashMap<char, u8> {
    // return a hashmap with the counts for each unique char in s

    // make an empty hashmap
    let mut map = HashMap::with_capacity(s.len());
    // loop through s and update the map
    for c in s.chars().into_iter() {
        map.entry(c).and_modify(|x| *x += 1).or_insert(1u8);
    }
    map
}

fn lazy_to_lowercase<'a>(s: &'a str) -> String {
    // convert to lowercase only if an uppercase char is in s
    if s.chars().any(|x| x.is_uppercase()) {
        s.to_lowercase()
    } else {
        s.to_string()
    }
}
