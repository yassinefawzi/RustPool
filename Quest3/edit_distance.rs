pub fn edit_distance(source: &str, target: &str) -> usize {
    let rows = source.len();
    let cols = target.len();
    let s: Vec<char> = source.chars().collect();
    let t: Vec<char> = target.chars().collect();
    let mut table = vec![vec![0; cols + 1]; rows + 1];

    for i in 0..=rows {
        table[i][0] = i;
    }
    for j in 0..=cols {
        table[0][j] = j;
    }
    for i in 1..=rows {
        for j in 1..=cols {
            if s[i - 1] == t[j - 1] {
                table[i][j] = table[i - 1][j - 1];
            } else {
                table[i][j] = 1 + std::cmp::min(
                    table[i - 1][j - 1],
                    std::cmp::min(
                        table[i - 1][j], // deletion
                        table[i][j - 1],
                    ),
                );
            }
        }
    }

    return table[rows][cols];
}
