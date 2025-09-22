use std::io::stdin;




fn main() {

    // inputs
    println!("Enter a first number: ");
    let mut first_number = String::new();
    stdin().read_line(&mut first_number).unwrap();
    let first_number: f64 = first_number.trim().parse().unwrap();

    println!("What kind of operation do you want to perform? (+, -, *, /): ");
    let mut operation = String::new();
    stdin().read_line(&mut operation).expect("Failed to read operation");
    let operation = operation.trim();

    println!("Enter a second number: ");
    let mut second_number = String::new();
    stdin().read_line(&mut second_number).unwrap();
    let second_number: f64 = second_number.trim().parse().unwrap();

    // Perform calculation based on operation (floating-point)
    let result = match operation {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => {
            if second_number == 0.0  {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            first_number / second_number
        },
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    // output
    println!("The result is: {} {} {} = {}", first_number, operation, second_number, result);
}
