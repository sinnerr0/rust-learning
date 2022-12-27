pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_default(&self) -> String {
        String::from("(계속 읽기)")
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {}, ({})",
            self.headline,
            self.summarize_author(),
            self.location
        )
    }

    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}님의 기사 더 읽기", self.summarize_author())
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
