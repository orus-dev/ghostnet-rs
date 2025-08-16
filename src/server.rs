use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use reqwest;
use reqwest::header::{HeaderName, HeaderValue};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio;

const DEFAULT_TARGET: &str = "https://crackmes.one";
const GHOST_ROUTE_HEADER: &str = "ghost-route";

async fn handle_request(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let target_url = req
        .headers()
        .get(GHOST_ROUTE_HEADER)
        .and_then(|v| v.to_str().ok())
        .unwrap_or(DEFAULT_TARGET)
        .to_string();

    let url_params = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("");

    let client = reqwest::Client::new();

    let builder = client
        .request(
            reqwest::Method::from_str(req.method().as_str()).unwrap(),
            format!("{}/{}", target_url.trim_end_matches('/'), url_params),
        )
        .headers(reqwest::header::HeaderMap::from_iter(
            req.headers()
                .into_iter()
                .map(|h| {
                    if h.0.as_str() == "host" {
                        (
                            HeaderName::from_str("host").unwrap(),
                            HeaderValue::from_bytes(
                                reqwest::Url::from_str(&target_url)
                                    .unwrap()
                                    .host_str()
                                    .unwrap()
                                    .as_bytes(),
                            )
                            .unwrap(),
                        )
                    } else {
                        (
                            HeaderName::from_str(h.0.as_str()).unwrap(),
                            HeaderValue::from_bytes(h.1.as_bytes()).unwrap(),
                        )
                    }
                })
                .collect::<Vec<_>>(),
        ))
        .body(hyper::body::to_bytes(req.body_mut()).await.unwrap());

    let response = builder.send().await.unwrap();

    return Ok(Response::new(Body::from(response.text().await.unwrap())));
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create service
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    // Create server
    let server = Server::bind(&addr).serve(make_svc);

    println!("HTTP Proxy Server running on http://{}", addr);
    println!("Usage:");
    println!("  - Add 'Ghost-Route: https://target.com' header to route to specific server");
    println!("  - Without header, routes to default: {}", DEFAULT_TARGET);
    println!("Press Ctrl+C to stop");

    // Run server with graceful shutdown
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}
