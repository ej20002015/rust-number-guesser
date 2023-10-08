use rand::{thread_rng, Rng};
use std::{io, ops::RangeInclusive};
use std::process::exit;

fn main() {
    const lowerBound : u32 = 0;
    const upperBound : u32 = 100;
    const guessableRange : RangeInclusive<u32> = lowerBound..=upperBound;

    let mut rng = thread_rng();
    let numberToGuess = rng.gen_range(guessableRange);

    let mut input = String::new();
    loop {
        println!("Guess a number within the range {lowerBound}..{upperBound}: ");
        io::stdin().read_line(&mut input).expect("Couldn't read guess from stdin");
        let Ok(guess) : Result<u32, _> = input.trim_end().parse() else {
            if (input == "q") {
                println!("Exiting");
                exit(0);
            }

            println!("Couldn't parse input {} as an integer", input.trim_end());
            input.clear();
            continue;
        };

        if guess == numberToGuess {
            break;
        }
        else if guess > numberToGuess {
            println!("Lower");
        }
        else if guess < numberToGuess {
            println!("Higher");
        }

        input.clear();
    }

    println!("That's the correct answer!")
}
