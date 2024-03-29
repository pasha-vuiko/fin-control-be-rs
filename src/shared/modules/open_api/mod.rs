use aide::transform::TransformOpenApi;
use aide::{
    axum::{
        routing::{get, get_with},
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
    redoc::Redoc,
};
use axum::{response::IntoResponse, Extension};
use axum_jsonschema::Json;
use std::sync::Arc;

pub fn get_open_api() -> OpenApi {
    aide::gen::on_error(|error| {
        println!("init Open API error: {error}");
    });
    aide::gen::extract_schemas(true);
    aide::gen::infer_responses(true);

    OpenApi::default()
}

pub fn get_api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Fin control BE").security_scheme(
        "BearerAuth",
        aide::openapi::SecurityScheme::Http {
            scheme: "".to_string(),
            bearer_format: Some("Bearer <token>".to_string()),
            description: Some("A key that is ignored.".into()),
            extensions: Default::default(),
        },
    )
}

pub fn get_open_api_router() -> ApiRouter {
    // We infer the return types for these routes
    // as an example.
    //
    // As a result, the `serve_redoc` route will
    // have the `text/html` content-type correctly set
    // with a 200 status.
    aide::gen::infer_responses(true);

    let router = ApiRouter::new()
        .api_route_with(
            "/",
            get_with(
                Redoc::new("/docs/private/api.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        .route("/private/api.json", get(serve_docs));

    // Afterwards we disable response inference because
    // it might be incorrect for other routes.
    aide::gen::infer_responses(false);

    router
}

async fn serve_docs(Extension(open_api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(open_api).into_response()
}
