pub struct Env {
    pub flag: String,
    pub server_host: String,
    pub server_port: u16,
    pub logger_level: String,
    pub captcha_difficulty: u8,
    pub criteria_path: String,
    pub expected_accuracy: f64,
}
