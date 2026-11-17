// Exact 2-sum and 3-sum pair coverage vs factor-base size, 20-bit profile curve.
use crypto_lib::cryptanalysis::ecbench::workload::{CurveSpec, Instance};
use std::collections::HashSet;
struct C { p: u64, a: u64 }
fn inv(a: u64, m: u64) -> u64 {
    let (mut t, mut nt, mut r, mut nr) = (0i128, 1i128, m as i128, (a % m) as i128);
    while nr != 0 { let q = r / nr; let x = t - q * nt; t = nt; nt = x; let y = r - q * nr; r = nr; nr = y; }
    if t < 0 { (t + m as i128) as u64 } else { t as u64 }
}
impl C {
    fn add(&self, p: (u64,u64), q: (u64,u64)) -> Option<(u64,u64)> {
        let m = self.p;
        if p == q { if p.1 == 0 { return None; } let l = (3*p.0%m*p.0%m + self.a)%m * inv((2*p.1)%m, m)%m;
            let x = (l*l%m + 2*(m - p.0%m))%m; Some((x, (l*((p.0+m-x)%m)%m + m - p.1)%m)) }
        else { let dx = (q.0 + m - p.0)%m; if dx == 0 { return None; }
            let l = (q.1 + m - p.1)%m * inv(dx, m)%m;
            let x = (l*l%m + 2*(m - p.0%m) + m - q.0)%m; Some((x, (l*((p.0+m-x)%m)%m + m - p.1)%m)) }
    }
}
fn main() {
    let spec = CurveSpec::PrimeSearch { bits: 20, seed: 59297 };
    let inst = spec.build().unwrap();
    let Instance::Prime(pi) = &inst else { panic!() };
    let c = C { p: pi.curve.p, a: pi.curve.a };
    let mut pool = Vec::new(); let mut x = 1u64;
    while pool.len() < 200 && x < c.p {
        let rhs = (x*x%c.p*x + c.a*x%c.p + pi.curve.b)%c.p;
        if pi.curve.legendre(rhs) >= 0 { if let Some(y) = pi.curve.sqrt(rhs) { pool.push((x, y.min(c.p-y))); } }
        x += 1;
    }
    let r = pi.r as f64;
    println!("N\tpts\tm2_keys\tm2_cov%\tm3_keys\tm3_cov%");
    for n in [24usize, 32, 45, 64, 90, 128] {
        if n > pool.len() { break; }
        let mut pts = Vec::new();
        for &(x,y) in &pool[..n] { pts.push((x,y)); if y != 0 && c.p - y != y { pts.push((x, c.p-y)); } }
        let mut two = HashSet::new();
        for i in 0..pts.len() { for j in i..pts.len() { if let Some((x,_)) = c.add(pts[i], pts[j]) { two.insert(x.min(c.p-x)); } } }
        let mut three = HashSet::new();
        for i in 0..pts.len() { for j in i..pts.len() { if let Some(s) = c.add(pts[i], pts[j]) {
            for k in j..pts.len() { if let Some((x,_)) = c.add(s, pts[k]) { three.insert(x.min(c.p-x)); } } } } }
        println!("{}\t{}\t{}\t{:.3}\t{}\t{:.3}", n, pts.len(), two.len(), 100.0*two.len() as f64/r, three.len(), 100.0*three.len() as f64/r);
    }
}
