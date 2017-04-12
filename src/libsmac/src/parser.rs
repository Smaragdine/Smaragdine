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
named!(float<f32>, 
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

// Parens
named!(parens <f32>,
    ws!(
        delimited!(
            tag!("("), expr, tag!(")")
        )
    )
);

// Factor := digit | float | (expr)
named!(factor<f32>,
    alt!(
        float
        |
        map_res!(
            map_res!(
                ws!(digit),
                str::from_utf8
            ),
            FromStr::from_str
        )
        |
        parens
    )
);

// Term := factor * | / factor
named!(term <f32>, do_parse!(
    init: factor >>
    res:  fold_many0!(
        pair!(alt!(tag!("*") | tag!("/")), factor),
        init,
        |acc, (op, val): (&[u8], f32)| {
            if op[0] as char == '*' {
                acc * val
            } else {
                acc / val
            }
        }
    ) >> (res)
  )
);

// Expression := term + | - term
named!(expr <f32>, 
    do_parse!(
        init: term >>
        res:  fold_many0!(
            pair!(alt!(tag!("+") | tag!("-")), term),
            init,
            |acc, (op, val): (&[u8], f32)| {
                if op[0] as char == '+' {
                    acc + val
                } else {
                    acc - val
                }
            }
        ) >> (res)
    )
);

#[test]
fn float_test() {
    assert_eq!(float(&b"13.37"[..]), IResult::Done(&b""[..], 13.37));
    assert_eq!(float(&b".1337"[..]), IResult::Done(&b""[..], 0.1337));
}

#[test]
fn factor_test() {
    assert_eq!(factor(&b"3.5"[..]), IResult::Done(&b""[..], 3.5f32));
    assert_eq!(factor(&b" 5"[..]), IResult::Done(&b""[..], 5f32));
}

#[test]
fn expr_test() {
    assert_eq!(expr(&b"1 + 1.5"[..]), IResult::Done(&b""[..], 2.5f32))
}

#[test]
fn ident_test() {
    assert_eq!(ident(&"test?"[..]), IResult::Done("", "test?"));
    assert_eq!(ident(&"test!"[..]), IResult::Done("", "test!"));
    assert_eq!(ident(&"test!?"[..]), IResult::Done("", "test!?"));
    assert_eq!(ident(&"_test!"[..]), IResult::Done("", "_test!"));    
}

#[test]
fn parens_test() {
    assert_eq!(parens(&b"(1.5 + .5)"[..]), IResult::Done(&b""[..], 2f32));
    assert_eq!(parens(&b"(0 * 2 + 3)"[..]), IResult::Done(&b""[..], 3f32));
    assert_eq!(parens(&b"(1 + 2 * (2 + 1))"[..]), IResult::Done(&b""[..], 7f32));
    assert_eq!(parens(&b"(1 + 3 / (2 + 1))"[..]), IResult::Done(&b""[..], 2f32));
}