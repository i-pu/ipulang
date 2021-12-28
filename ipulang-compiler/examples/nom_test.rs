use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, char, digit1, multispace0, multispace1, one_of},
    combinator::{map, opt},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn a_parser(s: &str) -> IResult<&str, String> {
    let (ss, m) = tag("a")(s)?;
    Ok((ss, m.to_string()))
}

fn ac_parser(s: &str) -> IResult<&str, String> {
    let (ss, m) = tag("ac")(s)?;
    Ok((ss, m.to_string()))
}

fn parser(s: &str) -> IResult<&str, String> {
    alt((a_parser, ac_parser))(s)
}

fn main() {
    let test = "ac";
    let (res, m) = parser(test).unwrap();
    dbg!((res, m));
}
