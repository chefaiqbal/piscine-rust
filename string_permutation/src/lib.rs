use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut map = HashMap::new();
    s1.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    s2.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);

    map.values().all(|&count| count == 0)
}