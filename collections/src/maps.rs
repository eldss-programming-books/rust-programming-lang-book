use std::collections::HashMap;

pub fn maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // Can also form from two vectors
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let scores_vec = vec![10, 50];
    let _scores2: HashMap<_, _> = teams.iter().zip(scores_vec.iter()).collect();

    let team3 = String::from("Green");
    scores.insert(team3, 90); // team3 var gives up ownership here

    // Getting values
    let team_name = "Blue";
    let blue_score = scores.get(team_name);
    println!("Blue: {}", blue_score.unwrap());

    // Loop through a map
    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }

    // Overwrite a value
    println!("Yellow score before: {}", scores.get("Yellow").unwrap());
    scores.insert(String::from("Yellow"), 120);
    println!("Yellow score after: {}", scores.get("Yellow").unwrap());

    // Only insert if not exists
    scores.entry("Yellow".to_string()).or_insert(50);
    scores.entry("Black".to_string()).or_insert(50);
    println!("{:?}", scores);

    // Update from an existing value
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count);
}
