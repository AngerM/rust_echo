use tide::{Request, Response};
use tide::prelude::*; // Pulls in the json! macro.
use tide::http::StatusCode;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct Echo {
    method: String,
    path: String,
    headers: HashMap<String, Vec<String>>,
    body: String
}

async fn echo(mut req: Request<()>) -> tide::Result<tide::Response> {
    let mut echoed = Echo{
        method: req.method().to_string(),
        path: req.url().to_string(),
        headers: HashMap::new(),
        body: req.body_string().await.unwrap_or(String::from(""))
    };
    req.iter().for_each(|(name, value_list)|{
        echoed.headers.insert(
            name.to_string(),
            value_list.iter().map(|value|{
                value.to_string()
            }).collect()
        );
    });
    let json_body = serde_json::to_string(&echoed);
    Ok(Response::builder(StatusCode::Ok)
        .header("Cache-Control", "no-cache")
        .body(json_body.unwrap())
        .build())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/").all(echo);
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    app.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}