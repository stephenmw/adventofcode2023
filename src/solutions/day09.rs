use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let hists = parse!(input);
    let ans: i64 = hists
        .into_iter()
        .map(|x| DerivativeIterator::new(x, |hist| hist.last().copied().unwrap_or(0)).sum::<i64>())
        .sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let hists = parse!(input);
    let ans: i64 = hists
        .into_iter()
        .map(|x| {
            DerivativeIterator::new(x, |hist| hist.first().copied().unwrap_or(0))
                .fold((0, false), |(acc, neg), x| {
                    // negate every other number. Ex: 10 - 3 + 0 - 2
                    (acc + x * if neg { -1 } else { 1 }, !neg)
                })
                .0
        })
        .sum();
    Ok(ans.to_string())
}

struct DerivativeIterator<F: Fn(&[i64]) -> O, O> {
    nums: Vec<i64>,
    f: F,
}

impl<F: Fn(&[i64]) -> O, O> DerivativeIterator<F, O> {
    fn new(nums: Vec<i64>, f: F) -> Self {
        Self { nums, f }
    }
}

impl<F: Fn(&[i64]) -> O, O> Iterator for DerivativeIterator<F, O> {
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nums.iter().all(|&x| x == 0) {
            return None;
        }

        let ret = (self.f)(&self.nums);

        (0..self.nums.len() - 1).for_each(|i| self.nums[i] = self.nums[i + 1] - self.nums[i]);
        self.nums.pop();

        Some(ret)
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
