use std::collections::HashMap;
fn main() {
    let blue: String = String::from("Blue");
    let yellow: String = String::from("Yellow");
    
    let mut score: HashMap<String, i32> = HashMap::new();
    
    score.insert(blue, 10);
    score.insert(yellow, 10); // this will move the ownership of the variables into the hashmap
    
    let (blue, score) = score.remove_entry("Blue").unwrap_or_else(|| (String::from("Null"), 0));
    
}