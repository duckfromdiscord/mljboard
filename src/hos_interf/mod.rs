use actix_web::web::Data;
use mljcl::MalojaCredentials;

use crate::state::AppState;
use reqwest::Client;
use hos_rv::json::HOSConnectionList;

pub async fn get_hos_connections(data: Data<AppState>, client: Client) -> HOSConnectionList {
    let response = client
            .get(data.hos_server_addr() + "/list")
            .send()
            .await;
    let list: HOSConnectionList = response.unwrap().json::<HOSConnectionList>().await.unwrap();
    return list;
}

pub fn get_maloja_creds_for_sid(sid: String, data: Data<AppState>) -> MalojaCredentials {
    MalojaCredentials {
        https: false,
        skip_cert_verification: true,
        ip: data.hos_server_ip.clone(),
        port: data.hos_server_port,
        path: Some("/sid/".to_owned() + &sid),
        api_key: None,
    }
}