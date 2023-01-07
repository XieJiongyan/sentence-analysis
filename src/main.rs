use nom::{
    IResult,
    sequence::delimited,
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::char,
    bytes::complete::is_not
};

use sentence_analysis::class::Class;

fn parens(input: &str) -> IResult<&str, &str> {
    delimited(char('('), is_not(")"), char(')'))(input)
}

fn main() {
    println!("Hello, world!");
    let ir = parens("(ogl)");
    println!("{:?}", ir);

    let name = String::from("Name");
    let class = Class{ name };
    println!("{}", class);
}
