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
            (pow, values) => call_gen_higher(pow, values),
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
        ['1', '0', '0'] => get_hundred_thousands(pow),
        ['0', '0', '1'] => get_one_thousands(pow),
        ['0', '1', number] => get_teen_thousands(pow, *number),
        [_, '1', number] => None,
        [_, _, _] => None,
        ['1', number] => get_teen_thousands(pow, *number),
        [_, _] => None,
        ['1'] => get_one_thousands(pow),
        [_] => None,
        [] => None,
        [..] => None,
    }
}

pub fn get_teen_thousands(pow: usize, number: char) -> Option<String> {
    let thousand = get_thousands(pow, true);
    let teen = get_teens(number);
    let mut result = String::new();
    if teen.is_some() {
        result.push_str(teen.unwrap());
        if thousand.is_some() {
            result.push(' ');
            result.push_str(thousand.unwrap());
        }
    }
    Some(result)
}

pub fn get_one_thousands(pow: usize) -> Option<String> {
    let thousand = get_thousands(pow, false);
    let mut result = String::from("Um");
    if thousand.is_some() {
        result.push(' ');
        result.push_str(thousand.unwrap());
    }
    Some(result)
}

pub fn get_hundred_thousands(pow: usize) -> Option<String> {
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
    fn doze_mil_e_cem() {
        let input = validator("12100");
        let result = parser_and_caller(input.unwrap().as_str());
        assert_eq!(result.unwrap().as_str(), "Doze Mil e Cem");
    }

    #[test]
    fn doze_mil_e_um() {
        let input = validator("12001");
        let result = parser_and_caller(input.unwrap().as_str());
        assert_eq!(result.unwrap().as_str(), "Doze Mil e Um");
    }

    #[test]
    fn cem_mil_e_doze() {
        let input = validator("100012");
        let result = parser_and_caller(input.unwrap().as_str());
        assert_eq!(result.unwrap().as_str(), "Cem Mil e Doze");
    }

    #[test]
    fn um_milhao_e_cem_mil_e_doze() {
        let input = validator("10001100");
        let result = parser_and_caller(input.unwrap().as_str());
        assert_eq!(result.unwrap().as_str(), "Dez Milhões e Um Mil e Cem");
    }

    #[test]
    fn dez_milhoes_e_um_mil_e_cem() {
        let input = validator("10001100");
        let result = parser_and_caller(input.unwrap().as_str());
        assert_eq!(result.unwrap().as_str(), "Dez Milhões e Um Mil e Cem");
    }

    #[test]
    fn one_hundred_thousand_and_one() {
        let input = validator("100001");
        let result = parser_and_caller(input.unwrap().as_str());
        assert_eq!(result.unwrap().as_str(), "Cem Mil e Um");
    }

    #[test]
    fn one_thousand_and_one_hundred() {
        let input = validator("1100");
        let result = parser_and_caller(input.unwrap().as_str());
        assert_eq!(result.unwrap().as_str(), "Um Mil e Cem");
    }

    #[test]
    fn validate() {
        assert_eq!(validator("123456"), Some(String::from("123456")));
        assert_eq!(validator("0000006"), Some(String::from("6")));
        assert_eq!(validator("o123456"), None);
    }
}
