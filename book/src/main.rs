fn main() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("Even number {x}"),
        Some(y) => println!("Odd number {y}"),
        None => ()
    }
}
