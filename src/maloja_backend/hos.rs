use crate::hos::json::*;
use actix_ws::Session;
use uuid::Uuid;

pub struct HOSBackend {
    pub pairing_code: String,
}

pub struct HOSConnection {
    pub incoming: Vec<HOSIncomingReq>,
    pub session: Session,
    pub pairing_code: Option<String>,
    pub connection_id: Uuid,
}
