use crate::solutions::prelude::*;

use std::cmp;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let games = parse!(input);
    let ans: usize = games
        .iter()
        .map(|g| (g.id, g.max_combined_set()))
        .filter(|(_, s)| s.red <= 12 && s.blue <= 14 && s.green <= 13)
        .map(|(id, _)| id)
        .sum();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let games = parse!(input);
    let ans: usize = games.iter().map(|g| g.max_combined_set().power()).sum();
    Ok(ans.to_string())
}

#[derive(Clone, Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl Game {
    fn max_combined_set(&self) -> Set {
        self.sets
            .iter()
            .fold(Set::default(), |acc, x| acc.max_combined(x))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Set {
    red: usize,
    blue: usize,
    green: usize,
}

impl Set {
    fn max_combined(&self, other: &Set) -> Set {
        Set {
            red: cmp::max(self.red, other.red),
            blue: cmp::max(self.blue, other.blue),
            green: cmp::max(self.green, other.green),
        }
    }

    fn power(&self) -> usize {
        self.red * self.blue * self.green
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
        let color = alt((tag("red"), tag("blue"), tag("green")));
        let set_value = separated_pair(uint, space1, color);
        let set = separated_list1(tag(", "), set_value).map(|values| {
            let mut ret = Set::default();
            for (count, color) in values {
                match color {
                    "red" => ret.red = count,
                    "blue" => ret.blue = count,
                    "green" => ret.green = count,
                    _ => unreachable!(),
                }
            }
            ret
        });
        let sets = separated_list1(tag("; "), set);
        let game_id = delimited(tag("Game "), uint, tag(":"));
        let game = separated_pair(game_id, space1, sets).map(|(id, sets)| Game { id, sets });
        ws_all_consuming(many1(ws_line(game)))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "8")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "2286")
    }
}
