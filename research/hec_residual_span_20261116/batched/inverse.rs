use hec_native_ic::fp::Fp;
// Exact cost for nonempty input: one inverse and 3(n−1) field multiplies.
pub fn invert(v: &[u64], f: &Fp) -> Vec<u64> {
    if v.is_empty() {
        return vec![];
    }
    assert!(v.iter().all(|&x| x != 0 && x < f.p));
    let n = v.len();
    let mut prefix = Vec::with_capacity(n);
    prefix.push(v[0]);
    for i in 1..n {
        prefix.push(f.mul(prefix[i - 1], v[i]))
    }
    let mut inv = f.inv(prefix[n - 1]);
    let mut out = vec![0; n];
    for i in (1..n).rev() {
        out[i] = f.mul(inv, prefix[i - 1]);
        inv = f.mul(inv, v[i])
    }
    out[0] = inv;
    out
}
