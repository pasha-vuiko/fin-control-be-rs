use axum::http::{header, Request};

pub fn get_bearer_token<B>(req: &Request<B>) -> Option<String> {
    let authorization_header = req.headers().get(header::AUTHORIZATION)?;
    let bearer_token = authorization_header.to_str().ok()?.to_string();
    let token = bearer_token.trim_start_matches("Bearer ");

    Some(token.to_string())
}
