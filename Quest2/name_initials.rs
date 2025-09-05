pub fn initials(names: Vec<&str>) -> Vec<String> {
    return names
        .into_iter()
        .map(|name| name.split_once(' ').unwrap())
        .map(|(first, last)| {
            format!(
                "{}. {}.",
                first.chars().next().unwrap(),
                last.chars().next().unwrap()
            )
        })
        .collect();
}
