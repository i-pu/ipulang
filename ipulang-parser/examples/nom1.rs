use nom::IResult;
use nom::{character::complete::digit1, character::complete::multispace0};

fn main() {
    let s = "63abc";
    let result: IResult<&str, &str> = digit1(&s);
    let (no_used, used) = result.unwrap();
    assert_eq!("63", used);
    assert_eq!("abc", no_used);

    let s = "6 3ab";
    let result: IResult<&str, &str> = digit1(&s);
    let (noused, used) = result.unwrap();
    dbg!(noused, used);

    let s = " ";
    let result: IResult<&str, &str> = multispace0(&s);
    dbg!(result.unwrap());
}
