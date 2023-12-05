use crate::solutions::prelude::*;

use std::cmp;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (seeds, maps) = parse!(input);

    let ans = seeds
        .iter()
        .map(|&s| maps.iter().fold(s, |acc, m| m.apply(acc)))
        .min()
        .unwrap();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (seeds, maps) = parse!(input);
    let seed_ranges: Vec<Range> = seeds
        .chunks_exact(2)
        .map(|xs| Range::new(xs[0], xs[0] + xs[1]).unwrap())
        .collect();

    let mapped_ranges = maps.iter().fold(seed_ranges, |acc, m| m.apply_ranges(acc));
    let ans = mapped_ranges.iter().map(|r| r.start).min().unwrap();

    Ok(ans.to_string())
}

// A range from [start, end)
#[derive(Clone, Copy, Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Option<Self> {
        if end > start {
            Some(Range { start, end })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.end - self.start
    }

    fn split(&self, other: &Self) -> (Option<Self>, Option<Self>, Option<Self>) {
        (
            Range::new(self.start, other.start),
            Range::new(
                cmp::max(self.start, other.start),
                cmp::min(self.end, other.end),
            ),
            Range::new(other.end, self.end),
        )
    }

    fn translate(&self, amt: isize) -> Self {
        Range {
            start: (self.start as isize + amt) as usize,
            end: (self.end as isize + amt) as usize,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct MapRange {
    dst_start: usize,
    src_start: usize,
    len: usize,
}

impl MapRange {
    fn contains(&self, v: usize) -> bool {
        v >= self.src_start && v < (self.src_start + self.len)
    }

    fn apply(&self, v: usize) -> Option<usize> {
        if !self.contains(v) {
            return None;
        }

        Some(v - self.src_start + self.dst_start)
    }

    // Returns the overlap with the mapping applied and the residual before and
    // after the overlap
    fn apply_range(&self, r: Range) -> Option<(Range, Option<Range>, Option<Range>)> {
        let (b, o, a) = r.split(&Range::new(self.src_start, self.src_start + self.len).unwrap());
        let overlap = o?;
        Some((
            overlap.translate(self.dst_start as isize - self.src_start as isize),
            b,
            a,
        ))
    }
}

#[derive(Clone, Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn apply(&self, v: usize) -> usize {
        self.ranges
            .iter()
            .filter_map(|r| r.apply(v))
            .next()
            .unwrap_or(v)
    }

    fn apply_ranges(&self, r: Vec<Range>) -> Vec<Range> {
        let mut to_process = r;
        let mut tmp = Vec::new();
        let mut mapped = Vec::new();

        for mrange in &self.ranges {
            for src_range in to_process.drain(..) {
                let (done, a, b) = match mrange.apply_range(src_range) {
                    Some(x) => x,
                    None => {
                        tmp.push(src_range);
                        continue;
                    }
                };

                mapped.push(done);
                if let Some(x) = a {
                    tmp.push(x);
                }
                if let Some(x) = b {
                    tmp.push(x);
                }
            }

            std::mem::swap(&mut to_process, &mut tmp);
        }

        mapped.extend_from_slice(&to_process);
        mapped
    }
}

mod parser {
    use nom::character::complete::multispace1;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<usize>, Vec<Map>)> {
        let seeds = preceded(tag("seeds: "), separated_list1(space1, uint));
        let range =
            tuple((uint, space1, uint, space1, uint)).map(|(dst_start, _, src_start, _, len)| {
                MapRange {
                    dst_start,
                    src_start,
                    len,
                }
            });

        let map_header = ws_line(tuple((
            take_while(|c: char| !c.is_whitespace()),
            space1,
            tag("map:"),
        )));

        let map = preceded(map_header, many1(ws_line(range))).map(|ranges| Map { ranges });
        let maps = separated_list1(multispace1, map);
        let parser = separated_pair(ws_line(seeds), multispace1, maps);
        ws_all_consuming(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "35")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "46")
    }
}
