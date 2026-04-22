pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for &d in apples {
        let pos = a + d;
        if pos >= s && pos <= t {
            apple_count += 1;
        }
    }

    for &d in oranges {
        let pos = b + d;
        if pos >= s && pos <= t {
            orange_count += 1;
        }
    }

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apples_and_oranges() {
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let result = count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);

        assert_eq!(result, (1, 1)); // ✅ правильне значення
    }
}