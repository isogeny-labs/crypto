//! Independent F343 arithmetic in the original 3-coefficient encoding.
pub fn add(a: u16, b: u16) -> u16 {
    (a % 7 + b % 7) % 7 + 7 * ((a / 7 % 7 + b / 7 % 7) % 7) + 49 * ((a / 49 + b / 49) % 7)
}
pub fn neg(a: u16) -> u16 {
    (7 - a % 7) % 7 + 7 * ((7 - a / 7 % 7) % 7) + 49 * ((7 - a / 49) % 7)
}
pub fn sub(a: u16, b: u16) -> u16 {
    add(a, neg(b))
}
pub fn mul(a: u16, b: u16) -> u16 {
    let a = [a % 7, a / 7 % 7, a / 49];
    let b = [b % 7, b / 7 % 7, b / 49];
    let mut v = [0_i32; 5];
    for i in 0..3 {
        for j in 0..3 {
            v[i + j] += i32::from(a[i] * b[j]);
        }
    }
    for i in (3..5).rev() {
        let c = v[i].rem_euclid(7);
        v[i - 1] -= c;
        v[i - 3] -= c;
    }
    (v[0].rem_euclid(7) + 7 * v[1].rem_euclid(7) + 49 * v[2].rem_euclid(7)) as u16
}
pub fn pow(mut a: u16, mut n: u16) -> u16 {
    let mut v = 1;
    while n > 0 {
        if n & 1 != 0 {
            v = mul(v, a);
        }
        n >>= 1;
        a = mul(a, a);
    }
    v
}
pub type Point = Option<[u16; 2]>;
pub fn ec_add(p: Point, q: Point) -> Point {
    let (Some([x, y]), Some([u, v])) = (p, q) else {
        return p.or(q);
    };
    if x == u && add(y, v) == 0 {
        return None;
    }
    let (num, den) = if x == u {
        (add(mul(3, mul(x, x)), 248), mul(2, y))
    } else {
        (sub(v, y), sub(u, x))
    };
    assert_ne!(den, 0);
    let m = mul(num, pow(den, 341));
    let r = sub(sub(mul(m, m), x), u);
    Some([r, sub(mul(m, sub(x, r)), y)])
}
