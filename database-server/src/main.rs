use axum::{
    Router,
    extract::{Query, State},
    routing::get,
};
use serde::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

// http://localhost:4000/set?somekey=somevalue
// http://localhost:4000/get?key=somekey

#[derive(Deserialize, Debug)]
pub struct GetQueryParams {
    pub key: String,
}

#[derive(Deserialize, Debug)]
pub struct SetQueryParams {
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
struct AppState {
    store: Arc<Mutex<HashMap<String, String>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = AppState {
        store: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/get", get(get_value))
        .route("/set", get(set_value))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn get_value(params: Query<GetQueryParams>, State(state): State<AppState>) -> String {
    let key = &params.key;

    let store = state.store.lock().expect("mutex was poisoned");
    let returned_value = match store.get(key) {
        Some(value) => value,
        None => "No Value Set",
    };

    format!("get - key: {}, returned value: {}", key, returned_value)
}

async fn set_value(params: Query<SetQueryParams>, State(state): State<AppState>) -> String {
    let key = &params.key;
    let value = &params.value;

    let mut store = state.store.lock().expect("mutex was poisoned");

    store.insert(key.to_string(), value.to_string());

    format!("set - key: {}, value: {}", key, value)
}
