use aggregate_2::Summary;

use std::fmt;
fn returns_summarizable<T: fmt::Debug>() -> impl Summary {
    use aggregate_2::tweet::Tweet;
    Tweet {
        username: String::from("mike's_podcast"),
        content: String::from("as you probably know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    println!("{:?}", returns_summarizable());
}
