use std::io::{self, stdout};
use std::io::Write;
extern crate rand;
use rand::Rng;

// rock is 1
// paper is 2
// scissors is 3
// return 1 if player 1 wins, 2 if player 2 wins, 0 if tie
fn decide_winner(p1: i32, p2: i32) -> i32 {
    if p1 == 1 && p2 == 2 {
        return 2;
    } else if p1 == 1 && p2 == 3 {
        return 1;
    } else if p1 == 2 && p2 == 1 {
        return 1;
    } else if p1 == 2 && p2 == 3 {
        return 2;
    } else if p1 == 3 && p2 == 1 {
        return 2;
    } else if p1 == 3 && p2 == 2 {
        return 1;
    } else {
        return 0;
    }
}

fn main() {

    let options = ["Rock", "Paper", "Scissors"];

    let mut input = String::new();
    
    print!("Rock (1), Paper (2), or Scissors (3): ");
    stdout().flush().ok();
    io::stdin().read_line(&mut input).unwrap();
    
    let player1 = input.trim().parse().unwrap();
    let player2 = rand::thread_rng().gen_range(1, 4);

    let winner = decide_winner(player1, player2);

    if winner == 1 {
        println!("You win! {} beats {}", options[(player1 as usize) - 1], options[(player2 as usize) - 1]);
    } else if winner == 2 {
        println!("You lose! {} beats {}", options[(player2 as usize) - 1], options[(player1 as usize) - 1]);
    } else {
        println!("Tie! {} vs {}", options[(player1 as usize) - 1], options[(player2 as usize) - 1]);
    }

    input.clear();

    print!("Play again? (y/n): ");
    stdout().flush().ok();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "y" {
        main();
    } else {
        println!("Thanks for playing!");
    }
}