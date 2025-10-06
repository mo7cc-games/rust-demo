// 06_enums_pattern_matching.rs
// 说明: 枚举及模式匹配 (match, if let)
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {} {}", x, y),
        Message::Write(s) => println!("Write: {}", s),
        Message::ChangeColor(r, g, b) => println!("Color: {} {} {}", r, g, b),
    }
}

fn main() {
    let m = Message::Move { x: 10, y: 20 };
    process(m);

    let opt: Option<i32> = Some(5);
    match opt {
        Some(x) if x > 3 => println!("大于3: {}", x),
        Some(x) => println!("Some: {}", x),
        None => println!("None"),
    }

    // if let
    if let Some(5) = opt {
        println!("正好是5");
    }
}
