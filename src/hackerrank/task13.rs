pub fn divisible_sum_pairs(k: i32, arr: &[i32]) -> i32 {
    let mut count = 0;

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if (arr[i] + arr[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_sum_pairs() {
        let arr = vec![1, 3, 2, 6, 1, 2];

        assert_eq!(divisible_sum_pairs(3, &arr), 5);
    }
}