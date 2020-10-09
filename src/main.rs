use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/").get(|req: Request<()>| async { Ok(req) });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}