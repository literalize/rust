use super::api_components;
use super::schema_json;

#[test]
fn partial_schema() {
    let schema: serde_json::Value =
        schema_json::<crate::fixtures::bool::FeatureEnabled>();

    assert_eq!(schema["type"], "boolean");
    assert_eq!(schema["enum"][0], true);
}

#[test]
fn registered_in_openapi() {
    let components: serde_json::Value = api_components();
    let schema: &serde_json::Value = &components["FeatureEnabled"];

    assert_eq!(schema["type"], "boolean");
    assert_eq!(schema["enum"][0], true);
}
