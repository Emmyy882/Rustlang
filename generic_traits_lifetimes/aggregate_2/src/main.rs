#[allow(unused_imports)]
use aggregate_2::{Summary, Tweet, NewsArticle};

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = returns_summarizable();
    println!("{}", tweet.summarize());
}
