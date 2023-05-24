use axum::body::{Body, HttpBody};
use axum::extract::OriginalUri;
use axum::http::{header, Method, Request};
use axum::response::{IntoResponse, Response};
use axum::RequestPartsExt;
use futures_util::future::BoxFuture;
use std::ops::Deref;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

use crate::shared::mods::auth::traits::role_based_bearer_auth::AuthService;
use crate::shared::mods::cache::traits::cache_service::CacheService;
use crate::shared::utils::get_bearer_token;

pub type DynAuthService = dyn AuthService + Send + Sync;

#[derive(Clone)]
pub struct JsonCacheLayer<C>
where
    C: CacheService + Send + Sync,
{
    cache_service: Arc<C>,
    auth_service: Arc<DynAuthService>,
}

impl<C> JsonCacheLayer<C>
where
    C: CacheService + Send + Sync,
{
    pub fn new(cache_service: Arc<C>, auth_service: Arc<DynAuthService>) -> Self {
        Self {
            cache_service,
            auth_service,
        }
    }
}

impl<S, C> Layer<S> for JsonCacheLayer<C>
where
    C: CacheService + Send + Sync,
    S: Clone,
{
    type Service = JsonCacheMiddleware<S, C>;

    fn layer(&self, inner: S) -> Self::Service {
        JsonCacheMiddleware {
            inner,
            cache_service: self.cache_service.clone(),
            auth_service: self.auth_service.clone(),
        }
    }
}

#[derive(Clone)]
pub struct JsonCacheMiddleware<S, C>
where
    C: CacheService + Send + Sync,
    S: Clone,
{
    inner: S,
    cache_service: Arc<C>,
    auth_service: Arc<DynAuthService>,
}

impl<S, C> JsonCacheMiddleware<S, C>
where
    C: CacheService + Send + Sync,
    S: Clone,
{
    fn is_admin(&self, bearer_token: Option<String>) -> bool {
        if let Some(bearer_token) = bearer_token {
            if let Ok(user) = self.auth_service.get_user(&bearer_token) {
                if user.is_admin() {
                    return true;
                }
            }
        }

        false
    }
}

impl<S, C> Service<Request<Body>> for JsonCacheMiddleware<S, C>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Future: Send + 'static,
    C: CacheService + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let bearer_token = get_bearer_token(&request);
        let is_admin = self.is_admin(bearer_token);

        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let cache_service = self.cache_service.clone();

        Box::pin(async move {
            if request.method() != Method::GET || is_admin {
                return inner.call(request).await;
            }

            let (mut parts, body) = request.into_parts();

            match parts.extract::<OriginalUri>().await {
                Ok(original_uri) => {
                    let cache_key = original_uri.to_string();
                    let cached_response = cache_service.get_str(&cache_key).await;

                    let mut response: Response = match cached_response {
                        Ok(cached_response_body) => cached_response_body.into_response(),
                        Err(_) => {
                            let request = Request::from_parts(parts, body);
                            let original_response = inner.call(request).await?;

                            let response_status = original_response.status();

                            let is_error = response_status.is_server_error()
                                || response_status.is_client_error();

                            if is_error {
                                return Ok(original_response);
                            }

                            set_response_cache(&cache_key, original_response, cache_service.clone())
                                .await
                        }
                    };

                    response.headers_mut().insert(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_static("application/json"),
                    );

                    Ok(response)
                }
                Err(_) => {
                    let request = Request::from_parts(parts, body);

                    inner.call(request).await
                }
            }
        })
    }
}

async fn set_response_cache<R>(
    cache_key: &str,
    response: Response,
    cache_service: Arc<R>,
) -> Response
where
    R: CacheService + Send + Sync,
{
    let mut response = response;
    let response_data = response.data().await;

    match response_data {
        Some(response_body_result) => match response_body_result {
            Ok(response_body_bytes) => {
                let response_body_vec = response_body_bytes.deref().to_vec();

                match String::from_utf8(response_body_vec) {
                    Ok(response_body_str) => {
                        let set_result = cache_service.set_str(cache_key, &response_body_str).await;

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
