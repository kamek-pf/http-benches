use actix_web::{http, server, App, Path, Responder};

fn index(name: Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

fn main() {
    server::new(
        || App::new()
            .route("/hello/{noob}", http::Method::GET, index))
        .bind("127.0.0.1:8080").unwrap()
        .run();
}
