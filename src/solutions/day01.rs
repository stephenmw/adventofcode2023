use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let lines = parse!(input);
    let ans: usize = lines.iter().map(|&x| extract_num(x)).sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let lines = parse!(input);

    let ans: usize = lines
        .iter()
        .map(|x| parser::parse_num_words(x).unwrap().1)
        .map(|xs| xs.first().unwrap() * 10 + xs.last().unwrap())
        .sum();

    Ok(ans.to_string())
}

fn extract_num(s: &str) -> usize {
    let digits: Vec<_> = s.chars().filter_map(|x| x.to_digit(10)).collect();
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    (first * 10 + last) as usize
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<&str>> {
        let l = ws_line(alphanumeric1);
        ws_all_consuming(many1(l))(input)
    }

    pub fn parse_num_words(input: &str) -> IResult<&str, Vec<usize>> {
        let mut num = alt::<_, _, (), _>((
            value(1, tag("one")),
            value(2, tag("two")),
            value(3, tag("three")),
            value(4, tag("four")),
            value(5, tag("five")),
            value(6, tag("six")),
            value(7, tag("seven")),
            value(8, tag("eight")),
            value(9, tag("nine")),
            one_of("0123456789").map(|x| x.to_digit(10).unwrap() as usize),
        ));

        let ret: Vec<_> = input
            .char_indices()
            .filter_map(|(i, _)| {
                let s = &input[i..];
                num(s).ok().map(|(_, x)| x)
            })
            .collect();

        Ok(("", ret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    const EXAMPLE_INPUT_2: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "142")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT_2).unwrap(), "281")
    }
}
