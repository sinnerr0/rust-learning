fn main() {
    let mut x = 5;
    println!("x 값: {}", x);
    x = 6;
    println!("x 값: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS 값: {}", MAX_POINTS);

    let x = x + 1;
    let x = x + 2;
    println!("x 값: {}", x);

    let spaces = "            ";
    let spaces = spaces.len();
    println!("spaces 값: {}", spaces);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("x, y 값: {}, {}", x, y);

    let sum = 5 + 10;
    println!("sum 값: {}", sum);
    let difference = 95.5 - 4.3;
    println!("difference 값: {}", difference);
    let product = 4 * 30;
    println!("product 값: {}", product);
    let remainder = 43 % 5;
    println!("remainder 값: {}", remainder);

    let t = true;
    let f = false;
    println!("t, f 값: {}, {}", t, f);

    let c = 'z';
    let z = '⃤';
    println!("c, z 값: {}, {}", c, z);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x, y, z 값: {}, {}, {}", x, y, z);
    println!("x, y, z 값: {}, {}, {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3, 4, 5];
    println!(
        "arr 값: {},{},{},{},{}",
        arr[0], arr[1], arr[2], arr[3], arr[4]
    );
    let arr = [3; 5];
    println!(
        "arr 값: {},{},{},{},{}",
        arr[0], arr[1], arr[2], arr[3], arr[4]
    );
}
