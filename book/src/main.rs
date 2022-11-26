fn main() {
    
    let result;
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());

    println!("The longest string: {}", result);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
