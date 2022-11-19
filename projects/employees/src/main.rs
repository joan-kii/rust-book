use std::{io, collections::HashMap};

fn main() {
    let mut people = HashMap::new();

    people.insert(String::from("Engineering"), Vec::new());
    people.insert(String::from("Sales"), Vec::<String>::new());
    people.insert(String::from("Marketing"), Vec::new());

    let mut input = String::new();

    loop {
        println!("To add a new employee, use: Add Lisa to Marketing");
        println!("To get all employees, use: Get people");
        println!("To get employees from a department, use: Get people from Marketing");
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    }

    let input_list = input.split(" ");

    if input_list.len() != 2 || input_list.len() != 4 {
        continue
    }
        

    for word in input_list {
        println!("{word}");
    }
}
