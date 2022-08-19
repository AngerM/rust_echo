use std::collections::HashMap;
use std::env;

use salvo::{hyper::HeaderMap, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Serialize)]
struct Echo {
    method: String,
    path: String,
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: String,
    parsed: Map<String, Value>,
}
static empty: Vec<u8> = vec![];

#[handler]
async fn echo(req: &mut Request, res: &mut Response) {
    // Try to parse body if Json
    let body_str = std::str::from_utf8(req.payload().await.unwrap_or(&empty))
        .unwrap_or("")
        .to_string();
    let parsed: Value = match serde_json::from_str(body_str.as_str()) {
        Ok(val) => val,
        Err(_) => Value::Object(Default::default()),
    };
    let mut echoed = Echo {
        method: req.method().to_string(),
        path: req.uri().path().to_string(),
        params: HashMap::new(),
        headers: HashMap::new(),
        body: body_str,
        parsed: parsed.as_object().unwrap().clone(),
    };
    req.queries().iter().for_each(|(k, v)| {
        echoed.params.insert(k.to_string(), v.to_string());
    });
    req.headers().iter().for_each(|(name, value_list)| {
        echoed
            .headers
            .insert(name.to_string(), value_list.to_str().unwrap().to_string());
    });
    let json_body = serde_json::to_string(&echoed);
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    res.set_status_code(StatusCode::OK);
    res.set_headers(headers);
    res.render(json_body.unwrap_or(String::from("")));
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let router = Router::new()
        .push(Router::new().path("<*>").handle(echo))
        .push(Router::new().handle(echo));

    let addr = format!("0.0.0.0:{}", port);
    Server::new(TcpListener::bind(addr.as_str()))
        .serve(router)
        .await;
}
