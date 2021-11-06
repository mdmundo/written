use super::*;
mod consts;

#[test]
fn entrada_invalida() {
    assert_eq!(currency("100", "500"), Err("Entrada Inválida"));
}

#[test]
fn cem_reais_e_cinquenta_centavos() {
    assert_eq!(
        currency("100", "50").unwrap().as_str(),
        "Cem Reais e Cinquenta Centavos"
    );
}

#[test]
fn consts_largest() {
    let input = u128::MAX.to_string();
    let input_str = input.as_str();
    let result = extended(input_str);
    assert_eq!(result.unwrap().as_str(), consts::LARGEST);
}

#[test]
fn quinhentos_milhoes_e_trezentos_mil_e_cem() {
    let input = "500300100";
    let result = extended(input);
    assert_eq!(
        result.unwrap().as_str(),
        "Quinhentos Milhões e Trezentos Mil e Cem"
    );
}

#[test]
fn dezesseis_milhoes_e_quinze_mil_e_dezenove() {
    let input = "16015019";
    let result = extended(input);
    assert_eq!(
        result.unwrap().as_str(),
        "Dezesseis Milhões e Quinze Mil e Dezenove"
    );
}

#[test]
fn cento_e_sessenta_e_nove_milhoes_e_cento_e_trinta_e_oito_mil_e_duzentos_e_noventa_e_quatro() {
    let input = "169138294";
    let result = extended(input);
    assert_eq!(
        result.unwrap().as_str(),
        "Cento e Sessenta e Nove Milhões e Cento e Trinta e Oito Mil e Duzentos e Noventa e Quatro"
    );
}

#[test]
fn nove_milhoes_e_cento_e_vinte_e_oito_mil_e_duzentos_e_quatro() {
    let input = "9128204";
    let result = extended(input);
    assert_eq!(
        result.unwrap().as_str(),
        "Nove Milhões e Cento e Vinte e Oito Mil e Duzentos e Quatro"
    );
}

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
