use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let hists = parse!(input);
    let ans: i64 = hists
        .into_iter()
        .flat_map(|x| DerivativeIterator::new(x))
        .sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut hists = parse!(input);
    hists.iter_mut().for_each(|x| x.reverse());
    let ans: i64 = hists
        .into_iter()
        .flat_map(|x| DerivativeIterator::new(x))
        .sum();
    Ok(ans.to_string())
}

struct DerivativeIterator {
    nums: Vec<i64>,
}

impl DerivativeIterator {
    fn new(nums: Vec<i64>) -> Self {
        Self { nums }
    }
}

impl Iterator for DerivativeIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nums.iter().all(|&x| x == 0) {
            return None;
        }

        let ret = self.nums.last().copied();

        for i in 0..self.nums.len() - 1 {
            self.nums[i] = self.nums[i + 1] - self.nums[i];
        }
        self.nums.pop();

        ret
    }
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
        let line = separated_list1(space1, int);
        ws_all_consuming(many1(ws_line(line)))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "114")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "2")
    }
}
