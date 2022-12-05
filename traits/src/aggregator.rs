use types::Summary;

use crate::types;

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
    println!();
}

pub fn summarize_sources() {
    let tweet = types::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(&tweet);

    let article = types::NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Ron Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    notify(&article);
}

pub fn returns_summarizable_tweet() -> impl types::Summary {
    types::Tweet {
        username: String::from("horse_ebooks"),
        content: "Event more horses, as you would have never guessed, people! ".to_string(),
        reply: false,
        retweet: false,
    }
}

