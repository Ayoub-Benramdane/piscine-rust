#[derive(Debug, Clone)]
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
        if self.numbers.len() == 0 {
            return None;
        }
        Some(self.numbers[self.numbers.len() - 1])
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut res: Vec<u32> = self.numbers.to_owned();
        res.sort();
        if self.numbers.len() < 3 {
            res.reverse();
            return res;
        }
        res = res[res.len() - 3..].to_vec();
        res.reverse();
        res
    }
}
