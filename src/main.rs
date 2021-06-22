mod blockchain;

use std::io;
use std::process;
use std::io::Write;

fn main() {
    println!("Welcome to Fiuba Coin");
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    loop{
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change Reward");
        println!("0) Exit");
        println!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap(){
            0 => {
                println!("Exiting");
                process::exit(0);
            },
            1 => {
                process_transaction();
            },
            2 => {
                println!("Here we process the mine block");
            },
            3 => {
                println!("Here we process the change reward");
            }
            _ => println!("Invalid option please retry"),
        }
    }
}

fn process_transaction() {
    println!("Here we process the transacttion");
}
