use axum::http::{HeaderName, HeaderValue, Request};
use tower::layer::util::{Identity, Stack};
use tower::ServiceBuilder;
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use uuid::Uuid;

pub const X_REQUEST_ID_HEADER_NAME: &str = "x-request-id";

#[derive(Clone, Default)]
pub struct MakeRequestIdGenerator;

impl MakeRequestId for MakeRequestIdGenerator {
    fn make_request_id<B>(&mut self, request: &Request<B>) -> Option<RequestId> {
        let request_id = request
            .headers()
            .iter()
            .find(|&(header_name, _)| header_name.as_str() == X_REQUEST_ID_HEADER_NAME)
            .map(|(_, request_id_header_value)| request_id_header_value.clone())
            .unwrap_or(HeaderValue::from_str(&generate_req_id()).ok()?);

        Some(RequestId::new(request_id))
    }
}

pub fn get_request_id_layer() -> RequestIdLayer {
    let x_request_id_header_name = HeaderName::from_static(X_REQUEST_ID_HEADER_NAME);

    ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            x_request_id_header_name.clone(),
            MakeRequestIdGenerator,
        ))
        // propagate `x-request-id` headers from request to response
        .layer(PropagateRequestIdLayer::new(x_request_id_header_name))
}

pub type RequestIdLayer = ServiceBuilder<
    Stack<PropagateRequestIdLayer, Stack<SetRequestIdLayer<MakeRequestIdGenerator>, Identity>>,
>;

fn generate_req_id() -> String {
    Uuid::new_v4().to_string()
}
