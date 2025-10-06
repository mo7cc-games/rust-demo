// 12 - 生命周期（Lifetimes）
// 说明：引用生命周期标注，避免悬垂引用

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(s1.as_str(), s2);
    println!("longest = {}", result);
}
