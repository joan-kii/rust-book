use std::{io, collections::HashMap};

fn main() {
    let mut people = HashMap::new();

    people.insert(String::from("engineering"), vec!["Sadie".to_string()]);
    people.insert(String::from("sales"), vec!["Millie".to_string()]);
    people.insert(String::from("marketing"), vec!["Noah".to_string()]);

    let mut input = String::new();
    let mut input_list: Vec<String>;

    loop {
        println!("To add a new employee, use: Add Lisa to Marketing/Engineering/Sales");
        println!("To get all employees, use: Get people");
        println!("To get employees from a department, use: Get people from Marketing");
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    
        input = input.trim().to_string().to_lowercase();
        input_list = input.split(" ").map(|w| w.to_string()).collect();
    
        if input_list.len() == 2 && input == "get people" {
            get_people(&people);
            break;
        } else if input_list[0] == "get" && input_list.len() == 4 {
            get_employees(&people, &input_list[3]);
            break;
        } else if input_list[0] == "add" && input_list.len() == 4 {
            people.get_mut(&input_list[3]).unwrap().push(input_list[1].to_string());
            println!("{} added to {}!", input_list[1], input_list[3]);
            break;
        } else {
            println!("Please, follow the instructions!");
            continue;
        }
    }
}

fn get_people(people: &HashMap<String, Vec<String>>) {
    for (k, v) in people {
        for emp in v {
            println!("{}: {}", k, emp);
        }
    }
}


fn get_employees(people: &HashMap<String, Vec<String>>, dept: &String) {
    for v in people.get(dept) {
        for emp in v {
            println!("{}: {}", dept, emp);
        }
    }
}
