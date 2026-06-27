use super::api_components;
use super::schema_json;
use crate::fixtures::int_inference;
use crate::fixtures::int_suffix;

#[test]
fn partial_schema() {
    let schema: serde_json::Value =
        schema_json::<crate::fixtures::int::HttpStatusCode>();

    assert_eq!(schema["type"], "integer");
    assert_eq!(schema["enum"][0], 404);
}

#[test]
fn registered_in_openapi() {
    let components: serde_json::Value = api_components();
    let schema: &serde_json::Value = &components["HttpStatusCode"];

    assert_eq!(schema["type"], "integer");
    assert_eq!(schema["enum"][0], 404);
}

#[test]
fn inference_boundary_schemas() {
    assert_eq!(schema_json::<int_inference::I32Max>()["type"], "integer",);
    assert_eq!(schema_json::<int_inference::AboveI32Max>()["type"], "integer",);
    assert_eq!(schema_json::<int_inference::I32Min>()["type"], "integer",);
    assert_eq!(schema_json::<int_inference::BelowI32Min>()["type"], "integer",);
}

#[test]
fn suffix_schemas() {
    assert_eq!(schema_json::<int_suffix::U8>()["type"], "integer");
    assert_eq!(schema_json::<int_suffix::U16>()["type"], "integer");
    assert_eq!(schema_json::<int_suffix::U32>()["type"], "integer");
    assert_eq!(schema_json::<int_suffix::U64>()["type"], "integer");
    assert_eq!(schema_json::<int_suffix::I8>()["type"], "integer");
    assert_eq!(schema_json::<int_suffix::I16>()["type"], "integer");
    assert_eq!(schema_json::<int_suffix::I32>()["type"], "integer");
    assert_eq!(schema_json::<int_suffix::I64>()["type"], "integer");
}
