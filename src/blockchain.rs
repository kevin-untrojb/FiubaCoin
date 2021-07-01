use chrono::Utc;
use crypto_hash::{hex_digest, Algorithm};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
  pub transaction_id: String,
  pub transaction_timestamp: i64,
  pub transaction_details: String,
  pub amount: i64,
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

pub struct Blockchain {
  blocks: Vec<Block>,
  current_transaction_list: Vec<Transaction>,
  reward: i64,
  difficulty: i32,
}

impl Blockchain {
  pub fn new() -> Self {
    Blockchain {
      blocks: vec![Block::genesis()],
      current_transaction_list: vec![],
      reward: 100,
      difficulty: 1
    }
  }

  pub fn last_block(self: &Self) -> &Block {
    self.blocks.last().unwrap()
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

  pub fn generate_new_block(self: &mut Self) {
    let reward_transaction = Transaction {
      transaction_id: self.generate_transaction_id(),
      transaction_details: String::from("reward"),
      transaction_timestamp: Utc::now().timestamp(),
      amount: self.reward
    };

    let mut new_block = Block {
      block_number: self.blocks.len() + 1,
      block_timestamp: Utc::now().timestamp(),
      block_nonce: 0,
      transaction_list: vec![reward_transaction],
      previous_block_hash: self.blocks.last().unwrap().get_hash(),
    };

    new_block.transaction_list.append(&mut self.current_transaction_list);

    self.blocks.push(new_block);
  }

  pub fn set_difficulty(self: &mut Self, difficulty: i32) {
    self.difficulty = difficulty;
  }

  pub fn proof_of_work(self: &Self, mut block: Block) -> String {
    loop {
      let hash = block.get_hash();
      let leading_zeros = &hash[0..self.difficulty as usize];
      match leading_zeros.parse::<u32>() {
        Ok(value) => {
          if value != 0 {
            block.block_nonce += 1;
          } else {
            return hash;
          }
        }
        Err(_) => {
          block.block_nonce += 1;
          continue;
        }
      }
    }
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

    let hash = blockchain.proof_of_work(new_block);

    assert_eq!(String::from("00"), hash[0..2]);
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

    let hash = blockchain.proof_of_work(new_block);

    assert_eq!(String::from("0"), hash[0..1]);
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

    let hash = blockchain.proof_of_work(new_block);

    assert_eq!(String::from("00000"), hash[0..5]);
  }
}