fn main() {

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    let third: &i32 = &v[2]; // Immutable borrow here
    v.push(6);         // Mutable borrow here 

    println!("the third element is: {}", third); // refenching the immuatbla refeence


}