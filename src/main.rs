use rand::Rng;
use std::cmp;
use std::cmp::Ordering;
use std::io;

fn main() {
    // let correct = rand::rng().random_range(1..=100); //With = inclusive, without exclusive
    // println!("The correct number is: {correct}");
    let mut guess_size = String::new();
    let mut game_over = false;

    //Loop to play game if user doesn't guess correctly
    loop {
        guess_size.clear();
        //Ask user how many numbers they want to guess
        println!("Hey, how many numbers do you want to guess?");

        io::stdin()
            .read_line(&mut guess_size)
            .expect("Error reading input.");

        let guess_size: u32 = match guess_size.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        //Loop to generate multiple correct numbers
        let mut correct_set = Vec::new();
        loop {
            let correct = rand::thread_rng().gen_range(1..=100);
            correct_set.push(correct);
            if correct_set.len() as u32 >= guess_size {
                break;
            }
        }

        //Loop to retake entries if user input is incorrect
        loop {
            //Loop to receive multiple user entries
            let mut guess_count = 0;
            let mut guesses = Vec::new();
            let mut guess = String::new();
            loop {
                guess.clear();
                if guess_count == 0 {
                    println!("Now, guess a number between 1 and 100:");
                } else {
                    println!("Guess another number:");
                }

                io::stdin()
                    .read_line(&mut guess)
                    .expect("Error reading input.");

                let guess_num: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please type a number!");
                        continue;
                    }
                };

                guesses.push(guess_num);
                guess_count += 1;

                if guess_count >= guess_size {
                    break;
                }
            }

            //Loop to check if guesses were correct
            let mut checker_index = 0;
            let mut correct_count = 0;
            loop {
                if checker_index >= correct_set.len() {
                    break;
                }
                let correct = correct_set[checker_index];
                let user_guess = guesses[checker_index];
                let mut message = "";

                match correct.cmp(&user_guess) {
                    Ordering::Greater => message = "Too low, try again!",
                    Ordering::Less => message = "Too high, try again!",
                    Ordering::Equal => {
                        message = "You guessed correctly!";
                        correct_count += 1;
                    }
                };

                println!(
                    "For number {} - {}: {}",
                    checker_index + 1,
                    user_guess,
                    message
                );
                checker_index += 1;
                if checker_index >= guess_size as usize {
                    break;
                }
            }

            if correct_count == guess_size {
                println!("Congratulations! You guessed all numbers correctly!");
                game_over = true;
                break;
            } else {
                println!(
                    "You guessed {}/{} numbers correctly. Better luck next time!",
                    correct_count, guess_size
                );
            }
        }
        if game_over {
            break;
        }

        //Inform user of win or loss status for this round

        // // let guessNum: u32 = guess.trim().parse().expect("Please type a number!");
        // let guessNum: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("Please type a number!");
        //         continue;
        //     }
        // };

        // let message = if correct == guessNum {
        //     // println!("You guessed correctly!");
        //     "You guessed correctly!"
        // } else if correct < guessNum {
        //     // println!("Too high, try again!");
        //     "Too high, try again!"
        // } else if correct > guessNum {
        //     // println!("Too low, try again!");
        //     "Too low, try again!"
        // } else {
        //     // println!("Wrong guess, try again!");
        //     "Wrong guess, try again!"
        // };

        // let message2 = match guessNum.cmp(&correct) {
        //     Ordering::Less => "Too low, try again!",
        //     Ordering::Greater => "Too high, try again!",
        //     Ordering::Equal => {
        //         println!("You guessed correctly!");
        //         break;
        //     }
        // };

        // println!("{message} again: {message2}");
    }
}
