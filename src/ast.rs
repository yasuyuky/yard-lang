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
            Some(exp) => match t {
                Token::Operator(s) => match s.as_str() {
                    "+" => stack.push(make_bexpl(exp, BinOp::PlusMinus(Additive::Plus))),
                    "-" => stack.push(make_bexpl(exp, BinOp::PlusMinus(Additive::Minus))),
                    "*" => stack.push(make_bexpl(exp, BinOp::MulDiv(Multitive::Mul))),
                    "/" => stack.push(make_bexpl(exp, BinOp::MulDiv(Multitive::Div))),
                    "=" => {
                        if let Exp::Ident(s) = exp {
                            stack.push(make_subst(&s))
                        }
                    }
                    ";" => {
                        stack.push(exp);
                        stack.push(Exp::Undef)
                    }
                    _ => panic!("Undefined arithmetic operator"),
                },
                Token::Number(s) => stack.push(comp_expr(exp, Exp::Num(s.to_string()))),
                Token::Ident(s) => stack.push(comp_expr(exp, Exp::Ident(s.to_string()))),
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
