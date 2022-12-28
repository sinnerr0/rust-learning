use docs::{mix, PrimaryColor};

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let secondary = mix(red, yellow);
    println!("{:?}", secondary);
}
