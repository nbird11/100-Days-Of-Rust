fn find_nemo(sentence: &str) -> String {
    let mut number = 0;
    for word in sentence.split_whitespace() {
        number += 1;
        if word == "Nemo" {
            return format!("I found Nemo at {number}!");
        }
    }
    return "I can't find Nemo :(".to_string();
}

fn main() {
    println!("Test 1: {}", find_nemo("I am finding Nemo"));
    println!("Test 2: {}", find_nemo("Nemo is me"));
    println!("Test 3: {}", find_nemo("I Nemo am"));
    println!("Test 4: {}", find_nemo("You won't find him here"));
}
