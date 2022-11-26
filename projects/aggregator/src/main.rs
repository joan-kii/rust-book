use aggregator::{Summary, Tweet, NewArticle, notify};

fn main() {
    let tweet = Tweet {
        username: String::from("joankii"),
        content: String::from("I love Rust"),
        reply: false,
        retweet: false
    };

    let article = NewArticle {
        headline: String::from("The title"),
        author: String::from("Joankii"),
        location: String::from("Somewhere"),
        content: String::from("En un lugar de La Mancha...")
    };

    println!("New tweet: {}", tweet.summarize());
    println!("New article: {}", article.summarize());

    notify(&tweet);
}