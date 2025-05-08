pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() < 2 {
        return Vec::new();
    }

    (0..arr.len())
        .map(|i| {
            arr.iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .fold(1, |acc, (_, &x)| acc * x)
        })
        .collect()
}