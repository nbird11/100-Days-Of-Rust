use std::collections::hash_map::HashMap;
use reqwest::header::{HeaderMap, HeaderValue, HOST, X_RAPIDAPI_KEY};
use reqwest::Error;

fn get_possible_letters(digits: &str) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    if let Err(_) = digits.parse::<u64>() {
        panic!("`digits` needs to be a string containing only integers.")
    }

    let mut combos: HashMap<u8, String> = HashMap::new();
    combos.insert(2, String::from("abc"));
    combos.insert(3, String::from("def"));
    combos.insert(4, String::from("ghi"));
    combos.insert(5, String::from("jkl"));
    combos.insert(6, String::from("mno"));
    combos.insert(7, String::from("pqrs"));
    combos.insert(8, String::from("tuv"));
    combos.insert(9, String::from("wxyz"));
    let combos = combos;

    let mut letters_for_digits = Vec::new();

    for digit_char in digits.chars() {
        let digit: u8 = digit_char.to_digit(10).expect("Digit char should be able to become a u8 digit.") as u8;
        let letters_for_digit: &str = combos.get(&digit).expect(&format!("Digit must be 2-9; was {digit}"));
        letters_for_digits.push(String::from(letters_for_digit));
    }

    letters_for_digits
}

fn generate_combos(possible_letters: &Vec<String>, index: usize, current_combo: String, combos: &mut Vec<String>) {
    if index == possible_letters.len() {
        combos.push(current_combo);
        return;
    }

    let letters = possible_letters[index].chars().collect::<Vec<char>>();
    for letter in letters {
        let mut new_combo = current_combo.clone();
        new_combo.push(letter);
        generate_combos(possible_letters, index + 1, new_combo, combos);
    }
}

fn get_combos_for_letters(possible_letters: &Vec<String>) -> Vec<String> {
    if possible_letters.is_empty() {
        return vec![];
    }

    let mut combos: Vec<String> = vec![];
    generate_combos(possible_letters, 0, String::new(), &mut combos);
    combos
}

fn get_combos(digits: &str) -> Vec<String> {
    get_combos_for_letters(&get_possible_letters(digits))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://wordsapiv1.p.rapidapi.com/words/{}";

    let mut headers = HeaderMap::new();
    headers.insert(HOST, HeaderValue::from_static("wordsapiv1.p.rapidapi.com"));
    headers.insert(X_RAPIDAPI_KEY, HeaderValue::from_static("your-rapidapi-key"));

    let client = reqwest::Client::new();
    let response = client.get(url).headers(headers).send().await?;

    let body = response.text().await?;
    println!("body = {:?}", body);

    Ok(())
}
