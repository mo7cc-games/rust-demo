// 01_variables_and_types.rs
// 说明: 变量与数据类型示例 (可变/不可变、标量、复合类型、类型注解)
fn main() {
    // 不可变变量 (默认)
    let x = 5;
    println!("不可变 x = {}", x);

    // 可变变量
    let mut y = 10;
    println!("可变 y = {}", y);
    y = 15;
    println!("修改后 y = {}", y);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);

    // 标量类型: 整数、浮点、布尔、字符
    let a: i32 = -3;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = '中';
    println!("{} {} {} {}", a, b, c, d);

    // 复合类型: 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup 解构: {} {} {}", x, y, z);

    // 数组
    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("arr[2] = {}", arr[2]);
}
