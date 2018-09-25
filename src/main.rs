use std::io::{self, Read};

enum Token {
    Number(String),
    Arithmetic(String),
}

fn tokenize(buf: &str) -> Vec<Token> {
    let mut res: Vec<Token> = vec![];
    for c in buf.chars() {
        if c.is_whitespace(){
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

fn compile_buffer(buf: &str) -> String {
    let mut res: String;
    res = format!("define i32 @main() {{\n");
    let tokens = tokenize(&buf.trim());
    let l = tokens.len();
    for (i, t) in tokens.into_iter().enumerate() {
        if i == 0 {
            if let Token::Number(s) = t {
                res += &format!(" %x0 = add i32 {}, 0 \n", s);
            }
        } else {
            match t {
                Token::Arithmetic(s) => match s.as_str() {
                    "+" => res += &format!(" %x{} = add i32 ", i + 1),
                    "-" => res += &format!(" %x{} = sub i32 ", i + 1),
                    _ => res += &format!(" %x{} = sub i32 ", i + 1),
                },
                Token::Number(s) => res += &format!("%x{}, {} \n", i - 2, s),
            }
        }
    }
    res += &format!(" ret i32 %x{}\n}}\n", l - 1);
    res
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    println!("{}", compile_buffer(&buffer));
    Ok(())
}
