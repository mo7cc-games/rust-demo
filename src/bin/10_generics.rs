// 10_generics.rs
// 说明: 泛型函数、泛型结构体、trait bounds
fn main() {
    println!("largest i32: {}", largest(&vec![1, 4, 2, 8, 3]));
    println!("largest char: {}", largest(&vec!['a', 'z', 'm']));

    let point = Point { x: 5, y: 10 };
    println!("point x = {}", point.x);
}

fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}
