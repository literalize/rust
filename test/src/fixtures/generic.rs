use literalize::literal;
use utoipa::ToSchema;

#[literal("not_found")]
pub struct NotFoundCode;

#[allow(dead_code)]
#[derive(ToSchema)]
pub struct ErrorResponse {
    pub code: NotFoundCode,
    pub message: String,
}
