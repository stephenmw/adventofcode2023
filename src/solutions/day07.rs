use crate::solutions::prelude::*;

const JACK: u8 = 11;
const JOKER: u8 = 1;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let mut hands = parse!(input);
    hands.sort_by_cached_key(|x| (x.typ(), x.cards));
    let ans: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bet).sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut hands = parse!(input);

    // Replace all jacks with jokers
    for h in hands.iter_mut() {
        for card in h.cards.iter_mut() {
            if *card == JACK {
                *card = JOKER
            }
        }
    }

    hands.sort_by_cached_key(|x| (x.typ(), x.cards));
    let ans: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bet).sum();
    Ok(ans.to_string())
}

type Card = u8;

#[derive(Clone, Debug)]
struct Hand {
    cards: [Card; 5],
    bet: usize,
}

impl Hand {
    fn new(cards: [Card; 5], bet: usize) -> Self {
        Self { cards, bet }
    }

    // Generates a number that corresponds to the type of the hand. A larger
    // number means a higher ranked type.
    fn typ(&self) -> u16 {
        let mut cards = self.cards;
        let mut seen = [0; 6];
        cards.sort_unstable();

        // jokers come first when sorted.
        let num_jokers = cards.iter().take_while(|x| **x == JOKER).count();
        let mut cs = &cards[num_jokers..];

        while let Some(&next) = cs.first() {
            let cnt = cs.iter().take_while(|x| **x == next).count();
            seen[cnt] += 1;
            cs = &cs[cnt..];
        }

        let top = seen.iter().enumerate().rev().find(|(_, x)| **x > 0);
        match top {
            Some((i, _)) => {
                seen[i] -= 1;
                seen[i + num_jokers] += 1;
            }
            None => seen[num_jokers] += 1,
        }

        seen[1..].iter().rev().fold(0, |acc, &x| (acc << 3) + x)
    }
}

mod parser {
    use nom::multi::fill;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
        let hand = separated_pair(five_cards, space1, uint).map(|(c, b)| Hand::new(c, b));
        ws_all_consuming(many1(ws_line(hand)))(input)
    }

    fn five_cards(input: &str) -> IResult<&str, [Card; 5]> {
        let card = |input| {
            alt((
                one_of("23456789").map(|c| c.to_digit(10).unwrap() as u8),
                value(10, tag("T")),
                value(11, tag("J")),
                value(12, tag("Q")),
                value(13, tag("K")),
                value(14, tag("A")),
            ))(input)
        };

        let mut buf = [0; 5];
        let (rest, ()) = fill(card, &mut buf)(input)?;
        Ok((rest, buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "6440")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "5905")
    }
}
