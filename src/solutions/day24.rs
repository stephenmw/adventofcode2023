use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let vectors = parse!(input);

    let ans = count_overlaps_test_area(&vectors, 200000000000000.0, 400000000000000.0);

    Ok(ans.to_string())
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    bail!("not yet implemented")
}

fn count_overlaps_test_area(vectors: &[Vector3], min_pos: f64, max_pos: f64) -> usize {
    let pairs = vectors
        .iter()
        .enumerate()
        .flat_map(|(i, a)| vectors[i + 1..].iter().map(move |b| (a, b)));

    pairs
        .filter(|(a, b)| {
            let Some((x, y)) = a.cross_xy(b) else {
                return false;
            };

            x >= min_pos && x <= max_pos && y >= min_pos && y <= max_pos
        })
        .count()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Point3 {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Vector3 {
    location: Point3,
    velocity: Point3,
}

impl Vector3 {
    // returns (x, y) of intersection ignoring z.
    fn cross_xy(&self, other: &Vector3) -> Option<(f64, f64)> {
        fn coefficients(stone: &Vector3) -> (f64, f64, f64) {
            // ax + by + c = 0
            let slope = stone.velocity.y as f64 / stone.velocity.x as f64;
            let a = slope;
            let b = -1.0;
            let c = stone.location.y as f64 - slope * stone.location.x as f64;
            (a, b, c)
        }

        let (a1, b1, c1) = coefficients(self);
        let (a2, b2, c2) = coefficients(other);

        // cross multiplication method
        let x = (b1 * c2 - b2 * c1) / (b2 * a1 - b1 * a2);
        let y = (c1 * a2 - c2 * a1) / (b2 * a1 - b1 * a2);

        // validate they cross at a time in the future
        let future_a = (x > self.location.x as f64) == (self.velocity.x > 0)
            && (y > self.location.y as f64) == (self.velocity.y > 0);
        let future_b = (x > other.location.x as f64) == (other.velocity.x > 0)
            && (y > other.location.y as f64) == (other.velocity.y > 0);

        if future_a && future_b {
            Some((x, y))
        } else {
            None
        }
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Vector3>> {
        let num = |input| delimited(space0, int, space0)(input);
        let point3 = move |input| {
            tuple((num, tag(","), num, tag(","), num))
                .map(|(x, _, y, _, z)| Point3 { x, y, z })
                .parse(input)
        };
        let vec3 = separated_pair(point3, tag("@"), point3)
            .map(|(location, velocity)| Vector3 { location, velocity });
        let vecs = many1(ws_line(vec3));
        ws_all_consuming(vecs)(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;

    const EXAMPLE_INPUT: &str = "
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    ";

    #[test]
    fn problem1_test() {
        let vecs = parser::parse(EXAMPLE_INPUT).finish().unwrap().1;
        assert_eq!(count_overlaps_test_area(&vecs, 7.0, 27.0), 2);
    }

    #[test]
    fn problem2_test() {
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "")
    }
}
