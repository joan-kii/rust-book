struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40
    };

    let rect3 = Rectangle {
        width: 40,
        height: 60
    };

    let square = Rectangle::square(20);

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
    println!("The size of the square is: {} x {}", square.width, square.height);
}
