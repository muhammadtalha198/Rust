pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This test should panic!");
    }

    #[test]
    fn greeting_contains_name() {
        let result = format!("Hello, Alice!");
        assert!(
            result.contains("Alice"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
