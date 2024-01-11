pub fn solve(input: &str, santas: u32) -> u32 {
    let mut houses: u32 = 0;
    let mut position: usize = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if position == 0 && floor == -1 {
            position = i + 1;
        }
    }
    houses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(solve("(())").floor, 0);
        assert_eq!(solve("()()").floor, 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("(((").floor, 3);
        assert_eq!(solve("(()(()(").floor, 3);
        assert_eq!(solve("))(((((").floor, 3);
    }

    #[test]
    fn test_minus_1() {
        assert_eq!(solve("())").floor, -1);
        assert_eq!(solve("))(").floor, -1);
    }

    #[test]
    fn test_minus_3() {
        assert_eq!(solve(")))").floor, -3);
        assert_eq!(solve(")())())").floor, -3);
    }
}
