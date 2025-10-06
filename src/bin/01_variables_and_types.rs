// 01 - 变量与数据类型
// 说明：演示可变与不可变变量、基本标量类型（整型、浮点、布尔、字符）、元组与数组

fn main() {
    // 不可变变量
    let x: i32 = 42;
    println!("不可变 x = {}", x);

    // 可变变量
    let mut y = 10u8;
    println!("初始可变 y = {}", y);
    y = 20;
    println!("修改后 y = {}", y);

    // 基本类型示例
    let f: f64 = 3.14;
    let b: bool = true;
    let c: char = '中';
    println!("f={}, b={}, c={}", f, b, c);

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, _b, _c) = tup;
    println!("tup 的第一个元素：{}", a);

    // 数组
    let arr: [i32; 3] = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);
}
