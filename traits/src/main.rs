use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArtice {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArtice {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub usernmae: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.usernmae, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.usernmae)
    }
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// use aggregator::{SocialPost, Summary}

fn main() {
    let post = SocialPost {
        usernmae: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let article = NewsArtice {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburg, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            r"The Pittsburg Penguins once again are the best \ 
            hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
