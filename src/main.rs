use std::collections::HashMap;
use std::env;

use salvo::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
struct Echo {
    method: String,
    path: String,
    params: HashMap<String, Vec<String>>,
    headers: HashMap<String, String>,
    body: String,
    parsed: Value,
}
#[handler]
async fn echo(req: &mut Request, res: &mut Response) {
    // Try to parse body if Json
    let body_str = req
        .payload()
        .await
        .ok()
        .and_then(|b| std::str::from_utf8(b).ok())
        .unwrap_or("")
        .to_string();

    let parsed: Value = serde_json::from_str(&body_str).unwrap_or(Value::Null);

    let params = req
        .queries()
        .iter_all()
        .map(|(k, v)| (k.to_string(), v.to_vec()))
        .collect();

    let headers = req
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    let echoed = Echo {
        method: req.method().to_string(),
        path: req.uri().path().to_string(),
        params,
        headers,
        body: body_str,
        parsed,
    };
    res.render(Json(echoed));
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let router = Router::new()
        .push(Router::new().path("<*>").goal(echo))
        .push(Router::new().goal(echo));

    let port_u16: u16 = port.parse().unwrap_or(8080);
    let listener = TcpListener::new(("0.0.0.0", port_u16));

    /*
    let config = RustlsConfig::new();
    let acceptor = QuinnListener::new(config, addr.as_str())
        .join(listener)
        .bind()
        .await;
     */
    Server::new(listener.bind().await).serve(router).await;
}
