pub fn delete_and_backspace(s: &mut String) {
    let s_copy = s.clone();
    s.clear();

    let mut skip_next = 0;
    for c in s_copy.chars() {
        if c == '-' {
            s.pop();
        } else if c == '+' {
            skip_next += 1;
        } else {
            if skip_next == 0 {
                s.push(c);
            } else {
                skip_next -= 1;
            }
        }
    }
}

pub fn do_operations(v: &mut [String]) {
    v.iter_mut().for_each(|num| {
        let index = num.find(|c| c == '+' || c == '-').unwrap();
        let sign = num.chars().nth(index).unwrap();
        let (first, second) = num.split_at(index);
        let first = first.parse::<i32>().unwrap();
        let second = second[1..].parse::<i32>().unwrap();
        *num = if sign == '+' {
            first + second
        } else {
            first - second
        }
        .to_string();
    });
}
