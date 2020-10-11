use tide::Request;
use tide::prelude::*; // Pulls in the json! macro.
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Echo {
    method: String,
    headers: HashMap<String, String>
}

async fn echo(req: Request<()>) -> tide::Result<String> {
    let echod = Echo{
        method: req.method().to_string(),
        headers: HashMap::new()
    };
    let jsonBody = serde_json::to_string(&echod);
    Ok(jsonBody.unwrap())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/").get(echo);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}