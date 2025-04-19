use axum::Router;
use tower_http::services::ServeFile;

const FRONTEND_PATH: &str = "./dist";

pub fn router() -> Router {
    Router::new().fallback_service(
        tower_http::services::ServeDir::new(FRONTEND_PATH)
            .precompressed_gzip()
            .not_found_service(ServeFile::new(format!("{}/index.html", FRONTEND_PATH))),
    )
}
