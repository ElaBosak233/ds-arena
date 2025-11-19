use std::io::Cursor;

use polars::prelude::{CsvReader, SerReader};

use crate::check;

pub async fn checker() {
    let mut rx = dsa_queue::subscribe("checker").await;
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Some(id) = msg.downcast_ref::<String>() {
                let submission = dsa_cache::get::<dsa_model::Submission>(id).await;
                if let Some(submission) = submission {
                    let reader = Cursor::new(submission.data.clone());
                    let mut submission = dsa_model::Submission {
                        data: vec![],
                        ..submission
                    };
                    if let Ok(df) = CsvReader::new(reader).finish() {
                        let accuracy = check(df).await.unwrap_or(0.0);
                        submission.accuracy = Some(accuracy);

                        submission.note =
                            Some(if accuracy >= dsa_env::get_env().expected_accuracy {
                                dsa_env::get_env().flag.to_owned()
                            } else {
                                format!(
                                    "give_you_flag_when_score_gte_{}%",
                                    dsa_env::get_env().expected_accuracy * 100f64
                                )
                            })
                    } else {
                        submission.accuracy = Some(0.0);
                        submission.note = Some("invalid input".to_owned());
                    }
                    dsa_cache::set(id.to_owned(), submission).await;
                }
            }
        }
    });
}
