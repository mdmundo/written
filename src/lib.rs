#[cfg(test)]
mod tests {
    #[test]
    fn input_approach() {
        let input = "123";
        let mut number = input.chars().enumerate();
        assert_eq!(number.next(), Some((0, '1')));
        assert_eq!(number.next(), Some((1, '2')));
        assert_eq!(number.next(), Some((2, '3')));
    }

    #[test]
    fn loop_approach() {
        let number = "1".chars().enumerate();
        for unit in number {
            match unit {
                (pow, val) => {
                    // call fn here
                    assert_eq!(0, pow);
                    assert_eq!('1', val);
                }
            }
        }
    }

    #[test]
    fn approach() {
        fn unit(val: char) -> Option<&'static str> {
            match val {
                '1' => Some("Um"),
                _ => None,
            }
        }
        fn gen(pow: usize, val: char) -> Option<&'static str> {
            match pow {
                0 => unit(val),
                _ => None,
            }
        }
        assert_eq!(gen(0, '1'), Some("Um"));
    }
}
