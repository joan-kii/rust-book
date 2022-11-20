struct Item<T, U> {
    Number: T,
    Char: U
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("The largest number is: {}", largest(&numbers));

    let chars = vec!['c', 'y', 'w', 'a', 'h'];
    println!("The largest character is: {}", largest(&chars));
}

fn largest<Item>(list: &[Item<T, U>]) -> &<Item> {
    let mut max = &list[0];

    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}
