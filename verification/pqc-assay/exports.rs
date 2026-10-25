
// Proof-only C ABI entry points. Bodies under test are above, from production.
#[no_mangle]
pub extern "C" fn assay_montgomery_reduce(a: i64) -> i32 {
    montgomery_reduce(a)
}

#[no_mangle]
pub extern "C" fn assay_reduce32(a: i32) -> i32 {
    reduce32(a)
}

#[no_mangle]
pub extern "C" fn assay_caddq(a: i32) -> i32 {
    caddq(a)
}

#[no_mangle]
pub extern "C" fn assay_canon(a: i32) -> i32 {
    canon(a)
}
