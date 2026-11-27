pub type Point = [u8; 3];
pub type EPoint = Option<(u8, u8)>;
pub fn m(n: i32) -> u8 {
    u8::try_from(n.rem_euclid(7)).unwrap()
}
pub fn add(a: u8, b: u8) -> u8 {
    m(i32::from(a) + i32::from(b))
}
pub fn sub(a: u8, b: u8) -> u8 {
    m(i32::from(a) - i32::from(b))
}
pub fn mul(a: u8, b: u8) -> u8 {
    m(i32::from(a) * i32::from(b))
}
pub fn pow(a: u8, n: usize) -> u8 {
    (0..n).fold(1, |x, _| mul(x, a))
}
pub fn inv(a: u8) -> u8 {
    (1..7u8).find(|b| mul(a, *b) == 1).unwrap()
}
pub fn valid(p: EPoint) -> bool {
    p.is_none_or(|(x, y)| pow(y, 2) == add(add(pow(x, 3), mul(2, x)), 6))
}
pub fn eneg(p: EPoint) -> EPoint {
    p.map(|(x, y)| (x, sub(0, y)))
}
pub fn eadd(a: EPoint, b: EPoint) -> EPoint {
    assert!(valid(a) && valid(b));
    let Some((x, y)) = a else {
        return b;
    };
    let Some((xx, yy)) = b else {
        return a;
    };
    if x == xx && add(y, yy) == 0 {
        return None;
    }
    let slope = if x == xx {
        mul(add(mul(3, pow(x, 2)), 2), inv(mul(2, y)))
    } else {
        mul(sub(yy, y), inv(sub(xx, x)))
    };
    let nx = sub(sub(pow(slope, 2), x), xx);
    let ny = sub(mul(slope, sub(x, nx)), y);
    let p = Some((nx, ny));
    assert!(valid(p));
    p
}
pub fn emul(mut n: usize, mut p: EPoint) -> EPoint {
    let mut r = None;
    while n != 0 {
        if n & 1 != 0 {
            r = eadd(r, p);
        }
        p = eadd(p, p);
        n >>= 1;
    }
    r
}
