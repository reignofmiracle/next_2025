use std::iter::Sum;

pub fn trait_main() {
    f1();
    f2();
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("default summary")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn f1() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new socal post: {}", post.summarize());
}

pub struct MyPost {}

impl Summary for MyPost {}

fn f2() {
    let post = MyPost {};

    println!("my post: {}", post.summarize());
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary + std::fmt::Display>(item1: &T, item2: &T) {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + std::fmt::Display,
    U: Clone + core::fmt::Debug,
{
    10
}
