use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    list.iter().sum::<i32>() as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;

    if sorted.len() % 2 == 1 {
        sorted[mid]
    } else {
        (sorted[mid - 1] + sorted[mid]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    list.iter()
        .fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        })
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0
}