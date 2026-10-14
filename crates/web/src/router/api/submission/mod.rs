use axum::{
    Router,
    extract::{Multipart, Query},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::traits::{WebError, WebResponse};

pub async fn router() -> Router {
    Router::new()
        .route(
            "/submissions",
            axum::routing::get(get_submission).post(create_submission),
        )
        .route(
            "/submissions/",
            axum::routing::get(get_submission).post(create_submission),
        )
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetSubmissionRequest {
    pub id: String,
}

pub async fn get_submission(
    Query(params): Query<GetSubmissionRequest>,
) -> Result<WebResponse<dsa_model::Submission>, WebError> {
    let submission = dsa_cache::get::<dsa_model::Submission>(&params.id)
        .await
        .ok_or_else(|| WebError::NotFound(json!("submission_not_found")))?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(submission),
        ..Default::default()
    })
}

pub async fn create_submission(
    mut multipart: Multipart,
) -> Result<WebResponse<dsa_model::Submission>, WebError> {
    let mut captcha_answer = dsa_captcha::Answer::default();
    let mut filename = String::new();
    let mut data = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| WebError::BadRequest(json!(err.to_string())))?
    {
        match field.name() {
            Some("file") => {
                filename = field
                    .file_name()
                    .filter(|name| !name.is_empty())
                    .ok_or_else(|| WebError::BadRequest(json!("filename_required")))?
                    .to_string();
                data = Some(match field.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(_err) => {
                        return Err(WebError::BadRequest(json!("size_too_large")));
                    }
                });
            }
            Some("captcha_id") => {
                captcha_answer.id = Some(
                    field
                        .text()
                        .await
                        .map_err(|err| WebError::BadRequest(json!(err.to_string())))?,
                )
            }
            Some("captcha_answer") => {
                captcha_answer.content = field
                    .text()
                    .await
                    .map_err(|err| WebError::BadRequest(json!(err.to_string())))?
            }
            _ => {}
        }
    }

    let data = data
        .filter(|data| !data.is_empty())
        .ok_or_else(|| WebError::BadRequest(json!("file_required")))?;

    if !dsa_captcha::check(&captcha_answer).await.unwrap_or(false) {
        return Err(WebError::BadRequest(json!("captcha_error")));
    }

    let submission = dsa_model::Submission {
        id: nanoid::nanoid!(),
        filename,
        data,
        accuracy: None,
        note: None,
    };

    dsa_cache::set(submission.id.clone(), submission.clone()).await;
    dsa_queue::publish("checker", submission.id.clone()).await;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(submission),
        ..Default::default()
    })
}
