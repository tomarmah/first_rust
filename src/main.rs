use rand::Rng;
use std::cmp;
use std::cmp::Ordering;
use std::io;

fn main() {
    /**
    //This is not an assignment, it's a binding in rust. It tells us who is responsible for this data and where it is stored.
    //Variables are immutable by default.
    let surname = "Armah";

    //To make a variable mutable, we use the mut keyword.
    let mut firstname = "Tom";
    let _ = firstname.to_lowercase();
    // firstname = firstname + " & Jerry";

    //println! is a macro that prints to the console. Macros can be extended to do more. Macros are invoked by exclamation mark.
    println!("Hello, {firstname} {surname}!");
    println!(
        "You can also say Hello this way, {} {}!",
        firstname,
        surname.to_lowercase()
    );

    //Arrays
    let numbers = [1, 2, 3, 4, 5];
    println!("The numbers are: {numbers:?}");

    //Play around
    let mut age = 30;
    age = age + 15;
    age = age - 12;
    age = age - 3;
    age = age + 23;
    let boy1 = "Kwame";
    let boy2 = "Kofi";
    let mut kofi_age = age - 2;
    println!("{boy1} is {kofi_age} years old");
    println!("{boy2} is {} years old", age + 2);
    kofi_age = age;
    println!("Now {} and {} are both {} years old", boy1, boy2, age);

    **/

    ///MAJOR CONCEPTS
    /// 1. STRINGS
    //This is a string slice.
    let first = "Firt String";
    let last = "Last String";
    println!("{first}");

    //This is a String type.
    let full = String::from("Full String");
    let mut guess = String::new();

    //Play around
    //Guessing game
    let correct = rand::rng().random_range(1..=100); //With = inclusive, without exclusive
    // println!("The correct number is: {correct}");

    loop {
        guess.clear();
        println!("Hey, guess a number");

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input.");

        // println!("You guessed {}", guess.trim());

        // let guessNum: u32 = guess.trim().parse().expect("Please type a number!");
        let guessNum: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        let message = if correct == guessNum {
            // println!("You guessed correctly!");
            "You guessed correctly!"
        } else if correct < guessNum {
            // println!("Too high, try again!");
            "Too high, try again!"
        } else if correct > guessNum {
            // println!("Too low, try again!");
            "Too low, try again!"
        } else {
            // println!("Wrong guess, try again!");
            "Wrong guess, try again!"
        };

        let message2 = match guessNum.cmp(&correct) {
            Ordering::Less => "Too low, try again!",
            Ordering::Greater => "Too high, try again!",
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            }
        };

        println!("{message} again: {message2}");
    }
}
