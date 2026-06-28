use super::api_components;

#[test]
fn literal_registered_when_generic_arg() {
    let components: serde_json::Value = api_components();
    let schema: &serde_json::Value = &components["NotFoundCode"];

    assert_eq!(schema["type"], "string");
    assert_eq!(schema["enum"][0], "not_found");
}
