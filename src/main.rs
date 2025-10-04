fn main() {
    println!("Hello, world!");
    // 命令行会立马退出
    std::thread::sleep(std::time::Duration::from_secs(10));
}
