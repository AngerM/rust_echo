use std::collections::HashMap;
use std::env;

use salvo::{hyper::{HeaderMap, body::Bytes}, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Serialize)]
struct Echo {
    method: String,
    path: String,
    params: HashMap<String, Vec<String>>,
    headers: HashMap<String, String>,
    body: String,
    parsed: Map<String, Value>,
}
#[handler]
async fn echo(req: &mut Request, res: &mut Response) {
    // Try to parse body if Json
    let body_str = std::str::from_utf8(req.payload().await.unwrap())
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
    echoed.params.extend(req.queries().iter_all().map(|(k, v)| (k.to_string(), v.to_vec())));
    echoed.headers.extend(req.headers().iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string())));
    let json_body = serde_json::to_string(&echoed);
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    res.status_code = Some(StatusCode::OK);
    res.set_headers(headers);
    res.write_body(
        json_body.unwrap_or(String::from("")).as_bytes().to_vec()
    ).ok();
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let router = Router::new()
        .push(Router::new().path("<*>").handle(echo))
        .push(Router::new().handle(echo));

    let addr = format!("0.0.0.0:{}", port);
    Server::new(
        TcpListener::new(addr.as_str())
        )
        .serve(router)
        .await;
}
