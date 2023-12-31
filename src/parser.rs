use nom::combinator::all_consuming;
use prelude::*;
use std::str::FromStr;

pub mod prelude {
    pub use nom::{
        branch::alt,
        bytes::complete::{is_a, tag, take_while},
        character::complete::{
            alpha1, alphanumeric1, anychar, char, digit1, line_ending, multispace0, one_of, space0,
            space1,
        },
        combinator::{eof, into, map, map_res, opt, recognize, value, verify},
        error::ParseError,
        multi::{count, many1, many1_count, separated_list1},
        sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
        AsChar, IResult, InputTakeAtPosition, Parser,
    };

    pub use super::{complete, fold_separated_list1, int, uint, ws_all_consuming, ws_line};
}

pub fn uint<T: FromStr>(input: &str) -> IResult<&str, T> {
    let digits = is_a("0123456789");
    let mut parser = map_res(digits, |x: &str| x.parse());
    parser(input)
}

#[allow(dead_code)]
pub fn int<T: FromStr>(input: &str) -> IResult<&str, T> {
    let digits = is_a("0123456789");
    let num = tuple((opt(tag("-")), digits));
    let mut parser = map_res(recognize(num), |x: &str| x.parse());
    parser(input)
}

#[allow(dead_code)]
pub fn complete<I, O, E, P>(parser: P) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: nom::InputLength + nom::InputTakeAtPosition + Clone,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    P: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
{
    terminated(parser, tuple((multispace0, eof)))
}

pub fn ws_all_consuming<I, O, E, P>(parser: P) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: nom::InputLength + nom::InputTakeAtPosition + Clone,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    P: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
{
    all_consuming(delimited(multispace0, parser, multispace0))
}

pub fn ws_line<'a, O, E, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    P: nom::Parser<&'a str, O, E>,
    E: nom::error::ParseError<&'a str>,
{
    let end_of_line = alt((line_ending, eof));
    delimited(space0, parser, tuple((space0, end_of_line)))
}

pub fn fold_separated_list1<I, O, O2, R, A, E, F, G, H>(
    mut sep: G,
    mut f: F,
    mut init: H,
    mut fold_fn: A,
) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Clone + nom::InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    A: FnMut(R, O) -> R,
    H: FnMut() -> R,
    E: ParseError<I>,
{
    move |input| {
        let mut acc = init();
        let (mut rest, o1) = f.parse(input)?;
        acc = fold_fn(acc, o1);

        let mut next_parser = preceded(|input| sep.parse(input), |input| f.parse(input));

        loop {
            let (r, o) = match next_parser.parse(rest.clone()) {
                Ok(x) => x,
                Err(_) => break,
            };

            // Check that the input advanced. It must advance to prevent infinte loops.
            if rest.input_len() == r.input_len() {
                return Err(nom::Err::Error(E::from_error_kind(
                    rest,
                    nom::error::ErrorKind::Many1,
                )));
            }

            rest = r;
            acc = fold_fn(acc, o);
        }

        Ok((rest, acc))
    }
}
