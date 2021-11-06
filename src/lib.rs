//! Números naturais por extenso
//!
//! # Quick Start
//!
//! ```
//! use written::extended;
//!
//! let input = "9128204";
//! let result = extended(input);
//!
//! assert_eq!(
//!     result.unwrap().as_str(),
//!     "Nove Milhões e Cento e Vinte e Oito Mil e Duzentos e Quatro"
//! );
//! ```

#[cfg(test)]
mod tests;

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
    let int: u128 = u128::from_str_radix(input, 10).ok()?;
    let as_str_again: String = int.to_string();
    Some(as_str_again)
}

fn parser_and_caller(input: &str) -> Option<String> {
    let numbers = input.chars().collect::<Vec<char>>();
    let chunks = numbers.rchunks(3).enumerate();
    match triple_joiner(chunks) {
        Some(result) => Some(result),
        None => None,
    }
}

fn triple_joiner(chunks: std::iter::Enumerate<std::slice::RChunks<char>>) -> Option<String> {
    let mut joiner: Vec<String> = Vec::new();
    for number in chunks {
        let called_result = match number {
            (pow, values) => triple_generator(pow, values),
        };
        if called_result.is_some() {
            joiner.push(called_result.unwrap());
        };
    }
    joiner.reverse();
    let result = joiner.join(" e ");
    Some(result)
}

fn triple_generator(pow: usize, values: &[char]) -> Option<String> {
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
        ['0'] => Some("Zero".to_owned()),
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
        (3, false) => Some("Bilhão"),
        (3, true) => Some("Bilhões"),
        (4, false) => Some("Trilhão"),
        (4, true) => Some("Trilhões"),
        (5, false) => Some("Quatrilhão"),
        (5, true) => Some("Quatrilhões"),
        (6, false) => Some("Quintilhão"),
        (6, true) => Some("Quintilhões"),
        (7, false) => Some("Sextilhão"),
        (7, true) => Some("Sextilhões"),
        (8, false) => Some("Septilhão"),
        (8, true) => Some("Septilhões"),
        (9, false) => Some("Octilhão"),
        (9, true) => Some("Octilhões"),
        (10, false) => Some("Nonilhão"),
        (10, true) => Some("Nonilhões"),
        (11, false) => Some("Decilhão"),
        (11, true) => Some("Decilhões"),
        (12, false) => Some("Undecilhão"),
        (12, true) => Some("Undecilhões"),
        (13, false) => Some("Duodecilhão"),
        (13, true) => Some("Duodecilhões"),
        _ => None,
    }
}

fn get_hundreds(number: char) -> Option<&'static str> {
    match number {
        '0' => None,
        '1' => Some("Cento"),
        '2' => Some("Duzentos"),
        '3' => Some("Trezentos"),
        '4' => Some("Quatrocentos"),
        '5' => Some("Quinhentos"),
        '6' => Some("Seiscentos"),
        '7' => Some("Setecentos"),
        '8' => Some("Oitocentos"),
        '9' => Some("Novecentos"),
        _ => None,
    }
}

fn get_teens(number: char) -> Option<&'static str> {
    match number {
        '0' => Some("Dez"),
        '1' => Some("Onze"),
        '2' => Some("Doze"),
        '3' => Some("Treze"),
        '4' => Some("Quatorze"),
        '5' => Some("Quinze"),
        '6' => Some("Dezesseis"),
        '7' => Some("Dezessete"),
        '8' => Some("Dezoito"),
        '9' => Some("Dezenove"),
        _ => None,
    }
}

fn get_tens(number: char) -> Option<&'static str> {
    match number {
        '0' => None,
        '2' => Some("Vinte"),
        '3' => Some("Trinta"),
        '4' => Some("Quarenta"),
        '5' => Some("Cinquenta"),
        '6' => Some("Sessenta"),
        '7' => Some("Setenta"),
        '8' => Some("Oitenta"),
        '9' => Some("Noventa"),
        _ => None,
    }
}

fn get_units(number: char) -> Option<&'static str> {
    match number {
        '0' => None,
        '1' => Some("Um"),
        '2' => Some("Dois"),
        '3' => Some("Três"),
        '4' => Some("Quatro"),
        '5' => Some("Cinco"),
        '6' => Some("Seis"),
        '7' => Some("Sete"),
        '8' => Some("Oito"),
        '9' => Some("Nove"),
        _ => None,
    }
}
