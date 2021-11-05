#[cfg(test)]
mod tests {
    #[test]
    fn correct_approach() {
        let numbers = "654312".chars().collect::<Vec<char>>();
        let chunks = numbers.rchunks(3).enumerate();

        for number in chunks {
            match number {
                (pow, values) => println!("{} {:#?}", pow, values),
            }
        }
    }

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
    fn gen_approach() {
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
        let number = "1".chars().enumerate();

        let mut result = String::new();
        for unit in number {
            if !result.is_empty() {
                result.push_str(" e ");
            }
            result.push_str(match unit {
                (pow, val) => gen(pow, val).expect("Invalid"),
            })
            // this approach is wrong.
            // how 19 would be?
        }
    }
}
