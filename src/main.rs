use poem::{listener::TcpListener, Route, Server, EndpointExt, middleware::Tracing};
use poem_openapi::OpenApiService;
use tracing::*;
use tracing_subscriber::util::SubscriberInitExt;

mod utils;
mod user;

#[tokio::main]
async fn main() {
    let default_logger = tracing_subscriber::fmt().with_max_level(Level::DEBUG).finish();
    default_logger.init();
    let api_service =
        OpenApiService::new(user::user_create::UserCreateApi, "Hello World", "1.0").server("0.0.0.0:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui).with(Tracing);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
        .unwrap();
}
