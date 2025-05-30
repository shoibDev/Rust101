// A shopping cart project
use std::io;

fn main() {

    let mut shopping_cart: [String; 10] = Default::default();
    for i in 0..10 {
        shopping_cart[i] = "Empty".to_string();
    }

    show_items(&shopping_cart);
    add_item(&mut shopping_cart);
    show_items(&shopping_cart);


}

fn add_item(shopping_cart: &mut [String; 10]) -> bool {
    let mut idx: usize = 101; // If idx is still equal to 101, then it didn't find an open spot.
    // We have to let the idx be of usize (repersents the native word size)
    // during compile type it won't know the actual size of the array??
    // its jsut a standard way that rust has designed it, its universal saftey.

    for i in 0..10 {
        if shopping_cart[i] == "Empty" {
            idx = i;
            break;
        }
    }

    if idx == 101 {
        println!("Shopping cart is full! Please remove an item.");
        return false;
    } else {
        println!("What item would you like to add in the bag?");
        let mut item: String = String::new();
        io::stdin()
            .read_line(&mut item)
            .expect("Failed"); // Item has the \n character
        shopping_cart[idx] = item.trim().to_string(); // trims returns slice (&str), so we must convert it.
    }
    true
}

fn remove_item(shopping_cart: &mut [String; 10]) -> bool {
    for item in shopping_cart.iter() {
        if *item != "Empty".to_string(){
            return false;
        }
    }

    // Going to assume that he didn't pick smth empty...
    println!("Which item would you like to remove?");
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read");

    let choice: usize = match choice.trim().parse() {
        Ok(num) if num < shopping_cart.len() => num,
        _ => {
            println!("Invalid choice!");
            return false;
        }
    };

    /*
        Err(_) catches any error, if we do _ => its like checking for every situation.
        Like we put in "abc" it would also out the "invalid choice"
        I also noticed that if we were to use a varible without the mut keyword and placed that
        into a function that requries a buffer it doesnt work. its because we are referencing a place without memory
        if we only did let choice: String;..
    */
    shopping_cart[choice] = String::new();
    true
}

fn show_items(shopping_cart: &[String; 10]) {
    for item in shopping_cart.iter() {
        println!("{}", item);
    }
}
