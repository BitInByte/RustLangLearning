// io library (input/output) comes from the standard library (std)
// used for reading input lines and check it
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // Mutable variable
        // String::new() returns a new instace of a String.
        // :: means that we are kind of calling a static method
        // from the String class
        let mut guess = String::new();

        // Stdin is used to get the input of the user
        io::stdin()
            // read_line will read the input from the user
            // The argument needs to be mutable so that
            // the method read_line can change the string's
            // content by adding the user input
            // The & indicates that this argument is a reference
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Converting guess to a number
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
