use rand::Rng;
use std::cmp::Ordering;
use std::io;

// this is a comment
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate a random number between 1-100 inclusive

    println!("The secret number is: {secret_number}");

    loop { //loop until correct guess

        println!("Please input your guess.");

        let mut guess = String::new(); //'mut' makes a variable mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //shadow guess and turn into a number

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { //compare guess to secret number
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //end when correct guess
            }
        }
    }
}

