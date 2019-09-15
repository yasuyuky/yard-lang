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
                Token::Number(s) => match exp {
                    Exp::BinOp(mut bo) => {
                        comp_bexpr(&mut bo, Exp::Num(s.to_string()));
                        stack.push(Exp::BinOp(bo))
                    }
                    Exp::Subst(mut subst) => stack.push(Exp::Subst(Substitution {
                        ident: subst.ident,
                        rhs: Box::new(match subst.rhs.as_mut() {
                            Exp::Undef => Exp::Num(s.to_string()),
                            Exp::BinOp(bo) => {
                                comp_bexpr(bo, Exp::Num(s.to_string()));
                                Exp::BinOp(bo.clone())
                            }
                            _ => unreachable!(),
                        }),
                    })),
                    Exp::Undef => stack.push(Exp::Num(s.to_string())),
                    _ => panic!("Unknown Syntax"),
                },
                Token::Ident(s) => match exp {
                    Exp::BinOp(mut bo) => {
                        comp_bexpr(&mut bo, Exp::Ident(s.to_string()));
                        stack.push(Exp::BinOp(bo))
                    }
                    Exp::Subst(subst) => stack.push(Exp::Subst(Substitution {
                        ident: subst.ident,
                        rhs: Box::new(Exp::Ident(s.to_string())),
                    })),
                    Exp::Undef => stack.push(Exp::Ident(s.to_string())),
                    _ => panic!("Invalid"),
                },
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
