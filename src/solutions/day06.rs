use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let races = parse!(input);
    let ans: usize = races.iter().map(|r| r.num_winning_waits()).product();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let race_components = parse!(input);

    let t_string: String = race_components.iter().map(|x| x.time.to_string()).collect();
    let d_string: String = race_components
        .iter()
        .map(|x| x.record_distance.to_string())
        .collect();
    let race = Race {
        time: usize::from_str_radix(&t_string, 10).unwrap(),
        record_distance: usize::from_str_radix(&d_string, 10).unwrap(),
    };

    Ok(race.num_winning_waits().to_string())
}

struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn distance(&self, wait: usize) -> usize {
        wait * (self.time - wait)
    }

    fn num_winning_waits(&self) -> usize {
        (0..self.time)
            //.into_par_iter()
            .map(|wait| self.distance(wait))
            .filter(|&d| d > self.record_distance)
            .count()
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Race>> {
        let times = preceded(tuple((tag("Time:"), space0)), separated_list1(space1, uint));
        let distances = preceded(
            tuple((tag("Distance:"), space0)),
            separated_list1(space1, uint),
        );
        let parser = tuple((ws_line(times), ws_line(distances))).map(|(times, distances)| {
            times
                .into_iter()
                .zip(distances.into_iter())
                .map(|(t, d)| Race {
                    time: t,
                    record_distance: d,
                })
                .collect::<Vec<_>>()
        });
        ws_all_consuming(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        Time:      7  15   30
        Distance:  9  40  200
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "288")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "71503")
    }
}
