mod linked_list;
mod block;
mod mresult;
use std::collections::LinkedList as List;


fn main() {
    let mut blockchain = block::BlockChain::new();
    let genesis_block = block::Block {
        hash: String::from("0000000000000000000"),
        id: 0,
        transactions: List::new(),
    };
    blockchain.add_block(genesis_block);

    // Query the genesis block by height
    if let Some(block) = blockchain.get_block_by_height(0) {
        println!("Genesis Block: {:?}", block.hash);
    }

    // Query the genesis block by hash
    if let Some(block) = blockchain.get_block_by_hash("0000000000000000000") {
        println!("Genesis Block: {:?}", block.hash);
    }

    println!("blockchain: {:?}", blockchain)
}