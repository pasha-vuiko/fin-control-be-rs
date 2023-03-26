use crate::shared::mods::auth::service::AuthService;
use crate::shared::mods::redis::redis_service::RedisService;
use axum::body::HttpBody;
use axum::extract::{OriginalUri, State};
use axum::http::{header, Method, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use std::ops::Deref;
use std::sync::Arc;

pub async fn json_cache<B>(
    OriginalUri(original_uri): OriginalUri,
    State(redis_service): State<Arc<RedisService>>,
    State(auth_service): State<Arc<AuthService>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    if request.method() != Method::GET {
        return next.run(request).await;
    }

    let cache_key = original_uri.to_string();
    let cached_response = redis_service.get_str(&cache_key).await;

    let mut response: Response = match cached_response {
        Ok(cached_response_body) => cached_response_body.into_response(),
        Err(_) => {
            let bearer_token = get_bearer_token(&request);
            let original_response = next.run(request).await;
            let response_status = original_response.status();

            if response_status.is_server_error() || response_status.is_client_error() {
                return original_response;
            }
            if let Some(bearer_token) = bearer_token {
                if let Ok(claims) = auth_service.get_claims(&bearer_token) {
                    if claims.is_admin() {
                        return original_response;
                    }
                }
            }

            set_response_cache(&cache_key, original_response, &redis_service).await
        }
    };

    response.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    response
}

async fn set_response_cache(
    cache_key: &str,
    response: Response,
    redis_service: &RedisService,
) -> Response {
    let mut response = response;
    let response_data = response.data().await;

    match response_data {
        Some(response_body_result) => match response_body_result {
            Ok(response_body_bytes) => {
                let response_body_vec = response_body_bytes.deref().to_vec();

                match String::from_utf8(response_body_vec) {
                    Ok(response_body_str) => {
                        let set_result =
                            redis_service.set_str(&cache_key, &response_body_str).await;

                        match set_result {
                            Ok(_) => tracing::debug!(
                                "Cache for endpoint '{}' is set successfully",
                                cache_key
                            ),
                            Err(err) => tracing::warn!(
                                "Cache for endpoint '{}' is failed to set with err: '{}'",
                                cache_key,
                                err
                            ),
                        };
                        response_body_str.into_response()
                    }
                    Err(_) => response,
                }
            }
            Err(_) => response,
        },
        None => response,
    }
}

fn get_bearer_token<B>(request: &Request<B>) -> Option<String> {
    let authorization_header = request.headers().get(header::AUTHORIZATION)?;
    let bearer_token = authorization_header.to_str().ok()?;
    let bearer_token = bearer_token.trim_start_matches("Bearer ");

    Some(bearer_token.to_string())
}
