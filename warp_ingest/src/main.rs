use validator_derive::Validate;
use validator::Validate;

use warp::{self, path, Filter};
use serde::Deserialize;
use serde_json;
use eui48::MacAddress;

#[derive(Debug, Validate, Deserialize)]
struct MerakiData {
    #[validate(length(equal = "3"), contains = "2.0")]
    version: String,
    secret: String,
    r#type: String,
    data: MerakiObs
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MerakiObs {
    ap_mac: MacAddress,
    ap_floors: Vec<String>,
    ap_tags: Vec<String>,
    observations: Vec<String>
}

fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = path!("ingest" / String)
        .map(|input: String| {
            let payload: Option<MerakiData> = serde_json::from_str(&input).ok();
            payload.is_some()
        })
        .map(|is_some| format!("Hello {}!", is_some));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3005));
}
