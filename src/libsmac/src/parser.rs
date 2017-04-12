use nom;
use nom::{alpha, alphanumeric, digit, multispace};
use std::str::from_utf8;

// Identifier
// IDENT := /[a-zA-Z_]\w*/g
named!(ident<&str, &str>, re_match!(r"[a-zA-Z_]\w*[\?\!]*"));

// Whitespace
named!(whitespace, chain!(many0!(multispace), || { &b""[..] }));
