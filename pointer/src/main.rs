use std::rc::Rc;

fn main() {
    pointer_box();
    pointer_deref();
    pointer_rc();
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn pointer_box() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("list = {:?}", list);
}

fn pointer_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = Box::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("안녕하세요 {}!", name);
}

enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

fn pointer_rc() {
    let a = Rc::new(ListRc::Cons(
        5,
        Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil))),
    ));
    println!("a를 생성한 후의 카운터 = {}", Rc::strong_count(&a));
    let b = ListRc::Cons(3, Rc::clone(&a));
    println!("b를 생성한 후의 카운터 = {}", Rc::strong_count(&a));
    {
        let c = ListRc::Cons(4, Rc::clone(&a));
        println!("c를 생성한 후의 카운터 = {}", Rc::strong_count(&a));
    }
    println!("c범위를 벗어난 후의 카운터 = {}", Rc::strong_count(&a));
}
