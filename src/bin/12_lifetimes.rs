// 12_lifetimes.rs
// 说明: 生命周期注解示例，引用的生命周期关系
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string");
    let s2 = "small";
    let result = longest(s1.as_str(), s2);
    println!("longest = {}", result);
}
