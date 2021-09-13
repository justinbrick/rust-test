use std::io;
use std::io::Write;
mod game_manager;
const GAME_VERSION : &str = "0.1.0";

fn main() {
    let mut points : u16 = 0;
    let mut exit : bool = false;
    println!("Rust Tests, Version {}", GAME_VERSION);
    while !exit {
        let game = game_manager::random();
        let mut answer = String::new();
        println!("Chose random game {}...", game.name);
        if (game.play)() {
            points += 10;
            println!("Congrats! +10 Points!");
            println!("You now have {} points.", points);
        }
        
        print!("Would you like to exit? (Y/N) ");
        io::stdout().flush().expect("Was unable to flush the output stream."); // You have to flush the stream for some reason! Yay :)
        while answer.is_empty() {
            io::stdin()
            .read_line(&mut answer)
            .expect("Unable to read string...");
        }
        
        exit = answer.to_lowercase().starts_with("y");
    }
}
