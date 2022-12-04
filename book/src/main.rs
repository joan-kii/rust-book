struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let stuff = CustomSmartPointer {
        data: String::from("my stuff")
    };

    let other_stuff = CustomSmartPointer {
        data: String::from("my other stuff")
    };

    println!("CustomSmartPointers created!");
}
