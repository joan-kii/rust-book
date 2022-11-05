fn main() {
    let s = String::from("Hello");

    string_func(s);

    let x = 5;

    number_func(x);

    println!("x = {}", x);
}

fn string_func(string: String) {
    println!("{string}");
}

fn number_func(num: i32) {
    println!("{num}");
}
