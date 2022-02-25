/* logging */
extern crate log;
extern crate env_logger;
/* http server */
extern crate actix_web;

use actix_web::{web, App, HttpServer, Error, HttpResponse};
use actix_web::middleware::Logger;
//use futures::StreamExt;

/* routes */
async fn ping_route(_request_payload: web::Payload) -> Result<HttpResponse, Error> {
    // ping_route response to client
    Ok(HttpResponse::Ok().content_type("text/plain").body("pong"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let vehicle_server_ip = "127.0.0.1";
    let vehicle_server_port = 3000;
    println!("binding vehicle-server to {}:{}", vehicle_server_ip, vehicle_server_port);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %Dms"))
            .service(web::resource("/ping").route(web::get().to(ping_route)))
        })
    .bind((vehicle_server_ip, vehicle_server_port))?
    .run()
    .await
}
