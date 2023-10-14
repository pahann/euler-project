pub fn run(until: i32) -> i32 {
    (1..until).filter(|num| num % 3 == 0 || num % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn base_case() {
        assert_eq!(23, super::run(10));
    }

    #[test]
    fn final_case() {
        assert_eq!(233168, super::run(1000));
    }
}
