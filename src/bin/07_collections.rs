// 07 - 常见集合及操作（Vectors, Strings, HashMap）
// 说明：Vec、String、HashMap 基本用法

use std::collections::HashMap;

fn main() {
    // Vec
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("v = {:?}", v);

    // String
    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("s = {}", s);

    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
}
