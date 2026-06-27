use utoipa::OpenApi;
use utoipa::PartialSchema;

use crate::fixtures::int_inference;
use crate::fixtures::int_suffix;

mod bool;
mod float;
mod int;
mod str;

#[allow(dead_code)]
#[derive(OpenApi)]
#[openapi(components(schemas(
    crate::fixtures::str::NotFound,
    crate::fixtures::int::HttpStatusCode,
    crate::fixtures::float::EulersNumber,
    crate::fixtures::bool::FeatureEnabled,
    int_inference::I32Max,
    int_inference::AboveI32Max,
    int_inference::I32Min,
    int_inference::BelowI32Min,
    int_suffix::U8,
    int_suffix::U16,
    int_suffix::U32,
    int_suffix::U64,
    int_suffix::I8,
    int_suffix::I16,
    int_suffix::I32,
    int_suffix::I64,
)))]
struct ApiDoc;

#[allow(dead_code)]
fn schema_json<T: PartialSchema>() -> serde_json::Value {
    serde_json::to_value(T::schema()).unwrap()
}

#[allow(dead_code)]
fn api_components() -> serde_json::Value {
    serde_json::to_value(&ApiDoc::openapi().components)
        .unwrap()
        .get("schemas")
        .unwrap()
        .clone()
}
