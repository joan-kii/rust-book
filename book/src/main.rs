enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("Method called!");
    }
}

fn main() {
    let m = Message::Write(String::from("My message"));
    m.call();
}
