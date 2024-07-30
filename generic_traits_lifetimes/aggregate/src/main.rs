use aggregate::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("mike's_podcast"),
        content: String::from("as you probaly know people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
