use serde::Serialize;
use warp::{self, path, Filter};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: i32,
    user_name: String,
}

fn main() {
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

    let hello = path!("hello" / String).map(|name| format!("Hello {}!", name));
    let users = path!("users" / i32).map(move |id| {
        all_users
            .iter()
            .find(|u| u.user_id == id)
            .and_then(|u| serde_json::to_string(u).ok())
            .unwrap_or_else(|| "{}".to_owned())
    });

    let routes = hello.or(users);
    warp::serve(routes).run(([127, 0, 0, 1], 8080));
}
