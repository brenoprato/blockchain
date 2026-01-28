pub mod blockchain{
    use log::info;
    use blockchain::block::Block;
    use anyhow::Result;
    const TARGET_HEXS: usize = 4;

     #[derive(Clone, Debug)]
    pub struct Blockchain {
        current_hash: String,
        db: sled::Db,
    }

    pub struct BlockchainInIter<'a>{
        current_hash: String,
        bc: &'a Blockchain,
    }

    impl Blockchain {
        pub fn new() -> Result<Blockchain> {
            info!("Open blockchain");

            let db = sled::open("data/blocks")?;
            match db.get("LAST")?{
                Some(hash) => {
                    let lasthash = String::from_utf8(hash.to_vec())?;
                    Ok(Blockchain{
                        current_hash: lasthash,
                        db,
                    })
                }
                None => {
                    let block = Block::new_genesis_block();
                    db.insert(block.get_hash(), postcard::to_allocvec(&block)?)?;
                    db.insert("LAST", block.get_hash().as_bytes())?;
                    let bc = Blockchain{
                        current_hash: block.get_hash(),
                        db,
                    };

                    bc.db.flush()?;

                    Ok(bc)
                }
            }
        }

        pub fn add_block(&mut self, data: String) -> Result<()> {

            let lasthash = self.db.get("LAST")?.unwrap();

            let new_block: Block = Block::new_block(
                data,
                String::from_utf8(lasthash.to_vec())?,
                TARGET_HEXS,
            )?;

            self.db.insert(new_block.get_hash(), postcard::to_allocvec(&new_block)?)?;
            self.db.insert("LAST", new_block.get_hash().as_bytes())?;
            self.current_hash = new_block.get_hash();

            Ok(())
        }

        pub fn iter(&self) -> BlockchainInIter<'_>{
            BlockchainInIter{
                current_hash: self.current_hash.clone(),
                bc: &self,
            }
        }
    }

    impl<'a> Iterator for BlockchainInIter<'a>{
        
        type Item = Block;
        
        fn next(&mut self) -> Option<Self::Item>{
            
            if let Ok(encode_block) = self.bc.db.get(&self.current_hash){
                return match encode_block{
                    Some(b) =>{ 
                        if let Ok(block) = postcard::from_bytes::<Block>(&b){
                            self.current_hash = block.get_prev_hash();
                            Some(block)
                        } else{
                            None
                        }
                    }

                    None => None
                }
            }

            None
        }
    }
}