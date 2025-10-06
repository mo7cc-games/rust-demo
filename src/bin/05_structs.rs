// 05 - 结构体（Structs）
// 说明：命名结构体、元组结构体、方法、关联函数（impl）示例

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("面积 = {}", rect.area());

    let sq = Rectangle::square(3);
    println!("square area = {}", sq.area());
}
