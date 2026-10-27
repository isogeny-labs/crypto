//! Bounded exact twist-torsion -> isogeny -> relation transport experiment.
//! No timing, novelty, cryptographic-size or solver-advantage claim.
#![allow(dead_code)]
#[path = "../registry.rs"]
mod registry;
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
#[path = "../../../../../src/cryptanalysis/twist_ic_field.rs"]
mod toy_field;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeSet, error::Error, fs, path::Path};
use toy_field::{inv, Quad, P, Q};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Poly = Vec<u64>;
#[derive(Clone, Copy)]
struct Ring(u64);
impl Ring {
    fn trim(self, mut a: Poly) -> Poly {
        while a.last() == Some(&0) {
            a.pop();
        }
        a
    }
    fn add(self, a: &[u64], b: &[u64]) -> Poly {
        self.trim(
            (0..a.len().max(b.len()))
                .map(|i| (a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0)) % self.0)
                .collect(),
        )
    }
    fn scale(self, a: &[u64], s: u64) -> Poly {
        self.trim(a.iter().map(|x| x * s % self.0).collect())
    }
    fn sub(self, a: &[u64], b: &[u64]) -> Poly {
        self.add(a, &self.scale(b, self.0 - 1))
    }
    fn mul(self, a: &[u64], b: &[u64]) -> Poly {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut out = vec![0; a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                out[i + j] = (out[i + j] + x * y) % self.0;
            }
        }
        self.trim(out)
    }
    fn pow(self, a: &[u64], n: usize) -> Poly {
        (0..n).fold(vec![1], |v, _| self.mul(&v, a))
    }
    fn dr(self, a: &[u64], b: &[u64]) -> (Poly, Poly) {
        assert!(!b.is_empty());
        let mut rem = self.trim(a.to_vec());
        let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
        let bi = inv(*b.last().unwrap(), self.0);
        while rem.len() >= b.len() {
            let d = rem.len() - b.len();
            let c = rem[rem.len() - 1] * bi % self.0;
            q[d] = c;
            for (i, x) in b.iter().enumerate() {
                rem[i + d] = (rem[i + d] + self.0 - c * x % self.0) % self.0;
            }
            rem = self.trim(rem);
        }
        (self.trim(q), rem)
    }
    fn exact(self, a: &[u64], b: &[u64]) -> Poly {
        let (q, r) = self.dr(a, b);
        assert!(r.is_empty());
        q
    }
    fn gcd(self, a: &[u64], b: &[u64]) -> Poly {
        let (mut a, mut b) = (a.to_vec(), b.to_vec());
        while !b.is_empty() {
            let r = self.dr(&a, &b).1;
            a = b;
            b = r;
        }
        if a.is_empty() {
            a
        } else {
            self.scale(&a, inv(*a.last().unwrap(), self.0))
        }
    }
    fn der(self, a: &[u64]) -> Poly {
        self.trim(
            a.iter()
                .enumerate()
                .skip(1)
                .map(|(i, x)| x * i as u64 % self.0)
                .collect(),
        )
    }
    fn eval(self, a: &[u64], x: u64) -> u64 {
        a.iter().rev().fold(0, |v, c| (v * x + c) % self.0)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Rat {
    n: Poly,
    d: Poly,
}
impl Rat {
    fn new(r: Ring, n: Poly, d: Poly) -> Self {
        assert!(!d.is_empty());
        let g = r.gcd(&n, &d);
        let n = r.exact(&n, &g);
        let d = r.exact(&d, &g);
        let s = inv(*d.last().unwrap(), r.0);
        Self {
            n: r.scale(&n, s),
            d: r.scale(&d, s),
        }
    }
    fn poly(a: Poly) -> Self {
        Self { n: a, d: vec![1] }
    }
    fn add(&self, r: Ring, b: &Self) -> Self {
        Self::new(
            r,
            r.add(&r.mul(&self.n, &b.d), &r.mul(&b.n, &self.d)),
            r.mul(&self.d, &b.d),
        )
    }
    fn sub(&self, r: Ring, b: &Self) -> Self {
        self.add(r, &b.scale(r, r.0 - 1))
    }
    fn mul(&self, r: Ring, b: &Self) -> Self {
        Self::new(r, r.mul(&self.n, &b.n), r.mul(&self.d, &b.d))
    }
    fn div(&self, r: Ring, b: &Self) -> Self {
        assert!(!b.n.is_empty());
        Self::new(r, r.mul(&self.n, &b.d), r.mul(&self.d, &b.n))
    }
    fn scale(&self, r: Ring, s: u64) -> Self {
        Self::new(r, r.scale(&self.n, s), self.d.clone())
    }
    fn pow(&self, r: Ring, n: usize) -> Self {
        Self::new(r, r.pow(&self.n, n), r.pow(&self.d, n))
    }
}
fn compose(r: Ring, a: &[u64], x: &Rat) -> Rat {
    a.iter().rev().fold(Rat::poly(vec![]), |v, c| {
        v.mul(r, x).add(r, &Rat::poly(r.trim(vec![*c])))
    })
}
// Generic point is (X(x), y*S(x)) in Fp(x)[y]/(y²-f(x)).
fn generic_add(
    r: Ring,
    a: u64,
    b: u64,
    p: &(Rat, Rat),
    q: &(Rat, Rat),
    double: bool,
) -> (Rat, Rat) {
    let f = Rat::poly(vec![b, a, 0, 1]);
    let l = if double {
        p.0.pow(r, 2)
            .scale(r, 3)
            .add(r, &Rat::poly(vec![a]))
            .div(r, &f.mul(r, &p.1).scale(r, 2))
    } else {
        q.1.sub(r, &p.1).div(r, &q.0.sub(r, &p.0))
    };
    let x = f.mul(r, &l.pow(r, 2)).sub(r, &p.0).sub(r, &q.0);
    let s = l.mul(r, &p.0.sub(r, &x)).sub(r, &p.1);
    (x, s)
}
fn generic_seven(r: Ring, a: u64, b: u64) -> (Rat, Rat) {
    let p = (Rat::poly(vec![0, 1]), Rat::poly(vec![1]));
    let p2 = generic_add(r, a, b, &p, &p, true);
    let p3 = generic_add(r, a, b, &p2, &p, false);
    let p6 = generic_add(r, a, b, &p3, &p3, true);
    generic_add(r, a, b, &p6, &p, false)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Velu {
    h: Poly,
    n: Poly,
    m: Poly,
    a: u64,
    b: u64,
    degree: u64,
}
impl Velu {
    fn roots(r: Ring, a: u64, b: u64, roots: &[u64]) -> Self {
        let h = roots
            .iter()
            .fold(vec![1], |h, t| r.mul(&h, &[(r.0 - t) % r.0, 1]));
        let mut n = r.mul(&[0, 1], &r.pow(&h, 2));
        let (mut ts, mut ws) = (0, 0);
        for t in roots {
            let q = r.exact(&h, &[(r.0 - t) % r.0, 1]);
            let tt = (3 * t * t + a) % r.0;
            let f = (t * t % r.0 * t + a * t + b) % r.0;
            n = r.add(
                &n,
                &r.add(
                    &r.scale(&r.mul(&q, &h), 2 * tt % r.0),
                    &r.scale(&r.pow(&q, 2), 4 * f % r.0),
                ),
            );
            ts = (ts + tt) % r.0;
            ws = (ws + 5 * t * t % r.0 * t + 3 * a * t + 2 * b) % r.0;
        }
        let m = r.sub(&r.mul(&r.der(&n), &h), &r.scale(&r.mul(&n, &r.der(&h)), 2));
        Self {
            h,
            n,
            m,
            a: (a + r.0 - 10 * ts % r.0) % r.0,
            b: (b + r.0 - 14 * ws % r.0) % r.0,
            degree: 2 * roots.len() as u64 + 1,
        }
    }
    fn certify(&self, r: Ring, a: u64, b: u64) -> bool {
        self.n.len() == self.degree as usize + 1
            && self.h.len() == (self.degree as usize + 1) / 2
            && r.gcd(&self.n, &self.h) == vec![1]
            && r.gcd(&self.h, &r.der(&self.h)) == vec![1]
            && r.mul(&[b, a, 0, 1], &r.pow(&self.m, 2))
                == r.add(
                    &r.add(
                        &r.pow(&self.n, 3),
                        &r.scale(&r.mul(&self.n, &r.pow(&self.h, 4)), self.a),
                    ),
                    &r.scale(&r.pow(&self.h, 6), self.b),
                )
    }
    fn eval(&self, e: Quad, p: P) -> P {
        let Some((x, y)) = p else { return None };
        let ev = |a: &[u64]| {
            a.iter()
                .rev()
                .fold(Q(0, 0), |v, c| e.add(e.mul(v, x), Q(*c, 0)))
        };
        let h = ev(&self.h);
        if h == Q(0, 0) {
            return None;
        }
        let hi = e.inv(h);
        let hi2 = e.mul(hi, hi);
        Some((
            e.mul(ev(&self.n), hi2),
            e.mul(y, e.mul(ev(&self.m), e.mul(hi2, hi))),
        ))
    }
    fn rat(&self, r: Ring, x: &Rat, s: &Rat) -> (Rat, Rat) {
        let h = compose(r, &self.h, x);
        (
            compose(r, &self.n, x).div(r, &h.pow(r, 2)),
            s.mul(r, &compose(r, &self.m, x).div(r, &h.pow(r, 3))),
        )
    }
}
fn scale_point(e: Quad, p: P, u: u64) -> P {
    p.map(|(x, y)| (e.scale(x, u * u % e.p), e.scale(y, u * u % e.p * u % e.p)))
}
fn rational(e: Quad) -> Vec<P> {
    let mut out = vec![None];
    for x in 0..e.p {
        if let Some(y) = toy_field::sqrt(e.rhs(Q(x, 0)).0, e.p) {
            out.push(Some((Q(x, 0), Q(y, 0))));
            if y != 0 {
                out.push(Some((Q(x, 0), Q(e.p - y, 0))));
            }
        }
    }
    out.sort();
    out
}
fn canon(e: Quad, x: u64) -> P {
    toy_field::sqrt(e.rhs(Q(x, 0)).0, e.p)
        .map(|y| Some((Q(x, 0), Q(y.min(e.p - y), 0))))
        .unwrap()
}
fn pair(p: P) -> Vec<u64> {
    p.map(|(x, y)| {
        assert_eq!((x.1, y.1), (0, 0));
        vec![x.0, y.0]
    })
    .unwrap_or(vec![0])
}
fn embed(e: Quad, p: P) -> P {
    p.map(|(x, y)| {
        (
            Q(x.0 * inv(e.d, e.p) % e.p, 0),
            Q(0, y.0 * inv(e.d * e.d % e.p, e.p) % e.p),
        )
    })
}
fn roots(points: &BTreeSet<P>) -> Vec<u64> {
    points
        .iter()
        .filter_map(|p| {
            p.map(|(x, _)| {
                assert_eq!(x.1, 0);
                x.0
            })
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
fn f3(e: Quad, x: u64, y: u64, z: u64) -> u64 {
    let a = (x + e.p - y) % e.p;
    let c = (x + y) % e.p;
    let v = (x * y + e.p - e.a) % e.p;
    (a * a % e.p * z % e.p * z + e.p
        - 2 * ((c * (x * y % e.p) + e.a * c + 2 * e.b) % e.p) * z % e.p
        + v * v % e.p
        + e.p
        - 4 * e.b % e.p * c % e.p)
        % e.p
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Witness {
    indices: Vec<usize>,
    mask: u64,
}
fn witness_sum(e: Quad, base: &[P], w: &Witness) -> P {
    w.indices.iter().enumerate().fold(None, |s, (i, j)| {
        e.pa(
            s,
            if w.mask & (1 << i) == 0 {
                base[*j]
            } else {
                e.pn(base[*j])
            },
        )
    })
}
fn collect(e: Quad, base: &[P], target: P, m: usize) -> Vec<Witness> {
    let mut out = vec![];
    for i in 0..base.len() {
        for j in i..base.len() {
            for k in 0..if m == 3 { base.len() } else { 1 } {
                for mask in 0..1 << m {
                    let w = Witness {
                        indices: if m == 3 { vec![i, j, k] } else { vec![i, j] },
                        mask,
                    };
                    if witness_sum(e, base, &w) == target {
                        out.push(w);
                    }
                }
            }
        }
    }
    out
}
fn solve(mut rows: Vec<Vec<u64>>, n: usize, p: u64) -> (usize, Option<Vec<u64>>) {
    let mut pivot = vec![];
    let mut rank = 0;
    for c in 0..n {
        let Some(k) = (rank..rows.len()).find(|k| rows[*k][c] != 0) else {
            continue;
        };
        rows.swap(k, rank);
        let s = inv(rows[rank][c], p);
        for x in &mut rows[rank] {
            *x = *x * s % p;
        }
        for i in 0..rows.len() {
            if i != rank {
                let s = rows[i][c];
                for j in c..=n {
                    rows[i][j] = (rows[i][j] + p - s * rows[rank][j] % p) % p;
                }
            }
        }
        pivot.push(c);
        rank += 1;
    }
    assert!(
        rows.iter()
            .all(|r| r[..n].iter().any(|x| *x != 0) || r[n] == 0),
        "inconsistent relation system"
    );
    if rank != n {
        return (rank, None);
    }
    let mut x = vec![0; n];
    for (i, c) in pivot.iter().enumerate() {
        x[*c] = rows[i][n];
    }
    (rank, Some(x))
}
fn row(w: &Witness, j: u64, b: usize) -> Vec<u64> {
    let mut r = vec![0; b + 2];
    for (i, k) in w.indices.iter().enumerate() {
        r[*k] = (r[*k] + if w.mask & (1 << i) == 0 { 1 } else { 822 }) % 823;
    }
    r[b] = 822;
    r[b + 1] = j;
    r
}
fn ppoly(a: &[u64]) -> String {
    if a.is_empty() {
        "Mod(0,p)".into()
    } else {
        format!("Polrev({})*Mod(1,p)", serde_json::to_string(a).unwrap())
    }
}
fn prat(a: &Rat) -> String {
    format!("({})/({})", ppoly(&a.n), ppoly(&a.d))
}
fn gp_point(p: P) -> String {
    serde_json::to_string(&pair(p)).unwrap()
}
fn gp_map(m: &Velu) -> String {
    format!(
        "[{},{},{}]",
        ppoly(&m.n),
        format!("y*({})", ppoly(&m.m)),
        ppoly(&m.h)
    )
}
fn order(e: Quad, p: P, n: u64) -> u64 {
    (1..=n).find(|k| e.pm(p, *k as u128).is_none()).unwrap()
}
fn produce(out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh output directory required".into());
    }
    fs::create_dir_all(out)?;
    let e = Quad::new(827, 1, 15);
    let t = Quad::new(827, 4, 120);
    let r = Ring(827);
    assert_eq!(e.d, 2);
    let g = Some((Q(2, 0), Q(5, 0)));
    assert!(e.on(g));
    assert_eq!(e.pm(g, 823), None);
    let q = e.pm(g, 37);
    let all = rational(e);
    let ta = rational(t);
    assert_eq!((all.len(), ta.len()), (823, 833));
    let ts: Vec<_> = ta
        .iter()
        .copied()
        .filter(|p| t.pm(*p, 7).is_none())
        .collect();
    assert_eq!(ts.len(), 49);
    let t1 = *ts.iter().find(|p| p.is_some()).unwrap();
    let span: BTreeSet<_> = (0..7).map(|k| t.pm(t1, k)).collect();
    let t2 = *ts.iter().find(|p| !span.contains(p)).unwrap();
    let basis: BTreeSet<_> = (0..7)
        .flat_map(|i| (0..7).map(move |j| t.pa(t.pm(t1, i), t.pm(t2, j))))
        .collect();
    assert_eq!(basis, ts.iter().copied().collect());
    let embedded: BTreeSet<_> = ts.iter().map(|p| embed(e, *p)).collect();
    for p in &ts {
        let ep = embed(e, *p);
        assert!(e.on(ep));
        assert_eq!(e.pm(ep, 7), None);
        assert_eq!(e.frob(ep), e.pn(ep));
        for z in &ts {
            assert_eq!(embed(e, t.pa(*p, *z)), e.pa(ep, embed(e, *z)));
        }
    }
    let seven = generic_seven(r, e.a, e.b);
    let ui = inv(7, 827);
    let mut maps = vec![];
    let mut models = vec![(e, None, None)];
    let mut gp=String::from("default(parisizemax,4000000000);\np=827;E=ellinit([Mod(1,p),Mod(15,p)]);T=ellinit([Mod(4,p),Mod(120,p)]);G=[Mod(2,p),Mod(5,p)];Q=ellmul(E,G,37);if(ellcard(E)!=823||ellcard(T)!=833||vecsort(ellgroup(T))!=[7,119],error(\"orders/structure\"));\n");
    gp.push_str("ck(c,s)=if(!c,error(s));\npa(e,a,b)=elladd(e,a,b);\npn(e,a)=ellneg(e,a);\n");
    gp.push_str(&format!("t1=Mod(1,p)*{};t2=Mod(1,p)*{};ck(ellorder(T,t1)==7&&ellorder(T,t2)==7,\"basis orders\");ck(sum(i=0,6,ellmul(T,t1,i)==t2)==0,\"independence\");\n",gp_point(t1),gp_point(t2)));
    gp.push_str(&format!("tt=List();for(i=0,6,for(j=0,6,listput(tt,lift(elladd(T,ellmul(T,t1,i),ellmul(T,t2,j))))));ck(Set(Vec(tt))==Set([{}]),\"all torsion\");\n",ts.iter().map(|p|gp_point(*p)).collect::<Vec<_>>().join(",")));
    let mut lines = BTreeSet::new();
    let mut dual_comp = 0;
    for c in 0..8 {
        let kg = if c == 7 { t2 } else { t.pa(t1, t.pm(t2, c)) };
        let kp: BTreeSet<_> = (0..7).map(|k| embed(e, t.pm(kg, k))).collect();
        let kr = roots(&kp);
        assert_eq!(kr.len(), 3);
        assert!(lines.insert(kr.clone()));
        let m = Velu::roots(r, e.a, e.b, &kr);
        gp.push_str(&format!("kg=elladd(T,t1,ellmul(T,t2,{}));{}ck(lift(kg)=={},\"line generator\");rr=Set(vector(6,i,lift(ellmul(T,kg,i)[1]/2)));ck(rr=={},\"kernel roots from twist\");\n",c,if c==7{"kg=t2;"}else{""},gp_point(kg),serde_json::to_string(&kr)?));
        assert!(m.certify(r, e.a, e.b));
        let dest = Quad::new(827, m.a, m.b);
        let da = rational(dest);
        assert_eq!(da.len(), 823);
        for p in &kp {
            assert_eq!(m.eval(e, *p), None);
        }
        let image: BTreeSet<_> = all
            .iter()
            .map(|p| {
                let z = m.eval(e, *p);
                assert!(dest.on(z));
                assert_eq!(m.eval(e, e.pa(*p, g)), dest.pa(z, m.eval(e, g)));
                z
            })
            .collect();
        assert_eq!(image, da.iter().copied().collect());
        let dk: BTreeSet<_> = embedded.iter().map(|p| m.eval(e, *p)).collect();
        assert_eq!(dk.len(), 7);
        let dr = roots(&dk);
        assert_eq!(dr.len(), 3);
        let d = Velu::roots(r, dest.a, dest.b, &dr);
        assert!(d.certify(r, dest.a, dest.b));
        assert_eq!(d.a, e.a * toy_field::pow(7, 4, 827) % 827);
        assert_eq!(d.b, e.b * toy_field::pow(7, 6, 827) % 827);
        let p = (Rat::poly(vec![0, 1]), Rat::poly(vec![1]));
        let z = m.rat(r, &p.0, &p.1);
        let z = d.rat(r, &z.0, &z.1);
        let comp = (
            z.0.scale(r, ui * ui % 827),
            z.1.scale(r, ui * ui % 827 * ui % 827),
        );
        assert_eq!(comp, seven);
        let z = d.rat(r, &p.0, &p.1);
        let z = (
            z.0.scale(r, ui * ui % 827),
            z.1.scale(r, ui * ui % 827 * ui % 827),
        );
        let comp2 = m.rat(r, &z.0, &z.1);
        let sev2 = generic_seven(r, dest.a, dest.b);
        assert_eq!(comp2, sev2);
        dual_comp += 2;
        let dg = m.eval(e, g);
        let dq = m.eval(e, q);
        for p in &all {
            let z = m.eval(e, *p);
            assert_eq!(scale_point(e, d.eval(dest, z), ui), e.pm(*p, 7));
            assert_eq!(e.pm(scale_point(e, d.eval(dest, z), ui), 588), *p);
        }
        for p in &da {
            assert_eq!(
                m.eval(e, scale_point(e, d.eval(dest, *p), ui)),
                dest.pm(*p, 7)
            );
        }
        let mut bad = m.clone();
        bad.n[0] = (bad.n[0] + 1) % 827;
        assert!(!bad.certify(r, e.a, e.b));
        let identity = registry::identity(827, dest.a, dest.b, 823, None);
        let source = registry::identity(827, e.a, e.b, 823, None);
        let map_id=format!("TWK1:{}",registry::digest(serde_json::to_string(&json!({"source":source["model_json"],"destination":identity["model_json"],"map":m,"dual_normalized":d,"inverse_scale":ui})).unwrap().as_bytes()));
        maps.push(json!({"line":c,"twist_kernel_generator":pair(kg),"kernel_roots":kr,"map_id":map_id,"destination":identity,"map":m,"dual_kernel_roots":dr,"dual_normalized":d,"dual_inverse_scale":ui,"mapped_generator":pair(dg),"mapped_target":pair(dq),"image_cardinality":image.len(),"rational_source_checks":all.len(),"rational_destination_checks":da.len(),"generic_dual_compositions_verified":2,"corrupt_coefficient_rejected":true,"multiplication_seven_source":seven,"multiplication_seven_destination":sev2}));
        models.push((dest, Some(m.clone()), Some(d.clone())));
        gp.push_str(&format!("h={};v=ellisogeny(E,h);D=ellinit(v[1]);f=v[2];ck(v[1]==[0,0,0,Mod({},p),Mod({},p)]&&f=={},\"forward map {}\");ck(ellcard(D)==823,\"destination count\");w=ellisogeny(D,{});fd=w[2];ck(fd=={}&&w[1]==[0,0,0,Mod({},p),Mod({},p)],\"reverse map\");ci=ellisogenyapply(fd,f);ck(ci[1]/ci[3]^2/7^2=={}&&ci[2]/y/ci[3]^3/7^3=={},\"generic dual source\");ci=ellisogenyapply(f,[fd[1]/7^2,fd[2]/7^3,fd[3]]);ck(ci[1]/ci[3]^2=={}&&ci[2]/y/ci[3]^3=={},\"generic dual destination\");\n",ppoly(&m.h),dest.a,dest.b,gp_map(&m),c,ppoly(&d.h),gp_map(&d),d.a,d.b,prat(&seven.0),prat(&seven.1),prat(&sev2.0),prat(&sev2.1)));
        gp.push_str("for(i=0,822,P=ellmul(E,G,i);Z=ellisogenyapply(f,P);ck(ellisoncurve(D,Z),\"image\");B=ellisogenyapply(fd,Z);if(B!=[0],B=[B[1]/7^2,B[2]/7^3]);ck(B==ellmul(E,P,7)&&ellmul(E,B,588)==P,\"dual/recovery source\");ck(ellisogenyapply(f,elladd(E,P,G))==elladd(D,Z,ellisogenyapply(f,G)),\"homomorphism\"));\n");
        gp.push_str("GD=ellisogenyapply(f,G);for(i=0,822,P=ellmul(D,GD,i);B=ellisogenyapply(fd,P);if(B!=[0],B=[B[1]/7^2,B[2]/7^3]);ck(ellisogenyapply(f,B)==ellmul(D,P,7),\"dual destination\"));\n");
        gp.push_str(&format!("bad={};bad[1]+=1;xx=bad[1]/bad[3]^2;yy=bad[2]/y/bad[3]^3;ck((x^3+x+15)*yy^2!=xx^3+{}*xx+{},\"corrupt detector\");\n",gp_map(&m),dest.a,dest.b));
    }
    assert_eq!(lines.len(), 8);
    let h = *ta
        .iter()
        .filter(|p| p.is_some())
        .min_by_key(|p| (std::cmp::Reverse(order(t, **p, 833)), **p))
        .unwrap();
    let ho = order(t, h, 833);
    assert_eq!(ho, 119);
    let mut b0 = vec![g];
    let mut b1 = vec![g];
    for x in 0..827 {
        if x != 2 && toy_field::sqrt(e.rhs(Q(x, 0)).0, 827).is_some() {
            b0.push(canon(e, x));
            if b0.len() == 8 {
                break;
            }
        }
    }
    let mut xs = BTreeSet::new();
    let mut scanned = 0;
    for k in 1..ho {
        let z = t.pm(h, k as u128).unwrap();
        let x = (z.0 .0 * inv(2, 827) + 1) % 827;
        scanned = k;
        if x != 2 && toy_field::sqrt(e.rhs(Q(x, 0)).0, 827).is_some() {
            xs.insert(x);
        }
        if xs.len() == 7 {
            break;
        }
    }
    for x in xs {
        b1.push(canon(e, x));
    }
    assert_eq!((b0.len(), b1.len()), (8, 8));
    let bases = [b0, b1];
    let mut cases = vec![];
    let mut systems = vec![];
    let mut references = vec![vec![]; 2];
    let mut chained_exceptions = 0;
    let mut signed_control = 0;
    let mut polynomial_checks = 0;
    gp.push_str("summ(e,bb,ii,mask)={my(s=[0],z);for(k=1,#ii,z=bb[ii[k]+1];if(bittest(mask,k-1),z=ellneg(e,z));s=elladd(e,s,z));s};\ncollect(e,bb,rr,m)={my(v=List(),ii);for(i=0,#bb-1,for(j=i,#bb-1,for(k=0,if(m==3,#bb-1,0),ii=if(m==3,[i,j,k],[i,j]);for(mask=0,2^m-1,if(summ(e,bb,ii,mask)==rr,listput(v,[ii,mask]))))));Vec(v)};\n");
    for (mi, (de, map, _)) in models.iter().enumerate() {
        for (bi, source_base) in bases.iter().enumerate() {
            let base: Vec<_> = source_base
                .iter()
                .map(|p| map.as_ref().map_or(*p, |m| m.eval(e, *p)))
                .collect();
            let dg = map.as_ref().map_or(g, |m| m.eval(e, g));
            let dq = map.as_ref().map_or(q, |m| m.eval(e, q));
            let mut rows = vec![];
            let mut anchor = vec![0; 10];
            anchor[0] = 1;
            anchor[9] = 1;
            rows.push(anchor);
            let mut local = vec![];
            let mut relation_count = 0;
            gp.push_str(&format!("D=ellinit([Mod({},p),Mod({},p)]);bb=Mod(1,p)*[{}];gg=Mod(1,p)*{};qq=Mod(1,p)*{};mat=List();listput(mat,[1,0,0,0,0,0,0,0,0,1]);\n",de.a,de.b,base.iter().map(|p|gp_point(*p)).collect::<Vec<_>>().join(","),gp_point(dg),gp_point(dq)));
            for m in [2, 3] {
                for j in 0..16 {
                    let target = de.pa(dq, de.pm(dg, j));
                    assert!(target.is_some());
                    let ws = collect(*de, &base, target, m);
                    for w in &ws {
                        rows.push(row(w, j as u64, 8));
                        relation_count += 1;
                        if m == 2 {
                            assert_eq!(
                                f3(
                                    *de,
                                    base[w.indices[0]].unwrap().0 .0,
                                    base[w.indices[1]].unwrap().0 .0,
                                    target.unwrap().0 .0
                                ),
                                0
                            );
                            polynomial_checks += 1;
                        } else {
                            let prefix = Witness {
                                indices: w.indices[..2].to_vec(),
                                mask: w.mask & 3,
                            };
                            let mid = witness_sum(*de, &base, &prefix);
                            if let Some((mx, _)) = mid {
                                assert_eq!(
                                    f3(
                                        *de,
                                        base[w.indices[0]].unwrap().0 .0,
                                        base[w.indices[1]].unwrap().0 .0,
                                        mx.0
                                    ),
                                    0
                                );
                                assert_eq!(
                                    f3(
                                        *de,
                                        mx.0,
                                        base[w.indices[2]].unwrap().0 .0,
                                        target.unwrap().0 .0
                                    ),
                                    0
                                );
                                polynomial_checks += 2;
                            } else {
                                chained_exceptions += 1;
                            }
                        }
                        let mut wrong = w.clone();
                        wrong.mask ^= 1;
                        assert_ne!(witness_sum(*de, &base, &wrong), target);
                        signed_control += 1;
                    }
                    let wg = ws
                        .iter()
                        .map(|w| {
                            format!(
                                "[{},{}]",
                                serde_json::to_string(&w.indices).unwrap(),
                                w.mask
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    gp.push_str(&format!("ww=collect(D,bb,elladd(D,qq,ellmul(D,gg,{})),{});ck(ww==[{}],\"relation set {} {} {} {}\");for(z=1,#ww,rr=vector(10);for(k=1,#ww[z][1],ii=ww[z][1][k]+1;rr[ii]+=if(bittest(ww[z][2],k-1),-1,1));rr[9]=-1;rr[10]={};listput(mat,rr);ck(summ(D,bb,ww[z][1],bitxor(ww[z][2],1))!=elladd(D,qq,ellmul(D,gg,{})),\"wrong sign\"));\n",j,m,wg,mi,bi,m,j,j,j));
                    local.push(json!({"arity":m,"offset":j,"target":pair(target),"witnesses":ws}));
                }
            }
            if mi == 0 {
                references[bi] = local.clone();
            } else {
                for (k, v) in local.iter().enumerate() {
                    assert_eq!(v["witnesses"], references[bi][k]["witnesses"]);
                }
            }
            let (rank, solution) = solve(rows.clone(), 9, 823);
            let recovered = solution.as_ref().map(|v| v[8]);
            if let Some(k) = recovered {
                assert_eq!(de.pm(dg, k as u128), dq);
                assert_eq!(e.pm(g, k as u128), q);
            }
            gp.push_str(&format!("MM=Mat(Vec(mat)~)*Mod(1,823);ck(matrank(MM[,1..9])=={}&&matrank(MM)=={},\"relation ranks\");",rank,rank));
            if let Some(k) = recovered {
                gp.push_str(&format!("ss=matsolve(MM[,1..9],MM[,10]);ck(lift(ss[9])=={}&&ellmul(D,gg,lift(ss[9]))==qq,\"relation scalar\");",k));
            }
            gp.push('\n');
            cases.push(json!({"model":mi,"base":bi,"points":base.iter().map(|p|pair(*p)).collect::<Vec<_>>(),"generator":pair(dg),"target":pair(dq),"queries":local}));
            systems.push(json!({"model":mi,"base":bi,"rows_including_anchor":rows.len(),"relation_rows":relation_count,"rank":rank,"unknowns":9,"solution":solution,"recovered_scalar":recovered,"scalar_verified":recovered.is_some(),"matrix_sha256":registry::digest(serde_json::to_string(&rows)?.as_bytes())}));
        }
    }
    gp.push_str("print(\"ALL_TWIST_KERNEL_RELATIONS_OK 8 576 18\");\n");
    fs::write(out.join("verify.gp"), &gp)?;
    let result = json!({"schema":"twist.kernel-relations/v1","source":registry::identity(827,1,15,823,None),"twist":registry::identity(827,4,120,833,None),"generator":pair(g),"target":pair(q),"torsion_basis":[pair(t1),pair(t2)],"torsion_points":ts.iter().map(|p|pair(*p)).collect::<Vec<_>>(),"twist_maximal_generator":pair(h),"twist_maximal_order":ho,"twist_base_scanned_terms":scanned,"source_bases":bases.iter().map(|b|b.iter().map(|p|pair(*p)).collect::<Vec<_>>()).collect::<Vec<_>>(),"maps":maps,"cases":cases,"systems":systems,"summary":{"kernel_lines":8,"twist_torsion_points":49,"embedding_group_law_checks":2401,"generic_dual_compositions":dual_comp,"rational_forward_checks":8*823,"rational_reverse_checks":8*823,"query_cases":576,"source_query_cases":64,"destination_query_cases":512,"systems":18,"recovered_systems":systems.iter().filter(|s|s["scalar_verified"]==true).count(),"f3_equation_checks":polynomial_checks,"identity_prefix_exceptions":chained_exceptions,"wrong_sign_controls_rejected":signed_control,"corrupt_map_controls_rejected":8},"gp_script_sha256":registry::digest(gp.as_bytes()),"performance_claim":false,"novelty_claim":false});
    fs::write(
        out.join("RESULTS.json"),
        serde_json::to_string_pretty(&result)? + "\n",
    )?;
    println!("{}", serde_json::to_string(&result["summary"])?);
    Ok(())
}
fn audit(out: &Path, pari: &Path, dest: &Path) -> Result<()> {
    if dest.exists() {
        return Err("fresh admission path required".into());
    }
    let v: Value = serde_json::from_slice(&fs::read(out.join("RESULTS.json"))?)?;
    let receipt: Value = serde_json::from_slice(&fs::read(pari.join("RECEIPT.json"))?)?;
    let gp = fs::read(out.join("verify.gp"))?;
    assert_eq!(v["gp_script_sha256"], registry::digest(&gp));
    assert_eq!(receipt["script_sha256"], v["gp_script_sha256"]);
    assert_eq!(receipt["exit_code"], 0);
    assert!(receipt["signal"].is_null());
    let stdout = fs::read(pari.join("pari.stdout"))?;
    let stderr = fs::read(pari.join("pari.stderr"))?;
    assert_eq!(receipt["stdout_sha256"], registry::digest(&stdout));
    assert_eq!(receipt["stderr_sha256"], registry::digest(&stderr));
    assert_eq!(
        String::from_utf8(stdout)?.trim(),
        "ALL_TWIST_KERNEL_RELATIONS_OK 8 576 18"
    );
    let err = String::from_utf8(stderr)?;
    assert!(
        err.trim().is_empty()
            || err.trim()
                == "***   Warning: new maximum stack size = 4000002048 (3814.699 Mbytes)."
    );
    let temp = out.parent().unwrap().join("native-replay-v1");
    produce(&temp)?;
    assert_eq!(
        fs::read(temp.join("RESULTS.json"))?,
        fs::read(out.join("RESULTS.json"))?
    );
    assert_eq!(fs::read(temp.join("verify.gp"))?, gp);
    let a = json!({"schema":"twist.kernel-relations-admission/v1","status":"native-complete-replay-and-independent-PARI-passed","summary":v["summary"],"maps":v["maps"].as_array().unwrap().iter().map(|m|json!({"line":m["line"],"destination":m["destination"],"map_id":m["map_id"],"kernel_roots":m["kernel_roots"],"a":m["map"]["a"],"b":m["map"]["b"]})).collect::<Vec<_>>(),"systems":v["systems"],"result_sha256":registry::digest(&fs::read(out.join("RESULTS.json"))?),"pari_receipt_sha256":registry::digest(&fs::read(pari.join("RECEIPT.json"))?),"all_same_base_witness_sets_equal":true,"all_same_base_matrices_equal":true,"speedup_established":false,"NIST_connection_established":false});
    fs::write(dest, serde_json::to_string_pretty(&a)? + "\n")?;
    println!("{}", serde_json::to_string(&a)?);
    Ok(())
}
fn export(v: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh export path required".into());
    }
    let read = |p: &Path| -> Result<Value> { Ok(serde_json::from_slice(&fs::read(p)?)?) };
    let p = read(&v.join("protocol.json"))?;
    let f = read(&v.join("FROZEN.json"))?;
    let a = read(&v.join("ADMISSION.json"))?;
    let h = read(&v.join("HOST.json"))?;
    assert_eq!(
        a["status"],
        "native-complete-replay-and-independent-PARI-passed"
    );
    let root = fs::canonicalize(v)?;
    let file = |path: &Path, kind: &str| -> Result<Value> {
        let abs = fs::canonicalize(path)?;
        let ac = abs.components().collect::<Vec<_>>();
        let bc = root.components().collect::<Vec<_>>();
        let common = ac.iter().zip(&bc).take_while(|(x, y)| x == y).count();
        let mut rel = std::path::PathBuf::new();
        for _ in common..bc.len() {
            rel.push("..");
        }
        for x in &ac[common..] {
            rel.push(x.as_os_str());
        }
        Ok(
            json!({"path":rel,"sha256":registry::digest(&fs::read(path)?),"bytes":fs::metadata(path)?.len(),"kind":kind}),
        )
    };
    let source =
        Path::new("research/twist_routes_1_3_20261021/runner/src/bin/twist-kernel-relations.rs");
    let transport = read(&v.join("runs/producer-transport.json"))?;
    let artifacts = vec![
        file(&v.join("runs.tar.xz"), "raw")?,
        file(&v.join("RAW_MEMBERS.json"), "receipt")?,
        file(&v.join("ADMISSION.json"), "receipt")?,
    ];
    let runs=p["arms"].as_array().unwrap().iter().map(|arm|json!({"run_id":format!("twist-kernel-{}-000",arm["id"].as_str().unwrap()),"arm":arm["id"],"instance_id":"p827-all-eight-kernels-all-576-query-cases","seed":0,"replicate":0,"warmup":false,"status":"succeeded","outcome_class":"completed","attempts":1,"resumed":0,"checkpoint":{"policy":"none","generations_written":0,"restored_from_generation":null},"measurements":{"counted":{"query_cases":a["summary"]["query_cases"],"kernel_lines":a["summary"]["kernel_lines"]},"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":{"generic_dual_compositions":a["summary"]["generic_dual_compositions"],"systems":a["summary"]["systems"]}},"isolation":{"tier":"L0","contended":null,"blockers":["Exact correctness experiment; no performance comparison."]},"verification":{"status":"verified","verifier":"native complete replay and independent PARI maps/group/relations/linear algebra"},"exclusion":{"excluded":false,"rule":null},"artifacts":artifacts,"started_at":if arm["id"]=="native"{transport["started"].clone()}else{Value::Null},"finished_at":if arm["id"]=="native"{transport["finished"].clone()}else{Value::Null},"worker":arm["instrument"]["binary_sha256"],"backend_ref":if arm["id"]=="native"{"runs/kernel-v1"}else{"runs/pari-v1"}})).collect::<Vec<_>>();
    let result = json!({"schema":"experiment.export/v1","export_id":"twist-kernel-relations-20261027-v1","created":String::from_utf8(std::process::Command::new("date").args(["-u","+%Y-%m-%dT%H:%M:%SZ"]).output()?.stdout)?.trim(),"protocol":{"path":"protocol.json","sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]},"producer":{"runtime":"codex","model":"GPT-6","session":"twist-kernel-relations-20261027","role":"research-and-implementation"},"source":{"repo":"crypto","commit":f["source_revision"],"dirty":false},"backend":"local","host":{"path":"HOST.json","sha256":registry::digest(&fs::read(v.join("HOST.json"))?),"class_id":null,"summary":h["summary"]},"arms":p["arms"].as_array().unwrap().iter().map(|arm|json!({"id":arm["id"],"role":arm["role"],"binary_sha256":arm["instrument"]["binary_sha256"],"commit":arm["instrument"]["commit"]})).collect::<Vec<_>>(),"runs":runs,"flow":{"planned":2,"attempted":2,"completed":2,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]},"files":[file(&v.join("FROZEN.json"),"receipt")?,file(&v.join("protocol.json"),"protocol")?],"analysis":{"script":file(source,"analysis")?,"blinded":false,"blinding_key_sha256":null,"outputs":[file(&v.join("ADMISSION.json"),"analysis")?],"unmasked_at":null},"notes":[{"kind":"observation","text":"Two actual complete finite native/CAS executions on one public curve and target. Sixteen offsets generate relations for the same target; they are not independent DLP samples. Same-base rows and ranks agree under injective transport. No performance, novelty, NIST or complete-IC improvement follows.","by":"native exporter"}],"reviews_dir":"reviews"});
    fs::write(out, serde_json::to_string_pretty(&result)? + "\n")?;
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("produce") if a.len() == 2 => produce(Path::new(&a[1])),
        Some("audit") if a.len() == 4 => {
            audit(Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]))
        }
        Some("export") if a.len() == 3 => export(Path::new(&a[1]), Path::new(&a[2])),
        _ => Err("produce FRESH-OUT | audit OUT PARI FRESH-ADMISSION".into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cubic_control() {
        let r = Ring(11);
        let e = Quad::new(11, 0, 1);
        let m = Velu::roots(r, 0, 1, &[0]);
        assert!(m.certify(r, 0, 1));
        assert_eq!((m.a, m.b), (0, 6));
        let d = Quad::new(11, m.a, m.b);
        for p in rational(e) {
            assert!(d.on(m.eval(e, p)));
        }
        let mut bad = m;
        bad.n[0] += 1;
        assert!(!bad.certify(r, 0, 1));
    }
    #[test]
    fn generic_scalar_control() {
        let r = Ring(101);
        let e = Quad::new(101, 2, 3);
        let (x, s) = generic_seven(r, 2, 3);
        for p in rational(e) {
            if let Some((xx, yy)) = p {
                let hd = r.eval(&x.d, xx.0);
                let sd = r.eval(&s.d, xx.0);
                if hd != 0 && sd != 0 {
                    let z = Some((
                        Q(r.eval(&x.n, xx.0) * inv(hd, 101) % 101, 0),
                        Q(yy.0 * r.eval(&s.n, xx.0) % 101 * inv(sd, 101) % 101, 0),
                    ));
                    assert_eq!(z, e.pm(p, 7));
                }
            }
        }
    }
    #[test]
    fn non_oracle_elimination() {
        let r = vec![vec![1, 0, 1], vec![2, 1, 7], vec![3, 1, 8]];
        assert_eq!(solve(r, 2, 11), (2, Some(vec![1, 5])));
        assert_eq!(solve(vec![vec![1, 1, 2]], 2, 11), (1, None));
    }
    #[test]
    fn summation_polynomial_control() {
        let e = Quad::new(11, 0, 1);
        for p in rational(e) {
            for q in rational(e) {
                if let (Some((x, _)), Some((y, _)), Some((z, _))) = (p, q, e.pa(p, q)) {
                    assert_eq!(f3(e, x.0, y.0, z.0), 0);
                }
            }
        }
        let e = Quad::new(101, 2, 3);
        for p in rational(e) {
            for q in rational(e) {
                if let (Some((x, _)), Some((y, _)), Some((z, _))) = (p, q, e.pa(p, q)) {
                    assert_eq!(f3(e, x.0, y.0, z.0), 0);
                }
            }
        }
    }
}
