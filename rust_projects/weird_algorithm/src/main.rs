use std::io;

// An algorithm that takes as input a positive interger n
// between 1 and 10^6. If n is even, the algorithm devides it by two.
// If n is odd, the algorithm multiplies it by 3 and adds 1.
// The algorithm repeats this until n is one. 
// ex. input "3" 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1

fn main() {
    println!("Enter a positive number between 1 and 10^6");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");


    // Turn n into a number
    // let n: u32 = match n.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => break,
    // };
    println!("You entered: {n}");
}
