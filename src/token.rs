pub enum Token {
    Number(String),
    Arithmetic(String),
}

pub fn tokenize(buf: &str) -> Vec<Token> {
    let mut res: Vec<Token> = vec![];
    for c in buf.chars() {
        if c.is_whitespace() {
            continue;
        } else if c.is_numeric() {
            let last = res.pop().unwrap_or(Token::Number(String::new()));
            match last {
                Token::Number(s) => {
                    res.push(Token::Number(s + c.to_string().as_str()));
                }
                Token::Arithmetic(s) => {
                    res.push(Token::Arithmetic(s));
                    res.push(Token::Number(c.to_string()));
                }
            }
        } else {
            res.push(Token::Arithmetic(c.to_string()));
        }
    }
    res
}
