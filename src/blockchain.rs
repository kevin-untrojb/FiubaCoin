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

  pub fn add_block(self: &mut Self, block: Block) {
    self.blocks.push(block);
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
  fn test_adding_a_transaction_to_the_blockchain() {
    let mut blockchain = Blockchain::new();
    let transaction = Transaction {
       transaction_id: String::from("2"),
       transaction_timestamp: Utc::now().timestamp(),
       transaction_details: String::from("a FiubaCoin"),
    };

    let block = Block {
      block_number: 2,
      block_timestamp: Utc::now().timestamp(),
      block_nonce: 0,
      transaction_list: vec![transaction],
      previous_block_hash: blockchain.last_block().get_hash(),
    };

    blockchain.add_block(block);

    let last_block = blockchain.last_block();

    assert_eq!(2, last_block.block_number);
    assert_eq!(Block::genesis().get_hash(), last_block.previous_block_hash);
  }
}