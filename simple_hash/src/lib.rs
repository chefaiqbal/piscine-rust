use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    words.iter().copied().fold(HashMap::new(), |mut hash, word| {
        *hash.entry(word).or_insert(0) += 1;
        hash
    })
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}