use std::io;

fn main() {
    println!("Say something!");

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let mut word_list = Vec::new();
    let mut word = String::new();

    for letter in input.trim().chars() {
        if letter != ' ' {
            word.push(letter);
        } else if letter == ' ' {
            word_list.push(word);
            word = String::from("");
        }
    }    
    
    word_list.push(word);
     
    for word in word_list {
        if !vowels.iter().any(|&val| word.starts_with(val)) {
            println!("{}", format!("{}{}{}{}", &word[1..], "-", &word[..1], "ay"));
        } else {
            println!("{}", format!("{}{}", &word, "-hay"));
        }
    }
}
