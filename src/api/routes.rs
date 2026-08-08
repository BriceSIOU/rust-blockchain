use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::models::Transaction;
use crate::api::state::AppState;

#[derive(Deserialize)]
pub struct NewTransaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}
// Structure de la reponse pour le status de la blockchain
#[derive(Serialize)]
pub struct ChainStatus {
    pub length: usize,
    pub is_valid: bool,
}


#[get("/blocks")]
pub async fn get_blocks(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    // Serialise la chain en json et retourne la reponse en http 200
    HttpResponse::Ok().json(&blockchain.chain)
}
#[get("/status")]
pub async fn get_status(data: web::Data<AppState>)-> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    let status = ChainStatus {
        length: blockchain.chain.len(),
        is_valid:blockchain.is_valid(),
    };
    HttpResponse::Ok().json(status)
}

#[post("/transactions")]
pub async fn add_transaction(
    body: web::Json<NewTransaction>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    let transaction = Transaction::new(
        body.sender.clone(),
        body.recipient.clone(),
        body.amount,
    );
    blockchain.add_transaction(transaction);
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Transaction ajoute au mempool"
    }))
}
//mine un nouveau block avec les transactions en attente
#[post("/mine")]
pub async fn mine_block(data: web::Data<AppState>) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    if blockchain.pending_transactions.is_empty(){
        return HttpResponse::BadRequest().json(serde_json::json!({
        "error": "Aucune transaction en attente a miner"}))
    }

    blockchain.mine_pending_transactions();
    let last_block = blockchain.last_block().clone();
    HttpResponse::Ok().json(last_block)
}
