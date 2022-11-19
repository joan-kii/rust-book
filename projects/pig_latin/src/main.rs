use std::io;

fn main() {
    println!("Say something!");
    let mut words = String::new();
    io::stdin()
        .read_line(&mut words)
        .expect("Failed to read input");
    println!("{words}");
}
