use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("인수를 구문분석하는 동안 오류가 발생되었습니다: {}", err);
        process::exit(1);
    });
    println!("검색어: {}", config.query);
    println!("대상 파일: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
