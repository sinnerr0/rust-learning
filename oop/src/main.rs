use oop::{Button, Post, Screen, SelectBox};

fn main() {
    opp_screen();
    oop_post();
}

fn opp_screen() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("예"),
                    String::from("아니요"),
                    String::from("모름"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("확인"),
            }),
        ],
    };

    screen.run();
}

fn oop_post() {
    let mut post = Post::new();
    post.add_text("나는 오늘 점심으로 샐러드를 먹었다.");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("나는 오늘 점심으로 샐러드를 먹었다.", post.content());
}
