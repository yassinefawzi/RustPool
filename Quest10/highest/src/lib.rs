#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.iter().last().map(|u| *u)
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().map(|u| *u)
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut ordered_numbers = self.numbers.clone().to_owned();
        ordered_numbers.sort_by(|a, b| b.cmp(a));
        ordered_numbers.iter().take(3).map(|u| *u).collect()
    }
}