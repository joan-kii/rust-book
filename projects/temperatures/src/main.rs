use std::io;


fn main() {
    println!("Convert temperatures!");

    let mut mode: u8;
    
    loop {
        println!("Fahrenheit to Celsius: Enter 1");
        println!("Celsius to Fahrenheit: Enter 2");

        let mut str_mode = String::new();

        io::stdin()
            .read_line(&mut str_mode)
            .expect("Failed to read the input");

        mode = match str_mode.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if mode == 1 || mode == 2 {
            break;
        }
        println!("Please, enter 1 or 2");
        continue;
    }

    loop {
        let mut temp = String::new();
    
        println!("Enter the temperature");
    
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read the input");

        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if mode == 1 {
            println!("{}ºF are {}ºC", temp, (temp - 32) * 5 / 9);
        } else {
            println!("{}ºC are {}ºF", temp, (temp * 9 / 5) + 32);
        }
        break;
    }
}
