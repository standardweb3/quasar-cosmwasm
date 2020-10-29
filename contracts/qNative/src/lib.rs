pub mod handler;
pub mod init;
pub mod msg;
pub mod querier;
pub mod state;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
