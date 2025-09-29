pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut ret = Vec::new();

    for i in 0..=arr.len() {
        let mut h = 0;
        for nu in 0..(arr.len() - i){
            h += arr[nu];
        }
        ret.push(h);
    }

    ret
}
