mod submission;

use axum::{Router, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::traits::{WebError, WebResponse};

pub async fn router() -> Router {
    Router::new()
        .route("/", axum::routing::any(index))
        .route("/captcha", axum::routing::get(get_captcha))
        .merge(submission::router().await)
}

pub async fn index() -> impl IntoResponse {
    WebResponse::<()> {
        code: StatusCode::OK.as_u16(),
        msg: Some(json!("This is the heart of DS Arena!")),
        ..Default::default()
    }
}

pub async fn get_captcha() -> Result<WebResponse<dsa_captcha::Captcha>, WebError> {
    let captcha = dsa_captcha::generate()
        .await
        .map_err(|_| WebError::BadRequest(json!("captcha_error")))?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(captcha.desensitize()),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn submission_routes_support_both_trailing_slash_forms() {
        let submission = dsa_model::Submission {
            id: "route-test".into(),
            filename: "input.csv".into(),
            data: vec![1, 2, 3],
            accuracy: Some(0.0),
            note: Some("complete".into()),
        };
        dsa_cache::set(submission.id.clone(), submission).await;

        for path in ["/submissions?id=route-test", "/submissions/?id=route-test"] {
            let response = super::router()
                .await
                .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let body = to_bytes(response.into_body(), 4096).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(body["data"]["accuracy"], 0.0);
            assert!(body["data"].get("data").is_none());
        }
    }

    #[tokio::test]
    async fn missing_submissions_return_not_found() {
        let response = super::router()
            .await
            .oneshot(
                Request::builder()
                    .uri("/submissions?id=missing")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
