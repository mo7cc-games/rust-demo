// 11 - Trait（类似接口）
// 说明：定义 trait、实现 trait、默认实现、trait 作为参数

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, &self.content[..10.min(self.content.len())])
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

fn main() {
    let article = NewsArticle { headline: String::from("Title"), content: String::from("Lots of content here") };
    notify(&article);
}
