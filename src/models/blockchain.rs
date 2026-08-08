use serde::{Deserialize, Serialize};
use crate::models::{Block, Transaction};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain{
    pub chain: Vec<Block>,

    pub pending_transactions: Vec<Transaction>,
    pub difficulty: usize,
}
impl Blockchain {
    pub fn new (difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain : Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,     
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self){
        let genesis = Block::new(
            0,
            Vec::new(),
            "0".to_string(),
        );
        self.chain.push(genesis);
    }

    pub fn last_block(&self) ->&Block {
        self.chain.last().expect("La chaine ne peut pas etre vide")
    }
    
    pub fn add_transaction(&mut self, transaction: Transaction){
        self.pending_transactions.push(transaction);
    }
    // le minage 
    pub fn mine_pending_transactions(&mut self){
        let index = self.chain.len() as u64;
        let previous_hash = self.last_block().hash.clone();
        // vide le mempol
        let transactions = self.pending_transactions.drain(..).collect();
        let mut block = Block::new(
            index,
            transactions,
            previous_hash
        );
        block = self.proof_of_work(block);
        println!("Bloc #{} mine, le hash est:{}", block.index, block.hash);
        self.chain.push(block);
    }

    fn proof_of_work(&self, mut block: Block)-> Block {
        while !block.has_valid_hash(self.difficulty){
            block.nonce +=1;
            block.hash = block.calculate_hash();
        }    
        println!(
            "Proof of work trouve, Nonce: {}, le Hash:{}", block.nonce, block.hash
        );
        block
    }

    //verification de l'integrite de toute la chaine 
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len(){
            let current = &self.chain[i];
            let previous = &self.chain[i-1];

            if current.hash != current.calculate_hash(){
                println!("Hash invalide au block #{}", current.index);
                return false;
            }

            if current.previous_hash != previous.hash {
                println!("Hash invalide au block # {}", current.index);
                return false;
            }
        }    
        true
    }
}
