use std::io;
// The io library is an input/output library
// It comes from the standard library std

fn main(){
    println!("Guess the fucking number! ");

    println!("Please input your guess.");

    // guess = variable name
    let mut guess = String::new();
    
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}