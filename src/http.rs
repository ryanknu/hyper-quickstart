use hyper::{Response, Body, StatusCode};

/// # Forbidden
/// Returns a response payload that indicates a 403 forbidden.
pub(crate) fn forbidden() -> Response<Body> {
    match Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Body::from("403 Forbidden"))
    {
        Ok(response) => response,
        Err(_) => internal_server_error(),
    }
}

/// # Not Found
/// Returns a response payload that indicates a 404 not found.
pub(crate) fn not_found() -> Response<Body> {
    match Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("404 Not found"))
    {
        Ok(response) => response,
        Err(_) => internal_server_error(),
    }
}

/// # Internal Server Serror
/// Returns a response payload that indicates a 500 internal server error.
pub(crate) fn internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("500 Internal Server Error"))
        .unwrap()
}
