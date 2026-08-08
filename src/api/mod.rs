pub mod routes;
pub mod state;

pub use routes::{get_blocks, get_status, add_transaction, mine_block};
pub use state::AppState;
