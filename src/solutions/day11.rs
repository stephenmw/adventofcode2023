use crate::grid::Point;
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let galaxies = parse!(input);
    let expanded = expand(&galaxies, 2);
    let ans = sum_of_distances(&expanded);
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let galaxies = parse!(input);
    let expanded = expand(&galaxies, 1_000_000);
    let ans = sum_of_distances(&expanded);
    Ok(ans.to_string())
}

fn expand(locs: &[Point], factor: usize) -> Vec<Point> {
    assert!(factor != 0);
    let f = factor - 1;

    let mut points = locs.to_vec();

    // expand x axis
    points.sort_by_key(|p| p.x);
    let mut last = 0;
    let mut expansion = 0;
    for p in points.iter_mut() {
        expansion += (p.x - last).saturating_sub(1) * f;
        last = p.x;
        p.x += expansion;
    }

    // expand y axis
    points.sort_by_key(|p| p.y);
    let mut last = 0;
    let mut expansion = 0;
    for p in points.iter_mut() {
        expansion += (p.y - last).saturating_sub(1) * f;
        last = p.y;
        p.y += expansion;
    }

    points
}

fn sum_of_distances(points: &[Point]) -> usize {
    let pairs = points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| std::iter::repeat(a).zip(&points[i + 1..]));

    pairs
        .map(|(a, b)| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
        .sum()
}

mod parser {
    use super::*;
    use crate::grid::Grid;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Point>> {
        let cell = alt((value(true, char('#')), value(false, char('.'))));
        let row = many1(cell);
        let grid = many1(ws_line(row)).map(|g| Grid::new(g));
        let galaxies = grid.map(|g| g.iter_points().filter(|&l| *g.get(l).unwrap()).collect());
        ws_all_consuming(galaxies)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "374")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "82000210")
    }
}
