pub mod block {
    use std::time::SystemTime;
    use serde::Serialize;
    use sha2::{Sha256, Digest};
    use log::info;
    use postcard;   
    use anyhow::Result;

    const TARGET_HEXS: usize = 4;

    #[derive(Serialize, Clone)]
    pub struct Block {
        timestamp: u128,
        transactions: String,
        prev_block_hash: String,
        hash: String,
        nonce: i32,
        height: i32,
    }

    pub struct Blockchain {
        blocks: Vec<Block>,
    }

    impl Block {
        pub fn new_genesis_block() -> Block {
            Block::new_block(String::from("Genesis Block"), String::from(""), 0).unwrap()
        }

        pub fn get_hash(&self) -> String {
            self.hash.clone()
        }

        pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_millis();

            let mut block = Block {
                timestamp,
                transactions: data,
                prev_block_hash,
                hash: String::new(),
                nonce: 0,
                height: height as i32,
            };

            block.run_proof_of_work()?;
            Ok(block)
        }

        fn prepare_hash_data(&self) -> Result<Vec<u8>> {
            let content = (
                self.prev_block_hash.clone(),
                self.transactions.clone(),
                self.timestamp,
                TARGET_HEXS,
                self.nonce,
            );

            let bytes = postcard::to_allocvec(&content)?;
            Ok(bytes)
        }

        fn validate(&self) -> Result<bool> {
            let data: Vec<u8> = self.prepare_hash_data()?;
            let mut hasher = Sha256::new();
            hasher.update(&data);
            let result = hasher.finalize();
            let hex_hash = format!("{:x}", result);
            
            let target = "0".repeat(TARGET_HEXS);
            Ok(hex_hash[0..TARGET_HEXS] == target)
        }

        fn run_proof_of_work(&mut self) -> Result<()> {
            info!("Mining the block");

            while !self.validate()? {
                self.nonce += 1;
            }

            let data: Vec<u8> = self.prepare_hash_data()?;
            let mut hasher = Sha256::new();
            hasher.update(&data);
            let result = hasher.finalize();
            self.hash = format!("{:x}", result);

            Ok(())
        }
    }

    impl Blockchain {
        pub fn new() -> Blockchain {
            Blockchain {
                blocks: vec![Block::new_genesis_block()],
            }
        }

        pub fn add_block(&mut self, data: String) -> Result<()> {
            let prev: &Block = self.blocks.last().unwrap();
            let new_block: Block = Block::new_block(data, prev.get_hash(), self.blocks.len())?;
            self.blocks.push(new_block);
            Ok(())
        }
    }
}