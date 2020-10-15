use tide::Request;
use tide::prelude::*; // Pulls in the json! macro.
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct Echo {
    method: String,
    path: String,
    headers: HashMap<String, Vec<String>>,
    body: String
}

async fn echo(mut req: Request<()>) -> tide::Result<String> {
    let mut echoed = Echo{
        method: req.method().to_string(),
        path: req.url().to_string(),
        headers: HashMap::new(),
        body: req.body_string().await.unwrap_or(String::from(""))
    };
    req.iter().for_each(|header|{
        echoed.headers.insert(
            header.0.to_string(),
            header.1.iter().map(|value|{
                value.to_string()
            }).collect()
        );
    });
    let json_body = serde_json::to_string(&echoed);
    Ok(json_body.unwrap())
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