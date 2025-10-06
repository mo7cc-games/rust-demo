// 04 - 所有权（Ownership）
// 说明：移动(move)、借用(borrow)、可变借用和引用规则示例

fn takes_ownership(s: String) {
    println!("takes_ownership 接收了：{}", s);
}

fn makes_copy(x: i32) {
    println!("makes_copy 接收了：{}", x);
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // s 在上面已被移动，此处不能使用 s

    let x = 5;
    makes_copy(x);
    println!("x 仍然可用：{}", x);

    // 借用
    let s2 = String::from("borrowed");
    let len = calculate_length(&s2);
    println!("s2 length = {}", len);

    // 可变借用
    let mut s3 = String::from("mut_str");
    change(&mut s3);
    println!("s3 = {}", s3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_changed");
}
