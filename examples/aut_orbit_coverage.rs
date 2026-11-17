// Curve-structure lever: on j=0 (Aut = 6) the same number of columns holds
// 3x the points, so the accepted set grows at fixed linear-algebra size.
use std::collections::HashSet;
fn pw(mut b: u64, mut e: u64, m: u64) -> u64 { let mut r = 1u64; b %= m; while e > 0 { if e & 1 == 1 { r = (r as u128 * b as u128 % m as u128) as u64; } b = (b as u128 * b as u128 % m as u128) as u64; e >>= 1; } r }
fn main() {
    let p: u64 = 1000003;           // prime, p ≡ 1 (mod 3) and 3 (mod 4)
    let mut w = pw(2, (p - 1) / 3, p);
    if w == 1 { w = pw(3, (p - 1) / 3, p); }
    let w2 = (w as u128 * w as u128 % p as u128) as u64;
    let mut cols = Vec::new();
    let mut x = 1u64;
    while cols.len() < 60 && x < p {
        let rhs = (x as u128 * x as u128 % p as u128 * x as u128 % p as u128) as u64; // a = 0, b = 0? use b=1
        let rhs = (rhs + 1) % p;
        let y = pw(rhs, (p + 1) / 4, p);
        if (y as u128 * y as u128 % p as u128) as u64 == rhs && y != 0 { cols.push((x, y)); }
        x += 1;
    }
    let n = 30usize;
    let mut pm: Vec<(u64, u64)> = Vec::new();
    for &(x, y) in cols.iter().take(n) { pm.push((x, y)); pm.push((x, p - y)); }
    let mut orb: Vec<(u64, u64)> = Vec::new();
    for &(x, y) in cols.iter().take(n) {
        for sx in [x, (x as u128 * w as u128 % p as u128) as u64, (x as u128 * w2 as u128 % p as u128) as u64] {
            orb.push((sx, y)); orb.push((sx, p - y));
        }
    }
    let add = |a: (u64, u64), b: (u64, u64)| -> Option<(u64, u64)> {
        if a.0 == b.0 && a.1 == b.1 {
            if a.1 == 0 { return None; }
            let lam = (3 * a.0 % p * a.0 % p) * pw(2 * a.1 % p, p - 2, p) % p;
            let x = (lam * lam % p + 2 * (p - a.0 % p)) % p;
            Some((x, (lam * ((a.0 + p - x) % p) % p + p - a.1) % p))
        } else {
            let dx = (b.0 + p - a.0) % p; if dx == 0 { return None; }
            let lam = (b.1 + p - a.1) % p * pw(dx, p - 2, p) % p;
            let x = (lam * lam % p + 2 * (p - a.0 % p) + p - b.0) % p;
            Some((x, (lam * ((a.0 + p - x) % p) % p + p - a.1) % p))
        }
    };
    let cov = |pts: &[(u64, u64)]| -> usize {
        let mut s = HashSet::new();
        for i in 0..pts.len() { for j in i..pts.len() { if let Some((x, _)) = add(pts[i], pts[j]) { s.insert(x.min(p - x)); } } }
        s.len()
    };
    let (cp, co) = (cov(&pm), cov(&orb));
    println!("columns={n}  pm_points={}  orbit_points={}", pm.len(), orb.len());
    println!("pair keys: +/- base {cp}, orbit base {co}, ratio {:.2}", co as f64 / cp as f64);
    println!("per column: {:.1} vs {:.1}", cp as f64 / n as f64, co as f64 / n as f64);
}
