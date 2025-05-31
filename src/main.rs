fn main() {
    /*
        Ownership rules:
        1. Each value in Rust has a variable that's called its owner.
        2. There can only be one owner at a time.
        3. When the owner goes out of scope, the value will be dropped.
        4. No more than 1 mutable reference per data per scope, thus insuring no data racing...
    */

    { // Creating a new scope
        // let s: &str = "hello"; // This is stored in binary in the stack.
        // let s: String = String::from("Hello"); // This is now stored on the heap.
    } // This scope is now over, and s is no longer valid.

    /*
    let x: i32 = 5;
    let y: i32 = x; // Copy

    let s1: String = String::from("hello");
    let s2: String = s1; // Move (not a shallow copy), s2 took over ownership.
    */

    /*
    let s: String = String::from("hello");
    {
        let x = s; We take ownership of s
        println!("{}", x);
    } // end of scope we drop x, therefore, we also drop s
    println!("{}", s); // This will give a compilation error/
    */

    /*
    let s1: String = String::from("hello");
    let len: usize = calculate_len(&s1);
    println!("The length of '{}' is {}.", s1, len);
    */
    /*
    let mut s: String = String::from("hello");

    let r1: &String = &s;
    let r2: &String = &s;
    // let r3: &mut String = &mut s; // This will give a compilation error as you can't mix mutable
                                    // With immutable references

    println!("{}, {}", r1, r2);

    let r3: &mut String = &mut s; // We can use it here as r1 and r2 are out of scope now (We don't use it after we set r3)

    */

    // let reference_to_nothing: &String = dangle();

    let mut s: String = String::from("hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..s.len()];
    
    let word: usize = first_word(&s);
    s.clear();

}
/*
fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
*/
/*
fn calculate_len(s: &String) -> usize {
    s.len()
}
*/
//
/*fn dangle() -> &String {
    let s: String = String::from("hello"); // This Value will get dropped so our pointer will be pointing to nothing...
    &s
}*/

fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for byte in bytes.iter() {
        println!("{}", byte); // Each letter has its own ascii value.   
    }

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // convert the space into a ascii value
            return i;
        }
    }
    s.len()
    
    /*
        First problem the return value is not tied to the string.
        Therefore we have to manauly keep each word lengght insync with the string 
        Second problem is that, wehat if we wanted to to get the 2nd word?
    */
}
