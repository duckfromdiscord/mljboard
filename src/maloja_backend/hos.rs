use crate::hos::{json::*, state::AppState};
use crate::maloja_backend::traits::MalojaBackend;
use actix_ws::Session;
use chrono::prelude::*;
use crossbeam::channel::{unbounded, Receiver};
use mljcl::{
    history::Scrobble,
    json::{ScrobblesReq, ScrobblesRes},
    range::*,
    types::Track,
    RequestError,
};
use std::collections::HashMap;
use uuid::Uuid;
use actix_web::web::Data;
use base64::{Engine as _, engine::general_purpose};

pub struct HOSBackend {
    pub sess: Uuid,
}

pub struct HOSConnection {
    pub incoming: Vec<HOSIncomingReq>,
    pub session: Session,
    pub pairing_code: Option<String>,
    pub channels: HashMap<String, crossbeam::channel::Sender<String>>,
    pub connection_id: Uuid,
}

impl HOSConnection {
    pub async fn req(
        &mut self,
        method: &str,
        url: &str,
    ) -> Result<Receiver<String>, serde_json::Error> {
        let request_id: Uuid = Uuid::new_v4();
        let (s, r) = unbounded();
        self.channels.insert(request_id.to_string(), s);
        let request_text = hos_request(method, url, request_id.to_string());
        match request_text {
            Ok(text) => {
                self.session.text(text).await.unwrap();
            }
            Err(err) => {
                return Err(err);
            }
        }
        Ok(r)
    }
}


pub async fn req_to_hos(sess: Uuid, method: String, url: String, data: Data<AppState>) -> String {
    let mut conns = data.hos_connections
    .lock()
    .await;
    let receiver = conns.get_mut(&sess).unwrap().req(&method, &url).await.unwrap().clone();
    // It is crucial that we drop our mutex to the HashMap of connections here,
    // or else the HOS handler cannot access the crossbeam Sender
    drop(conns);
    let b64 = receiver.recv().unwrap();
    let bytes = general_purpose::STANDARD.decode(b64).unwrap();
    dbg!(String::from_utf8(bytes).unwrap())
}

impl MalojaBackend for HOSBackend {
    async fn get_scrobbles(
        &mut self,
        artist: Option<String>,
        range: Range,
        page_number: Option<u64>,
        scrobbles_per_page: Option<u64>,
        data: Data<AppState>,
    ) -> Result<Vec<Scrobble>, RequestError> {
        let from_until_in = process_range(range);
        let requestbody = ScrobblesReq {
            from: from_until_in.0,
            until: from_until_in.1,
            _in: from_until_in.2,
            artist,
            page: page_number,
            perpage: scrobbles_per_page,
        };
        let query_string = serde_qs::to_string(&requestbody).unwrap();
        let mut path: String = "/apis/mlj_1/scrobbles".to_string();
        if !query_string.is_empty() {
            path = path.to_owned() + "?" + &query_string;
        }
        let res: ScrobblesRes = serde_json::from_str(
                &req_to_hos(self.sess, "GET".to_string(), path, data).await
        )
        .unwrap();
        match res.error {
            None => {
                let mut scrobbles: Vec<Scrobble> = vec![];
                for scrobble in res.list.unwrap() {
                    let dt: DateTime<Utc> =
                        DateTime::from_timestamp(scrobble.time.try_into().unwrap(), 0).unwrap();
                    scrobbles.push(Scrobble {
                        time: dt,
                        track: Track::from_trackres(scrobble.track, None),
                    });
                }
                Ok(scrobbles)
            }
            Some(err) => {
                Err(RequestError::ServerError(err.desc))
            }
        }
    }
}
