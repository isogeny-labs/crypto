// Independent polynomial arithmetic: no calls into the producer's curve code.
use serde_json::{json, Value};
use std::collections::BTreeMap;
type P = Vec<u64>;
#[derive(Clone, Debug, PartialEq, Eq)]
struct D {
    u: P,
    v: P,
}
struct K {
    p: u64,
    mul: u64,
    inv: u64,
    add: u64,
}
impl K {
    fn m(&mut self, a: u64, b: u64) -> u64 {
        self.mul += 1;
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn plus(&mut self, a: u64, b: u64) -> u64 {
        self.add += 1;
        (a + b) % self.p
    }
    fn minus(&mut self, a: u64, b: u64) -> u64 {
        self.add += 1;
        (a + self.p - b) % self.p
    }
    fn inv(&mut self, a: u64) -> u64 {
        assert_ne!(a, 0);
        self.inv += 1;
        let (mut b, mut e, mut z) = (a, self.p - 2, 1);
        while e > 0 {
            if e & 1 == 1 {
                z = self.m(z, b);
            }
            b = self.m(b, b);
            e >>= 1;
        }
        z
    }
    fn addp(&mut self, a: &P, b: &P, minus: bool) -> P {
        let mut c = vec![0; a.len().max(b.len())];
        for (i, x) in c.iter_mut().enumerate() {
            let aa = a.get(i).copied().unwrap_or(0);
            let bb = b.get(i).copied().unwrap_or(0);
            *x = if minus {
                self.minus(aa, bb)
            } else {
                self.plus(aa, bb)
            };
        }
        trim(c)
    }
    fn mulp(&mut self, a: &P, b: &P) -> P {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut c = vec![0; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                let t = self.m(x, y);
                c[i + j] = self.plus(c[i + j], t);
            }
        }
        trim(c)
    }
    fn scale(&mut self, a: &P, z: u64) -> P {
        trim(a.iter().map(|&x| self.m(x, z)).collect())
    }
    fn div(&mut self, a: &P, b: &P) -> (P, P) {
        assert!(!b.is_empty());
        let mut r = a.clone();
        let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
        let inv = self.inv(*b.last().unwrap());
        while !r.is_empty() && r.len() >= b.len() {
            let d = r.len() - b.len();
            let v = self.m(*r.last().unwrap(), inv);
            q[d] = v;
            for (i, &x) in b.iter().enumerate() {
                let t = self.m(v, x);
                r[d + i] = self.minus(r[d + i], t);
            }
            r = trim(r);
        }
        (trim(q), r)
    }
    fn exact(&mut self, a: &P, b: &P) -> P {
        let (q, r) = self.div(a, b);
        assert!(r.is_empty(), "non-exact Cantor division");
        q
    }
    fn xgcd(&mut self, a: &P, b: &P) -> (P, P, P) {
        let (mut r, mut s, mut t) = (a.clone(), vec![1], vec![]);
        let (mut rr, mut ss, mut tt) = (b.clone(), vec![], vec![1]);
        while !rr.is_empty() {
            let (q, rn) = self.div(&r, &rr);
            let qs = self.mulp(&q, &ss);
            let sn = self.addp(&s, &qs, true);
            let qt = self.mulp(&q, &tt);
            let tn = self.addp(&t, &qt, true);
            (r, rr) = (rr, rn);
            (s, ss) = (ss, sn);
            (t, tt) = (tt, tn);
        }
        let z = self.inv(*r.last().unwrap());
        (self.scale(&r, z), self.scale(&s, z), self.scale(&t, z))
    }
    fn id() -> D {
        D {
            u: vec![1],
            v: vec![],
        }
    }
    fn valid(&mut self, d: &D, f: &P) -> bool {
        if d.u.is_empty() || d.u.len() > 5 || d.u.last() != Some(&1) || d.v.len() >= d.u.len() {
            return false;
        }
        let vv = self.mulp(&d.v, &d.v);
        let z = self.addp(f, &vv, true);
        self.div(&z, &d.u).1.is_empty()
    }
    fn compose(&mut self, a: &D, b: &D, f: &P) -> D {
        let (d1, e1, e2) = self.xgcd(&a.u, &b.u);
        let vs = self.addp(&a.v, &b.v, false);
        let (d, c1, c2) = self.xgcd(&d1, &vs);
        let s1 = self.mulp(&c1, &e1);
        let s2 = self.mulp(&c1, &e2);
        let au = self.mulp(&a.u, &b.u);
        let dd = self.mulp(&d, &d);
        let mut u = self.exact(&au, &dd);
        let a1 = self.mulp(&s1, &a.u);
        let a1 = self.mulp(&a1, &b.v);
        let a2 = self.mulp(&s2, &b.u);
        let a2 = self.mulp(&a2, &a.v);
        let vv = self.mulp(&a.v, &b.v);
        let ff = self.addp(&vv, f, false);
        let a3 = self.mulp(&c2, &ff);
        let z = self.addp(&a1, &a2, false);
        let z = self.addp(&z, &a3, false);
        let z = self.exact(&z, &d);
        let mut v = self.div(&z, &u).1;
        while u.len() > 5 {
            let vv = self.mulp(&v, &v);
            let z = self.addp(f, &vv, true);
            let un = self.exact(&z, &u);
            let inv = self.inv(*un.last().unwrap());
            u = self.scale(&un, inv);
            let vn = self.scale(&v, self.p - 1);
            v = self.div(&vn, &u).1;
        }
        let d = D { u, v };
        assert!(self.valid(&d, f));
        d
    }
    fn scalar(&mut self, a: &D, mut n: u64, f: &P) -> D {
        let (mut r, mut b) = (Self::id(), a.clone());
        while n > 0 {
            if n & 1 == 1 {
                r = self.compose(&r, &b, f);
            }
            n >>= 1;
            if n > 0 {
                b = self.compose(&b, &b, f);
            }
        }
        r
    }
}
fn trim(mut a: P) -> P {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn p(v: &Value) -> P {
    trim(
        v.as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect(),
    )
}
fn d(v: &Value) -> D {
    D {
        u: p(&v[0]),
        v: p(&v[1]),
    }
}
fn n(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap()
}
pub fn relations(path: &str) {
    let rows = super::rows(path);
    let (mut count, mut mul, mut inv, mut add) = (0, 0, 0, 0);
    for row in rows {
        let mut k = K {
            p: n(&row, "p"),
            mul: 0,
            inv: 0,
            add: 0,
        };
        let f = p(&row["f"]);
        let g = d(&row["G"]);
        let t = d(&row["T"]);
        let cur = d(&row["cur"]);
        assert!(k.valid(&cur, &f));
        let ag = k.scalar(&g, n(&row, "ca"), &f);
        let bt = k.scalar(&t, n(&row, "cb"), &f);
        assert_eq!(k.compose(&ag, &bt, &f), cur, "walk coefficient invariant");
        let mut sum = K::id();
        for xy in row["places"].as_array().unwrap() {
            let x = xy[0].as_u64().unwrap();
            let y = xy[1].as_u64().unwrap();
            let pt = D {
                u: vec![(k.p - x) % k.p, 1],
                v: trim(vec![y]),
            };
            assert!(k.valid(&pt, &f));
            sum = k.compose(&sum, &pt, &f);
        }
        assert_eq!(sum, cur, "complete split relation");
        count += 1;
        mul += k.mul;
        inv += k.inv;
        add += k.add;
    }
    println!(
        "{}",
        json!({"source":path,"relations_checked":count,"failed":0,"checks":"each place lies on curve; split sum equals current divisor; current divisor equals ca*G+cb*T in independent arithmetic","verifier_field_mul":mul,"verifier_field_inv":inv,"verifier_field_add":add})
    );
}
fn mm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn power(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut z = 1;
    while e > 0 {
        if e & 1 == 1 {
            z = mm(z, a, p);
        }
        a = mm(a, a, p);
        e >>= 1;
    }
    z
}
fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'next: for a in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        if a % n == 0 {
            continue;
        }
        let mut x = power(a % n, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mm(x, x, n);
            if x == n - 1 {
                continue 'next;
            }
        }
        return false;
    }
    true
}
fn isqrt(n: u64) -> u64 {
    let mut s = (n as f64).sqrt() as u64;
    while s * s > n {
        s -= 1;
    }
    while (s + 1) * (s + 1) <= n {
        s += 1;
    }
    s
}
pub fn transfer(path: &str) {
    let mut out = Vec::new();
    for row in super::rows(path).iter().filter(|x| x["kind"] == "instance") {
        let pp = n(row, "p");
        let r = n(row, "r");
        let h = n(row, "h");
        assert!(prime(r) && h <= 2 && h * r == n(row, "order"));
        let b = (isqrt(pp) + 2).pow(2);
        assert!(r > b);
        let s = isqrt(pp * 64 * 64) as u128;
        let den = 64u128.pow(8);
        let lo = (s - 64).pow(8).div_ceil(den);
        let hi = (s + 65).pow(8) / den;
        let first = lo.div_ceil(r as u128);
        let last = hi / (r as u128);
        assert_eq!(first, last);
        assert_eq!(first, h as u128);
        out.push(json!({"p":pp,"curve_seed":n(row,"curve_seed"),"r":r,"r_prime":true,"h":h,"elliptic_order_upper_bound":b,"jacobian_order_weil_interval":[lo as u64,hi as u64],"unique_multiple_of_r":first as u64,"conclusion":"every homomorphism from an odd prime-order E(F_p) subgroup to this J(F_p) is zero; r exceeds the elliptic Hasse bound and h has no odd prime factor"}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"status":"derived finite-group obstruction, conditional on independently checked r-torsion witness for each curve","instances":out})).unwrap());
}
pub fn certify(path: &str) {
    let rows = super::rows(path);
    let mut instances = BTreeMap::new();
    let (mut checks, mut failures) = (0, 0);
    let mut reports = Vec::new();
    for row in &rows {
        if row["kind"] == "instance" {
            instances.insert(
                (n(row, "p"), n(row, "curve_seed"), n(row, "target_seed")),
                row.clone(),
            );
        }
    }
    for row in &rows {
        let is_replay = row["kind"] == "replay";
        if row["recovered"].is_null() && row["kind"] != "instance" {
            continue;
        }
        let key = (n(row, "p"), n(row, "curve_seed"), n(row, "target_seed"));
        let inst = if is_replay {
            row
        } else {
            instances.get(&key).unwrap()
        };
        let f = p(&inst["f"]);
        let g = d(&inst["G"]);
        let t = d(&inst["T"]);
        let scalar = if row["kind"] == "instance" {
            n(row, "t")
        } else {
            n(row, "recovered")
        };
        let mut k = K {
            p: key.0,
            mul: 0,
            inv: 0,
            add: 0,
        };
        assert_eq!(f.len(), 10, "genus four, odd degree model");
        assert_eq!(f.last(), Some(&1));
        assert_ne!(g, K::id(), "nonzero order witness required");
        assert!(f
            .iter()
            .chain(&g.u)
            .chain(&g.v)
            .chain(&t.u)
            .chain(&t.v)
            .all(|&x| x < k.p));
        assert!(k.valid(&g, &f) && k.valid(&t, &f));
        let derivative = trim(
            f.iter()
                .enumerate()
                .skip(1)
                .map(|(i, &v)| v * i as u64 % k.p)
                .collect(),
        );
        assert_eq!(k.xgcd(&f, &derivative).0, vec![1]);
        let ok = k.scalar(&g, scalar, &f) == t;
        assert!(ok, "independent scalar check");
        if is_replay {
            assert_eq!(k.scalar(&g, n(row, "r"), &f), K::id());
        }
        checks += 1;
        if !ok {
            failures += 1;
        }
        reports.push(json!({"p":key.0,"curve_seed":key.1,"target_seed":key.2,"seed":row.get("seed"),"arm":row.get("arm"),"scalar":scalar,"valid":ok,"independent_mul":k.mul,"independent_inv":k.inv,"independent_add":k.add}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"source":path,"checks":checks,"failures":failures,"independent_arithmetic":"dynamic-polynomial extended-gcd Cantor composition, right-to-left scalar ladder; no producer arithmetic called","rows":reports})).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cantor_identity_inverse_and_corruption() {
        let mut k = K {
            p: 17,
            mul: 0,
            inv: 0,
            add: 0,
        };
        let f = vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 1];
        let a = D {
            u: vec![0, 1],
            v: vec![1],
        };
        assert!(k.valid(&a, &f));
        assert_eq!(k.compose(&a, &K::id(), &f), a);
        let neg = D {
            u: a.u.clone(),
            v: vec![16],
        };
        assert_eq!(k.compose(&a, &neg, &f), K::id());
        assert!(!k.valid(&D { u: a.u, v: vec![2] }, &f));
    }
}
