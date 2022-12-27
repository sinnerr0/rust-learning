use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("더 긴 문자열: {}", result);

    struct_lifetime();

    let result = longest_with_an_announcement("abcd", "xyz", "Test!!!");
    println!("더 긴 문자열: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> u32 {
        3
    }

    fn annouce_and_return_part(&self, annoucement: &str) -> &str {
        println!("주목해 주세요! {}", annoucement);
        self.part
    }
}

fn struct_lifetime() {
    let novel = String::from("스타워즈. 오래 전 멀고 먼 은하계에 ....");
    let first_sentence = novel
        .split(".")
        .next()
        .expect("문장에서 마침표'.'를 찾을 수 없습니다.");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!(
        "{} {} {}",
        i.part,
        i.level(),
        i.annouce_and_return_part("test!!!")
    );
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("주목하세요: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
