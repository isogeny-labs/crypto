fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
fn is_prime_u64(p: u64) -> bool {
    p >= 2 && !(2..).take_while(|d| d * d <= p).any(|d| p % d == 0)
}
pub fn scalar(j: &QuarticJacobian, mut a: QClass, mut n: u64) -> QClass {
    let mut r = j.zero();
    while n > 0 {
        if n & 1 != 0 {
            r = j.add_raw(&r, &a)
        }
        a = j.add_raw(&a, &a);
        n >>= 1;
    }
    r
}
pub fn difference(j: &QuarticJacobian, p: [u64; 3], q: [u64; 3]) -> QClass {
    if p == q {
        return j.zero();
    }
    let a: Vec<_> = j
        .points
        .iter()
        .filter(|x| **x != p && **x != q)
        .copied()
        .take(2)
        .collect();
    let x = j.from_points([p, a[0], a[1]]).unwrap();
    let y = j.from_points([q, a[0], a[1]]).unwrap();
    j.add_raw(&x, &j.neg_raw(&y))
}
