use chrono::Utc;
use crypto_hash::{hex_digest, Algorithm};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
  pub transaction_id: String,
  pub transaction_timestamp: i64,
  pub transaction_details: String,
}

#[derive(Serialize, Deserialize)]
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

pub struct Blockchain {
  blocks: Vec<Block>,
}

impl Blockchain { 
  pub fn new() -> Self {
    Blockchain {
      blocks: vec![Block::genesis()]
    }
  }

  pub fn last_block(self: &Self) -> &Block {
    self.blocks.last().unwrap()
  }

  pub fn new_transaction(self: &mut Self, transaction_details: String) {
    let last_block_index = self.blocks.len() - 1;
    let last_transaction_id = self.blocks[last_block_index].transaction_list.last().unwrap().transaction_id.parse::<i32>().unwrap();

    let new_transaction = Transaction {
      transaction_id: (last_transaction_id + 1).to_string(),
      transaction_details: transaction_details,
      transaction_timestamp: Utc::now().timestamp(),
    };

    self.blocks[last_block_index].add_transaction(new_transaction);
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

  #[test]
  fn test_mining_new_block() {
    // TODO
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

    blockchain.new_transaction(String::from("a FiubaCoin goes somewhere"));

    assert_eq!(1, blockchain.blocks.len());
  }

  #[test]
  fn test_adding_a_transaction_to_the_blockchain_stores_it_in_the_current_block() {
    let mut blockchain = Blockchain::new();

    blockchain.new_transaction(String::from("a FiubaCoin goes somewhere"));

    assert_eq!(String::from("a FiubaCoin goes somewhere"), blockchain.last_block().transaction_list[1].transaction_details);
  }

  #[test]
  fn test_adding_a_block_to_the_blockchain_rewards_the_winner() {
    // TODO
  }
}