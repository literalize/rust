use crate::fixtures::str;

#[test]
fn serialize() {
    assert_eq!(
        serde_json::to_value(str::NotFound).unwrap(),
        serde_json::json!("not_found"),
    );
}

#[test]
fn deserialize_ok() {
    let value: str::NotFound =
        serde_json::from_value(serde_json::json!("not_found")).unwrap();

    assert_eq!(value, str::NotFound);
}

#[test]
fn deserialize_mismatch() {
    let result: Result<str::NotFound, _> =
        serde_json::from_value(serde_json::json!("internal_server_error"));

    assert!(result.is_err());
}

#[test]
fn round_trip() {
    let json: serde_json::Value = serde_json::to_value(str::NotFound).unwrap();
    let back: str::NotFound = serde_json::from_value(json).unwrap();

    assert_eq!(back, str::NotFound);
}
