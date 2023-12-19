use std::cmp::Ordering;

use ahash::AHashMap;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (workflows, ratings) = parse!(input);
    let workflows: AHashMap<String, Workflow> =
        workflows.into_iter().map(|w| (w.name.clone(), w)).collect();

    fn filter_rating(workflows: &AHashMap<String, Workflow>, rating: &Rating) -> bool {
        let mut cur = "in";
        while cur != "A" && cur != "R" {
            cur = workflows.get(cur).unwrap().eval(rating)
        }

        cur == "A"
    }

    let ans: usize = ratings
        .iter()
        .filter(|r| filter_rating(&workflows, r))
        .map(|r| r.sum() as usize)
        .sum();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (workflows, _) = parse!(input);
    let workflows: AHashMap<String, Workflow> =
        workflows.into_iter().map(|w| (w.name.clone(), w)).collect();

    fn rec(workflows: &AHashMap<String, Workflow>, cur: &str) -> Vec<RatingRange> {
        const DEFAULT_RATING_RANGE: RatingRange = RatingRange {
            x: Range::new(1, 4000),
            m: Range::new(1, 4000),
            a: Range::new(1, 4000),
            s: Range::new(1, 4000),
        };

        let mut ret = Vec::new();
        let workflow = workflows.get(cur).unwrap();
        for (i, rule) in workflow.rules.iter().enumerate() {
            match rule.target.as_str() {
                "A" => {
                    if let Some(r) = DEFAULT_RATING_RANGE.apply_rules([rule], &workflow.rules[..i])
                    {
                        ret.push(r);
                    }
                }
                "R" => (), // skip
                _ => {
                    let res = rec(workflows, &rule.target)
                        .into_iter()
                        .filter_map(|r| r.apply_rules([rule], &workflow.rules[..i]));
                    ret.extend(res);
                }
            }
        }

        match workflow.default_target.as_str() {
            "A" => {
                if let Some(r) = DEFAULT_RATING_RANGE.apply_rules([], &workflow.rules) {
                    ret.push(r)
                }
            }
            "R" => (), // skip
            _ => ret.extend(
                rec(workflows, &workflow.default_target)
                    .into_iter()
                    .filter_map(|r| r.apply_rules([], &workflow.rules)),
            ),
        }

        ret
    }

    let ranges = rec(&workflows, "in");
    let ans: u64 = ranges.iter().map(|r| r.num_ratings()).sum();

    Ok(ans.to_string())
}

#[derive(Clone, Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_target: String,
}

impl Workflow {
    fn eval(&self, rating: &Rating) -> &str {
        self.rules
            .iter()
            .filter_map(|r| (r.eval(&rating)).then_some(r.target.as_str()))
            .next()
            .unwrap_or(self.default_target.as_str())
    }
}

#[derive(Clone, Debug)]
struct Rule {
    category: Category,
    op: Ordering,
    value: u32,
    target: String,
}

impl Rule {
    fn eval(&self, rating: &Rating) -> bool {
        rating.get(self.category).cmp(&self.value) == self.op
    }
}

// Inclusive range: [start, end]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn len(&self) -> u32 {
        self.end - self.start + 1
    }

    fn is_valid(&self) -> bool {
        self.start <= self.end
    }

    fn subset_with_start(&self, start: u32) -> Option<Self> {
        if start <= self.start {
            return Some(*self);
        }

        Some(Self {
            start,
            end: self.end,
        })
        .filter(|x| x.is_valid())
    }
    fn subset_with_end(&self, end: u32) -> Option<Self> {
        if end >= self.end {
            return Some(*self);
        }

        Some(Self {
            start: self.start,
            end,
        })
        .filter(|x| x.is_valid())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RatingRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl RatingRange {
    fn num_ratings(&self) -> u64 {
        self.x.len() as u64 * self.m.len() as u64 * self.a.len() as u64 * self.s.len() as u64
    }

    fn apply_rule(&mut self, rule: &Rule, opposite: bool) -> bool {
        let range = self.get_mut(rule.category);
        let (op, offset) = match opposite {
            true => (rule.op.reverse(), 0),
            false => (rule.op, 1),
        };
        let new_r = match op {
            Ordering::Greater => range.subset_with_start(rule.value + offset),
            Ordering::Less => range.subset_with_end(rule.value - offset),
            _ => panic!("bad op"),
        };

        let Some(new_r) = new_r else {
            return false;
        };

        *range = new_r;

        true
    }

    fn apply_rules<'a>(
        &self,
        rules: impl IntoIterator<Item = &'a Rule>,
        opposite_rules: impl IntoIterator<Item = &'a Rule>,
    ) -> Option<Self> {
        let mut ret = *self;

        for r in rules {
            if !ret.apply_rule(r, false) {
                return None;
            }
        }

        for r in opposite_rules {
            if !ret.apply_rule(r, true) {
                return None;
            }
        }

        Some(ret)
    }

    fn get_mut(&mut self, category: Category) -> &mut Range {
        match category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Rating {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Rating {
    fn get(&self, category: Category) -> u32 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn set(&mut self, category: Category, value: u32) {
        match category {
            Category::X => self.x = value,
            Category::M => self.m = value,
            Category::A => self.a = value,
            Category::S => self.s = value,
        }
    }

    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

mod parser {
    use nom::character::complete::multispace1;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Rating>)> {
        let id = |input| alpha1.map(|s: &str| s.to_owned()).parse(input);
        let category = |input| {
            alt((
                value(Category::X, char('x')),
                value(Category::M, char('m')),
                value(Category::A, char('a')),
                value(Category::S, char('s')),
            ))(input)
        };
        let op = alt((
            value(Ordering::Greater, char('>')),
            value(Ordering::Less, char('<')),
        ));
        let rule = tuple((category, op, uint, preceded(char(':'), id))).map(
            |(category, op, value, target)| Rule {
                category,
                op,
                value,
                target,
            },
        );
        let rules = tuple((many1(terminated(rule, char(','))), id));
        let workflow_body = delimited(char('{'), rules, char('}'));
        let workflow = tuple((id, workflow_body)).map(|(name, (rules, default_rule))| Workflow {
            name,
            rules,
            default_target: default_rule,
        });
        let workflows = many1(ws_line(workflow));

        let rating_category = separated_pair(category, char('='), uint);
        let rating_body = fold_separated_list1(
            char(','),
            rating_category,
            Rating::default,
            |mut acc, (cat, val)| {
                acc.set(cat, val);
                acc
            },
        );
        let rating = delimited(char('{'), rating_body, char('}'));
        let ratings = many1(ws_line(rating));

        let parser = separated_pair(workflows, multispace1, ratings);
        ws_all_consuming(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "19114")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "167409079868000")
    }
}
