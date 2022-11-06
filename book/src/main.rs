fn main() {
    let x = Some(5);
    plus_one(x);
    plus_one(None);
}

fn plus_one(x: Option<i32>) {
    if let Some(val) = x {
        println!("{:?}", val);
    } else {
        println!("None")
    }
}
