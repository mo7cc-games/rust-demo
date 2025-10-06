// 08 - 包（crate）与模块（mod）
// 说明：演示模块声明、pub、use

mod greeting {
    pub fn hello() {
        println!("Hello from greeting::hello");
    }
}

use greeting::hello;

fn main() {
    hello();
}
