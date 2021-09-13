use std::io;
use std::io::Write;
use std::num::ParseIntError;
use std::char::ParseCharError;
use rand::Rng;

type GameFunction = fn() -> bool;

// These will be compile time constants, I'm pretty sure.
pub struct Game {
    pub play : GameFunction,
    pub name : &'static str 
}


const COIN_FLIP_MAX : i16 = 300;
fn coin_flip_game() -> bool {
    let mut currency : i16 = 120;
    while currency < COIN_FLIP_MAX {
        let mut answer = String::new();
        let random_choice : u8 = rand::thread_rng().gen_range(0..=1);
        println!("Your current balance ({})", currency);
        print!("Bet Amount: ");
        io::stdout().flush().expect("Could not flush stream.");
        io::stdin().read_line(&mut answer).expect("Could not read answer");
        let bet_amount : Result<u8, ParseIntError> = answer.trim().parse();
        let bet_amount : u8 = match bet_amount {
            Ok(bet) => bet,
            Err(e) => {
                println!("{}", e);
                1
            }
        };

        print!("Heads or tails? (h/t): ");
        io::stdout().flush().expect("Could not flush stream.");
        io::stdin()
            .read_line(&mut answer)
            .expect("Could not read answer");
        let user_choice : u8 = answer.to_lowercase().starts_with("h") as u8;
        if user_choice == random_choice {
            currency += bet_amount as i16;
        } else {
            currency -= bet_amount as i16;
        }
        if currency < 1 {
            return false;
        } else if currency > COIN_FLIP_MAX {
            return true;
        }
    }
    return true;
}

fn guessing_game() -> bool {
    let mut attempts : i8 = 3;
    let random_number = rand::thread_rng().gen_range(1..=10);
    while attempts > 0 {
        let mut answer = String::new();
        print!("Enter a number from 1-10! ");
        io::stdout().flush().expect("Could not flush the output stream!");
        io::stdin()
            .read_line(&mut answer)
            .expect("Could not read line!");
        let number : Result<i8, ParseIntError> = answer.trim().parse();
        let number = match number {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                -1
            }
        };

        if number < 1 && number > 10 {
            println!("Please enter a valid number!");
        } else if number == random_number {
            println!("You did it!");
            return true;
        } else {
            if number < random_number {
                println!("Too low!");
            } else {
                println!("Too high!");
            }
            attempts -= 1;
        }
    }
    return false;
}

const WORD_LIST_SIZE : usize = 9;
const WORD_LIST : [&str; WORD_LIST_SIZE] = [
    "Rust", "Programming", "Video",
    "Half-Life", "Sandwich", "Functional",
    "Minecraft", "Tetris", "Mario"
];

fn hangman_game() -> bool {
    let mut attempts : i8 = 10;
    println!("Starting a new game of hangman! You have 10 attempts to guess the word. Please guess each character individually.");
    let random_word = WORD_LIST[rand::thread_rng().gen_range(0..WORD_LIST_SIZE)];
    let random_word : Vec<char> = random_word.chars().collect(); // Turn this into a char array.
    let mut blanked : Vec<char> = random_word.clone(); 
    for i in 0..blanked.len() {
        let character = blanked[i];
        if character != ' ' {
            blanked[i] = '_';
        }
    }
    while attempts != 0 {
        let mut tmp = [0u8;4];
        let mut blanked_string = String::new();
        for character in &blanked {blanked_string += character.encode_utf8(&mut tmp);}
        println!("{}", blanked_string);
        print!("Guess a letter: ");
        io::stdout().flush().expect("Could not flush stream.");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Could not read input.");
        let answer : Result<char, ParseCharError> = answer.trim().parse();
        let answer = match answer {
            Ok(result) => result,
            Err(e) => {
                println!("{}", e);
                '-'
            }
        };
        let mut found = false;
        for i in 0..random_word.len() {
            let character = random_word[i].to_ascii_lowercase();
            if answer == character && blanked[i] == '_' {
                blanked[i] = random_word[i];
                found = true;
            }
        }
        if !found { 
            attempts -= 1;
            println!("{} is not in this word! -1 attempt. You now have {} attempts left.", answer, attempts);
        } else if random_word == blanked {
            println!("You have beat the game! Congratulations.");
            return true;
        }
    }
    return false;
}

const GAME_LIST_SIZE : usize = 3;
const GAME_LIST : [Game; GAME_LIST_SIZE] = [
    Game{name:"Number Guessing", play:guessing_game},
    Game{name:"Coinflip", play:coin_flip_game},
    Game{name:"Hangman", play:hangman_game}
];

// What the fuck???
pub fn random<'a>() -> &'a Game {
    return &GAME_LIST[rand::thread_rng().gen_range(0..GAME_LIST_SIZE)];
}