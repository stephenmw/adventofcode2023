use anyhow::Ok;

use crate::solutions::prelude::*;

const JACK: u8 = 11;
const JOKER: u8 = 1;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let mut hands = parse!(input);
    hands.sort_by_cached_key(|x| (HandType::compute_type(x.cards), x.cards));
    let ans: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bet).sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut hands = parse!(input);
    for h in hands.iter_mut() {
        h.cards
            .iter_mut()
            .filter(|x| **x == JACK)
            .for_each(|x| *x = JOKER);
    }
    hands.sort_by_cached_key(|x| (HandType::compute_type(x.cards), x.cards));
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    fn compute_type(mut cards: [Card; 5]) -> Self {
        let mut seen = [0; 6];
        cards.sort_unstable();

        let num_jokers = cards.iter().filter(|x| **x == JOKER).count();

        let mut cur_card = 0;
        let mut count = 0;
        for &c in cards[num_jokers..].iter() {
            if c != cur_card {
                seen[count] += 1;
                count = 0;
            }

            cur_card = c;
            count += 1;
        }

        seen[count] += 1;

        let top = seen
            .iter()
            .enumerate()
            .rev()
            .find(|(_, x)| **x > 0)
            .map(|(i, _)| i)
            .unwrap_or(0);

        seen[top] -= 1;
        seen[top + num_jokers] += 1;

        if seen[5] > 0 {
            Self::FiveOfAKind
        } else if seen[4] > 0 {
            Self::FourOfAKind
        } else if seen[3] == 1 && seen[2] == 1 {
            Self::FullHouse
        } else if seen[3] > 0 {
            Self::ThreeOfAKind
        } else if seen[2] == 2 {
            Self::TwoPair
        } else if seen[2] == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

mod parser {
    use nom::multi::many_m_n;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
        let card = alt((
            one_of("23456789").map(|c| c.to_digit(10).unwrap() as u8),
            value(10, tag("T")),
            value(11, tag("J")),
            value(12, tag("Q")),
            value(13, tag("K")),
            value(14, tag("A")),
        ));

        let five_cards = many_m_n(5, 5, card).map(|s| {
            let mut a = [0; 5];
            a[..].copy_from_slice(&s);
            a
        });

        let hand = separated_pair(five_cards, space1, uint).map(|(c, b)| Hand::new(c, b));
        ws_all_consuming(many1(ws_line(hand)))(input)
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
