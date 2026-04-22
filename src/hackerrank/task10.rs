pub fn sock_merchant(socks: &[i32]) -> i32 {
    let mut counts = std::collections::HashMap::new();

    for &sock in socks {
        *counts.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;

    for &count in counts.values() {
        pairs += count / 2;
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        let socks = vec![1, 2, 1, 2, 1, 3, 2];
        assert_eq!(sock_merchant(&socks), 2);
    }
}