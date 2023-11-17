pub mod mappings;

use crate::shared::config::{LogFormat, LogLevel};
use axum::{body::Body, http::Request};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::TraceLayer,
};
use tower_request_id::RequestId;
use tracing::metadata::LevelFilter;
use tracing::{error_span, Span};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

pub fn init_logger(format: &LogFormat, level: &LogLevel) {
    let log_style_layer = tracing_subscriber::fmt::layer();
    let log_filter = filter::Targets::new().with_default(LevelFilter::from(level));

    let tracing_subscriber_registry = match format {
        LogFormat::Pretty => tracing_subscriber::registry()
            .with(log_style_layer.pretty().with_filter(log_filter).boxed()),
        LogFormat::Json => tracing_subscriber::registry()
            .with(log_style_layer.json().with_filter(log_filter).boxed()),
    };

    tracing_subscriber_registry.init();
}

pub type TraceLayerAlias =
    TraceLayer<SharedClassifier<ServerErrorsAsFailures>, fn(&Request<Body>) -> Span>;
pub fn get_logger_layer() -> TraceLayerAlias {
    // Let's create a tracing span for each request
    TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        // We get the request id from the extensions
        let request_id = request
            .extensions()
            .get::<RequestId>()
            .map(ToString::to_string)
            .unwrap_or_else(|| "unknown".into());
        // And then we put it along with other information into the `request` span
        error_span!(
            "request",
            id = %request_id,
            method = %request.method(),
            uri = %request.uri(),
        )
    })
}
