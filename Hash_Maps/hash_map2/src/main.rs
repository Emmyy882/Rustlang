use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 15);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score for team {} : {}\n", team_name, score);

    println!("Printing the content of the scores container");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
