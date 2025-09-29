pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for j in 1..=i {
        let j = j as usize;
        let space = " ".repeat(j);
        let c = v.repeat(j);
        res.push(space + &c);
    }
    for j in (1..i).rev() {
        let j = j as usize;
        let space = " ".repeat(j);
        let c = v.repeat(j);
        res.push(space + &c);
    }
    res
}
