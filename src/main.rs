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
use clap::{App};
use dialoguer::Input;
use rand::prelude::*;
// use std::env:: { current_dir, join_paths };
use std::fs::File;
use std::io:: { BufReader, BufRead };
use std::path::Path;

fn main() {
    App::new("Hangman")
        .version("0.1.0")
        .author("YJ Kim <yjkimjunior@gmail.com>")
        .about("hangman cli game written in Rust")
        .get_matches();

    let mut words: Vec<String> = Vec::new();

    // FIXME: dont hard code the path
    let words_list_path = Path::new("/Users/yj/Developer/hangman/resources/words.txt");

    // FIXME: dont panic
    let reader = BufReader::new(File::open(words_list_path).unwrap());

    // Get all the valid words
    for line in reader.lines() {
        // FIXME: dont panic
        words.push(line.unwrap_or("something went wrong getting this line".to_string()));
    }

    // Use Random Number Gen to pick heads or tails (0 for heads, 1 for tails)
    let coin_flip_result = match rand::thread_rng().gen_range(0, 1) {
        0 => "H".to_string(),
        1 => "T".to_string(),
        _ => panic!("Coin flip fucked up")
    };

    // Prompt user for their choice H or T
    let coin_flip_guess: String = Input::new().with_prompt("Heads (H) or Tails (T)?").interact().unwrap();

    let game_word: String;

    if coin_flip_guess == coin_flip_result {
        // If Human choosing word - prompt and make sure it's in the word list
        game_word = Input::new().with_prompt("Congrats you get to start! Pick a Word For The Game").interact().unwrap();
    } else {
        // If Computer choosing word - select a word for the game using random number
        let random_num: usize = rand::thread_rng().gen_range(0, words.len());
        game_word = words[random_num].clone();
    }

    let answer: Vec<char> = game_word.chars().collect();
    let mut correctGuesses: Vec<char> = Vec::new();
    let wrongGuesses: Vec<char> = Vec::new();

    // Loop time: Hangman State -> Guess -> Updated Hangman State -> Repeat
    // 6 wrong guesses permitted: head, body, 2 arms, 2 legs
    while correctGuesses != answer && wrongGuesses.len() < 6 {
        let guess: char = Input::new().with_prompt("What's your guess?").interact().unwrap();

        // assigns index of the guessed char to Some(number) or None
        let index = answer.iter().position(|&letter| letter == guess);

        match index {
            Some(i) => {
                &correctGuesses.insert(i, guess);
                println!("{:?}", correctGuesses);
            }
            _ => println!("noooo")
        }
    }
}
