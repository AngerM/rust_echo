use std::collections::HashMap;
use std::env;

use serde_json::{Map, Value};
// Pulls in the json! macro.
use tide::{Request, Response};
use tide::http::StatusCode;
use tide::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct Echo {
    method: String,
    path: String,
    headers: HashMap<String, Vec<String>>,
    body: String,
    parsed: Map<String, Value>,
}

async fn echo(mut req: Request<()>) -> tide::Result<tide::Response> {
    // Try to parse body if Json
    let body_str = req.body_string().await.unwrap_or(String::from(""));
    let parsed: Value = match serde_json::from_str(body_str.as_str()) {
        Ok(val) => val,
        Err(_) => Value::Object(Default::default()),
    };
    let mut echoed = Echo {
        method: req.method().to_string(),
        path: req.url().path().to_string(),
        headers: HashMap::new(),
        body: body_str,
        parsed: parsed.as_object().unwrap().clone(),
    };
    // Swap the headers into a Vec<String>
    req.iter().for_each(|(name, value_list)| {
        echoed.headers.insert(
            name.to_string(),
            value_list.iter().map(|value| value.to_string()).collect(),
        );
    });
    let json_body = serde_json::to_string(&echoed);
    Ok(Response::builder(StatusCode::Ok)
        .header("Cache-Control", "no-cache")
        .body(json_body.unwrap_or(String::from("")))
        .build())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/").all(echo);
    app.at("/*path").all(echo);
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    app.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}
