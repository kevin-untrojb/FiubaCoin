mod blockchain;
mod logger;
mod routes;

#[macro_use]
extern crate rocket;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io;
use std::process;
use std::io::Write;
use serde_json::Result;

fn main(){
    routes::rocket().launch()
}
fn main2() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    logger::init(true);
    logger::log(format!("[Main] Program Start"));
    println!("Welcome to Fiuba Coin");
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0;1024];
        stream.read(&mut buffer).unwrap();
        let block_request = b"GET /blockchain HTTP/1.1\r\n";
        if buffer.starts_with(block_request){
            let result = String::from("Hola mundo");
            let response = format!(
                "HTTP/1.1 200 OK \r\n Content-Length: \r\n{}",
                result
            );
            stream.write(response.as_bytes()).unwrap();
            let mut blockchain = blockchain::Blockchain::new();
            loop{
                println!();
                
                println!("");
        
                println!("Menu");
                println!("1) New Transaction");
                println!("2) Mine Block");
                println!("3) Change Reward");
                println!("4) Change Miners Ammount");
                println!("5) Show current blockchain");
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
                        match blockchain.is_transaction_empty() {
                            true => {
                                println!("No transactions to Mine...Press enter to continue");
                                io::stdin().read_line(&mut choice);
                            },
                            false => {
                                blockchain.generate_new_block();
                            }
                        }
                    },
                    3 => {
                        let mut reward = String::new();
                        print!("Enter new reward: ");
                        io::stdout().flush();
                        reward.clear();
                        io::stdin().read_line(&mut reward);
                        blockchain.change_reward(reward.trim().parse().unwrap());
                    },
                    4 => {
                        let mut miners = String::new();
                        print!("Enter Miners quantity: ");
                        io::stdout().flush();
                        miners.clear();
                        io::stdin().read_line(&mut miners);
                        blockchain.set_miners(miners.trim().parse().unwrap());
                    },
                    5 => {
                        println!("Current Blockchain:");
                        println!("{:#?}", &blockchain);
                    },
                    _ => println!("Invalid option please retry"),
                }
            }
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
