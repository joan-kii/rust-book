use add_one;
use add_two;
use rand;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus 1 is {}", add_one::add_one(num));
    println!("Hello, world! {num} plus 2 is {}", add_two::add_two(num));
}
