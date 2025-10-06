// 08_packages_and_modules.rs
// 说明: 模块、包、use 与路径、pub 可见性
mod greetings {
    pub fn hello() {
        println!("hello from greetings");
    }
    fn secret() {
        println!("this is secret");
    }
}

use greetings::hello;

fn main() {
    hello();
    // greetings::secret(); // 私有，不能访问
}
