fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..];

    println!("{} {}", hello, world);

    let word = first_word(&s);
    println!("first word: {}", word);

    let literal = "hello world";
    let word = first_word(literal);
    println!("literal first word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
