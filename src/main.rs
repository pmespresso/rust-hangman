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

extern crate clap;
use clap::{Arg, App};

use std::env:: { current_dir, join_paths };
use std::fs::File;
use std::io:: { BufReader, BufRead };
use std::path::Path;

fn main() {
    let matches = App::new("Hangman")
        .version("0.1.0")
        .author("YJ Kim <yjkimjunior@gmail.com>")
        .about("hangman cli game written in Rust")
        .get_matches();

    let mut words: Vec<String> = Vec::new();

    // FIXME: dont hard code the path
    let words_list_path = Path::new("/Users/yj/Developer/hangman/resources/words.txt");

    // FIXME: dont panic
    let reader = BufReader::new(File::open(words_list_path).unwrap());

    for line in reader.lines() {
        // FIXME: dont panic
        words.push(line.unwrap_or("something went wrong getting this line".to_string()));
    }

    println!("{:?}", words);
}
