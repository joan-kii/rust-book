fn main() {
    let mut numbers = vec![40, 20, 20, 70, 60, 50, 30, 30];
    numbers.sort();

    println!("The median is: {}", get_median(&numbers));
    println!("The mode is: {}", get_mode(&numbers));
}

fn get_median(vec: &Vec<i32>) -> i32 {
    vec[vec.len() / 2]
}

fn get_mode(vec: &Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut values = HashMap::new();

    for val in vec {
        let count = values.entry(val).or_insert(0);
        *count += 1;
    }
    
    let mut max_value = 0;
    let mut number = 0;

    for (k, v) in values {
        if v > max_value {
            max_value = v;
            number = *k;
        }
    }
    number
}