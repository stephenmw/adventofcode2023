use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let rows = parse!(input);
    let ans: usize = rows.iter().map(num_arrangements).sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut rows = parse!(input);

    for r in rows.iter_mut() {
        let cell_len = r.cells.len();
        let group_len = r.groups.len();
        for _ in 0..4 {
            r.cells.push(Cell::Unknown);
            r.cells.extend_from_within(..cell_len);
            r.groups.extend_from_within(..group_len);
        }
    }

    let ans: usize = rows.iter().map(num_arrangements).sum();
    Ok(ans.to_string())
}

fn num_arrangements(r: &Row) -> usize {
    let mut memo = vec![None; r.cells.len() * (r.groups.len() + 1)];
    num_arrangement_rec(r, 0, 0, &mut memo)
}

fn num_arrangement_rec(
    r: &Row,
    mut pos: usize,
    group_index: usize,
    memo: &mut Vec<Option<usize>>,
) -> usize {
    while r.cells.get(pos) == Some(&Cell::Operational) {
        pos += 1;
    }

    let Some(&c) = r.cells.get(pos) else {
        return if group_index == r.groups.len() { 1 } else { 0 };
    };

    let memo_index = pos * (r.groups.len() + 1) + group_index;
    if let Some(v) = memo[memo_index] {
        return v;
    }

    let assume_operational = if c == Cell::Unknown {
        num_arrangement_rec(r, pos + 1, group_index, memo)
    } else {
        0
    };

    let assume_damaged = 'block: {
        let Some(&group_size) = r.groups.get(group_index) else {
            break 'block 0;
        };
        let Some(next_springs) = r.cells.get(pos..pos + group_size) else {
            break 'block 0;
        };

        if next_springs.iter().all(|x| x != &Cell::Operational) {
            pos += group_size;
            if r.cells.get(pos) == Some(&Cell::Damaged) {
                0
            } else {
                num_arrangement_rec(r, pos + 1, group_index + 1, memo)
            }
        } else {
            0
        }
    };

    let ret = assume_operational + assume_damaged;
    memo[memo_index] = Some(ret);
    ret
}

#[derive(Clone, Debug)]
struct Row {
    cells: Vec<Cell>,
    groups: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Unknown,
    Operational,
    Damaged,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Row>> {
        let cell = alt((
            value(Cell::Unknown, char('?')),
            value(Cell::Operational, char('.')),
            value(Cell::Damaged, char('#')),
        ));
        let cells = many1(cell);
        let groups = separated_list1(tag(","), uint);
        let row =
            separated_pair(cells, space1, groups).map(|(cells, groups)| Row { cells, groups });
        let parser = many1(ws_line(row));
        ws_all_consuming(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "21")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "525152")
    }
}
