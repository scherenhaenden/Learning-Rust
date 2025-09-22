use std::io::stdin;

fn main() {

    println!("Enter a sentence: ");


    // read sentence from user
    let mut sentence = String::new();
    stdin().read_line(&mut sentence).unwrap();

    // reverse the sentence
    let reversed_sentence: String = sentence.chars().rev().collect();

    // uppercase the sentence
    let uppercased_sentence_owned = reversed_sentence.to_uppercase().to_string();

    // print the result
    println!("your sentence is: {}", uppercased_sentence_owned);

}
