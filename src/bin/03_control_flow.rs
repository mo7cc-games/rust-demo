// 03 - 流程控制
// 说明：if/else、loop、while、for 示例

fn main() {
    let number = 7;
    if number < 5 {
        println!("小于 5");
    } else if number == 5 {
        println!("等于 5");
    } else {
        println!("大于 5");
    }

    // loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }
    println!("loop 结束，count={}", count);

    // while
    let mut n = 3;
    while n > 0 {
        println!("while: {}", n);
        n -= 1;
    }

    // for
    for i in 0..3 {
        println!("for: {}", i);
    }
}
