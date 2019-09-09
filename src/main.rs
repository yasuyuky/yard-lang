use std::io::{self, Read};

mod exps;
use exps::*;

enum Token {
    Number(String),
    Arithmetic(String),
}

#[derive(Debug)]
enum Ast {
    Exp(Exp),
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

fn make_bexpl(mut lhs: Exp, op: BinOp) -> Exp {
    match lhs {
        Exp::Num(s) => Exp::BinOp(BinOpExp::new(Exp::Num(s.to_string()), op, Exp::Undef)),
        Exp::BinOp(ref mut bo) => {
            if bo.op >= op {
                Exp::BinOp(BinOpExp::new(lhs, op, Exp::Undef))
            } else {
                bo.rhs = Box::new(make_bexpl(bo.rhs.as_ref().clone(), op));
                lhs
            }
        }
        Exp::Undef => unreachable!(),
    }
}

fn comp_bexpr(bexpl: &mut BinOpExp, rhs: Exp) {
    match bexpl.rhs.as_mut() {
        Exp::BinOp(bo) => comp_bexpr(bo, rhs),
        _ => bexpl.rhs = Box::new(rhs),
    }
}

fn make_ast(mut tokens: Vec<Token>) -> Ast {
    let mut stack: Vec<Exp> = Vec::new();
    for t in tokens.iter_mut() {
        match stack.pop() {
            Some(exp) => match t {
                Token::Arithmetic(s) => match s.as_str() {
                    "+" => stack.push(make_bexpl(exp, BinOp::PlusMinus(Additive::Plus))),
                    "-" => stack.push(make_bexpl(exp, BinOp::PlusMinus(Additive::Minus))),
                    "*" => stack.push(make_bexpl(exp, BinOp::MulDiv(Multitive::Mul))),
                    "/" => stack.push(make_bexpl(exp, BinOp::MulDiv(Multitive::Div))),
                    _ => panic!("Undefined arithmetic operator"),
                },
                Token::Number(s) => {
                    if let Exp::BinOp(mut bo) = exp {
                        comp_bexpr(&mut bo, Exp::Num(s.to_string()));
                        stack.push(Exp::BinOp(bo))
                    } else {
                        panic!("Number Sequence")
                    }
                }
            },
            None => match t {
                Token::Number(s) => stack.push(Exp::Num(s.to_string())),
                _ => panic!("First token is restricted to number"),
            },
        }
    }
    Ast::Exp(stack.pop().unwrap())
}

fn gen_from_exp(exp: &Exp, no: usize) -> (String, String, usize) {
    match exp {
        Exp::Num(s) => (String::new(), s.to_string(), no),
        Exp::BinOp(bo) => {
            let (lhs, lr, ln) = gen_from_exp(&bo.lhs, no);
            let (rhs, rr, rn) = gen_from_exp(&bo.rhs, ln);
            let op = match bo.op {
                BinOp::PlusMinus(Additive::Plus) => "add",
                BinOp::PlusMinus(Additive::Minus) => "sub",
                BinOp::MulDiv(Multitive::Mul) => "mul",
                BinOp::MulDiv(Multitive::Div) => "udiv",
            };
            let s = format!(" %{} = {} i32 {}, {}\n", rn, op, lr, rr);
            (lhs + &rhs + &s, format!("%{}", rn), rn + 1)
        }
        _ => unreachable!(),
    }
}

fn gen(ast: Ast) -> String {
    let Ast::Exp(ref exp) = ast;
    let mut res: String;
    res = "define i32 @main() {\n".to_string();
    let (s, r, _) = gen_from_exp(exp, 1);
    res += &s;
    res += &format!(" ret i32 {}\n}}\n", r);
    res
}

fn dump_ast_comment(ast: &Ast) {
    for l in format!("{:#?}", ast).lines() {
        println!("; {}", l);
    }
}

fn compile_buffer(buf: &str) -> String {
    let tokens = tokenize(&buf.trim());
    let ast = make_ast(tokens);
    dump_ast_comment(&ast);
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

#[allow(unused_imports)]
mod test {
    use crate::compile_buffer;
    use std::io::Write;
    use std::process::{Command, Stdio};

    #[test]
    fn test_line_with_status_code() {
        let test_sets = vec![
            ("0", 0),
            ("1 + 1", 2),
            (" 12 + 34 - 5 ", 12 + 34 - 5),
            ("1 * 2 + 3", 1 * 2 + 3),
            ("1 + 2*3", 1 + 2 * 3),
            ("1*2 + 3*4", 1 * 2 + 3 * 4),
            ("1*2 + 3*4/5", 1 * 2 + 3 * 4 / 5),
        ];
        for (input, result) in test_sets {
            let ir = compile_buffer(&input);
            let mut clang = Command::new("clang")
                .args(&["-Wno-override-module", "-o", "tmp", "-x", "ir", "-"])
                .stdin(Stdio::piped())
                .spawn()
                .expect("compile ir");
            let cstdin = clang.stdin.as_mut().expect("open stdin");
            cstdin.write_all(ir.as_bytes()).expect("write to stdin");
            clang.wait().expect("exit clang compile");
            let status = Command::new("./tmp").status().expect("execute bin");
            assert_eq!(status.code(), Some(result));
        }
    }
}
