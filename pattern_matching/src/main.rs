fn main() {
    let value = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Coin value is {}", value);

    print_plus_one();
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn print_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    print_option(&six);
    print_if_let(&six);

    let none = plus_one(None);
    print_option(&none);
    print_if_let(&none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_option(v: &Option<i32>) {
    match v {
        Some(v) => println!("{}", v),
        None => println!("None"),
    }
}

fn print_if_let(v: &Option<i32>) {
    if let Some(v) = v {
        println!("{}", v);
    } else {
        println!("None");
    }
}
