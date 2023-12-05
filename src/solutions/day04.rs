use ahash::AHashSet;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let cards = parse!(input);
    Ok(cards.iter().map(|c| c.points()).sum::<usize>().to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let cards = parse!(input);
    let mut card_counts = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let cnt = card_counts[i];
        let start = i + 1;
        let end = start + card.num_matches();
        for c in card_counts[start..end].iter_mut() {
            *c += cnt;
        }
    }

    Ok(card_counts.iter().sum::<usize>().to_string())
}

struct ScratchCard {
    #[allow(dead_code)]
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl ScratchCard {
    fn num_matches(&self) -> usize {
        let a = AHashSet::from_iter(self.winning_numbers.iter().copied());
        let b = AHashSet::from_iter(self.numbers.iter().copied());
        a.intersection(&b).count()
    }

    fn points(&self) -> usize {
        let count = self.num_matches();
        match count {
            0 => 0,
            _ => 1 << (count - 1),
        }
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<ScratchCard>> {
        let id = delimited(tuple((tag("Card"), space1)), uint, tag(":"));
        let num_list = || delimited(space0, separated_list1(space1, uint), space0);
        let card =
            tuple((id, num_list(), tag("|"), num_list())).map(|(id, win, _, have)| ScratchCard {
                id,
                winning_numbers: win,
                numbers: have,
            });
        let parser = many1(ws_line(card));
        ws_all_consuming(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "13")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "30")
    }
}
