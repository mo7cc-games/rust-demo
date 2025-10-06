// 07_collections.rs
// 说明: 常见集合 Vec, String, HashMap, HashSet 及常用操作
use std::collections::{HashMap, HashSet};

fn main() {
    // Vec: 可增长数组
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    for i in &v {
        println!("vec 元素: {}", i);
    }

    // String
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("String: {}", s);

    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // HashSet
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2); // 重复会被忽略
    println!("set contains 2? {}", set.contains(&2));
}
