use actix_files::Files;
use actix_web::{
    middleware, web, App, HttpServer, Responder, web::Redirect
};
use clap::{Arg, Command};
use tokio::sync::broadcast;
use tera::Tera;
use mljboard::state::AppState;

async fn root_redir() -> impl Responder {
    Redirect::to("/board").permanent()
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
        .arg(Arg::new("hos_ip")
                .short('j')
                .value_name("HOS_IP")
                .help("HOS server IP address")
        )
        .arg(Arg::new("hos_port")
            .short('k')
            .value_name("HOS_PORT")
            .help("HOS server port")
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
    
    let hos_server_ip: String = matches
    .get_one::<String>("hos_ip")
    .expect("HOS IP required").to_string();
    
    let hos_server_port: u16 = matches
    .get_one::<String>("hos_port")
    .expect("HOS port required")
    .parse::<u16>()
    .expect("Invalid HOS port");
    
    log::info!(
        "starting HTTP server at http://{}:{}",
        listen_ip,
        listen_port
    );

    let (tx, _) = broadcast::channel::<web::Bytes>(128);

    let mut tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    
    tera.add_template_file("templates/board_main_page.html", Some("board_main_page.html"));

    let state = web::Data::new(AppState {
        hos_server_ip,
        hos_server_port,
        tera,
    });


    HttpServer::new(move || {
        App::new()
        .app_data(state.clone())
            .service(web::resource("/").to(root_redir))
            .service(Files::new("/static/", "./static/").index_file("index.html"))
            .service(web::resource("/board").to(mljboard::leaderboard::main_page::board_main_page))
            .app_data(web::Data::new(tx.clone()))
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind((listen_ip, listen_port))?
    .run()
    .await
}
