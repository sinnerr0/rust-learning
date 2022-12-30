fn main() {
    if_let();
    while_let();
    for_loop();
    let_and_parameter();
    matching();
    destructure_struct();
    destructure_enum();
    ignore();
    match_guard();
    binding();
}

fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "32".parse();

    if let Some(color) = favorite_color {
        println!("선호하는 {}색을 배경으로 사용합니다.", color);
    } else if is_tuesday {
        println!("화요일엔 녹색이죠!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("보라색을 배경으로 사용합니다.");
        } else {
            println!("오렌지색을 배경으로 사용합니다.");
        }
    } else {
        println!("파란색을 배경으로 사용합니다.");
    }
}

fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loop() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("인덱스 {}의 값: {}", index, value);
    }
}

fn let_and_parameter() {
    let (x, y, z) = (1, 2, 3);
    println!("{} {} {}", x, y, z);

    let point = (3, 5);
    print_coordinates(&point);

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("현재 위치: ({}, {})", x, y);
    }

    let some_option_vlaue = Some(5);
    if let Some(x) = some_option_vlaue {
        println!("{}", x);
    }
}

fn matching() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("50"),
        Some(y) => println!("일치, y = {:?}", y),
        _ => println!("일치하지 않음, x = {:?}", x),
    }
    println!("결과: x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("1 or 2"),
        3 => println!("3"),
        _ => println!("그 외 나머지 값"),
    }

    let x = 5;
    match x {
        1..=5 => println!("1에서 5 중 하나"),
        _ => println!("그 외 나머지 값"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("ASCII 문자의 전반부"),
        'k'..='z' => println!("ASCII 문자의 후반부"),
        _ => println!("그 외 나머지 값"),
    }
}

fn destructure_struct() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("x 축 {}에 위치하는 점", x),
        Point { x: 0, y } => println!("y 축 {}에 위치하는 점", y),
        Point { x, y } => println!("좌표 ({}, {})에 위치하는 점", x, y),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{} {} {} {}", feet, inches, x, y);
}

fn destructure_enum() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u32, u32, u32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("Quit: 해체할 값이 없습니다."),
        Message::Move { x, y } => println!("Move: x = {}, y = {}", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("ChangeColor: R = {}, G = {}, B = {}", r, g, b),
    }
}

fn ignore() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("이미 설정된 값을 덮어쓸 수 없습니다."),
        _ => setting_value = new_setting_value,
    }
    println!("현재 설정: {:?}", setting_value);

    let _x = 5;

    let s = Some(String::from("안녕하세요"));
    if let Some(_) = s {
        println!("문자열을 찾았습니다.")
    }
    println!("{:?}", s);

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x = {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("first = {}, last = {}", first, last),
    }
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("5보다 작은 값: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("예"),
        _ => println!("아니요"),
    }
}

fn binding() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("id를 범위에서 찾았습니다: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("id를 다른 범위에서 찾았습니다."),
        Message::Hello { id } => println!("다른 id {}를 찾았습니다.", id),
    }
}
