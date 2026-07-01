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

pub fn app_router() -> Router {
    Router::new()
        .push(Router::new().path("{**rest}").goal(echo))
        .push(Router::new().goal(echo))
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let router = app_router();

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

#[cfg(test)]
mod tests {
    use super::*;
    use salvo::test::{ResponseExt, TestClient};

    #[tokio::test]
    async fn test_root_path_get() {
        let service = Service::new(app_router());
        let mut res = TestClient::get("http://127.0.0.1/").send(&service).await;
        
        assert_eq!(res.status_code, Some(StatusCode::OK));
        let echo_res: Echo = res.take_json().await.unwrap();
        assert_eq!(echo_res.method, "GET");
        assert_eq!(echo_res.path, "/");
        assert!(echo_res.body.is_empty());
    }

    #[tokio::test]
    async fn test_nested_path_get() {
        let service = Service::new(app_router());
        let mut res = TestClient::get("http://127.0.0.1/foo/bar/baz/qux").send(&service).await;
        
        assert_eq!(res.status_code, Some(StatusCode::OK));
        let echo_res: Echo = res.take_json().await.unwrap();
        assert_eq!(echo_res.method, "GET");
        assert_eq!(echo_res.path, "/foo/bar/baz/qux");
    }

    #[tokio::test]
    async fn test_query_parameters() {
        let service = Service::new(app_router());
        let mut res = TestClient::get("http://127.0.0.1/echo?foo=bar&baz=qux&foo=another")
            .send(&service)
            .await;
        
        assert_eq!(res.status_code, Some(StatusCode::OK));
        let echo_res: Echo = res.take_json().await.unwrap();
        
        assert_eq!(
            echo_res.params.get("foo").unwrap(),
            &vec!["bar".to_string(), "another".to_string()]
        );
        assert_eq!(
            echo_res.params.get("baz").unwrap(),
            &vec!["qux".to_string()]
        );
    }

    #[tokio::test]
    async fn test_http_methods() {
        let service = Service::new(app_router());

        for method in &["POST", "PUT", "DELETE", "PATCH"] {
            let mut res = match *method {
                "POST" => TestClient::post("http://127.0.0.1/"),
                "PUT" => TestClient::put("http://127.0.0.1/"),
                "DELETE" => TestClient::delete("http://127.0.0.1/"),
                "PATCH" => TestClient::patch("http://127.0.0.1/"),
                _ => unreachable!(),
            }
            .send(&service)
            .await;

            assert_eq!(res.status_code, Some(StatusCode::OK));
            let echo_res: Echo = res.take_json().await.unwrap();
            assert_eq!(echo_res.method, *method);
            assert_eq!(echo_res.path, "/");
        }
    }

    #[tokio::test]
    async fn test_custom_headers() {
        let service = Service::new(app_router());
        let mut res = TestClient::get("http://127.0.0.1/")
            .add_header("X-Custom-Echo-Header", "hello-world", true)
            .send(&service)
            .await;

        assert_eq!(res.status_code, Some(StatusCode::OK));
        let echo_res: Echo = res.take_json().await.unwrap();
        assert_eq!(
            echo_res.headers.get("x-custom-echo-header").map(|s| s.as_str()),
            Some("hello-world")
        );
    }

    #[tokio::test]
    async fn test_post_json_payload() {
        let service = Service::new(app_router());
        let payload = serde_json::json!({
            "message": "test echo body",
            "number": 12345,
            "nested": {
                "status": true
            }
        });

        let mut res = TestClient::post("http://127.0.0.1/submit")
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(res.status_code, Some(StatusCode::OK));
        let echo_res: Echo = res.take_json().await.unwrap();
        assert_eq!(echo_res.method, "POST");
        assert_eq!(echo_res.path, "/submit");
        assert_eq!(echo_res.parsed, payload);
        assert!(echo_res.body.contains("test echo body"));
    }
}

