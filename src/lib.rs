#![forbid(unsafe_code)]

/// Map an integer to a ternary digit: -1, 0, or +1
pub fn ternary_value(n: i64) -> i8 {
    match n % 3 {
        0 => 0,
        1 | -2 => 1,
        -1 | 2 => -1,
        _ => 0, // unreachable for i64
    }
}

/// One step of the Collatz sequence: if even, n/2; if odd, 3n+1
pub fn collatz_step(n: i64) -> i64 {
    if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
}

/// Generate the Collatz sequence up to max_steps, stopping if we reach 1
pub fn collatz_sequence(start: i64, max_steps: usize) -> Vec<i64> {
    let mut seq = Vec::new();
    let mut n = start;
    seq.push(n);
    for _ in 0..max_steps {
        if n == 1 { break; }
        n = collatz_step(n);
        seq.push(n);
    }
    seq
}

/// Generate the ternary representation of a Collatz sequence
pub fn ternary_collatz_sequence(start: i64, max_steps: usize) -> Vec<i8> {
    collatz_sequence(start, max_steps)
        .iter()
        .map(|&n| ternary_value(n))
        .collect()
}

/// Check if a Collatz sequence diverges (doesn't reach 1 within max_steps)
pub fn diverges(start: i64, max_steps: usize) -> bool {
    let seq = collatz_sequence(start, max_steps);
    seq.last() != Some(&1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_value_zero() {
        assert_eq!(ternary_value(0), 0);
    }

    #[test]
    fn test_ternary_value_one() {
        assert_eq!(ternary_value(1), 1);
    }

    #[test]
    fn test_ternary_value_negative_one() {
        assert_eq!(ternary_value(-1), -1);
    }

    #[test]
    fn test_ternary_value_two() {
        assert_eq!(ternary_value(2), -1);
    }

    #[test]
    fn test_ternary_value_three() {
        assert_eq!(ternary_value(3), 0);
    }

    #[test]
    fn test_ternary_value_four() {
        assert_eq!(ternary_value(4), 1);
    }

    #[test]
    fn test_collatz_step_even() {
        assert_eq!(collatz_step(4), 2);
    }

    #[test]
    fn test_collatz_step_odd() {
        assert_eq!(collatz_step(3), 10);
    }

    #[test]
    fn test_collatz_step_one() {
        assert_eq!(collatz_step(1), 4);
    }

    #[test]
    fn test_collatz_sequence_basic() {
        let seq = collatz_sequence(6, 20);
        assert_eq!(seq[0], 6);
        assert_eq!(seq[1], 3);
        assert_eq!(seq[2], 10);
        assert!(seq.last() == Some(&1));
    }

    #[test]
    fn test_collatz_sequence_one() {
        let seq = collatz_sequence(1, 100);
        assert_eq!(seq.len(), 1);
        assert_eq!(seq[0], 1);
    }

    #[test]
    fn test_ternary_collatz_sequence() {
        let tseq = ternary_collatz_sequence(6, 20);
        let seq = collatz_sequence(6, 20);
        assert_eq!(tseq.len(), seq.len());
        assert_eq!(tseq[0], ternary_value(6));
    }

    #[test]
    fn test_diverges_false_for_known() {
        assert!(!diverges(27, 200));
    }

    #[test]
    fn test_diverges_large_with_small_steps() {
        // A very large number with only a few steps likely won't reach 1
        assert!(diverges(999999999999i64, 5));
    }

    #[test]
    fn test_collatz_sequence_negative() {
        // Negative Collatz: -5 → -14 → -7 → -20 → -10 → -5 (cycle)
        let seq = collatz_sequence(-5, 20);
        assert!(seq.len() > 1);
    }
}
