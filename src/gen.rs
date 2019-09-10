use crate::ast::*;
use crate::exps::*;

pub fn gen_from_exp(exp: &Exp, no: usize) -> (String, String, usize) {
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

pub fn gen(ast: Ast) -> String {
    let Ast::Exp(ref exp) = ast;
    let mut res: String;
    res = "define i32 @main() {\n".to_string();
    let (s, r, _) = gen_from_exp(exp, 1);
    res += &s;
    res += &format!(" ret i32 {}\n}}\n", r);
    res
}