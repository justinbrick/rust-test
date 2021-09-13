mod game_manager;
const GAME_VERSION : &str = "0.1.0";

fn main() {
    println!("Example Game {}", GAME_VERSION);
    let game = game_manager::random();
    println!("Chose random game {}...", game.name);
    if game.play() {
        println!("Congrats, loser!");
    }
}
