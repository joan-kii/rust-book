pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add_two() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 not equal 4"))
        }
    }
}
