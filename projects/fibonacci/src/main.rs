use std::io;


fn main() {
    println!("Fibonacci numbers!!!");
  
    let position: u8;

    println!("Enter a position between 0 and 255 to get the fibonacci value: ");

    loop {
        let mut str_position = String::new();

        io::stdin()
            .read_line(&mut str_position)
            .expect("Failed to read the input");

        position = match str_position.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number between 0 and 255");
                continue
            }
        };

        break;
    }

    let result: u16 = get_fibonacci(position);

    println!("The fibonacci value is: {result}");
}

fn get_fibonacci(pos: u8) -> u16 {

    let mut value: u16 = 0;

    if pos <= 1 {
        value = pos as u16;
        value
    } else {
        get_fibonacci((&value - 1).try_into().unwrap()) + get_fibonacci((&value - 2).try_into().unwrap())
    }

}
