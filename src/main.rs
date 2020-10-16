use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use lazy_static::lazy_static;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

mod auth;
mod cors;
mod http;

lazy_static! {
    pub static ref REQWEST: reqwest::Client = reqwest::Client::new();
}

fn main() {
    dotenv::dotenv().ok();
    check_env();
    start();
}

#[tokio::main]
pub async fn start() {
    let addr = SocketAddr::from(([127, 0, 0, 1], port()));
    let server = Server::bind(&addr);
    let server = server.serve(make_service_fn(|_connection| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {
            let req = route(&req).await;
            let mut res = match req.path {
                "<options>" => cors::ok(),
                "/" => Response::new(Body::from(req.user_id.clone())),
                "<forbidden>" => http::forbidden(),
                _ => http::not_found(),
            };
            cors::inject_headers(&req, &mut res);
            Ok::<_, Infallible>(res)
        }))
    }));

    crate::log_message("Hyper server started");

    if let Err(e) = server.await {
        crate::log_error(&format!("Hyper server error: {:?}", e));
    }
}

fn check_env() {
    log_message("Checking env...");
    env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set");
}

/// # Route Request
/// Implement custom routing and "middleware" here. I like to use paths that are impossible for
/// routes that have been pre-processed, like &lt;route&gt;, so they can't be invoked from the URI.
///
/// Custom routing example (for a URL like /filters/:id):
/// ```
/// else if req.uri().path()[0..9].eq("/filters/") {
///     let filterName = req.uri().path()[9..];
///     let path = "<filter>"
/// }
/// ```
///
/// Middleware example (inform all actions that it is midMinute when second == 30)
/// ```
/// if chrono::Utc::now().second() == 30 { context.midMinute = true }
/// ```
async fn route<'a>(req: &'a Request<Body>) -> RequestContext<'a, Request<Body>> {
    let is_options = req.method().eq(&Method::OPTIONS);
    let user_id = if !is_options {
        auth::verify(req).await
    } else {
        String::from("")
    };

    RequestContext {
        cors_allowed_origin: cors::cors_host(req),
        path: if is_options {
            "<options>"
        } else if user_id.len() < 1 {
            "<forbidden>"
        } else {
            req.uri().path()
        },
        request: req,
        user_id,
    }
}

struct RequestContext<'a, T> {
    cors_allowed_origin: &'a str,
    path: &'a str,
    request: &'a T,
    user_id: String,
}

fn port() -> u16 {
    match env::var("PORT") {
        Ok(port) => match port.parse() {
            Ok(port) => port,
            _ => 80,
        },
        _ => 80,
    }
}

/// # Log Message
/// Logs a message to the journal.
pub(crate) fn log_message(message: &str) {
    println!("msg [{}] {}", chrono::Utc::now(), message);
}

/// # Log Error
/// Just a convenience method to make all error logs go through one method. This makes all log
/// entries have the date prepended to them.
pub(crate) fn log_error(message: &str) {
    eprintln!("err [{}] {}", chrono::Utc::now(), message);
}
