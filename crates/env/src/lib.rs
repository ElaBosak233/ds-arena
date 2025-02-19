use std::borrow::ToOwned;

use once_cell::sync::Lazy;

use crate::traits::Env;

mod traits;

static ENVS: Lazy<Env> = Lazy::new(|| Env {
    server_host: std::env::var("SERVER_HOST").unwrap_or("0.0.0.0".to_owned()),
    server_port: std::env::var("SERVER_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8889),
    captcha_difficulty: std::env::var("CAPTCHA_DIFFICULTY")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2),
    logger_level: std::env::var("LOGGER_LEVEL").unwrap_or("info".to_owned()),
    criteria_path: std::env::var("CRITERIA_PATH").unwrap_or("resources/criteria.csv".to_owned()),
    flag_content: std::env::var("FLAG_CONTENT").unwrap_or("cdsctf{replace_with_ur_flag}".to_owned()),
});

pub fn get_env() -> &'static Env {
    &ENVS
}
