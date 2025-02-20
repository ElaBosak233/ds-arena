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
        .route("/", axum::routing::get(get_submission))
        .route("/", axum::routing::post(create_submission))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetSubmissionRequest {
    pub id: String,
}

pub async fn get_submission(
    Query(params): Query<GetSubmissionRequest>,
) -> Result<WebResponse<dsa_model::Submission>, WebError> {
    let submission = dsa_cache::get::<dsa_model::Submission>(&params.id).await;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: submission,
        ..Default::default()
    })
}

pub async fn create_submission(
    mut multipart: Multipart,
) -> Result<WebResponse<dsa_model::Submission>, WebError> {
    let mut captcha_answer = dsa_captcha::Answer::default();
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name() {
            Some("file") => {
                filename = field.file_name().unwrap().to_string();
                data = match field.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(_err) => {
                        return Err(WebError::BadRequest(json!("size_too_large")));
                    }
                };
            },
            Some("captcha_id") => {
                captcha_answer.id = Some(field.text().await.unwrap())
            },
            Some("captcha_answer") => {
                captcha_answer.content = field.text().await.unwrap()
            }
            _ => {}
        }
    }

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
