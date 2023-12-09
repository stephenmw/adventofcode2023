use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let hists = parse!(input);
    let ans: i64 = hists
        .into_iter()
        .map(|x| {
            compute_derivatives(x)
                .into_iter()
                .map(|xs| xs.last().copied().unwrap_or(0))
                .sum::<i64>()
        })
        .sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let hists = parse!(input);
    let ans: i64 = hists
        .into_iter()
        .map(|x| {
            compute_derivatives(x)
                .into_iter()
                .rev()
                .map(|xs| xs.first().copied().unwrap_or(0))
                .fold(0, |acc, x| x - acc)
        })
        .sum();
    Ok(ans.to_string())
}

fn compute_derivatives(nums: Vec<i64>) -> Vec<Vec<i64>> {
    fn next_line(nums: &[i64]) -> Vec<i64> {
        nums.windows(2).map(|xs| xs[1] - xs[0]).collect()
    }

    let mut derivatives = vec![nums];
    while derivatives.last().unwrap().iter().any(|&x| x != 0) {
        let n = next_line(derivatives.last().unwrap());
        derivatives.push(n);
    }

    derivatives
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
