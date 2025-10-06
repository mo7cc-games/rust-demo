// 09 - 错误处理（panic!, Result, ? 运算符）
// 说明：展示 panic、Result、文件读取与错误传播

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("Cargo.toml")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    match read_username_from_file() {
        Ok(s) => println!("read {} bytes from Cargo.toml", s.len()),
        Err(e) => println!("error: {}", e),
    }

    // panic! 示例（不推荐在生产中使用）
    // panic!("explicit panic for demo");
}
