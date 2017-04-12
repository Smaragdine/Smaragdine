use nom;
use nom::{IResult, alpha, alphanumeric, digit, multispace};

use std::str;
use std::str::{from_utf8, FromStr};

// Identifier
// IDENT := /[a-zA-Z_]\w*/g
named!(ident<&str, &str>, re_match!(r"[a-zA-Z_](?:\w|[?!])*"));

// Whitespace
named!(whitespace, chain!(many0!(multispace), || { &b""[..] }));

// Float literal e.g. `.123` `0.123`
named!(
    float<f32>, 
    map_res!(
        map_res!(
            recognize!(
                alt!(
                    delimited!(digit, tag!("."), opt!(complete!(digit))) |
                    delimited!(opt!(digit), tag!("."), digit)
                )
            ),
            str::from_utf8
        ),
        FromStr::from_str
    )
);

#[test]
fn float_test() {
    assert_eq!(float(&b"13.37"[..]), IResult::Done(&b""[..], 13.37));
    assert_eq!(float(&b".1337"[..]), IResult::Done(&b""[..], 0.1337));
}

#[test]
fn ident_test() {
    assert_eq!(ident(&"test?"[..]), IResult::Done("", "test?"));
    assert_eq!(ident(&"test!"[..]), IResult::Done("", "test!"));
    assert_eq!(ident(&"test!?"[..]), IResult::Done("", "test!?"));
    assert_eq!(ident(&"_test!"[..]), IResult::Done("", "_test!"));    
}