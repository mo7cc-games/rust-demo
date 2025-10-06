// 06 - 枚举与模式匹配（Enums & pattern matching）
// 说明：定义枚举、match、if let 示例

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
    process(Message::Move { x: 10, y: 20 });
    process(Message::Quit);
    process(Message::Write(String::from("hello")));
    process(Message::ChangeColor(255, 0, 0));

    let some_option = Some(5);
    if let Some(v) = some_option {
        println!("if let matched {}", v);
    }
}
