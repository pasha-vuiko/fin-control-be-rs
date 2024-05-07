mod errors;
pub mod mappings;
pub mod middlewares;
mod utils;

use crate::shared::config::{LogFormat, LogLevel};
use crate::shared::modules::logger::middlewares::X_REQUEST_ID_HEADER_NAME;
use crate::shared::modules::logger::utils::get_crates_log_filter;
use axum::{body::Body, http::Request};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::TraceLayer,
};
use tracing::metadata::LevelFilter;
use tracing::Span;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

pub fn init_logger(format: &LogFormat, level: &LogLevel) {
    let log_style_layer = tracing_subscriber::fmt::layer();
    let tracing_layer = match format {
        LogFormat::Pretty => log_style_layer.pretty().boxed(),
        LogFormat::Json => log_style_layer.json().boxed(),
    };

    let global_log_level_filter = filter::Targets::new().with_default(LevelFilter::from(level));
    let crates_log_level_filter = get_crates_log_filter().expect("Failed to get crates log filter");

    tracing_subscriber::registry()
        .with(
            tracing_layer
                .with_filter(crates_log_level_filter)
                .with_filter(global_log_level_filter),
        )
        .init();
}

pub type TraceLayerAlias =
    TraceLayer<SharedClassifier<ServerErrorsAsFailures>, fn(&Request<Body>) -> Span>;
pub fn get_logger_layer() -> TraceLayerAlias {
    // Let's create a tracing span for each request
    TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        // We get the request id from the extensions
        let request_id = request
            .headers()
            .iter()
            .find(|&(header_name, _)| header_name.as_str() == X_REQUEST_ID_HEADER_NAME)
            .map(|(_, request_id_header_value)| request_id_header_value)
            .map(|request_id| request_id.to_str())
            .transpose()
            .ok()
            .flatten()
            .unwrap_or("unknown");

        // And then we put it along with other information into the `request` span
        tracing::error_span!(
            "request",
            id = %request_id,
            method = %request.method(),
            uri = %request.uri(),
        )
    })
}
