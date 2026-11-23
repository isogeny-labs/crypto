pub fn sqrt(n: u128) -> u128 {
    let (mut lo, mut hi) = (0, 1u128 << 64);
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if mid <= n / mid {
            lo = mid
        } else {
            hi = mid - 1
        }
    }
    lo
}
pub fn coefficients(p: u64, counts: &[u64]) -> Vec<i128> {
    let mut c = vec![1];
    let mut sums = vec![0];
    for k in 1..=counts.len() {
        sums.push(p.pow(k as u32) as i128 + 1 - counts[k - 1] as i128);
        assert!(sums[k] * sums[k] <= 64 * p.pow(k as u32) as i128);
        let s: i128 = (1..=k).map(|j| c[k - j] * sums[j]).sum();
        assert_eq!(s % k as i128, 0);
        c.push(-s / k as i128)
    }
    c
}
pub fn interval(p: u64, counts: [u64; 2]) -> (u64, u64) {
    let q = p as i128;
    let a = q.pow(4) + 28 * q.pow(3) + 70 * q * q + 28 * q + 1;
    let b = 8 * q.pow(3) + 56 * q * q + 56 * q + 8;
    let t = sqrt((b * b * q) as u128) as i128;
    let c = coefficients(p, &counts);
    let k = q.pow(4) + 1 + c[1] * (q.pow(3) + 1) + c[2] * (q * q + 1);
    let s = sqrt(p as u128);
    let ceil = s + u128::from(s * s < p as u128);
    let r = (q + 1) * 56 * q * ceil as i128 + 70 * q * q;
    let lo = (a - t).max(k - r).max(1);
    let hi = (a + t).min(k + r);
    assert!(lo <= hi);
    (lo.try_into().unwrap(), hi.try_into().unwrap())
}
pub fn unique(lo: u64, hi: u64, l: u64) -> Option<u64> {
    let first = ((lo as u128 + l as u128 - 1) / l as u128) * l as u128;
    (first <= hi as u128 && first + l as u128 > hi as u128).then_some(first as u64)
}
pub fn weil(p: u64, n: u64, counts: [u64; 3]) -> [i128; 9] {
    let c = coefficients(p, &counts);
    let q = p as i128;
    let middle =
        n as i128 - (q.pow(4) + 1 + c[1] * (q.pow(3) + 1) + c[2] * (q * q + 1) + c[3] * (q + 1));
    let s4 = -4 * middle
        - c[1] * (q.pow(3) + 1 - counts[2] as i128)
        - c[2] * (q * q + 1 - counts[1] as i128)
        - c[3] * (q + 1 - counts[0] as i128);
    assert!(s4 * s4 <= 64 * q.pow(4));
    [
        q.pow(4),
        q.pow(3) * c[1],
        q * q * c[2],
        q * c[3],
        middle,
        c[3],
        c[2],
        c[1],
        1,
    ]
}
