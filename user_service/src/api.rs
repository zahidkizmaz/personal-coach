use axum::{
    Router,
    extract::MatchedPath,
    http::{HeaderName, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
};
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tracing::{error, info_span};

const REQUEST_ID_HEADER: &str = "x-request-id";

pub fn app() -> Router {
    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);
    let request_id_and_logging_middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the request id as generated.
                let request_id = request.headers().get(REQUEST_ID_HEADER);
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                match request_id {
                    Some(request_id) => info_span!(
                        "http_request",
                        request_id = ?request_id,
                        method = ?request.method(),
                        matched_path,
                    ),
                    None => {
                        error!("could not extract request_id");
                        info_span!("http_request",
                            request_id = ?request_id,
                            method = ?request.method(),
                            matched_path,
                        )
                    }
                }
            }),
        )
        // send headers from request to response headers
        .layer(PropagateRequestIdLayer::new(x_request_id));
    Router::new()
        .route("/", get(home))
        .layer(request_id_and_logging_middleware)
}

async fn home() -> Result<(), AppError> {
    Ok(())
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request, http::StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn app_with_error() -> Router {
        Router::new().route("/", get(handler))
    }

    async fn handler() -> Result<(), AppError> {
        try_thing()?;
        Ok(())
    }

    fn try_thing() -> Result<(), anyhow::Error> {
        anyhow::bail!("it failed!")
    }

    #[tokio::test]
    async fn test_anyhow_error() {
        let response = app_with_error()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let html = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(html, "Something went wrong: it failed!");
    }
}
