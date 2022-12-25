use std::collections::HashMap;

fn main() {
    vector();
    vector_reference();
    vector_iterate();
    vector_enums();
    string();
    hashmap();
    hashmap_manipulate();
}

fn vector() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
}

fn vector_reference() {
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("세 번째 원소: {}", third);

    match v.get(2) {
        Some(third) => println!("세 번째 원소: {}", third),
        None => println!("세 번째 원소가 없습니다."),
    }
}

fn vector_iterate() {
    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}

fn vector_enums() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("블루")),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(v) => println!("{}", v),
            SpreadsheetCell::Float(v) => println!("{}", v),
            SpreadsheetCell::Text(v) => println!("{}", v),
        }
    }
}

fn string() {
    let data = "문자열 초기값";
    let s = data.to_string();
    let ss = String::from("문자열 초기값");
    assert_eq!(s, ss);

    let mut s = String::new();
    s.push_str("foo");
    s.push('!');
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = "안녕하세요";
    for c in hello.chars() {
        println!("{}", c);
    }
}

fn hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("옐로"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("블루"), String::from("옐로")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let team_name = String::from("블루");
    let score = scores.get(&team_name);
    println!("{}", score.unwrap());

    for (k, v) in &scores {
        println!("{}, {}", k, v);
    }
}

fn hashmap_manipulate() {
    let mut scores = HashMap::new();
    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("블루"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("옐로")).or_insert(50);
    scores.entry(String::from("블루")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
