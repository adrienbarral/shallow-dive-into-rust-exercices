
fn main() {

}

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn ex5_can_find_first_odd_number() {
        assert_eq!(None, find_first_odd_number(&vec![0,2,4,6]));
        assert_eq!(Some(1), find_first_odd_number(&vec![0,2,1,6]));
    }
}