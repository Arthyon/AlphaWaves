use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
mod engine;
use engine::Engine;
fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}
fn uci_info() {
    println!("id name AlphaWaves");
    println!("id author Christian Hochlin");
    println!("uciok");
}
fn is_ready() {
    println!("readyok");
}
fn stop() {}
fn new_game() {}
fn parse_go_command(game_state: Arc<Mutex<Engine>>) {
    thread::spawn(move || {
        game_state.lock().unwrap().go();
    });
}
fn main() {
    let game_state = Arc::new(Mutex::new(Engine::new()));
    loop {
        let game_state = game_state.clone();
        let input = read_line();
        match input {
            Err(_) => println!("info string Error reading input"),
            Ok(s) => {
                let tokens: Vec<_> = s.split_whitespace().collect();
                match tokens[0] {
                    "uci" => uci_info(),
                    "isready" => is_ready(),
                    "ucinewgame" => new_game(),
                    "position" => game_state.lock().unwrap().update_position(tokens),
                    "go" => parse_go_command(game_state),
                    "stop" => stop(),
                    "quit" => break,
                    unknown => println!("info string Unknown command: {}", unknown),
                }
            }
        }
    }
}
