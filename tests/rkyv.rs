#![cfg(feature = "rkyv")]

use microfloat::f8e4m3;

#[test]
fn rkyv_archives_raw_value() {
    let value = f8e4m3::ONE;
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
    let archived = unsafe { rkyv::access_unchecked::<f8e4m3>(&bytes) };
    let deserialized: f8e4m3 = rkyv::deserialize::<f8e4m3, rkyv::rancor::Error>(archived).unwrap();

    assert_eq!(archived.to_bits(), value.to_bits());
    assert_eq!(deserialized.to_bits(), value.to_bits());
}
