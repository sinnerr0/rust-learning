fn main() {
    working();
    reference_example();
}

// fn not_working() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
//     println!("{}, world!", s2);
// }

fn working() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);
    println!("{}, world!", s2);
}

fn reference_example() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{}'의 길이는 {}입니다.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("가변 참조: {}", s);
}

/// 매개변수를 참조로 전달하는 것을 borrowing이라고 한다.
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// 가변 참조
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
