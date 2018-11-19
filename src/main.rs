use std::collections::VecDeque;
use std::io::{self, Read};

enum Token {
    Number(String),
    Arithmetic(String),
}

#[derive(Debug)]
enum Ast {
    Exp(Exp),
}

#[derive(Debug, Clone)]
enum Exp {
    BinExp(Box<BinExp>),
    Number(String),
    Undefined,
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
struct BinExp {
    lhs: Box<Exp>,
    op: BinOp,
    rhs: Box<Exp>,
}

fn tokenize(buf: &str) -> Vec<Token> {
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

fn make_bexpl(lhs: &Exp, op: BinOp) -> Exp {
    Exp::BinExp(Box::new(BinExp {
        lhs: Box::new(lhs.clone()),
        op,
        rhs: Box::new(Exp::Undefined),
    }))
}

fn comp_bexpr(bexpl: &mut Box<BinExp>, rhs: Exp) {
    bexpl.rhs = Box::new(rhs);
}

fn make_ast(mut tokens: Vec<Token>) -> Ast {
    let mut stack: VecDeque<Exp> = VecDeque::new();
    for t in tokens.iter_mut() {
        match (stack.pop_back(), t) {
            (None, Token::Number(s)) => stack.push_back(Exp::Number(s.to_string())),
            (Some(ref exp), Token::Arithmetic(s)) => match s.as_str() {
                "+" => stack.push_back(make_bexpl(exp, BinOp::Plus)),
                "-" => stack.push_back(make_bexpl(exp, BinOp::Minus)),
                _ => panic!("Undefined arithmetic operator"),
            },
            (Some(Exp::BinExp(ref mut be)), Token::Number(s)) => {
                comp_bexpr(be, Exp::Number(s.to_string()));
                stack.push_back(Exp::BinExp(Box::new(*be.clone())))
            }
            (None, Token::Arithmetic(_)) => panic!("First token is restricted to number"),
            (Some(Exp::Number(_)), Token::Number(_)) => panic!("Number Sequence"),
            (Some(Exp::Undefined), _) => unreachable!(),
        }
    }
    Ast::Exp(stack.pop_back().unwrap())
}

fn gen_from_exp(exp: &Exp, count: usize) -> (String, usize) {
    match exp {
        Exp::Number(s) => (format!(" %x{} = add i32 {}, 0 \n", count, s), count),
        Exp::BinExp(be) => {
            let (lhs, cl) = gen_from_exp(&be.as_ref().lhs, count);
            let (rhs, cr) = gen_from_exp(&be.as_ref().rhs, cl + 1);
            let op = match be.as_ref().op {
                BinOp::Plus => "add",
                BinOp::Minus => "sub",
            };
            (
                lhs + &rhs + &format!(" %x{} = {} i32 %x{}, %x{}\n", cr + 1, op, cl, cr),
                cr + 1,
            )
        }
        _ => unreachable!(),
    }
}

fn gen(ast: Ast) -> String {
    let Ast::Exp(ref exp) = ast;
    let mut res: String;
    res = "define i32 @main() {\n".to_string();
    let (s, i) = gen_from_exp(exp, 0);
    res += &s;
    res += &format!(" ret i32 %x{}\n}}\n", i);
    res
}

fn compile_buffer(buf: &str) -> String {
    let tokens = tokenize(&buf.trim());
    let ast = make_ast(tokens);
    gen(ast)
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    println!("{}", compile_buffer(&buffer));
    Ok(())
}
