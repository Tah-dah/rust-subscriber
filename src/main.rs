use std::{net::TcpListener, sync::Arc};
use subscriber::run;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
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