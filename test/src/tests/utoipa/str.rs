use super::api_components;
use super::schema_json;

#[test]
fn partial_schema() {
    let schema: serde_json::Value =
        schema_json::<crate::fixtures::str::NotFound>();

    assert_eq!(schema["type"], "string");
    assert_eq!(schema["enum"][0], "not_found");
}

#[test]
fn registered_in_openapi() {
    let components: serde_json::Value = api_components();
    let schema: &serde_json::Value = &components["NotFound"];

    assert_eq!(schema["type"], "string");
    assert_eq!(schema["enum"][0], "not_found");
}
