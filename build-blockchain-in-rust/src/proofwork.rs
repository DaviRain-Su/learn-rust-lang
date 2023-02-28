use crate::block::{Block, Time, Digest as BlockDigiest, Hasher};
use primitive_types::U256;

use sha2::Digest;
use sha2::Sha256;


pub fn hash256(data: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    output
}

const TARGET_BITS: u32 = 24;

pub struct ProofOfWork<TimeStamp, Data, Hash> {
    pub block: Block<TimeStamp, Data, Hash>,
    pub target: U256,
}

impl<TimeStamp, Data, Hash> ProofOfWork<TimeStamp, Data, Hash> 
where
    TimeStamp: Time,
    Hash: Hasher + std::convert::From<[u8; 32]>,
    Data: BlockDigiest,
{
    pub fn new(block: Block<TimeStamp, Data, Hash>) -> Self {
        let mut target: U256 = 1.into();
        target <<= 256u32 - TARGET_BITS;

        Self { block, target }
    }

    pub fn prepare_data(&self, none: u64) -> Vec<u8> {
        let mut data = vec![];
        data.append(&mut self.block.prev_block_hash.to_vec());
        data.append(&mut self.block.data.to_vec());
        data.append(&mut self.block.timestamp.to_vec());
        data.append(&mut format!("{}", TARGET_BITS).as_bytes().to_vec());
        data.append(&mut format!("{}", none).as_bytes().to_vec());

        data
    }

    /// 
    /// 准备数据
    /// 用 SHA-256 对数据进行哈希
    /// 将哈希转换成一个大整数
    /// 将这个大整数与目标进行比较
    pub fn run(&self) -> (u64, Hash) {
        let mut hash = [0u8; 32];
        let mut nonce = 0u64;

        println!("Mining the block containing {}", String::from_utf8_lossy(&self.block.data.to_vec()));

        loop {
            if nonce < u64::MAX {
                let data = self.prepare_data(nonce);
                hash = hash256(data);

                let left_hash: U256 = U256::from_big_endian(&hash);
                let right_hash = self.target;
                if left_hash < right_hash {
                    println!("0x{}", hex::encode(hash));
                    break;
                } else {
                    nonce += 1;
                }
            } else {
                break;
            }
        }
        println!();

        (nonce, hash.into())
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let hash = hash256(data);
        let left_hash: U256 = U256::from_big_endian(&hash);

        left_hash < self.target
    }

}

#[test]
fn test_proof_of_work() {

    let data1 = b"I like donuts";
    let data2 = b"I like donutsca07ca";
    let target_bits = 24;
    let mut target: U256 = 1.into();
    target <<= 256u32 - target_bits;
    println!("0x{}", hex::encode(hash256(data1.to_vec())));
    println!("0x{target:0>64x}");
    println!("0x{}", hex::encode(hash256(data2.to_vec())));
}