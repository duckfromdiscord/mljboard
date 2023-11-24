use crate::{
    hos::{
        hos_holder::get_hos_connections,
        json::HOSIncomingReq,
    },
    maloja_backend::hos::HOSConnection,
};
use actix_ws::{Message, MessageStream, Session};
use futures_util::stream::StreamExt;
use uuid::Uuid;

pub async fn hos_ws(mut session: Session, mut msg_stream: MessageStream) {
    
    // the pairing_code and the connection_id are not the same
    // pairing_code is like a password, connection_id is used exclusively internally on serverside to identify connections
    let mut pairing_code: Option<String> = None;
    let connection_id: Uuid = Uuid::new_v4();

    log::info!("Connected to session ID {}", connection_id.to_string());

    get_hos_connections().lock().await.insert(
        connection_id,
        HOSConnection {
            incoming: vec![],
            session: session.clone(),
            pairing_code: pairing_code.clone(),
            connection_id,
        },
    );

    let close_reason = loop {
        match msg_stream.next().await {
            Some(Ok(msg)) => {
                log::debug!("msg: {msg:?}");

                match msg {
                    Message::Text(text) => match serde_json::from_str(&text) {
                        Ok(request) => {
                            let incoming: HOSIncomingReq = request;
                            match incoming._type.as_str() {
                                "pairing" => {
                                    pairing_code = incoming.code;
                                    log::info!(
                                        "Paired with pairing code {}, session ID {}",
                                        pairing_code
                                            .clone()
                                            .unwrap_or("[no pairing code]".to_string()),
                                        connection_id.to_string()
                                    );
                                }
                                "response" => {
                                    log::info!("Response received from pairing code {}, request ID {}, session ID {}",
                                            pairing_code.clone().unwrap_or("[no pairing code]".to_string()),
                                            incoming.clone().id.unwrap_or("[no id]".to_string()),
                                            connection_id.to_string()
                                        );
                                    get_hos_connections()
                                        .lock()
                                        .await
                                        .get_mut(&connection_id)
                                        .unwrap()
                                        .incoming
                                        .push(incoming);
                                }
                                _ => {}
                            }
                        }
                        Err(err) => {
                            log::info!(
                                "Error deserializing a request for session ID {}",
                                connection_id.to_string()
                            );
                            log::debug!("{}", err);
                        }
                    },

                    Message::Close(reason) => {
                        break reason;
                    }

                    Message::Ping(bytes) => {
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {}

                    // no-op; ignore
                    Message::Nop => {}

                    _ => {}
                };
            }

            // error or end of stream
            _ => break None,
        }
    };

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;

    get_hos_connections().lock().await.remove(&connection_id);

    log::info!("Disconnected from session ID {}", connection_id.to_string());
}