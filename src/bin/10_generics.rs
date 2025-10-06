// 10 - 泛型（Generics）
// 说明：泛型函数、泛型结构体、特化 trait 约束

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    println!("largest = {}", largest(&nums));

    let chars = vec!['a', 'y', 'm'];
    println!("largest char = {}", largest(&chars));
}
