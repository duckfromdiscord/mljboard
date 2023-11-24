use actix_files::NamedFile;
use actix_web::{
    middleware, rt, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use clap::{Arg, Command};
use mljboard::hos::hos_handler;
use mljboard::hos::json::hos_request;
use tokio::sync::broadcast;
use uuid::Uuid;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

async fn hos_ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;
    rt::spawn(hos_handler::hos_ws(session, msg_stream));

    Ok(res)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let matches = Command::new("mljboard")
        .arg(Arg::new("ip").short('i').value_name("IP").help("Listen IP"))
        .arg(
            Arg::new("port")
                .short('p')
                .value_name("PORT")
                .help("Listen Port"),
        )
        .get_matches();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let listen_ip: String = matches
        .get_one::<String>("ip")
        .unwrap_or(&"127.0.0.1".to_string())
        .to_string();

    let listen_port: u16 = matches
        .get_one::<String>("port")
        .unwrap_or(&"9002".to_string())
        .parse::<u16>()
        .expect("Invalid port");

    log::info!(
        "starting HTTP server at http://{}:{}",
        listen_ip,
        listen_port
    );

    let (tx, _) = broadcast::channel::<web::Bytes>(128);

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(index))
            .service(web::resource("/ws").route(web::get().to(hos_ws_route)))
            .app_data(web::Data::new(tx.clone()))
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind((listen_ip, listen_port))?
    .run()
    .await
}
