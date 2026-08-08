use std::sync::Mutex;
use crate::models::Blockchain;

pub struct AppState {
    pub blockchain: Mutex<Blockchain>,

}

impl AppState {
    pub fn new(difficulty: usize) -> Self {
        AppState {
            blockchain: Mutex::new(Blockchain::new(difficulty)),
        }
    }
}
