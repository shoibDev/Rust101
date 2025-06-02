use std::io;
// Break Expression into tokens
#[derive(Debug)]
enum Token {
    Number(u8),
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LParentheses,
    RParentheses,
}
fn main() {

    let mut token_list: Vec<Token> = Vec::new();
    let mut expression: String = String::new();

    println!("Enter expression to be evaluated: ");

    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line");
    expression = expression.trim().to_string();

    // Tokenize the expression
    for char in expression.chars() {
        if char == ' ' { continue }
        match char {
            '+' => token_list.push(Token::Addition),
            '-' => token_list.push(Token::Subtraction),
            '*' => token_list.push(Token::Multiplication),
            '/' => token_list.push(Token::Division),
            '(' => token_list.push(Token::LParentheses),
            ')' => token_list.push(Token::RParentheses),
            '0'..='9' => {
                let digit = char.to_digit(10).unwrap() as u8;
                token_list.push(Token::Number(digit));
            }
            _ => {
                println!("Unexpected character: {}", char);
            }
        }
    }
}