#![allow(unused)]

use std::collections::LinkedList as List;


#[derive(Debug)]
pub struct BlockChain {
    pub blocks: List<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: List::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push_back(block);
    }

    pub fn get_block_by_height(&self, height: usize) -> Option<&Block> {
        self.blocks.iter().nth(height)
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.blocks.iter().find(|block: &&Block| block.hash == hash)
    }
}
#[derive(Debug)]
pub struct Block {
    pub hash: String,
    pub id: u128,
    pub transactions: List<Transaction>,
}
#[derive(Debug)]
pub struct Transaction {
    pub inputs: List<TxIn>,
    pub outputs: List<TxOut>,
    pub txid: String,
}
#[derive(Debug)]
pub struct TxIn {
    pub prev_txid: String,
    pub out: usize,
    pub signature: String, // to spend the output
}

#[derive(Debug)]
pub struct TxOut {
    pub public_address: String,
    pub satoshis: u64, 
   // 1 btc = 10^8 satoshis, in total 10^8 * 21 * 10^6 = 2.1 * 10^15
    // maximum value of u64 is greater than 10^19
    // so u64 is enough to store all valid satoshis
}



// Try to include bitcoin related functionalities like serialization, computing addresses etc.,
// You can add your own methods for different types and associated unit tests
