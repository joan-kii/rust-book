use std::io;

fn main() {
    println!("Say something!");

   let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut words = String::new();

    io::stdin()
        .read_line(&mut words)
        .expect("Failed to read input");
    
    let mut word_list = Vec::new();
    let mut word = String::new();

    for letter in words.chars() {
        if letter != ' ' {
            word.push(letter);
        } else if letter == ' ' {
            word_list.push(word);
            word = String::from("");
        }
    }    
    
    word_list.push(word);
     
    for mut word in word_list {
        if vowels.iter().any(|&val| word.starts_with(val)) {
            word += "ay";
        }
        println!("{word}");
    }

    
}
