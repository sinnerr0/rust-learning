fn main() {
    raw_pointer();
    unsafe_variable();
    trait_overroading();
    trait_identify1();
    trait_identify2();
    super_trait();
    newtype_pattern();
    function_pointer();
    closure_return();
}

fn raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }
}

static mut COUNTER: u32 = 0;

fn unsafe_variable() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    fn add_to_count(inc: u32) {
        unsafe { COUNTER += inc }
    }
}

fn trait_overroading() {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    #[derive(Debug, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }

    assert_eq!(Millimeters(4000) + Meters(1), Millimeters(5000));
}

fn trait_identify1() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("안녕하세요 기장입니다.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("날아라!")
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*양팔을 펄떡거린다*");
        }
    }

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

fn trait_identify2() {
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("점박이")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("멍멍이")
        }
    }

    println!("새끼 강아지 이름은 {}", <Dog as Animal>::baby_name());
}

fn super_trait() {
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    Point { x: 3, y: 1 }.outline_print();
}

fn newtype_pattern() {
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![
        String::from("안녕하세요"),
        String::from("러스트입니다."),
    ]);

    println!("w = {}", w);
}

fn function_pointer() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    println!("정답은 {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}

fn closure_return() {
    fn return_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    println!("{}", return_closure()(1));
}
