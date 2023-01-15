use nom::IResult;
use nom::bytes::complete::{take_while_m_n, take_while};


//FIXME Not a keyword
pub fn parse_name(i: &str) -> IResult<&str, String> {
    let cond1 = |s: char| {
        s >= 'a' && s <= 'z' || s >= 'A' && s <= 'Z' || s == '_'
    };
    let (i, s0) = take_while_m_n(1, 1, cond1)(i)?;

    let cond2 = |s: char| {cond1(s) || s >= '0' && s <= '9'};
    let (remaining_input, s1) = take_while(cond2)(i)?;

    Ok((remaining_input, format!("{}{}", s0, s1)))
}
