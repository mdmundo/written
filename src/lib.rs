pub fn caller() {
    let numbers = "654312".chars().collect::<Vec<char>>();
    let chunks = numbers.rchunks(3).enumerate();
    let result = for_loop(chunks);
    println!("{}", result.unwrap());
}

pub fn for_loop(chunks: std::iter::Enumerate<std::slice::RChunks<char>>) -> Option<String> {
    let mut result = String::new();
    for number in chunks {
        let called_result = match number {
            (pow, values) => call_gen(pow, values).unwrap(),
        };
        result.push_str(if !result.is_empty() && !called_result.is_empty() {
            " e "
        } else {
            ""
        });
        result.push_str(called_result.as_str());
    }
    Some(result)
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
    // 1200112
    let mut result = String::new();
    result.push_str(hundreds.unwrap_or(""));
    result.push_str(if tens.is_none() { "" } else { " e " });
    result.push_str(tens.unwrap_or(""));
    result.push_str(if units.is_none() { "" } else { " e " });
    result.push_str(units.unwrap_or(""));
    result.push_str(append_thousands.unwrap_or(""));
    Some(result)
}

pub fn get_one_thousand(number: usize) -> Option<&'static str> {
    match number {
        0 => None,
        1 => Some("Mil"),
        2 => Some("Milhão"),
        _ => None,
    }
}

pub fn get_many_thousands(number: usize) -> Option<&'static str> {
    match number {
        0 => None,
        1 => Some("Mil"),
        2 => Some("Milhões"),
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
