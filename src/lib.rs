pub fn extended(input: &str) -> Result<String, &'static str> {
    let int = validator(input);
    if int.is_some() {
        let partial = parser_and_caller(int.unwrap().as_str());
        if partial.is_some() {
            Ok(partial.unwrap())
        } else {
            Err("Erro desconhecido")
        }
    } else {
        Err("Entrada inválida")
    }
}

fn validator(input: &str) -> Option<String> {
    // let int: usize = input.parse().ok()?;
    let int: usize = usize::from_str_radix(input, 10).ok()?;
    let as_str_again: String = int.to_string();
    Some(as_str_again)
}

fn parser_and_caller(input: &str) -> Option<String> {
    let numbers = input.chars().collect::<Vec<char>>();
    let chunks = numbers.rchunks(3).enumerate();
    match for_loop(chunks) {
        Some(result) => Some(result),
        None => None,
    }
}

fn for_loop(chunks: std::iter::Enumerate<std::slice::RChunks<char>>) -> Option<String> {
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

fn call_gen_higher(pow: usize, values: &[char]) -> Option<String> {
    match values {
        ['0', '0', '0'] => None,
        ['0', '0', '1'] => get_one_thousands(pow),
        ['0', '0', units] => get_units_thousands(pow, *units),
        ['0', '1', units] => get_teens_thousands(pow, *units),
        ['0', tens, units] => get_tens_thousands(pow, *tens, *units),
        ['1', '0', '0'] => get_hundred_thousands(pow),
        [hundreds, '1', units] => get_hundreds_teens_thousands(pow, *hundreds, *units),
        [hundreds, tens, units] => get_others_thousands(pow, *hundreds, *tens, *units),
        ['1', units] => get_teens_thousands(pow, *units),
        [tens, units] => get_tens_thousands(pow, *tens, *units),
        ['1'] => get_one_thousands(pow),
        [units] => get_units_thousands(pow, *units),
        [] => None,
        [..] => None,
    }
}

fn get_units_thousands(pow: usize, units: char) -> Option<String> {
    let thousand = get_thousands(pow, true);
    let units_extended = get_units(units);
    let mut result = String::new();
    if units_extended.is_some() {
        result.push_str(units_extended.unwrap());
    };
    if thousand.is_some() {
        result.push(' ');
        result.push_str(thousand.unwrap());
    };
    Some(result)
}

fn get_tens_thousands(pow: usize, tens: char, units: char) -> Option<String> {
    let thousand = get_thousands(pow, true);
    let tens_extended = get_tens(tens);
    let units_extended = get_units(units);
    let mut result = String::new();
    if tens_extended.is_some() {
        result.push_str(tens_extended.unwrap());
    };
    if units_extended.is_some() {
        result.push_str(" e ");
        result.push_str(units_extended.unwrap());
    };
    if thousand.is_some() {
        result.push(' ');
        result.push_str(thousand.unwrap());
    };
    Some(result)
}

fn get_others_thousands(pow: usize, hundreds: char, tens: char, units: char) -> Option<String> {
    let thousand = get_thousands(pow, true);
    let hundreds_extended = get_hundreds(hundreds);
    let tens_extended = get_tens(tens);
    let units_extended = get_units(units);
    let mut result = String::new();
    if hundreds_extended.is_some() {
        result.push_str(hundreds_extended.unwrap());
    };
    if tens_extended.is_some() {
        result.push_str(" e ");
        result.push_str(tens_extended.unwrap());
    };
    if units_extended.is_some() {
        result.push_str(" e ");
        result.push_str(units_extended.unwrap());
    };
    if thousand.is_some() {
        result.push(' ');
        result.push_str(thousand.unwrap());
    };
    Some(result)
}

fn get_hundreds_teens_thousands(pow: usize, hundreds: char, units: char) -> Option<String> {
    let thousand = get_thousands(pow, true);
    let hundreds_extended = get_hundreds(hundreds);
    let teens_extended = get_teens(units);
    let mut result = String::new();
    if hundreds_extended.is_some() {
        result.push_str(hundreds_extended.unwrap());
    };
    if teens_extended.is_some() {
        result.push_str(" e ");
        result.push_str(teens_extended.unwrap());
    };
    if thousand.is_some() {
        result.push(' ');
        result.push_str(thousand.unwrap());
    };
    Some(result)
}

fn get_teens_thousands(pow: usize, units: char) -> Option<String> {
    let thousand = get_thousands(pow, true);
    let teen = get_teens(units);
    let mut result = String::new();
    if teen.is_some() {
        result.push_str(teen.unwrap());
        if thousand.is_some() {
            result.push(' ');
            result.push_str(thousand.unwrap());
        };
    };
    Some(result)
}

fn get_one_thousands(pow: usize) -> Option<String> {
    let thousand = get_thousands(pow, false);
    let mut result = String::from("Um");
    if thousand.is_some() {
        result.push(' ');
        result.push_str(thousand.unwrap());
    };
    Some(result)
}

fn get_hundred_thousands(pow: usize) -> Option<String> {
    let thousand = get_thousands(pow, true);
    let mut result = String::from("Cem");
    if thousand.is_some() {
        result.push(' ');
        result.push_str(thousand.unwrap());
    };
    Some(result)
}

fn get_thousands(number: usize, many: bool) -> Option<&'static str> {
    match (number, many) {
        (0, _) => None,
        (1, _) => Some("Mil"),
        (2, false) => Some("Milhão"),
        (2, true) => Some("Milhões"),
        _ => None,
    }
}

