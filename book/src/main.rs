struct Point (i32, i32, i32);

fn main() {
    let new_point: Point = Point(12, 24, 48);
    
    println!("First value: {}", new_point.0);
}
