use std::str::Chars;

use crate::Token::*;

#[derive(Clone, Debug, PartialEq, Eq)] // Copy not possible on enum()
pub enum Token {
    Whitespace,         // see is_whitespace()
    OpenBrace,          // {
    CloseBrace,         // }
    OpenParenthesis,    // (
    CloseParenthesis,   // )
    Semicolon,          // ;
    Keyword(String),    // only int right now
    Identifier(String), // [a-zA-Z]\w*
    Integer(String),    // [0-9]+
    Unknown,
}

pub struct Lexer<'a> {
    //input: &'a str,
    chars: Chars<'a>,
}

/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            //input,
            chars: input.chars(),
        }
    }

    fn whitespace(&mut self) -> Token {
        let mut next_chars = self.chars.clone();
        while let Some(c) = next_chars.next() {
            if is_whitespace(c) {
                self.chars.next();
            } else {
                break;
            }
        }
        Whitespace
    }

    fn number(&mut self, first_digit: char) -> Token {
        let mut number = String::new();
        number.push(first_digit);
        let mut next_chars = self.chars.clone();
        while let Some(c) = next_chars.next() {
            if c.is_digit(10) {
                number.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        Integer(number)
    }

    fn keyword_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first_char);
        let mut next_chars = self.chars.clone();
        while let Some(c) = next_chars.next() {
            if c.is_ascii_alphanumeric() || c == '_' {
                identifier.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        match identifier.as_str() {
            "return" => Keyword(identifier),
            "int" => Keyword(identifier),
            _ => Identifier(identifier),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let first_char = self.chars.next()?;
        let token = match first_char {
            c if is_whitespace(c) => self.whitespace(),
            '{' => OpenBrace,
            '}' => CloseBrace,
            '(' => OpenParenthesis,
            ')' => CloseParenthesis,
            ';' => Semicolon,
            c @ '0'..='9' => self.number(c),
            c @ 'a'..='z' | c @ 'A'..='Z' => self.keyword_identifier(c),
            _ => Unknown,
        };
        Some(token)
    }
}
