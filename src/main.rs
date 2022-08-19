use std::collections::HashMap;
use std::env;

use serde_json::{Map, Value};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Echo {
    method: String,
    path: String,
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: String,
    parsed: Map<String, Value>,
}

#[handler]
async fn echo(req: &mut Request, res: &mut Response) {
    // Try to parse body if Json
    let body_str = req.parse_body().await.unwrap_or(String::from(""));
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
        echoed.headers.insert(
            name.to_string(),
            value_list.to_str().unwrap().to_string(),
        );
    });
    let json_body = serde_json::to_string(&echoed);
    res.set_status_code(StatusCode::OK);
    res.render(json_body.unwrap_or(String::from("")));
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let router = Router::new()
        .path("<*>")
        .handle(echo);
    let addr = format!("0.0.0.0:{}", port);
    Server::new(
        TcpListener::bind(
            addr.as_str()
        )
    ).serve(router).await;
}
