// 09_error_handling.rs
// 说明: panic, Result, ? 运算符, 自定义错误类型
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // panic!
    // panic!("crash and burn");

    // Result
    match read_username_from_file() {
        Ok(s) => println!("file content: {}", s),
        Err(e) => println!("error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("Cargo.toml")?;
    f.read_to_string(&mut s)?;
    Ok(s)
}
