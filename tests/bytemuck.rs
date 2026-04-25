#![cfg(feature = "bytemuck")]

use microfloat::f8e4m3;

#[test]
fn bytemuck_views_raw_bits() {
    let value = f8e4m3::ONE;
    assert_eq!(bytemuck::bytes_of(&value), &[value.to_bits()]);
}
