
use crate::blockchain::Blockchain;
use clap::{Command, arg};
use anyhow::Result;
use std::fs;

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Self> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust")
            .version("0.1")
            .author("prato")
            .about("Blockchain in Rust - a simple blockchain for learning")
            .subcommand(
                Command::new("printchain")
                    .about("Print all the blocks")
            )
            .subcommand(
                Command::new("clear")
                    .about("Clear the blockchain")
            )
            .subcommand(
                Command::new("addblock")
                    .about("Add a block to the blockchain")
                    .arg(arg!(<DATA> "The block data"))
            )
            .get_matches();

        match matches.subcommand() {
            Some(("addblock", sub_matches)) => {
                if let Some(data) = sub_matches.get_one::<String>("DATA") {
                    self.addblock(data.clone())?;
                } else {
                    eprintln!("Error: DATA argument is required for addblock");
                    std::process::exit(1);
                }
            }
            Some(("printchain", _)) => {
                self.cmd_print_chain()?;
            }
            Some(("clear", _)) => {
                self.clear_blockchain()?;
            }
            _ => {
                // No subcommand or unknown subcommand
                eprintln!("Use --help to see available commands");
                std::process::exit(1);
            }
        }

        Ok(())
    }

    pub fn cmd_print_chain(&self) -> Result<()> {
        for block in self.bc.iter() {
            println!("{:?}", block);
        }
        Ok(())
    }

    pub fn clear_blockchain(&mut self) -> Result<()> {
        let db_path = "data/blocks";
        if fs::metadata(db_path).is_ok() {
            drop(self.bc.db.clone());
            fs::remove_dir_all(db_path)?;
            println!("Blockchain cleared");
        } else {
            println!("No blockchain database found");
        }

        self.bc = Blockchain::new()?;
        Ok(())
    }

    pub fn addblock(&mut self, data: String) -> Result<()> {
        println!("Adding block with data: {:?}", data);
        self.bc.add_block(data)?;
        Ok(())
    }
}
