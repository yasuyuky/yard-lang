use std::iter::FromIterator;

pub enum Token {
    Number(String),
    Operator(String),
    Ident(String)
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum CharType {
    WhiteSpace,
    Digit,
    Alphabetic,
    Punctuation,
    Unknown,
}

impl From<&char> for CharType {
    fn from(c: &char) -> Self {
        if c.is_ascii_whitespace() {
            Self::WhiteSpace
        } else if c.is_ascii_digit() {
            Self::Digit
        } else if c.is_ascii_alphabetic() {
            Self::Alphabetic
        } else if c.is_ascii_punctuation() {
            Self::Punctuation
        } else {
            Self::Unknown
        }
    }
}

fn split_to_raw_tokens(buf: &str) -> Vec<(CharType, Vec<char>)> {
    let mut ret: Vec<(CharType, Vec<char>)> = vec![];
    for c in buf.chars() {
        match ret.last_mut() {
            None => ret.push((CharType::from(&c), vec![c])),
            Some((ty, v)) => {
                if *ty == CharType::from(&c) {
                    v.push(c)
                } else {
                    ret.push((CharType::from(&c), vec![c]))
                }
            }
        }
    }
    ret
}

pub fn tokenize(buf: &str) -> Vec<Token> {
    let mut res: Vec<Token> = vec![];
    for (ty, v) in split_to_raw_tokens(buf) {
        match ty {
            CharType::Alphabetic => res.push(Token::Ident(String::from_iter(v.into_iter()))),
            CharType::Digit => res.push(Token::Number(String::from_iter(v.into_iter()))),
            CharType::Punctuation => res.push(Token::Operator(String::from_iter(v.into_iter()))),
            _ => continue,
        }
    }
    res
}
