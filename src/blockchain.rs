use chrono::Utc;
use crypto_hash::{hex_digest, Algorithm};
use serde::{Serialize, Deserialize};
use std::thread::{sleep};
use std::time::{Duration};
use std::sync::{Condvar, Mutex, Arc};
use crate::logger::{log};
use rand::Rng;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
  pub transaction_id: String,
  pub transaction_timestamp: i64,
  pub transaction_details: String,
  pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
  pub block_number: usize,
  block_timestamp: i64,
  pub block_nonce: u64,
  pub transaction_list: Vec<Transaction>,
  previous_block_hash: String,
}

impl Block {
  // generates the genesis block (i.e origin of blockchain)
  pub fn genesis() -> Self {
    let transaction = Transaction {
        transaction_id: String::from("1"),
        transaction_details: String::from("This is dummy transaction as genesis block has no transactions"),
        transaction_timestamp: Utc::now().timestamp(),
        amount: 0, // Genesis has no reward? 
    };
    Block {
        block_number: 1,
        block_timestamp: Utc::now().timestamp(),
        block_nonce: 0,
        transaction_list: vec![transaction],
        previous_block_hash: String::from("0"),
    }
  }
  
  pub fn serialize(self: &Self) -> String {
    serde_json::to_string(&self).unwrap()
  }

  pub fn get_hash(self: &Self) -> String {
    hex_digest(Algorithm::SHA256, self.serialize().as_bytes())
  }

  pub fn add_transaction(self: &mut Self, transaction: Transaction) {
    self.transaction_list.push(transaction);
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
  pub blocks: Vec<Block>,
  current_transaction_list: Vec<Transaction>,
  pub reward: i64,
  difficulty: i32,
  miners: u32,
}

impl Blockchain {
  pub fn new() -> Self {
    Blockchain {
      blocks: vec![Block::genesis()],
      current_transaction_list: vec![],
      reward: 100,
      difficulty: 1,
      miners: 1,
    }
  }

  pub fn last_block(self: &Self) -> &Block {
    self.blocks.last().unwrap()
  }

  pub fn change_reward(&mut self, reward: i64) {
    self.reward = reward;
  }

  pub fn generate_transaction_id(self: &Self) -> String {
    let last_transaction_id = self.blocks[self.blocks.len() - 1].transaction_list.last().unwrap().transaction_id.parse::<i32>().unwrap();
    let transaction_id_to_assign = match self.current_transaction_list.last() {
      Some(transaction) => transaction.transaction_id.parse::<i32>().unwrap(),
      None => last_transaction_id
    };

    return (transaction_id_to_assign + 1).to_string()
  }

  pub fn new_transaction(self: &mut Self, transaction_details: String, amount: i64) {
    let new_transaction = Transaction {
      transaction_id: self.generate_transaction_id(),
      transaction_details: transaction_details,
      transaction_timestamp: Utc::now().timestamp(),
      amount: amount,
    };

    self.current_transaction_list.push(new_transaction);
  }

  pub fn is_transaction_empty(self: &mut Self) -> bool {
    return self.current_transaction_list.is_empty();
  }

  pub fn generate_new_block(self: &mut Self) -> bool {
    
    if self.current_transaction_list.len() == 0 {
      return false;
    }
    

    let reward_transaction = Transaction {
      transaction_id: self.generate_transaction_id(),
      transaction_details: String::from("reward"),
      transaction_timestamp: Utc::now().timestamp(),
      amount: self.reward
    };

    let new_block = Block {
      block_number: self.blocks.len() + 1,
      block_timestamp: Utc::now().timestamp(),
      block_nonce: 0,
      transaction_list: vec![reward_transaction],
      previous_block_hash: self.blocks.last().unwrap().get_hash(),
    };

    let mut mined_block = self.proof_of_work(new_block);

    mined_block.transaction_list.append(&mut self.current_transaction_list);

    self.blocks.push(mined_block);
    self.difficulty += 1;

    return true;
  }

  pub fn set_difficulty(self: &mut Self, difficulty: i32) {
    self.difficulty = difficulty;
  }

  pub fn set_miners(self: &mut Self, miners: u32) {
    self.miners = miners;
  }

