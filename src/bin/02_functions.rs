// 02_functions.rs
// 说明: 函数定义、参数、返回值、表达式与语句
fn main() {
    println!("sum = {}", add(2, 3));
    println!("five = {}", five());

    let x = 6;
    let y = another_function(x);
    println!("another_function({}) = {}", x, y);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // 表达式，没有分号，返回值
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) -> i32 {
    println!("接收到: {}", x);
    x + 1
}
