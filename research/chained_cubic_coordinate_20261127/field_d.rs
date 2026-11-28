use crate::{accounting, field_reference};

pub fn add(a: u8, b: u8) -> u8 {
    accounting::fp_add();
    field_reference::add(a, b)
}
pub fn mul(a: u8, b: u8) -> u8 {
    accounting::fp_mul();
    field_reference::mul(a, b)
}
pub fn sub(a: u8, b: u8) -> u8 {
    add(a, mul(6, b))
}
pub fn inv(a: u8) -> u8 {
    assert!(a > 0 && a < 7);
    accounting::fp_inv();
    (0..5).fold(1, |b, _| mul(b, a))
}
