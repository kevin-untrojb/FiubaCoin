use chrono::Utc;

pub struct Transaction {
  pub transaction_id: String,
  pub transaction_timestamp: i64,
  pub transaction_details: String,
}

pub struct Block {
  pub block_number: u64,
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
  fn test_new_creates_a_block_chain_with_a_genesis_block() {
    let blockchain = Blockchain::new();

    assert_eq!(1, blockchain.last_block().block_number)
  }

}