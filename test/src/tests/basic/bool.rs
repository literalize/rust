use std::any::TypeId;
use std::ops::Deref;

use crate::fixtures::bool;

#[test]
fn value() {
    let instance: bool::FeatureEnabled = bool::FeatureEnabled;

    assert_eq!(*instance, bool::FeatureEnabled::VALUE);
}

#[test]
fn deref() {
    let instance: bool::FeatureEnabled = bool::FeatureEnabled;

    assert!(*instance);
    assert_eq!(
        TypeId::of::<<bool::FeatureEnabled as Deref>::Target>(),
        TypeId::of::<bool>(),
    );
}

#[test]
fn default() {
    assert!(*bool::FeatureEnabled);
}

#[test]
fn debug() {
    assert_eq!(format!("{:?}", bool::FeatureEnabled), format!("{:?}", true),);
}

#[test]
fn partial_eq() {
    assert_eq!(bool::FeatureEnabled, bool::FeatureEnabled);
}
