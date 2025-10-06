// 03_control_flow.rs
// 说明: if/else, loop, while, for
fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("能被4整除");
    } else if number % 3 == 0 {
        println!("能被3整除");
    } else {
        println!("既不能被4也不能被3整除");
    }

    // loop + break with value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("loop 返回值: {}", result);

    // while
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }

    // for
    let a = [10, 20, 30, 40];
    for element in a.iter() {
        println!("数组元素: {}", element);
    }
}
