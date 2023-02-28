use std::fmt::Display;


/// Block in Simple blockchain
#[derive(Debug, Default)]
pub struct Block<TimeStamp, Data, Hash> {
    /// current timestamp, also just say block created time.
    pub timestamp: TimeStamp,
    /// block store valid message, just to say transaction.
    pub data: Data,
    /// prev block hash
    pub prev_block_hash: Hash,
    /// current block hash
    pub hash: Hash,
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

    TimeStamp: Time<TimeStamp = TimeStamp> + std::default::Default,
    Hash: Hasher<Output = Hash> + std::default::Default,
    Data: Digest + std::default::Default + std::convert::From<Vec<u8>>,
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

        block.set_hash();
        
        block
    }

    pub fn set_hash(&mut self) {
        let mut timestamp = self.timestamp.to_vec();
        let mut headers = vec![];
        headers.append(&mut self.prev_block_hash.to_vec());
        headers.append(&mut self.data.to_vec());
        headers.append(&mut timestamp);
        let hash = Hash::sha256_hash(headers);
        
        self.hash = hash;
    }
}