use std::io; // for input output
use std::{thread, time::Duration}; // to delay time

use crate::player::Player;
use crate::monster::Monster;

pub fn read_player_input() -> String {
    // TAKE USER INPUT FOR PLAYER'S MOVE

    println!("Enter your move (attack/defend/flee): ");

    let mut player_move = String::new();

    io::stdin()
        .read_line(&mut player_move)
        .expect("failed to read line");

    // shadow it without a .trim() to convert to a &str 
    // so it can accept literal string
    player_move.trim().to_string()
}

// random function to clear terminal (w/o using external crate)
pub fn clear_terminal() {
    // \x1B[2J clears the screen
    // \x1B[1;1H moves the cursor to the top-left corner
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    
    // Alternatively, using standard escape syntax:
    // print!("\x1B[2J\x1B[1;1H");
}

pub fn print_hp(player: &Player, monster: &Monster) {
    // print out remaining hp of both player and monster
    println!("User has {} hp left.", player.hp);
    println!("Monster has {} hp left.", monster.hp);
}

pub fn print_winner(player: &Player, monster: &Monster) {
    if !player.is_alive() && !monster.is_alive() {
        println!("Draw! You both lose.");
    } else if !player.is_alive() {
        println!("Monster wins!");
    } else {
        println!("Player wins!");
    };
}

pub fn pause_and_clear() {
    thread::sleep(Duration::from_secs(2)); // delay 2 seconds before clearing
    clear_terminal();
}
