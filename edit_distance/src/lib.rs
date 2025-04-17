use std::mem;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.chars().count(); 
    let n = target.chars().count(); 

    let mut v0 = (0..=n).collect::<Vec<_>>();
    let mut v1 = vec![0; n + 1]; 

    for (i, sc) in source.chars().enumerate() {
        v1[0] = i + 1; 

        for (j, tc) in target.chars().enumerate() {
            let deletion_cost = v0[j + 1] + 1; 
            let insertion_cost = v1[j] + 1; 
            let substitution_cost = v0[j] + if sc == tc { 0 } else { 1 };
        
            v1[j + 1] = *[deletion_cost, insertion_cost, substitution_cost]
                .iter()
                .min()
                .unwrap();
        }

        mem::swap(&mut v0, &mut v1); 
    }

    v0[n] // 
}