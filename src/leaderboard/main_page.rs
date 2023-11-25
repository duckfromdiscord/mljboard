use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{
    web, HttpResponse
};
use mljcl::history::numscrobbles_async;
use mljcl::range::Range;
use reqwest::Client;
use crate::hos_interf::{self, get_maloja_creds_for_sid};
use crate::state::AppState;
use tera::Context;

pub async fn board_main_page(data: web::Data<AppState>) -> HttpResponse {
    
    let mut context = Context::new();

    let mut table: Vec<Vec<String>> = vec![];


    let mut sessids: Vec<String> = vec![];


    let client = Client::builder()
    .danger_accept_invalid_certs(false)
    .build()
    .unwrap();

    let hos_connections = hos_interf::get_hos_connections(data.clone(), client.clone()).await;
    
    for connection in hos_connections.connections {
        sessids.push(connection.0);        
    }

    for sessid in sessids {
        let creds = get_maloja_creds_for_sid(sessid, data.clone());
        let scrobbles = numscrobbles_async(None, Range::In("thisyear".to_string()), creds, client.clone()).await;
        table.push(vec!["User".to_string(), scrobbles.unwrap().to_string()]);
    }

    context.insert("table", &table);
    
    HttpResponse::build(StatusCode::OK)
    .content_type(ContentType::html())
    .body(data.tera.render("board_main_page.html", &context).unwrap())
}