mod submission;

use axum::{Router, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::traits::{WebError, WebResponse};

pub async fn router() -> Router {
    Router::new()
        .route("/", axum::routing::any(index))
        .route("/captcha", axum::routing::get(get_captcha))
        .nest("/submissions", submission::router().await)
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
        .map_err(|err| WebError::BadRequest(json!("captcha_error")))?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(captcha.desensitize()),
        ..Default::default()
    })
}
