use std::io;
use std::io::Write;
use std::num::ParseIntError;
use rand::Rng;

type GameFunction = fn() -> bool;

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

const GAME_LIST_SIZE : usize = 2;
const GAME_LIST : [Game; GAME_LIST_SIZE] = [
    Game{name:"Number Guessing",play:guessing_game},
    Game{name:"Coinflip", play:coin_flip_game}
];

// What the fuck???
pub fn random<'a>() -> &'a Game {
    return &GAME_LIST[rand::thread_rng().gen_range(0..GAME_LIST_SIZE)];
}