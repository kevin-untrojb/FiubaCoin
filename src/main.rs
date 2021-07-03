mod blockchain;

use std::io;
use std::process;
use std::io::Write;

fn main() {
    println!("Welcome to Fiuba Coin");
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    /*
    let mut block = blockchain::Block::genesis();

    println!("{}", block.serialize());
    println!("Perno test");

    let mut blockchain = blockchain::Blockchain::new();

    println!("{:#?}", &blockchain);

    println!("##########################");
    blockchain.new_transaction(String::from("Transaction test"), 16);

    println!("{:#?}", &blockchain);
    println!("##########################");

    blockchain.generate_new_block();
    println!("{:#?}", &blockchain);
    */

    let mut blockchain = blockchain::Blockchain::new();
    
    loop{
        println!("{:#?}", &blockchain);

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
                process_transaction(&mut blockchain);
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

fn process_transaction(blockchain: &mut blockchain::Blockchain) {
    //println!("Here we process the transacttion");
    let mut details = String::new();
    let mut amount = String::new();

    print!("Enter transaction details: ");
    io::stdout().flush();
    io::stdin().read_line(&mut details);

    print!("Enter amount: ");
    io::stdout().flush();
    io::stdin().read_line(&mut amount);

    blockchain.new_transaction(details.trim().to_string(), amount.trim().parse().unwrap());
}
