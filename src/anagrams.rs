use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<_>>();
    sorted_word.sort();

    possible_anagrams.iter()
        .filter(|&possible_anagram| {
            let mut sorted_possible_anagram = possible_anagram.to_lowercase().chars().collect::<Vec<_>>();
            sorted_possible_anagram.sort();
            sorted_word == sorted_possible_anagram && word.to_lowercase() != *possible_anagram.to_lowercase()
        })
        .map(|&possible_anagram| possible_anagram)
        .collect()
}

pub fn main() {
    let word = "BANANA";
    let inputs = &["Banana"];
    let anagrams = anagrams_for(word, inputs);
    println!("Anagrams for the '{}' word: {:?}", word, anagrams);
}