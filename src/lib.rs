pub fn validator(input: &str) -> Option<String> {
    // let int: usize = input.parse().ok()?;
    let int: usize = usize::from_str_radix(input, 10).ok()?;
    let as_str_again: String = int.to_string();
    Some(as_str_again)
}

pub fn parser_and_caller(input: &str) -> Option<String> {
    let numbers = input.chars().collect::<Vec<char>>();
    let chunks = numbers.rchunks(3).enumerate();
    match for_loop(chunks) {
        Some(result) => Some(result),
        None => None,
    }
}

pub fn for_loop(chunks: std::iter::Enumerate<std::slice::RChunks<char>>) -> Option<String> {
    let mut joiner: Vec<String> = Vec::new();
    for number in chunks {
        let called_result = match number {
            (pow, values) => call_gen(pow, values),
        };
        if called_result.is_some() {
            joiner.push(called_result.unwrap());
        };
    }
    joiner.reverse();
    let result = joiner.join(" e ");
    Some(result)
}

pub fn call_gen_higher(pow: usize, values: &[char]) -> Option<String> {
    match values {
        ['0', '0', '0'] => None,
        ['1', '0', '0'] => None,
        ['0', '0', '1'] => None,
        [_, '1', _] => None,
        [_, _, _] => None,
        ['1', _] => None,
        [_, _] => None,
        ['1'] => None,
        [_] => None,
        [] => None,
        [..] => None,
    }
}

pub fn call_gen(pow: usize, values: &[char]) -> Option<String> {
    if values[0] == '0' && values[1] == '0' && values[2] == '0' {
        return None;
    };
    let append_thousands = if values[0] == '0' && values[1] == '0' && values[2] == '1' {
        get_one_thousand(pow)
    } else {
        get_many_thousands(pow)
    };
    let hundreds = get_hundreds(values[0]);
    let (tens, units) = if values[1] == '1' {
        (get_teens(values[2]), None)
    } else {
        (get_tens(values[1]), get_units(values[1]))
    };
    // return hundreds and tens and units thousands...
    // 100200112
    // 1200112
    let mut result = String::new();
    result.push_str(hundreds.unwrap_or(""));
    // result.push_str(if tens.is_none() { "" } else { " e " });
    result.push_str(tens.unwrap_or(""));
    // result.push_str(if units.is_none() { "" } else { " e " });
    result.push_str(units.unwrap_or(""));
    result.push_str(append_thousands.unwrap_or(""));
    Some(result)
}

pub fn get_hundred_thousands(pow: usize, number: char) -> Option<String> {
    let thousand = get_thousands(pow, true);
    let mut result = String::from("Cem");
    if thousand.is_some() {
        result.push(' ');
        result.push_str(thousand.unwrap());
    }
    Some(result)
}

pub fn get_thousands(number: usize, many: bool) -> Option<&'static str> {
    match (number, many) {
        (0, _) => None,
        (1, _) => Some("Mil"),
        (2, false) => Some("Milhão"),
        (2, true) => Some("Milhões"),
        _ => None,
    }
}

pub fn get_hundreds(number: char) -> Option<&'static str> {
    match number {
        '0' => None,
        '1' => Some("Cento"),
        '2' => Some("Duzentos"),
        _ => None,
    }
}

pub fn get_teens(number: char) -> Option<&'static str> {
    match number {
        '0' => Some("Dez"),
        '1' => Some("Onze"),
        '2' => Some("Doze"),
        _ => None,
    }
}

pub fn get_tens(number: char) -> Option<&'static str> {
    match number {
        '0' => None,
        '2' => Some("Vinte"),
        _ => None,
    }
}

pub fn get_units(number: char) -> Option<&'static str> {
    match number {
        '0' => None,
        '1' => Some("Um"),
        '2' => Some("Dois"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate() {
        assert_eq!(validator("123456"), Some(String::from("123456")));
        assert_eq!(validator("0000006"), Some(String::from("6")));
        assert_eq!(validator("o123456"), None);
    }

    #[test]
    fn caller1() {
        let numbers = "654312".chars().collect::<Vec<char>>();
        let chunks = numbers.rchunks(3).enumerate();
        let result = for_loop(chunks);
        println!("{}", result.unwrap());
    }

    #[test]
    fn caller2() {
        let numbers = "100200112".chars().collect::<Vec<char>>();
        let chunks = numbers.rchunks(3).enumerate();
        let result = for_loop(chunks);
        println!("{}", result.unwrap());
    }

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
}
