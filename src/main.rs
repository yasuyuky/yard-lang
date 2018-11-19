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
    BinaryOp(BinOpExp),
    Number(String),
    Undefined,
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
struct BinOpExp {
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
    Exp::BinaryOp(BinOpExp {
        lhs: Box::new(lhs.clone()),
        op,
        rhs: Box::new(Exp::Undefined),
    })
}

fn comp_bexpr(bexpl: &mut BinOpExp, rhs: Exp) {
    bexpl.rhs = Box::new(rhs);
}

fn make_ast(mut tokens: Vec<Token>) -> Ast {
    let mut stack: Vec<Exp> = Vec::new();
    for t in tokens.iter_mut() {
        match (stack.pop(), t) {
            (None, Token::Number(s)) => stack.push(Exp::Number(s.to_string())),
            (Some(ref exp), Token::Arithmetic(s)) => match s.as_str() {
                "+" => stack.push(make_bexpl(exp, BinOp::Plus)),
                "-" => stack.push(make_bexpl(exp, BinOp::Minus)),
                _ => panic!("Undefined arithmetic operator"),
            },
            (Some(Exp::BinaryOp(ref mut bo)), Token::Number(s)) => {
                comp_bexpr(bo, Exp::Number(s.to_string()));
                stack.push(Exp::BinaryOp(bo.clone()))
            }
            (None, Token::Arithmetic(_)) => panic!("First token is restricted to number"),
            (Some(Exp::Number(_)), Token::Number(_)) => panic!("Number Sequence"),
            (Some(Exp::Undefined), _) => unreachable!(),
        }
    }
    Ast::Exp(stack.pop().unwrap())
}

fn gen_from_exp(exp: &Exp, no: usize) -> (String, usize) {
    match exp {
        Exp::Number(s) => (format!(" %{} = add i32 {}, 0 \n", no, s), no),
        Exp::BinaryOp(bo) => {
            let (lhs, ln) = gen_from_exp(&bo.lhs, no);
            let (rhs, rn) = gen_from_exp(&bo.rhs, ln + 1);
            let op = match bo.op {
                BinOp::Plus => "add",
                BinOp::Minus => "sub",
            };
            let s = format!(" %{} = {} i32 %{}, %{}\n", rn + 1, op, ln, rn);
            (lhs + &rhs + &s, rn + 1)
        }
        _ => unreachable!(),
    }
}

fn gen(ast: Ast) -> String {
    let Ast::Exp(ref exp) = ast;
    let mut res: String;
    res = "define i32 @main() {\n".to_string();
    let (s, i) = gen_from_exp(exp, 1);
    res += &s;
    res += &format!(" ret i32 %{}\n}}\n", i);
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
