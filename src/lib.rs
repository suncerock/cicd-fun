pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(left: usize) -> usize {
    left + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_add_two() {
        let result = add_two(5);
        assert_eq!(result, 7);
    }
}
