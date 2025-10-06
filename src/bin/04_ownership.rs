// 04_ownership.rs
// 说明: 所有权、借用、可变借用、切片
fn main() {
    // 所有权移动
    let s1 = String::from("hello");
    let s2 = s1; // s1 被移动到 s2，不能再使用 s1
    println!("s2 = {}", s2);

    // 克隆
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // 借用 (不可变)
    let s = String::from("borrow");
    let len = calculate_length(&s);
    println!("'{}' 长度 = {}", s, len);

    // 可变借用
    let mut s_mut = String::from("mut");
    change(&mut s_mut);
    println!("可变借用后: {}", s_mut);

    // 切片
    let s_slice = String::from("hello world");
    let word = first_word(&s_slice);
    println!("first word: {}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("able");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
