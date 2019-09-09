use std::io::{self, Read};

mod exps;
use exps::*;
mod token;
use token::*;
mod ast;
use ast::*;

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
