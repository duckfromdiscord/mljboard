pub struct AppState {
    pub hos_server_ip: String,
    pub hos_server_port: u16,
    pub tera: tera::Tera,
}

impl AppState {
    pub fn hos_server_addr(&self) -> String {
        "http://".to_owned() + &self.hos_server_ip.clone() + ":" + &self.hos_server_port.to_string()
    }
}