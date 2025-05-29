use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Welcome to Guess the Number!\nPick a number between 1 and 100.");

    /**
        We use an unsigned 8-bit integer (u8) to conserve memory since our number is between 1 and 100.
        An unsigned 8-bit integer can represent values from 0 to 255 (2^8 - 1), which easily covers our range
        and avoids using unnecessary memory compared to larger integer types like u32.
    **/
    let secret_number: u8 = rand::rng().random_range(1..101);
    
    println!("The secret number is {}.", secret_number);
    
    let mut guess_number: String = String::new();

    loop {
        guess_number.clear();
        println!("Please input your guess.");
        io::stdin().read_line(&mut guess_number).expect("Failed to read line");

        let guess_number: u8 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        /**
            In the beginning I tried to convert &secret_number.to_string() and thus tried to compare them,
            but that failed to compare correctly as my guess_number, which is a string, has a \n to it.
        **/
        match guess_number.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }
    }
}