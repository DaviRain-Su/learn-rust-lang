use std::fmt::Display;

use crate::proofwork::ProofOfWork;


/// Block in Simple blockchain
#[derive(Debug, Default, Clone)]
pub struct Block<TimeStamp, Data, Hash> {
    /// current timestamp, also just say block created time.
    pub timestamp: TimeStamp,
    /// block store valid message, just to say transaction.
    pub data: Data,
    /// prev block hash
    pub prev_block_hash: Hash,
    /// current block hash
    pub hash: Hash,
    /// nonce
    pub nonce: u64,
}

impl<TimeStamp, Data, Hash> Display for Block<TimeStamp, Data, Hash> 
where 
    Data: Digest,
    Hash: Hasher,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pre_block_hash = hex::encode(self.prev_block_hash.to_vec());
        writeln!(f, "Prev. hash: 0x{}", pre_block_hash)?;
        writeln!(f, "Data: {}", String::from_utf8_lossy(&self.data.to_vec()))?;
        writeln!(f, "Hash: 0x{}", hex::encode(self.hash.to_vec()))
    }
}

pub trait Time {
    type TimeStamp;

    fn to_vec(&self) -> Vec<u8>;

    fn now() -> Self::TimeStamp;
}


pub trait Hasher {
    type Output;

    fn to_vec(&self) -> Vec<u8>;

    fn sha256_hash(data: Vec<u8>) -> Self::Output;
}

pub trait Digest {
    fn to_vec(&self) -> Vec<u8>;
}

impl<TimeStamp, Data, Hash> Block<TimeStamp, Data, Hash>
where 

    TimeStamp: Time<TimeStamp = TimeStamp> + std::default::Default + Clone,
    Hash: Hasher<Output = Hash> + std::default::Default + Clone + std::convert::From<[u8; 32]>,
    Data: Digest + std::default::Default + std::convert::From<Vec<u8>> + Clone,
 {  
    pub fn genesis_block() -> Self {
        Block::new(b"Genesis Block".to_vec().into(), Hash::default())
    }

    pub fn new(data: Data, prev_block_hash: Hash) -> Self {
        let mut block = Block { 
            timestamp: TimeStamp::now(),
            data,
            prev_block_hash,
            ..Block::default()
        };

        let pow = ProofOfWork::new(block.clone());
        let (nonce, hash) = pow.run();
        block.hash = hash;
        block.nonce = nonce;
        
        block
    }
}