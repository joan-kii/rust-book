//! # My Crate
//! `my_crate` is a collection of utilities make performing certain 
//! calculations more convinient. 

/// Adds one to the number given.
/// # Example
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(5);
/// 
/// assert_eq!(answer, 6);
/// ```
pub fn add_one(x: u32) -> u32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
