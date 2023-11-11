use crate::shared::errors::http_error::HttpError;
use crate::shared::extractors::open_api_json::Json;
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

pub fn get_open_api() -> OpenApi {
    aide::gen::on_error(|error| {
        println!("init Open API error: {error}");
    });
    aide::gen::extract_schemas(true);

    OpenApi::default()
}

pub fn get_api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Fin control be")
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
        .default_response_with::<Json<HttpError>, _>(|res| {
            res.example(HttpError::Internal("Internal Server Error".into()))
        })
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

// async fn serve_docs(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
async fn serve_docs(Extension(open_api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(open_api).into_response()
}
