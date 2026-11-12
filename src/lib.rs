use axum::Router;
pub mod config;
pub mod document;
pub mod history;
pub mod operations;
pub mod storage;
pub mod websocket;
// pub mod error;
//
pub fn create_router() -> Router {
    Router::new()
}
