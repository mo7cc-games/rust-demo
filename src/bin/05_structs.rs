// 05_structs.rs
// 说明: 结构体、实现块、关联函数、方法
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(username: &str, email: &str) -> User {
        User {
            username: String::from(username),
            email: String::from(email),
            sign_in_count: 1,
            active: true,
        }
    }

    fn display(&self) {
        println!("User: {} <{}> active={} count={}", self.username, self.email, self.active, self.sign_in_count);
    }
}

fn main() {
    let mut user1 = User::new("alice", "alice@example.com");
    user1.sign_in_count += 1;
    user1.display();

    // 元组结构体
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black = ({}, {}, {})", black.0, black.1, black.2);
}
