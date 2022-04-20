pub mod user_create;

use poem_openapi::payload::Json;
use nft_macros::CommonResponse;
use poem_openapi::{payload::PlainText, Object, OpenApi};

use crate::utils::common_response::{CommonResponse, Message};

#[derive(Object, CommonResponse, Debug)]
pub struct User {
    id: i64
}


pub struct Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/users", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}
