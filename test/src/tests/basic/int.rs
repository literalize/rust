use std::any::TypeId;
use std::ops::Deref;

use crate::fixtures::int;
use crate::fixtures::int_inference;
use crate::fixtures::int_suffix;

#[test]
fn value() {
    assert_eq!(int::HttpStatusCode::VALUE, 404);
}

#[test]
fn deref() {
    let instance: int::HttpStatusCode = int::HttpStatusCode;

    assert_eq!(*instance, 404);
    assert_eq!(
        TypeId::of::<<int::HttpStatusCode as Deref>::Target>(),
        TypeId::of::<i32>(),
    );
}

#[test]
fn default() {
    assert_eq!(*int::HttpStatusCode, 404);
}

#[test]
fn debug() {
    assert_eq!(format!("{:?}", int::HttpStatusCode), format!("{:?}", 404),);
}

#[test]
fn partial_eq() {
    assert_eq!(int::HttpStatusCode, int::HttpStatusCode);
}

#[test]
fn suffix_target_types() {
    assert_eq!(
        TypeId::of::<<int_suffix::U8 as Deref>::Target>(),
        TypeId::of::<u8>(),
    );
    assert_eq!(
        TypeId::of::<<int_suffix::U16 as Deref>::Target>(),
        TypeId::of::<u16>(),
    );
    assert_eq!(
        TypeId::of::<<int_suffix::U32 as Deref>::Target>(),
        TypeId::of::<u32>(),
    );
    assert_eq!(
        TypeId::of::<<int_suffix::U64 as Deref>::Target>(),
        TypeId::of::<u64>(),
    );
    assert_eq!(
        TypeId::of::<<int_suffix::I8 as Deref>::Target>(),
        TypeId::of::<i8>(),
    );
    assert_eq!(
        TypeId::of::<<int_suffix::I16 as Deref>::Target>(),
        TypeId::of::<i16>(),
    );
    assert_eq!(
        TypeId::of::<<int_suffix::I32 as Deref>::Target>(),
        TypeId::of::<i32>(),
    );
    assert_eq!(
        TypeId::of::<<int_suffix::I64 as Deref>::Target>(),
        TypeId::of::<i64>(),
    );
}

#[test]
fn inference_boundaries() {
    assert_eq!(
        TypeId::of::<<int_inference::I32Max as Deref>::Target>(),
        TypeId::of::<i32>(),
    );
    assert_eq!(
        TypeId::of::<<int_inference::AboveI32Max as Deref>::Target>(),
        TypeId::of::<i64>(),
    );
    assert_eq!(
        TypeId::of::<<int_inference::I32Min as Deref>::Target>(),
        TypeId::of::<i32>(),
    );
    assert_eq!(
        TypeId::of::<<int_inference::BelowI32Min as Deref>::Target>(),
        TypeId::of::<i64>(),
    );
}

#[test]
fn negative_value() {
    assert_eq!(int_inference::BelowI32Min::VALUE, -2_147_483_649);
}
