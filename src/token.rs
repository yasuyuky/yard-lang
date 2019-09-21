use std::iter::FromIterator;

pub struct Token {
    pub t: TokenType,
    pub s: String,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum TokenType {
    Number,
    Operator,
    Ident,
    Keyword(KeywordType),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum CharType {
    WhiteSpace,
    Digit,
    Alphabetic,
    Punctuation,
    Unknown,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum KeywordType {
    Return,
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
        let s = String::from_iter(v.into_iter());
        let t = match ty {
            CharType::Alphabetic => match s.as_str() {
                "return" => TokenType::Keyword(KeywordType::Return),
                _ => TokenType::Ident,
            },
            CharType::Digit => TokenType::Number,
            CharType::Punctuation => TokenType::Operator,
            _ => continue,
        };
        res.push(Token { t, s })
    }
    res
}
