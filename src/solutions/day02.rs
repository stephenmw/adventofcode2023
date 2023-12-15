use crate::solutions::prelude::*;

use std::cmp;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let games = parse!(input);
    let ans: usize = games
        .iter()
        .map(|g| (g.id, g.max_combined_draw()))
        .filter(|(_, s)| s.red <= 12 && s.blue <= 14 && s.green <= 13)
        .map(|(id, _)| id)
        .sum();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let games = parse!(input);
    let ans: usize = games.iter().map(|g| g.max_combined_draw().power()).sum();
    Ok(ans.to_string())
}

#[derive(Clone, Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl Game {
    fn max_combined_draw(&self) -> Draw {
        self.draws
            .iter()
            .fold(Draw::default(), |acc, x| acc.max_combined(x))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Draw {
    red: usize,
    blue: usize,
    green: usize,
}

impl Draw {
    fn max_combined(&self, other: &Draw) -> Draw {
        Draw {
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
        let draws = separated_list1(tag("; "), draw);
        let game_id = delimited(tag("Game "), uint, tag(":"));
        let game = separated_pair(game_id, space1, draws).map(|(id, draws)| Game { id, draws });
        ws_all_consuming(many1(ws_line(game)))(input)
    }

    fn draw(input: &str) -> IResult<&str, Draw> {
        let color = alt((tag("red"), tag("blue"), tag("green")));
        let draw_value = separated_pair(uint, space1, color);
        let g = |mut acc: Draw, (count, color)| {
            match color {
                "red" => acc.red = count,
                "blue" => acc.blue = count,
                "green" => acc.green = count,
                _ => unreachable!(),
            };
            acc
        };

        fold_separated_list1(tag(", "), draw_value, Draw::default, g)(input)
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
