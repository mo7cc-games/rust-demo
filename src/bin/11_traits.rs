// 11_traits.rs
// 说明: trait 定义、实现、默认方法、trait 作为参数返回值
trait Summary {
    fn summarize(&self) -> String;
    fn author(&self) -> String { String::from("anonymous") }
}

struct Article {
    headline: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking: {}", item.summarize());
}

fn main() {
    let a = Article { headline: String::from("News"), author: String::from("Bob"), content: String::from("...") };
    notify(&a);
}
