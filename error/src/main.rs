use core::panic;
use std::{
    fs,
    fs::File,
    io::{Error, ErrorKind, Read},
};

fn main() {
    // panic();
    error_result();
    error_propagation();
}

fn panic() {
    panic!("crash and burn");
}

fn error_result() {
    let file_name = "hello.txt";
    let f = File::open(file_name).unwrap();
    let f = File::open(file_name).expect("파일을 열 수 없습니다.");
    let f = File::open(file_name);
    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("파일을 생성하지 못했습니다: {:?}", e),
            },
            other_error => panic!("파일을 열지 못했습니다: {:?}", other_error),
        },
    };
}

fn error_propagation() {
    let file_name = String::from("hello.txt");
    // let r = read_usename_from_file_match(&file_name).expect("에러 발생");
    // let r = read_usename_from_file(&file_name).expect("에러 발생");
    // let r = read_usename_from_file_short(&file_name).expect("에러 발생");
    let r = read_usename_from_file_shortest(&file_name).expect("에러 발생");
    println!("파일 내용: {}", r);
}

fn read_usename_from_file_match(file_name: &String) -> Result<String, Error> {
    let f = File::open(file_name);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_usename_from_file(file_name: &String) -> Result<String, Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_usename_from_file_short(file_name: &String) -> Result<String, Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_usename_from_file_shortest(file_name: &String) -> Result<String, Error> {
    fs::read_to_string(file_name)
}
