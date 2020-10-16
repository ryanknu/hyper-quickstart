use hyper::{Body, Request};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Claims {
    sub: String,
}

fn access_token(req: &Request<Body>) -> String {
    match req.headers().get("authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(header_str) => header_str.replace("Bearer ", "").clone(),
            Err(_) => String::from(""),
        },
        None => String::from(""),
    }
}

pub(crate) async fn verify(req: &Request<Body>) -> String {
    match verify_online(req).await {
        Ok(t) => t,
        _ => String::from(""),
    }
}

async fn verify_online(req: &Request<Body>) -> Result<String, reqwest::Error> {
    let domain = env::var("AUTH0_DOMAIN").unwrap();

    let access_token = access_token(req);
    if access_token.len() < 1 {
        return Ok(String::from(""));
    }

    let body = crate::REQWEST
        .get(&format!("https://{}/userinfo", domain))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .text()
        .await?;

    let claims = serde_json::from_str::<Claims>(&body);
    if let Ok(claims) = claims {
        return Ok(claims.sub.clone());
    }

    Ok(String::from(""))
}
