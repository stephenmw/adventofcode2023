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

#[derive(Clone, Copy, Debug)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn num_winning_waits(&self) -> usize {
        // solution from https://www.wolframalpha.com/input?i=solve+for+w+in+w+*+%28t+-+w%29+%3D+d
        let t = self.time as f64;
        let d = self.record_distance as f64;
        let shared = (t * t - 4.0 * d).sqrt();
        let low_root = 0.5 * (t - shared);
        let high_root = 0.5 * (t + shared);

        let low = low_root.ceil() as usize + if low_root.fract() == 0.0 { 1 } else { 0 };
        let high = high_root.floor() as usize - if high_root.fract() == 0.0 { 1 } else { 0 };

        std::cmp::max(high - low + 1, 0)
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
