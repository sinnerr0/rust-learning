fn main() {
    let number = 3;

    if number < 5 {
        println!("조건이 일치합니다.")
    } else {
        println!("조건이 일치하지 않습니다.")
    }

    if number != 0 {
        println!("변수에 저장된 값이 0이 아닙니다.")
    }

    if number % 2 == 0 {
        println!("변수가 2로 나누어 떨어집니다.")
    } else if number % 3 == 0 {
        println!("변수가 3로 나누어 떨어집니다.")
    }

    let number = if true { 100 } else { 200 };
    println!("number값: {}", number)
}
