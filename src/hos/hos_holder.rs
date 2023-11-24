use crate::maloja_backend::hos::HOSConnection;
use std::{
    collections::HashMap,
    sync::OnceLock,
};
use tokio::sync::Mutex;
use uuid::Uuid;

pub fn get_hos_connections() -> &'static Mutex<HashMap<Uuid, HOSConnection>> {
    static CONNECTIONS: OnceLock<Mutex<HashMap<Uuid, HOSConnection>>> = OnceLock::new();
    CONNECTIONS.get_or_init(|| {
        Mutex::new(HashMap::new())
    })
}
