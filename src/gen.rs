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
        Exp::Ident(s) => (
            format!(" %{} = load i32, i32* %{} \n", no, s),
            format!("%{}", no),
            no + 1,
        ),
        Exp::Assign(sub) => {
            let (rhs, rr, rn) = gen_from_exp(&sub.rhs, no);
            let alloca = format!(" %{} = alloca i32 \n", sub.ident);
            let store = format!(" store i32 {}, i32* %{} \n", rr, sub.ident);
            (rhs + &alloca + &store, format!("%{}", sub.ident), rn)
        }
        _ => unreachable!(),
    }
}

pub fn gen(ast: Ast) -> String {
    let Ast::Stmt(ref exps) = ast;
    let mut res: String;
    res = "define i32 @main() {\n".to_string();
    let mut no = 1;
    let mut last = String::new();
    for exp in exps {
        let (s, r, n) = gen_from_exp(exp, no);
        no = n;
        res += &s;
        last = r;
    }
    res += &format!(" ret i32 {}\n}}\n", last);
    res
}
