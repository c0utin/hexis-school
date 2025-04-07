use sha256::digest;
use std::fmt;
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Block<'a> {
    index: i32,
    timestamp: u64,
    prev_hash: &'a str,
    data: &'a str,
    nonce: i32,
    difficulty: i32,
    hash: String,
}

impl<'a> fmt::Display for Block<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block {{ index: {}, timestamp: {}, prev_hash: {}, data: {}, nonce: {}, difficulty: {}, hash: {} }}",
               self.index, self.timestamp, self.prev_hash, self.data, self.nonce, self.difficulty, self.hash)
    }
}

impl<'a> Block<'a> {
    pub fn new(
        index: i32,
        timestamp: u64,
        prev_hash: &'a str,
        data: &'a str,
        nonce: i32,
        difficulty: i32,
    ) -> Self {
        let mut block = Block {
            index,
            timestamp,
            prev_hash,
            data,
            nonce,
            difficulty,
            hash: String::new(),
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut record = String::new();
        write!(
            &mut record,
            "{}{}{}{}{}",
            self.index, self.timestamp, self.prev_hash, self.data, self.nonce
        )
        .unwrap();

        let hasher = digest(record);
        hasher
    }
}

pub fn generate_block_with_pow<'a>(
    prev_block: &'a Block,
    data: &'a str,
    difficulty: i32,
) -> Block<'a> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut nonce = 0;

    let mut new_block = Block::new(
        prev_block.index + 1,
        timestamp,
        &prev_block.hash,
        data,
        nonce,
        difficulty,
    );

    while !new_block.hash.starts_with(&"0".repeat(difficulty as usize)) {
        nonce += 1;
        new_block.nonce = nonce;
        new_block.hash = new_block.calculate_hash();
        println!("Nonce: {}, Hash: {}", nonce, new_block.hash);
    }

    new_block
}
