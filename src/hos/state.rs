use crate::maloja_backend::hos::HOSConnection;
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;
pub struct AppState {
    pub hos_connections: Mutex<HashMap<Uuid, HOSConnection>>,
}
