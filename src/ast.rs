use crate::exps::*;
use crate::token::Token;

#[derive(Debug)]
pub enum Ast {
    Exp(Exp),
}

pub fn make_ast(mut tokens: Vec<Token>) -> Ast {
    let mut stack: Vec<Exp> = Vec::new();
    for t in tokens.iter_mut() {
        eprintln!("{:?}", stack);
        match stack.pop() {
            Some(exp) => match t {
                Token::Operator(s) => match s.as_str() {
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

pub fn dump_ast_comment(ast: &Ast) {
    for l in format!("{:#?}", ast).lines() {
        println!("; {}", l);
    }
}
