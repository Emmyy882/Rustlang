pub mod tweet;
pub mod news_article;

pub trait Summary {
    fn summarize(&self) -> String;
}
