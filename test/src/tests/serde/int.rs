use crate::fixtures::int;
use crate::fixtures::int_inference;
use crate::fixtures::int_suffix;

#[test]
fn serialize() {
    assert_eq!(
        serde_json::to_value(int::HttpStatusCode).unwrap(),
        serde_json::json!(404),
    );
}

#[test]
fn deserialize_ok() {
    let value: int::HttpStatusCode =
        serde_json::from_value(serde_json::json!(404)).unwrap();

    assert_eq!(value, int::HttpStatusCode);
}

#[test]
fn deserialize_mismatch() {
    let result: Result<int::HttpStatusCode, _> =
        serde_json::from_value(serde_json::json!(200));

    assert!(result.is_err());
}

#[test]
fn round_trip() {
    let json: serde_json::Value =
        serde_json::to_value(int::HttpStatusCode).unwrap();
    let back: int::HttpStatusCode = serde_json::from_value(json).unwrap();

    assert_eq!(back, int::HttpStatusCode);
}

#[test]
fn suffix_round_trips() {
    assert_eq!(
        serde_json::to_value(int_suffix::U8).unwrap(),
        serde_json::json!(0),
    );
    let _: int_suffix::U8 =
        serde_json::from_value(serde_json::json!(0)).unwrap();

    assert_eq!(
        serde_json::to_value(int_suffix::U64).unwrap(),
        serde_json::json!(0),
    );
    let _: int_suffix::U64 =
        serde_json::from_value(serde_json::json!(0)).unwrap();

    assert_eq!(
        serde_json::to_value(int_suffix::I64).unwrap(),
        serde_json::json!(0),
    );
    let _: int_suffix::I64 =
        serde_json::from_value(serde_json::json!(0)).unwrap();
}

#[test]
fn inference_boundary_round_trips() {
    let _: int_inference::I32Max =
        serde_json::from_value(serde_json::json!(2_147_483_647i32)).unwrap();
    let _: int_inference::AboveI32Max =
        serde_json::from_value(serde_json::json!(2_147_483_648i64)).unwrap();
    let _: int_inference::I32Min =
        serde_json::from_value(serde_json::json!(-2_147_483_648i32)).unwrap();
    let _: int_inference::BelowI32Min =
        serde_json::from_value(serde_json::json!(-2_147_483_649i64)).unwrap();
}
