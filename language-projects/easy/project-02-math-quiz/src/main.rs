





fn main() {

    // inputs
    println!("Enter a first number: ");
    let mut first_number = String::new();
    std::io::stdin().read_line(&mut first_number).unwrap();
    let first_number: i32 = first_number.trim().parse().unwrap();

    println!("what kind of operation do you want to perform? (+, -, *, /): ");
    let mut operation = String::new();
    std::io::stdin().read_line(&mut operation).unwrap();
    let operation = operation.trim();

    println!("Enter a second number: ");
    let mut second_number = String::new();
    std::io::stdin().read_line(&mut second_number).unwrap();
    let second_number: i32 = second_number.trim().parse().unwrap();

    // validation and floating point division
    let result = match operation {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => {
            if second_number == 0 {
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
