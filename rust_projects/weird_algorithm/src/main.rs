use std::io;

// An algorithm that takes as input a positive interger n
// between 1 and 10^6. If n is even, the algorithm devides it by two.
// If n is odd, the algorithm multiplies it by 3 and adds 1.
// The algorithm repeats this until n is one. 
// ex. input "3" 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1

fn main() {
    println!("Weird Algorithm!");
    
    // let sequence: [u32; 0] = [];
    // uncomment this^ when you figure it out
    // use a loop to check if even, odd, or equal to 1
    // append each term to an array then print the array
    // maybe move the input part into the loop?
    
    loop {
        let mut input_n = String::new();

        io::stdin()
            .read_line(&mut input_n)
            .expect("Failed to read line");

        println!("You input {input_n}");

        let mut input_n = match input_n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        while input_n > 1 {

        }

    }
}
