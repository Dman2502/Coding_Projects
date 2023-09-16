use std::io;
// this is a comment
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //'mut' makes a variable mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
