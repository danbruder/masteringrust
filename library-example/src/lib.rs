#[allow(unused_variables)]
fn sum(a: i8, b: i8) -> i8 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::sum;
    #[test]
    fn sum_one_and_one_equals_two() {
        assert_eq!(sum(1, 1), 2);
    }
}
