use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object,
};

#[derive(ApiResponse)]
pub enum CommonResponse<T: ParseFromJSON + ToJSON> {
    #[oai(status = 200)]
    Ok(Json<Message<T>>),
    #[oai(status = 400)]
    Failed,
}

#[derive(Object)]
#[oai(inline)]
pub struct Message<T: ParseFromJSON + ToJSON> {
    pub data: T,
}
