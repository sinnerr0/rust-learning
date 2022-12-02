fn main() {
    println!("Hello, world!");
    // another_function();
    another_function(5);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("x, y의 값: {}, {}", x, y);

    let five = five();
    println!("five의 값: {}", five);
    let plus_one = plus_one(five);
    println!("plus_one의 값: {}", plus_one);

    // 주석
    // 추가 주석
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32) {
    println!("x의 값: {}", x);
}
