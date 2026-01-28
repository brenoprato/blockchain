mod blockchain;
use blockchain::blockchain::Blockchain;
use anyhow::{Ok, Result};


fn main() -> Result<()> {
    let mut blockchain: Blockchain = Blockchain::new()?;
    /*let _ = blockchain.add_block("data1".to_string());
    let _ = blockchain.add_block("data2".to_string());
    let _ = blockchain.add_block("data3".to_string());*/

    for b in blockchain.iter(){
        print!("{:?} \n", b);
    }

    Ok(())

}
