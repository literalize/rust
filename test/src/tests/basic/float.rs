use std::any::TypeId;
use std::ops::Deref;

use crate::fixtures::float;
use crate::fixtures::float_suffix;

#[test]
fn value() {
    assert_eq!(float::EulersNumber::VALUE, 2.71);
}

#[test]
fn deref() {
    let instance: float::EulersNumber = float::EulersNumber;

    assert!((*instance - 2.71).abs() < f64::EPSILON);
    assert_eq!(
        TypeId::of::<<float::EulersNumber as Deref>::Target>(),
        TypeId::of::<f64>(),
    );
}

#[test]
fn default() {
    assert!((*float::EulersNumber - 2.71).abs() < f64::EPSILON);
}

#[test]
fn debug() {
    assert_eq!(format!("{:?}", float::EulersNumber), format!("{:?}", 2.71f64),);
}

#[test]
fn partial_eq() {
    assert_eq!(float::EulersNumber, float::EulersNumber);
}

#[test]
fn suffix_target_types() {
    assert_eq!(
        TypeId::of::<<float_suffix::F32 as Deref>::Target>(),
        TypeId::of::<f32>(),
    );
    assert_eq!(
        TypeId::of::<<float_suffix::F64 as Deref>::Target>(),
        TypeId::of::<f64>(),
    );
}
