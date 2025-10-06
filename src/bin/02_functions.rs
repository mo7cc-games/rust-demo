// 02 - 函数
// 说明：函数定义、参数、返回值、表达式与语句、闭包简要示例

fn add(a: i32, b: i32) -> i32 {
    a + b // 表达式，无分号，作为返回值
}

fn main() {
    let sum = add(2, 3);
    println!("2 + 3 = {}", sum);

    // 闭包
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    println!("3 * 4 = {}", multiply(3, 4));
}
