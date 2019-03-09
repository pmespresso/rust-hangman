// note on notation
// :: accesses items of a module
// . accesses fields and methods of a struct

/*
    Basic Hangman Command Line Game - Made With Rust

    Game:
        - Start With Prompting User: Heads or Tails?
            - If user gets it right, they get to guess
            - else they let the computer guess
        - Either Way:
            - A word is chosen (for a computer from a preselected list, user can put whatever)
            - Reveal the word as _ _ _ _ _ _
            - Reveal the Hanging stand
            =========
            |       |
            |
            |
            |
            |
            - Each Turn:
                - Guesser guesses a letter
                - If letter in word:
                    - reveal letter in position _ a _ _ _ _
                - else:
                    - add a body part to the hangstands
                    - Order: Head, Neck, Body, Arms, Left Leg, Right Leg
                - If word complete: Guesser wins! Add point
                - If hangman complete: Chooser wins! Add point
            - End Game:
                - Show the score 1 - 0
                - Prompt user: Play Again?
*/

// use std::env;

use std::io;

fn main() {
    println!("{}", "Welcome to Hangman!");

    println!("{}", "We will begin the game with a coin flip. Heads (H) or Tails (T)?");

    let mut input: String = String::new();

    io::stdin().read_line(&mut input);

    let choice = input.trim();

    if choice.chars().nth(0) != Some('H') && choice.chars().nth(0) != Some('T') {
        println!("{}", "invalid choice. Please choose H or T");
    }
}
