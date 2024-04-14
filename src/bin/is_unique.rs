use std::collections::HashMap;

fn main() {
    let mut dictionary: HashMap<char, bool> = HashMap::new();
    
    let word = "no";

    if word.len() > 26 {
        println!("not unique");
    } else {
        for char in word.chars() {
            if let Some(_) = dictionary.get(&char) {
                println!("not unique");
                return;
            } else {
                dictionary.insert(char, true);
            }
        }
        println!("unique");
    }
}
