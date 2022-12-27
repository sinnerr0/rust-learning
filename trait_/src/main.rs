use trait_::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = return_summarize();

    println!("새 트윗 1개: {}", tweet.summarize());
    println!("{}", tweet.summarize_default());

    let article = NewsArticle {
        headline: String::from("대한민국, 러시아 월드컵 예선에서 독일을 이겼다."),
        location: String::from("카잔 아레나, 러시아"),
        author: String::from("위키백과"),
        content: String::from("2018년 6월 27일 러시아 카잔의 카잔 아레나에서 열린 2018 월드컵 F조 3차전 경기에서 대한민국이 독일에 2:0 승리를 거뒀다."),
    };

    notifyImplTrait(&article);
    notifyGenericTrait(&article);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = lagest(&number_list);
    println!("가장 큰 숫자: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = lagest(&char_list);
    println!("가장 큰 문자: {}", result);
}

fn notifyImplTrait(item: &impl Summary) {
    println!("속보! {}", item.summarize());
}

fn notifyGenericTrait<T: Summary>(item: &T) {
    println!("속보! {}", item.summarize());
}

fn return_summarize() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("러스트 언어 공부를 시작했습니다."),
        reply: false,
        retweet: false,
    }
}

fn lagest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
