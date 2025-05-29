// A shopping cart project 
fn main() {
    let mut shopping_cart: [&str; 10] = ["Empty"; 10];
    show_items(shopping_cart);
}

fn show_items(shopping_cart: [&str; 10]) {
    for item in shopping_cart.iter() {
        println!("{}", item);
    }
}
