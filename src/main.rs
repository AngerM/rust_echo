use tide::Request;
use tide::prelude::*; // Pulls in the json! macro.

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/").get(|req: Request<()>| async {
            Ok("hello")
        }
    );
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}