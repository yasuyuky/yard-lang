use crate::exps::*;
use crate::token::Token;

#[derive(Debug)]
pub enum Ast {
    Stmt(Vec<Exp>),
}

pub fn make_ast(mut tokens: Vec<Token>) -> Ast {
    let mut stack: Vec<Exp> = vec![Exp::Undef];
    for t in tokens.iter_mut() {
        eprintln!("{:?}", stack);
        match stack.pop() {
            Some(ref mut exp) => match t {
                Token::Operator(s) => match s.as_str() {
                    "+" => stack.push(exp.make_bexpl(BinOp::PlusMinus(Additive::Plus))),
                    "-" => stack.push(exp.make_bexpl(BinOp::PlusMinus(Additive::Minus))),
                    "*" => stack.push(exp.make_bexpl(BinOp::MulDiv(Multitive::Mul))),
                    "/" => stack.push(exp.make_bexpl(BinOp::MulDiv(Multitive::Div))),
                    "=" => {
                        if let Exp::Ident(s) = exp {
                            stack.push(make_assign(&s))
                        }
                    }
                    ";" => {
                        stack.push(exp.clone());
                        stack.push(Exp::Undef)
                    }
                    _ => panic!("Undefined arithmetic operator"),
                },
                Token::Number(s) => stack.push(exp.extend(Exp::Num(s.to_string()))),
                Token::Ident(s) => stack.push(exp.extend(Exp::Ident(s.to_string()))),
            },
            None => unreachable!(),
        }
    }
    Ast::Stmt(stack)
}

pub fn dump_ast_comment(ast: &Ast) {
    for l in format!("{:#?}", ast).lines() {
        println!("; {}", l);
    }
}
