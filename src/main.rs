use std::io::{self, Read};

fn main() ->io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    println!("define i32 @main() {{");
    println!(" ret i32 {} }}", buffer.trim());
    Ok(())
}
