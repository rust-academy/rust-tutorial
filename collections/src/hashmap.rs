use std::collections::HashMap;

pub fn test_hashmap() {
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue.clone(), 10);
    scores.insert(yellow.clone(), 50);

    println!("Scores");
    for (key, value) in scores.iter() {
        println!(" Team: {}, score: {}", key, value);
    }
}

pub fn test_update_hashmap() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrite existing score

    println!("Scores");
    for (key, value) in scores.iter() {
        println!(" Team: {}, score: {}", key, value);
    }
}

pub fn test_exists_and_if_not_create() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);

    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

pub fn test_string_word_count() {
    let text = "Hello world wonderful world Hello again Hello";

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in map.iter() {
        println!(" Word: {}, Count: {}", key, value);
    }
}
