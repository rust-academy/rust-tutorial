mod types;
mod aggregator;
mod pair;

use types::Summary;

fn main() {
    test_summary();
    test_summarize_return_type();
}

fn test_summary() {
    aggregator::summarize_sources();
}

fn test_summarize_return_type(){
    let tweet = aggregator::returns_summarizable_tweet();
    println!("Tweet summary: {}", tweet.summarize());

}
