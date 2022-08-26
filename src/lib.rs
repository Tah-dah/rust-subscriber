use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

//returns a 200 OK response
async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(||{
        App::new()
            .route("health_check", web::get().to(health_check))
            .route("/subscriptions", web::get().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)

}


