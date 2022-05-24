use std::io;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("192.168.1.7:3030")?
    .run()
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
