fn main(){

    let  word1 = "dop";
    let  word2 = "dog";

    let mut chars1:Vec<char> = word1.chars().collect();
    let mut chars2:Vec<char> = word2.chars().collect();


    chars1.sort();
    chars2.sort();

    if(chars1.len() != chars2.len()) {
        println!("not premutation");
    }else {
    for (idx,_) in chars1.iter().enumerate() {
        if chars1[idx].to_lowercase().next() != chars2[idx].to_lowercase().next() {
            println!("not premutation");
            return;
        }

    }
    println!("premutation found");
}
    
}