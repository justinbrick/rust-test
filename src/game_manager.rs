use std::io;
use std::num::ParseIntError;
use rand::Rng;

pub struct Game {
    pub play : fn(&self) -> bool,
    pub name : String
}

fn guessing_game() -> bool {
    let mut attempts : i8 = 3;
    let random_number = rand::thread_rng().gen_range(1..=10);
    println!("{}", random_number);
    while attempts > 0 {
        let mut answer = String::new();
        println!("Enter a number from 1-10!");
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

const game_list : [Game; 1] = [
    Game{name:String::from("Test"),play:guessing_game}
];

pub fn random() -> Game {
    return game_list[rand::thread_rng().gen_range(0..game_list.len())]
}