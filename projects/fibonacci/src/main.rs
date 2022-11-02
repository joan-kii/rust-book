use std::io;


fn main() {
    println!("Fibonacci numbers!!!");
  
    let position: u16;

    println!("Enter a position between 0 and 24 to get the fibonacci value: ");

    loop {
        let mut str_position = String::new();

        io::stdin()
            .read_line(&mut str_position)
            .expect("Failed to read the input");

        position = match str_position.trim().parse() {
            Ok(num) => {
                if num > 24 {
                    println!("Please, enter a number between 0 and 24");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("Please, enter a number between 0 and 24");
                continue;
            }
        };
        break;
    }

    let result: u16 = get_fibonacci(position);

    println!("The fibonacci value is: {result}");
}

fn get_fibonacci(pos: u16) -> u16 {

    if pos <= 1 {
        pos 
    } else {
        get_fibonacci(pos - 1) + get_fibonacci(pos - 2)
    }
}
