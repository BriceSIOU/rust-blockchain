mod models;
mod api;

use actix_web::{web, App, HttpServer};
use api::{AppState, get_blocks, get_status, add_transaction, mine_block};

#[tokio::main]
async fn main(){
    env_logger::init();
    println!("Demarrage de la blockchain...");
    println!("Difficulte de mining: 2");

    // Arc Atomic Reference Counted
    let state = web::Data::new(AppState::new(2));
    println!("Serveur demarree sur http://127.0.0.1:8080");

    // creation et demarage du serveur
    HttpServer::new(move || {
        //move transfere la propriete de state dans la closure
        App::new()
            // enregistre la propriete de state dans la closure
       .app_data(state.clone())
       // enregistre le routes
       .service(get_blocks)
       .service(get_status)
       .service(add_transaction)
       .service(mine_block)
    })
    .bind("127.0.0.1:8080")
    .expect("Impossible de demarrer le serveur sur le port 8080")
    .run()
    .await
    .expect("Erreur lors de l'execution du serveur");
}
