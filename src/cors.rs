use crate::RequestContext;
use hyper::header::HeaderValue;
use hyper::{Body, Request, Response};

fn origin(req: &Request<Body>) -> String {
    match req.headers().get("origin") {
        Some(header_value) => match header_value.to_str() {
            Ok(header_str) => header_str.to_lowercase(),
            Err(_) => String::from(""),
        },
        None => String::from(""),
    }
}

pub(crate) fn cors_host(req: &Request<Body>) -> &str {
    match &*origin(req) {
        "http://localhost:3000" => "http://localhost:3000",
        "http://localhost:5000" => "http://localhost:5000",
        _ => "http://localhost:5000",
    }
}

pub(crate) fn inject_headers(req: &RequestContext<Request<Body>>, res: &mut Response<Body>) {
    res.headers_mut().insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_str(req.cors_allowed_origin).unwrap(),
    );
    res.headers_mut().insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_str("authorization").unwrap(),
    );
}

pub(crate) fn ok() -> Response<Body> {
    Response::new(Body::from("OK"))
}
