use poem_openapi::{OpenApi, payload::Json};
use tracing::*;

use super::User;
use crate::utils::common_response::{CommonResponse};

pub struct UserCreateApi;

#[OpenApi]
impl UserCreateApi {
    /// Hello world
    #[oai(path = "/users", method = "post")]
    async fn create_user(&self, user: Json<User>) -> CommonResponse<User> {
        info!("{:?}", user.0);
        user.0.into()
    }
}
