use std::{net::TcpListener, sync::Arc};
use subscriber::run;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, dev::Server};
#[tokio::main]

async fn health_check() -> impl Responder {
    //request handler
  HttpResponse::ok();
}

  

    
#[tokio::main]
 async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { 
        App::new()
            .route("health_check", web::get().to(health_check))
            
    }) 
    .bind("127.0.0.1:8000")? 
    .run()
    .await
    }

fn spawn_app() {
    let server = subscriber::run().expect("failed to bind address");
    let _ = tokio::spawn(server);
}