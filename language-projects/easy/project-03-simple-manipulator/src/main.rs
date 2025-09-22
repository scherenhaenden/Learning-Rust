use std::io::stdin;

fn main() {

    println!("Enter a sentence: ");


    // read sentence from user
    let mut sentence = String::new();
    stdin().read_line(&mut sentence).expect("Failed to read from stdin");

    // trim the input to remove trailing newline and spaces
    let sentence = sentence.trim();

    // reverse the sentence
    let reversed_sentence: String = sentence.chars().rev().collect();

    // The assignment explicitly requires using to_string() to demonstrate ownership transfer
    let uppercased_sentence_owned = reversed_sentence.to_uppercase().to_string();

    // print the result
    println!("your sentence is: {}", uppercased_sentence_owned);

}
