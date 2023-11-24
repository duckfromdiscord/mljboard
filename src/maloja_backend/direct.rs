use mljcl::{
    history::{scrobbles_async, Scrobble},
    range::Range,
    *,
};
use actix_web::web::Data;
use crate::{maloja_backend::traits::MalojaBackend, hos::state::AppState};

pub struct DirectBackend {
    pub creds: MalojaCredentials,
}

impl MalojaBackend for DirectBackend {
    async fn get_scrobbles(
        &mut self,
        artist: Option<String>,
        range: Range,
        page_number: Option<u64>,
        scrobbles_per_page: Option<u64>,
        _data: Data<AppState>,
    ) -> Result<Vec<Scrobble>, RequestError> {
        let client = mljcl::get_client_async(&self.creds);
        scrobbles_async(
            artist,
            range,
            page_number,
            scrobbles_per_page,
            self.creds.clone(),
            client,
        )
        .await
    }
}