  pub fn proof_of_work(self: &Self, mut original_block: Block) -> Block {
    
    let mut miners = vec![];
    
    let mut _found = Arc::new(AtomicBool::new(false));

    for i in 0..self.miners {
      let found = _found.clone();
      let mut block = original_block.clone();
      let difficulty = self.difficulty.clone();
      let join_handle: thread::JoinHandle<(Option<u64>)> = thread::spawn( move || {

      //println!("This is thread number {}", i);
      //println!("{} Referencia a Found: {:p}", i, &test);
      //return i;
        let mut rng = rand::thread_rng();
        while ! found.load(Ordering::Relaxed) {
          let hash = block.get_hash();
          let leading_zeros = &hash[0..difficulty as usize];
          log(format!("[Miner-{}] Obtained hash : {}", i, hash));
              
          let random_sleep: u64 = rng.gen_range(100, 500);
          log(format!("[Miner-{}] Va a hacer un sleep de  {}", i, random_sleep));
          //sleep(Duration::from_millis(random_sleep));
          
          match leading_zeros.parse::<u32>() {
            Ok(value) => {
              if value != 0 {
                block.block_nonce += 1;
              } else {
                if ! found.load(Ordering::Relaxed) {
                  println!("Winner Miner is {}", i);
                  found.swap(true, Ordering::Relaxed);
                  return Some(block.block_nonce);
                }
              }
              }
              Err(_) => {
                block.block_nonce += 1;
                continue;
              }
            }
        }
        return None;
      });
      miners.push(join_handle);  
    };

    for miner in miners {
      let x = miner.join();
      match x {
        Ok(result) => {
          //println!("MAIN {}", result);
          //original_block.block_nonce = v;
          match result {
            Some(x) => original_block.block_nonce = x,
            None => ()
          }
        }
        Err(e) => { println!( "Error: {:?}", e ); },
      }
    }

    /*
    loop {
      let hash = block.get_hash();
      let leading_zeros = &hash[0..self.difficulty as usize];
      log(format!("[Miner] Obtained hash : {}", hash));
              
      let random_sleep: u64 = rng.gen_range(100, 500);
      sleep(Duration::from_millis(random_sleep));
      match leading_zeros.parse::<u32>() {
        Ok(value) => {
          if value != 0 {
            block.block_nonce += 1;
          } else {
            return block;
          }
        }
        Err(_) => {
          block.block_nonce += 1;
          continue;
        }
      }
    }
    */
    return original_block;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // Block

  #[test]
  fn test_genesis_block_is_the_first_block() {
    let genesis = Block::genesis();

    assert_eq!(1, genesis.block_number)
  }

  #[test]
  fn test_genesis_has_one_transaction() {
    let genesis = Block::genesis();

    assert_eq!(1, genesis.transaction_list.len())
  }

  #[test]
  fn test_genesis_has_one_transaction_with_dummy_date() {
    let genesis = Block::genesis();

    assert_eq!("This is dummy transaction as genesis block has no transactions", genesis.transaction_list[0].transaction_details)
  }

  #[test]
  fn test_genesis_has_one_transaction_with_id_one() {
    let genesis = Block::genesis();

    assert_eq!("1", genesis.transaction_list[0].transaction_id)
  }

  // Blockchain

  #[test]
  fn test_new_creates_a_blockchain_with_a_genesis_block() {
    let blockchain = Blockchain::new();

    assert_eq!(1, blockchain.last_block().block_number)
  }

  #[test]
  fn test_adding_a_transaction_to_the_blockchain_does_not_create_new_block() {
    let mut blockchain = Blockchain::new();

    blockchain.new_transaction(String::from("a FiubaCoin goes somewhere"), 1);

    assert_eq!(1, blockchain.blocks.len());
  }

  #[test]
  fn test_adding_a_transaction_to_the_blockchain_stores_it_in_the_current_transaction_vector() {
    let mut blockchain = Blockchain::new();

    blockchain.new_transaction(String::from("a FiubaCoin goes somewhere"), 1);

    assert_eq!(String::from("a FiubaCoin goes somewhere"), blockchain.current_transaction_list[0].transaction_details);
  }

  #[test]
  fn test_adding_a_transaction_uses_incremental_id() {
    let mut blockchain = Blockchain::new();

    blockchain.new_transaction(String::from("a FiubaCoin goes somewhere"), 1);
    blockchain.new_transaction(String::from("a FiubaCoin goes somewhere else"), 1);

    assert_eq!("2", blockchain.current_transaction_list[0].transaction_id);
    assert_eq!("3", blockchain.current_transaction_list[1].transaction_id);
  }

  #[test]
  fn test_adding_a_transaction_sets_correct_amount() {
    let mut blockchain = Blockchain::new();

    blockchain.new_transaction(String::from("a FiubaCoin goes somewhere"), 1);
    blockchain.new_transaction(String::from("a FiubaCoin goes somewhere else"), 10);

    assert_eq!(1, blockchain.current_transaction_list[0].amount);
    assert_eq!(10, blockchain.current_transaction_list[1].amount);
  }

  #[test]
  fn test_generating_a_block_pushes_a_reward_transaction() {
    let mut blockchain = Blockchain::new();

    blockchain.generate_new_block();

    let last_block = blockchain.blocks.last().unwrap();

    assert_eq!(100, last_block.transaction_list[0].amount);
  }

  #[test]
  fn test_generating_a_block_pushes_all_current_transactions() {
    let mut blockchain = Blockchain::new();

    blockchain.new_transaction(String::from("a FiubaCoin goes somewhere"), 1);
    blockchain.new_transaction(String::from("a FiubaCoin goes somewhere else"), 10);

    blockchain.generate_new_block();

    let last_block = blockchain.blocks.last().unwrap();
    let last_block_transaction_list = &last_block.transaction_list;

    assert_eq!(3, last_block_transaction_list.len());
    assert_eq!(100, last_block_transaction_list[0].amount);
    assert_eq!(1, last_block_transaction_list[1].amount);
    assert_eq!(10, last_block_transaction_list[2].amount);
  }

  #[test]
  fn test_proof_of_work_returns_block_hash_with_difficulty_amount_of_leading_zeros() {
    let mut blockchain = Blockchain::new();

    blockchain.set_difficulty(2);

    let new_block = Block {
      block_number: 1,
      block_timestamp: Utc::now().timestamp(),
      block_nonce: 0,
      transaction_list: vec![],
      previous_block_hash: String::from("00"),
    };

    let mined_block = blockchain.proof_of_work(new_block);

    assert_eq!(String::from("0"), mined_block.get_hash()[0..1]);
  }

  #[test]
  fn test_proof_of_work_returns_has_difficulty_1_by_default() {
    let blockchain = Blockchain::new();

    let new_block = Block {
      block_number: 1,
      block_timestamp: Utc::now().timestamp(),
      block_nonce: 0,
      transaction_list: vec![],
      previous_block_hash: String::from("00"),
    };

    let mined_block = blockchain.proof_of_work(new_block);

    assert_eq!(String::from("0"), mined_block.get_hash()[0..1]);
  }

  #[test]
  fn test_proof_of_work_with_higher_difficulty() {
    let mut blockchain = Blockchain::new();

    blockchain.set_difficulty(5);

    let new_block = Block {
      block_number: 1,
      block_timestamp: Utc::now().timestamp(),
      block_nonce: 0,
      transaction_list: vec![],
      previous_block_hash: String::from("00"),
    };

    let mined_block = blockchain.proof_of_work(new_block);

    assert_eq!(String::from("0"), mined_block.get_hash()[0..1]);
  }

  #[test]
  fn test_debug_macro() {
    let mut blockchain = Blockchain::new();

    blockchain.set_difficulty(5);

    let new_block = Block {
      block_number: 1,
      block_timestamp: Utc::now().timestamp(),
      block_nonce: 0,
      transaction_list: vec![],
      previous_block_hash: String::from("00"),
    };
    println!("This is an example of a Block {:?}",new_block);

    let mined_block = blockchain.proof_of_work(new_block);

    assert_eq!(String::from("0"), mined_block.get_hash()[0..1]);
  }

  #[test]
  fn test_clone_macro() {
    let mut blockchain = Blockchain::new();

    blockchain.set_difficulty(5);

    let transac = Transaction {
      transaction_id: String::from("Nueva trans"),
      transaction_timestamp: Utc::now().timestamp(),
      transaction_details: String::from("probando"),
      amount: 7677
    };
    let copy_trans = transac.clone();

    println!("This is an example of a Transaction {:?}",transac);

    println!("This is an example of a copy trans {:?}",copy_trans);

    //assert_eq!(transac, copy_trans);
  }
}