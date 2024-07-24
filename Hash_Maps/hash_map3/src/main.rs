/*
 * Program to add a key and value only if the key isn't present
 * if the key does exists in the hash map, the existing value should..
 * ....remain the way it is
 */
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new(); // creating new instance of HashMap
    let team_1 = String::from("Blue");

    scores.insert(team_1, 10);
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(20);

    println!("scores : {:#?}", scores);
}
