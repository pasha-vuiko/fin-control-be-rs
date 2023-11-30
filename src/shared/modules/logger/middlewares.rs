use axum::http::{HeaderName, Request};
use tower::layer::util::{Identity, Stack};
use tower::ServiceBuilder;
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use uuid::Uuid;

pub const X_REQUEST_ID_HEADER_NAME: HeaderName = HeaderName::from_static("x-request-id");

#[derive(Clone, Default)]
pub struct MakeRequestIdGenerator;

impl MakeRequestId for MakeRequestIdGenerator {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().try_into().ok()?;

        Some(RequestId::new(request_id))
    }
}

pub fn get_request_id_layer() -> RequestIdLayer {
    ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            X_REQUEST_ID_HEADER_NAME.clone(),
            MakeRequestIdGenerator::default(),
        ))
        // propagate `x-request-id` headers from request to response
        .layer(PropagateRequestIdLayer::new(X_REQUEST_ID_HEADER_NAME))
}

pub type RequestIdLayer = ServiceBuilder<
    Stack<PropagateRequestIdLayer, Stack<SetRequestIdLayer<MakeRequestIdGenerator>, Identity>>,
>;
