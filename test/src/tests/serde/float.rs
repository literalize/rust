use crate::fixtures::float;

#[test]
fn serialize() {
    assert_eq!(
        serde_json::to_value(float::EulersNumber).unwrap(),
        serde_json::json!(2.71),
    );
}

#[test]
fn deserialize_ok() {
    let value: float::EulersNumber =
        serde_json::from_value(serde_json::json!(2.71)).unwrap();

    assert_eq!(value, float::EulersNumber);
}

#[test]
fn deserialize_mismatch() {
    let result: Result<float::EulersNumber, _> =
        serde_json::from_value(serde_json::json!(1.41));

    assert!(result.is_err());
}

#[test]
fn round_trip() {
    let json: serde_json::Value =
        serde_json::to_value(float::EulersNumber).unwrap();
    let back: float::EulersNumber = serde_json::from_value(json).unwrap();

    assert_eq!(back, float::EulersNumber);
}
