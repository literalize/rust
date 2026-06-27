use super::api_components;
use super::schema_json;

#[test]
fn partial_schema() {
    let schema: serde_json::Value =
        schema_json::<crate::fixtures::float::EulersNumber>();

    assert_eq!(schema["type"], "number");
    assert!((schema["enum"][0].as_f64().unwrap() - 2.71).abs() < f64::EPSILON);
}

#[test]
fn registered_in_openapi() {
    let components: serde_json::Value = api_components();
    let schema: &serde_json::Value = &components["EulersNumber"];

    assert_eq!(schema["type"], "number");
    assert!((schema["enum"][0].as_f64().unwrap() - 2.71).abs() < f64::EPSILON);
}
