use mljcl::{history::Scrobble, range::Range, RequestError};
use actix_web::web::Data;
use crate::hos::state::AppState;

pub trait MalojaBackend {
    fn get_scrobbles(
        &mut self,
        artist: Option<String>,
        range: Range,
        page_number: Option<u64>,
        scrobbles_per_page: Option<u64>,
        data: Data<AppState>,
    ) -> impl std::future::Future<Output = Result<Vec<Scrobble>, RequestError>> + Send;
    fn get_scrobbles_within(
        &mut self,
        artist: Option<String>,
        within: String,
        data: Data<AppState>,
    ) -> impl std::future::Future<Output = Result<Vec<Scrobble>, RequestError>> + Send {
        Self::get_scrobbles(self, artist, Range::In(within), None, None, data)
    }
}
