use rand::Rng;
use std::cmp::Ordering; // bring Ordering. Less, Greater, Equal into scope
use std::io; // bring io library into scope if you want to use something not in the prelude // bring the Rng trait into scope

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // generate a random number between 1 and 100

    // println!("The secret number is: {}", secret_number); this prints the secret number

    loop {
        println!("Please input your guess.");

        // create a variable to store the guess
        let mut guess: String = String::new(); // the new function creates a new instance of a String
                                               // it is mutable because we want to change the value of guess later
                                               // :: syntax indicates that new is an associated function of the String type
                                               // mutabale variable that is currently bound to a new, empty instance of a String

        io::stdin()
            .read_line(&mut guess) // read_line takes whatever the user types into standard input and places that into a string
            .expect("Failed to read line"); // if this returns an error, the program will crash and display the message

        let guess: u32 = match guess.trim().parse(){
            // parse returns a Result type, which is an enum with the variants Ok or Err
            Ok(num) => num, // if parse is successful, it will return an Ok value that contains the number
            Err(_) => continue, // if parse fails, it will return an Err value that contains more information about the error
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"), // match expre
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        } // match expression is made up of arms
    }
}
