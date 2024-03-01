// write a method that takes a slice to a string in argument and return a string slice to the first word in the string.
// If the argument is an empty string, the method should return a slice to an empty string.
// If the argument contains only one word, this word should be returned as a string slice.

fn main() {
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex5_1_can_find_first_word() {
        assert_eq!("Hello", find_first_word("Hello World"));
        assert_eq!("Hello", find_first_word("Hello"));
        assert_eq!("", find_first_word(""));
    }
}