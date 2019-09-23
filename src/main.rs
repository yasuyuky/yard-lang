use std::io::{self, Read};

mod exps;
mod token;
use token::*;
mod ast;
use ast::*;
mod gen;
use gen::*;

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
            ("return 0", 0),
            ("return 1 + 1", 2),
            ("return  12 + 34 - 5 ", 12 + 34 - 5),
            ("return 1 * 2 + 3", 1 * 2 + 3),
            ("return 1 + 2*3", 1 + 2 * 3),
            ("return 1*2 + 3*4", 1 * 2 + 3 * 4),
            ("return 1*2 + 3*4/5", 1 * 2 + 3 * 4 / 5),
            ("1*2 ; return 3*4/5", 3 * 4 / 5),
            ("a = 1; b = 2; return a*b", 1 * 2),
            ("a = 1 + 2*3; b = 4; return a*b*b", (1 + 2 * 3) * 4 * 4),
            ("a = 1 + 2*3; b = 2*a; return a*b", (1 + 2 * 3) * 14),
            ("return ((1 + 2) * 3 + 4) * 5", ((1 + 2) * 3 + 4) * 5),
            ("a = (1 + 2) * 3; b = a; return a*b", ((1 + 2) * 3) * 9),
        ];
        for (input, result) in test_sets {
            let ir = compile_buffer(&input);
            eprintln!("{}", ir);
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
