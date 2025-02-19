mod traits;

use nanoid::nanoid;
pub use traits::{Answer, Captcha};

use crate::traits::CaptchaError;

pub async fn generate() -> Result<Captcha, CaptchaError> {
    let challenge = nanoid!(16);

    let captcha = Captcha {
        id: nanoid!(),
        challenge: format!("{}#{}", dsa_env::get_env().captcha_difficulty, challenge),
        criteria: Some(challenge),
    };

    dsa_cache::set_ex(captcha.id.clone(), captcha.clone(), 5 * 60).await;

    Ok(captcha)
}

pub async fn check(answer: &Answer) -> Result<bool, CaptchaError> {
    let captcha = dsa_cache::get_del::<Captcha>(
        &answer
            .id
            .clone()
            .ok_or(CaptchaError::MissingField("id".to_owned()))?,
    )
    .await
    .ok_or(CaptchaError::Gone)?;

    let challenge = captcha
        .criteria
        .ok_or(CaptchaError::MissingField("criteria".to_owned()))?;

    let mut context = ring::digest::Context::new(&ring::digest::SHA256);
    context.update(answer.content.as_bytes());
    let result = hex::encode(context.finish().as_ref());

    if answer.content.trim().starts_with(challenge.trim())
        && result.starts_with(
            "0".repeat((dsa_env::get_env().captcha_difficulty + 1) as usize)
                .as_str(),
        )
    {
        return Ok(true);
    }

    Ok(false)
}
