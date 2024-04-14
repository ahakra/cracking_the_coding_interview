fn urlify(input: &str) -> String {
    let mut result = String::new();
    let mut prev_char = ' ';

    for c in input.chars() {
        if c == ' ' && prev_char == ' ' {
            continue; 
        } else if c == ' ' {
            result.push_str("%20");
        } else {
            result.push(c);
        }
        prev_char = c;
    }

    result
}

fn main() {
    let input = "Mr Jonh   Smith          ";
    let urlified = urlify(input);
    println!("{}", urlified); 
}
