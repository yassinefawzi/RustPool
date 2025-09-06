use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
	return list.iter().sum::<i32>() as f64 / list.len() as f64;
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        return (sorted[mid - 1] + sorted[mid]) / 2;
	}
    return sorted[mid];
}


pub fn mode(list: &[i32]) -> i32 {
	let mut map = HashMap::new();
	for &num in list {
		*map.entry(num).or_insert(0) += 1;
	}
	return *map.iter().max_by_key(|key| key.1).unwrap().0;
}