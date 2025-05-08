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
        let mut numbers_copy = self.numbers.to_vec();
        numbers_copy.sort_unstable_by(|a, b| b.cmp(a));
        numbers_copy.truncate(3);
        numbers_copy
    }
}