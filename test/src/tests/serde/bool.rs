use crate::fixtures::bool;

#[test]
fn serialize() {
    assert_eq!(
        serde_json::to_value(bool::FeatureEnabled).unwrap(),
        serde_json::json!(true),
    );
}

#[test]
fn deserialize_ok() {
    let value: bool::FeatureEnabled =
        serde_json::from_value(serde_json::json!(true)).unwrap();

    assert_eq!(value, bool::FeatureEnabled);
}

#[test]
fn deserialize_mismatch() {
    let result: Result<bool::FeatureEnabled, _> =
        serde_json::from_value(serde_json::json!(false));

    assert!(result.is_err());
}

#[test]
fn round_trip() {
    let json: serde_json::Value =
        serde_json::to_value(bool::FeatureEnabled).unwrap();
    let back: bool::FeatureEnabled = serde_json::from_value(json).unwrap();

    assert_eq!(back, bool::FeatureEnabled);
}
