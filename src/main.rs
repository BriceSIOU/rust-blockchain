mod models;

use models::{Transaction, Block, Blockchain};

fn main(){
    env_logger::init();
    println!("====Creation of the blockchain====");
    let mut blockchain = Blockchain::new(2);

    println!("\n ==== Add of transaction ====");
    blockchain.add_transaction(Transaction::new("Brice".to_string(), 
            "Bod".to_string(), 20.0));
    
    blockchain.add_transaction(Transaction::new("Bod".to_string(),
            "Marie".to_string(),
            10.0,
            ));
    println!("\n==== Mining of the second block ====");
    blockchain.mine_pending_transactions();

    println!("\n ==== State of the blockchain ====");
    println!("Number of blocks: {}", blockchain.chain.len());
    println!("Is blockchain valide: {}", if blockchain.is_valid(){"yes"} 
        else {"No"});

    println!("\n ==== Detail of the blockchain");
    for block in &blockchain.chain{
        println!(
            "Block #{} | Nonce: {} | Hash: {}...{}",
            block.index,
            block.nonce,
            &block.hash[..6],
            &block.hash[58..]
        );
    }
    blockchain.add_transaction(Transaction::new("Marie".to_string(),
    "Brice".to_string(), 0.5,));
    blockchain.mine_pending_transactions();

    println!("\n ==== Test de falsification ====");
    blockchain.chain[1].transactions[0].amount = 99999.0;
    blockchain.chain[1].hash = blockchain.chain[1].calculate_hash();
    println!("Blockchain valide apres falsification: {}", if blockchain.is_valid(){"yes"} 
        else{"No"} );
}
