use crate::fixtures::str;

#[test]
fn value() {
    assert_eq!(str::NotFound::VALUE, "not_found");
}

#[test]
fn deref() {
    let instance: str::NotFound = str::NotFound;

    assert_eq!(&*instance, "not_found");
}

#[test]
fn default() {
    assert_eq!(&*str::NotFound, "not_found");
}

#[test]
fn debug() {
    assert_eq!(format!("{:?}", str::NotFound), format!("{:?}", "not_found"),);
}

#[test]
fn partial_eq() {
    assert_eq!(str::NotFound, str::NotFound);
}
