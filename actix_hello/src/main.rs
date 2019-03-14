use actix_web::{http, server, App, Path, Responder};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: i32,
    user_name: String,
}

fn index(name: Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

fn find_user(id: Path<i32>) -> impl Responder {
    let all_users = vec![
        User {
            user_id: 1,
            user_name: "bob".to_owned(),
        },
        User {
            user_id: 2,
            user_name: "alice".to_owned(),
        },
    ];

    let id = id.into_inner();
    all_users
        .iter()
        .find(|u| u.user_id == id)
        .and_then(|u| serde_json::to_string(u).ok())
        .unwrap_or_else(|| "{}".to_owned())
}

fn main() {
    server::new(|| {
        App::new()
            .route("/hello/{noob}", http::Method::GET, index)
            .route("/users/{id}", http::Method::GET, find_user)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
