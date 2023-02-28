use crate::block::{Block, BlockHash, BlockData, BlockTimestamp};

type TimeStamp = i64;
type Data = Vec<u8>;
type Hash = [u8;32];

use chrono::offset::Local;
use sha2::{Sha256, Digest};


impl BlockTimestamp for TimeStamp {
    type TimeStamp = TimeStamp;

    fn to_vec(&self) -> Vec<u8> {
        format!("{self}").as_bytes().to_vec()
    }

    fn now() -> Self::TimeStamp {
        Local::now().timestamp() as i64
    }
}

impl BlockHash for Hash {
    type Output = Hash;

    fn to_vec(&self) -> Vec<u8> {
        self.into_iter().map(|v| *v).collect()
    }

    fn sha256_hash(data: Vec<u8>) -> Self::Output {
        let mut hasher = Sha256::new();
        hasher.update(data);

        let result = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&result);
        output
    }
}

impl BlockData for Data {
    fn to_vec(&self) -> Vec<u8> {
        self.into_iter().map(|v| *v).collect()
    }
}

pub struct BlockChain {
    pub blocks: Vec<Block<TimeStamp, Data, Hash>>,
}


impl BlockChain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::genesis_block()]
        }
    }

    pub fn add_block(&mut self, data:  Data)  {
        let len = self.blocks.len();
        if let Some(pre_block) = self.blocks.get(len - 1){
            let new_block = Block::<TimeStamp, _, _>::new(data, pre_block.hash);
            self.blocks.push(new_block)
        } else {
            println!("add block error: index error ({})", len - 1);
        }
    }
}