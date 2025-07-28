use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut hash = HashMap::new();
    for i in words  {
        *hash.entry(*i).or_insert(0) += 1 ;
    }
    hash
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.values().len()
}