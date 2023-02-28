pub mod block;
pub mod blockchain;
pub mod proofwork;

pub use blockchain::BlockChain;

use crate::proofwork::ProofOfWork;

fn main() {
    let mut new_blockchain = BlockChain::new();

    new_blockchain.add_block(b"Send 1 BTC to Ivan".to_vec());
    new_blockchain.add_block(b"Send 2 more BTC to Ivan".to_vec());

    for block in new_blockchain.blocks {
        println!("{}", block);
        let pow = ProofOfWork::new(block);
        println!("PoW: {}", pow.validate());
    }
}
