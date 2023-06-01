use actix_web::{web, App, HttpServer};

mod service;

struct WebConfig {
        domain: &'static str,
        port: u32,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
        let web_config = WebConfig {
                domain: "127.0.0.1",
                port: 8800
        };

        

        HttpServer::new(|| 
                App::new()
                        .configure(service::routes::config)
                )
                .bind(format!("{}:{}", web_config.domain, web_config.port))?
                .run()
                .await
}