fn get_hundreds(number: char) -> Option<&'static str> {
    match number {
        '0' => None,
        '1' => Some("Cento"),
        '2' => Some("Duzentos"),
        _ => None,
    }
}

fn get_teens(number: char) -> Option<&'static str> {
    match number {
        '0' => Some("Dez"),
        '1' => Some("Onze"),
        '2' => Some("Doze"),
        _ => None,
    }
}

fn get_tens(number: char) -> Option<&'static str> {
    match number {
        '0' => None,
        '2' => Some("Vinte"),
        _ => None,
    }
}

fn get_units(number: char) -> Option<&'static str> {
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
    fn dois_milhoes_e_dois_mil_e_dois() {
        let input = "2002002";
        let result = extended(input);
        assert_eq!(result.unwrap().as_str(), "Dois Milhões e Dois Mil e Dois");
    }

    #[test]
    fn um_milhao_e_um_mil_e_um() {
        let input = "1001001";
        let result = extended(input);
        assert_eq!(result.unwrap().as_str(), "Um Milhão e Um Mil e Um");
    }

    #[test]
    fn vinte_e_dois_milhoes_e_cem_mil_e_doze() {
        let input = "22100012";
        let result = extended(input);
        assert_eq!(
            result.unwrap().as_str(),
            "Vinte e Dois Milhões e Cem Mil e Doze"
        );
    }

    #[test]
    fn vinte_mil_e_um() {
        let input = "20001";
        let result = extended(input);
        assert_eq!(result.unwrap().as_str(), "Vinte Mil e Um");
    }

    #[test]
    fn dez() {
        let input = "10";
        let result = extended(input);
        assert_eq!(result.unwrap().as_str(), "Dez");
    }

    #[test]
    fn duzentos_e_vinte_e_dois_milhoes_e_cento_e_onze_mil_e_duzentos_e_doze() {
        let input = "222111212";
        let result = extended(input);
        assert_eq!(
            result.unwrap().as_str(),
            "Duzentos e Vinte e Dois Milhões e Cento e Onze Mil e Duzentos e Doze"
        );
    }

    #[test]
    fn cento_e_dez() {
        let input = "110";
        let result = extended(input);
        assert_eq!(result.unwrap().as_str(), "Cento e Dez");
    }

    #[test]
    fn cento_e_doze_mil_e_um() {
        let input = "112001";
        let result = extended(input);
        assert_eq!(result.unwrap().as_str(), "Cento e Doze Mil e Um");
    }

    #[test]
    fn um() {
        let input = "1";
        let result = extended(input);
        assert_eq!(result.unwrap().as_str(), "Um");
    }

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
