use std::cmp::Ordering;

use ahash::AHashMap;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (workflows, ratings) = parse!(input);
    let workflows: AHashMap<String, Workflow> =
        workflows.into_iter().map(|w| (w.name.clone(), w)).collect();

    let ans: usize = ratings
        .iter()
        .filter(|r| filter_rating(&workflows, r))
        .map(|r| r.sum() as usize)
        .sum();

    Ok(ans.to_string())
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    todo!()
}

fn filter_rating(workflows: &AHashMap<String, Workflow>, rating: &Rating) -> bool {
    let mut cur = "in";
    while cur != "A" && cur != "R" {
        cur = workflows.get(cur).unwrap().eval(rating)
    }

    cur == "A"
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
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "")
    }
}
