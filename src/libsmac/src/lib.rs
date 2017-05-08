#![feature(plugin)]
#![plugin(indoc)]

pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use lexer::{Lexer, TokenType};
    use lexer::grab_smaragdine_lexer;
    use std::iter::Iterator;

    macro_rules! match_seq {
        (src: $src:expr $(, $ttype:path => $tvalue:expr)*) => {{
            let mut lexer = grab_smaragdine_lexer(&mut format!("{}", $src).chars());
            match_seq!(lex: lexer $(, $ttype => $tvalue)*);
        }};
        (lex: $lex:expr $(, $ttype:path => $tvalue:expr)*) => {{
            let mut lexer = &mut $lex as &mut Lexer;
            $(match_seq!(inner: lexer, $ttype => $tvalue);)*
        }};
        (inner: $lex:expr, $ttype:path => $tvalue:expr) => {{
            let mut lexer = &mut $lex as &mut Lexer;
            let token_type = $ttype as TokenType;
            let token_content = format!("{}", $tvalue);
            match Iterator::next(lexer) {
                Some(token) => {
                    assert_eq!(token.token_type().to_owned(), token_type);
                    assert_eq!(token.content().to_owned(), token_content);
                }
                None => unimplemented!()
            };
        }};
    }

    #[test]
    fn lex_single_integer_basic() {
        match_seq!(
            src: indoc!("
                0
                12
                1234
                8765
                192843718371235601
            "),
            TokenType::IntLiteral => 0,
            TokenType::IntLiteral => 12,
            TokenType::IntLiteral => 1234,
            TokenType::IntLiteral => 8765,
            TokenType::IntLiteral => 192843718371235601_u64
        );
    }

    #[test]
    fn lex_single_integer_hex() {
        match_seq!(
            src: indoc!("
                0x0
                0xFF
                0x1234
                0x00000A
                0xA00000
                0xABCDEF
                0xFFFFFF
            "),
            TokenType::IntLiteral => 0x0,
            TokenType::IntLiteral => 0xFF,
            TokenType::IntLiteral => 0x1234,
            TokenType::IntLiteral => 0x00000A,
            TokenType::IntLiteral => 0xA00000,
            TokenType::IntLiteral => 0xFFFFFF
        )
    }
}
