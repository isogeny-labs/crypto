//! Explicit Tate11 / oriented7 incidence circuits and diamond twist, with direct torsion controls.
use crate::round04::Field;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};
static OPS: AtomicU64 = AtomicU64::new(0);
fn tick() {
    assert!(
        OPS.fetch_add(128, Ordering::Relaxed) < 8000000000,
        "CM17 operation cap"
    );
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "op", content = "args")]
enum Op {
    Input(usize),
    Constant(u32),
    Add(usize, usize),
    Mul(usize, usize),
    Inverse(usize),
}
struct Build {
    nodes: Vec<Op>,
    memo: BTreeMap<String, usize>,
    res: Vec<usize>,
    units: Vec<usize>,
}
impl Build {
    fn new() -> Self {
        Self {
            nodes: vec![],
            memo: BTreeMap::new(),
            res: vec![],
            units: vec![],
        }
    }
    fn node(&mut self, o: Op) -> usize {
        let k = serde_json::to_string(&o).unwrap();
        if let Some(i) = self.memo.get(&k) {
            return *i;
        }
        let i = self.nodes.len();
        self.nodes.push(o);
        self.memo.insert(k, i);
        assert!(i < 10000);
        i
    }
    fn c(&mut self, c: u32) -> usize {
        self.node(Op::Constant(c))
    }
    fn add(&mut self, a: usize, b: usize) -> usize {
        if a == b {
            return self.c(0);
        }
        self.node(Op::Add(a.min(b), a.max(b)))
    }
    fn mul(&mut self, a: usize, b: usize) -> usize {
        self.node(Op::Mul(a.min(b), a.max(b)))
    }
    fn sq(&mut self, a: usize) -> usize {
        self.mul(a, a)
    }
    fn div(&mut self, a: usize, b: usize) -> usize {
        let i = self.node(Op::Inverse(b));
        self.units.push(b);
        self.mul(a, i)
    }
    fn unit(&mut self, a: usize) {
        self.node(Op::Inverse(a));
        self.units.push(a);
    }
    fn pa(&mut self, a: &[usize], b: &[usize]) -> Vec<usize> {
        let z = self.c(0);
        (0..a.len().max(b.len()))
            .map(|i| self.add(*a.get(i).unwrap_or(&z), *b.get(i).unwrap_or(&z)))
            .collect()
    }
    fn pm(&mut self, a: &[usize], b: &[usize]) -> Vec<usize> {
        let z = self.c(0);
        let mut out = vec![z; a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                let p = self.mul(*x, *y);
                out[i + j] = self.add(out[i + j], p);
            }
        }
        assert!(out.len() <= 9);
        out
    }
    fn scale(&mut self, a: &[usize], b: usize) -> Vec<usize> {
        a.iter().map(|x| self.mul(*x, b)).collect()
    }
    fn rem(&mut self, a: &[usize], q: &[usize]) -> Vec<usize> {
        assert_eq!(q.len(), 4);
        let mut r = a.to_vec();
        for k in (3..r.len()).rev() {
            let c = r[k];
            for j in 0..4 {
                let t = self.mul(c, q[j]);
                r[k - 3 + j] = self.add(r[k - 3 + j], t);
            }
        }
        r.truncate(3);
        r
    }
}
fn circuit(twist: bool) -> (Build, [usize; 3], [usize; 2]) {
    let mut p = Build::new();
    let x = (0..4).map(|i| p.node(Op::Input(i))).collect::<Vec<_>>();
    let (b, c, l, n) = (x[0], x[1], x[2], x[3]);
    let one = p.c(1);
    let a = p.add(one, c);
    let aa = p.sq(a);
    let bb = p.sq(b);
    let ac = p.mul(a, c);
    let a3 = p.mul(aa, a);
    let ca3 = p.mul(c, a3);
    let disc0 = p.add(b, ca3);
    p.unit(b);
    p.unit(disc0);
    p.unit(c);
    // Tate P2=(b,bc), P3=(c,b+c); explicit independent scalar formulas.
    let bc = p.div(b, c);
    let bc2 = p.sq(bc);
    let abc = p.mul(a, bc);
    let x4 = p.add(bc2, abc);
    let x4 = p.add(x4, b);
    let x4b = p.add(x4, b);
    let y4 = p.mul(bc, x4b);
    let m5 = p.div(y4, x4);
    let m52 = p.sq(m5);
    let am5 = p.mul(a, m5);
    let x5 = p.add(m52, am5);
    let x5 = p.add(x5, b);
    let x5 = p.add(x5, x4);
    let m5a = p.add(m5, a);
    let y5 = p.mul(m5a, x5);
    let y5 = p.add(y5, b);
    let c2 = p.sq(c);
    let y3 = p.add(b, c);
    let ay3 = p.mul(a, y3);
    let m6n = p.add(c2, ay3);
    let m6d = p.add(ac, b);
    let m6 = p.div(m6n, m6d);
    let m62 = p.sq(m6);
    let am6 = p.mul(a, m6);
    let x6 = p.add(m62, am6);
    let x6 = p.add(x6, b);
    let m6a = p.add(m6, a);
    let y6 = p.mul(m6a, x6);
    let m6c = p.mul(m6, c);
    let y6 = p.add(y6, c);
    let y6 = p.add(y6, m6c);
    let ex = p.add(x5, x6);
    let ey = p.add(y5, y6);
    let ax5 = p.mul(a, x5);
    let ey = p.add(ey, ax5);
    let ey = p.add(ey, b);
    p.res.extend([ex, ey]);
    let ll = p.sq(l);
    let al = p.mul(a, l);
    let s1 = p.add(b, ll);
    let mut s1 = p.add(s1, al);
    let an = p.mul(a, n);
    let bl = p.mul(b, l);
    let s2 = p.add(an, bl);
    let nn = p.sq(n);
    let bn = p.mul(b, n);
    let mut s3 = p.add(nn, bn);
    if twist {
        s1 = p.add(s1, aa);
        s3 = p.add(s3, bb);
    }
    let q = [s3, s2, s1, one];
    let s12 = p.mul(s1, s2);
    let sep = p.add(s12, s3);
    p.unit(sep);
    let b3 = p.mul(bb, b);
    let ab2 = p.mul(a, bb);
    let ns1 = p.mul(ab2, s1);
    let a2b = p.mul(aa, b);
    let ns2 = p.mul(a2b, s2);
    let ns3 = p.mul(a3, s3);
    let norm = p.add(b3, ns1);
    let norm = p.add(norm, ns2);
    let norm = p.add(norm, ns3);
    p.unit(norm);
    let d = [b, a];
    let m = [an, al, one];
    let mm = p.pm(&m, &m);
    let md = p.pm(&m, &d);
    let amd = p.scale(&md, a);
    let dd = p.pm(&d, &d);
    let bdd = p.scale(&dd, b);
    let nn = p.pa(&mm, &amd);
    let mut nn = p.pa(&nn, &bdd);
    if twist {
        let aadd = p.scale(&dd, aa);
        nn = p.pa(&nn, &aadd);
    }
    let ap = p.add(a, l);
    let apd = p.scale(&d, ap);
    let t1 = p.pa(&m, &apd);
    let ld = p.scale(&d, l);
    let t2 = p.pa(&m, &ld);
    let lhs = p.pm(&t1, &nn);
    let xdd = {
        let mut t = vec![p.c(0)];
        t.extend(dd.clone());
        t
    };
    let rhs = p.pm(&t2, &xdd);
    let ddd = p.pm(&dd, &d);
    let bddd = p.scale(&ddd, b);
    let pol = p.pa(&lhs, &rhs);
    let pol = p.pa(&pol, &bddd);
    let rem = p.rem(&pol, &q);
    p.res.extend(rem);
    (p, [s1, s2, s3], [a, b])
}
fn eval(p: &Build, f: Field, vars: [u32; 4]) -> Option<Vec<u32>> {
    let mut z = vec![];
    for o in &p.nodes {
        tick();
        let v = match *o {
            Op::Input(i) => vars[i],
            Op::Constant(c) => c,
            Op::Add(a, b) => z[a] ^ z[b],
            Op::Mul(a, b) => f.mul(z[a], z[b]),
            Op::Inverse(a) => {
                if z[a] == 0 {
                    return None;
                }
                f.inv(z[a])
            }
        };
        z.push(v);
    }
    if p.res.iter().any(|i| z[*i] != 0) {
        None
    } else {
        Some(z)
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Pt {
    O,
    A(u32, u32),
}
struct Curve {
    f: Field,
    b: u32,
    c: u32,
    twist: bool,
}
impl Curve {
    fn a(&self) -> u32 {
        1 ^ self.c
    }
    fn a2(&self) -> u32 {
        self.b ^ if self.twist { self.f.sq(self.a()) } else { 0 }
    }
    fn a6(&self) -> u32 {
        if self.twist {
            self.f.sq(self.b)
        } else {
            0
        }
    }
    fn valid(&self, p: Pt) -> bool {
        let Pt::A(x, y) = p else {
            return true;
        };
        let f = self.f;
        f.mul(y, y) ^ f.mul(self.a(), f.mul(x, y)) ^ f.mul(self.b, y)
            == f.mul(x, f.mul(x, x)) ^ f.mul(self.a2(), f.mul(x, x)) ^ self.a6()
    }
    fn neg(&self, p: Pt) -> Pt {
        match p {
            Pt::O => p,
            Pt::A(x, y) => Pt::A(x, y ^ self.f.mul(self.a(), x) ^ self.b),
        }
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        tick();
        let (Pt::A(x, y), Pt::A(u, v)) = (p, q) else {
            return if p == Pt::O { q } else { p };
        };
        let f = self.f;
        let a = self.a();
        let (m, n) = if x == u {
            if q == self.neg(p) {
                return Pt::O;
            }
            assert_eq!(p, q);
            let d = f.mul(a, x) ^ self.b;
            if d == 0 {
                return Pt::O;
            }
            let m = f.mul(f.mul(x, x) ^ f.mul(a, y), f.inv(d));
            (m, y ^ f.mul(m, x))
        } else {
            let m = f.mul(y ^ v, f.inv(x ^ u));
            (m, y ^ f.mul(m, x))
        };
        let xx = f.mul(m, m) ^ f.mul(a, m) ^ self.a2() ^ x ^ u;
        let yy = f.mul(m ^ a, xx) ^ n ^ self.b;
        let r = Pt::A(xx, yy);
        assert!(self.valid(r));
        r
    }
    fn times(&self, mut p: Pt, mut n: usize) -> Pt {
        let mut r = Pt::O;
        while n > 0 {
            if n & 1 == 1 {
                r = self.add(r, p);
            }
            n >>= 1;
            if n > 0 {
                p = self.add(p, p);
            }
        }
        r
    }
}
fn embed(base: Field, ext: Field) -> Vec<u32> {
    let bits = base.modulus as u32;
    let root = (0..1u32 << ext.m)
        .find(|a| {
            tick();
            let mut z = 0;
            for i in (0..=base.m).rev() {
                z = ext.mul(z, *a) ^ ((bits >> i) & 1);
            }
            z == 0
        })
        .unwrap();
    (0..1u32 << base.m)
        .map(|v| {
            let mut z = 0;
            for i in (0..base.m).rev() {
                z = ext.mul(z, root) ^ ((v >> i) & 1);
            }
            z
        })
        .collect()
}
fn independent_lines(f: Field, b: u32, c: u32, twist: bool) -> BTreeSet<(u32, u32)> {
    let ext = Field::find(f.m * 3);
    let em = embed(f, ext);
    let ce = Curve {
        f: ext,
        b: em[b as usize],
        c: em[c as usize],
        twist,
    };
    // z^2+z root table makes the q^3 torsion control nonvacuous at q=32.
    let size = 1u32 << ext.m;
    let mut roots = vec![None; size as usize];
    for z in 0..size {
        tick();
        roots[(ext.sq(z) ^ z) as usize] = Some(z);
    }
    let mut points = vec![];
    for x in 0..size {
        tick();
        let d = ext.mul(ce.a(), x) ^ ce.b;
        let rhs = ext.mul(x, ext.sq(x)) ^ ext.mul(ce.a2(), ext.sq(x)) ^ ce.a6();
        let ys = if d == 0 {
            vec![ext.pow(rhs, 1u64 << (ext.m - 1))]
        } else {
            let w = ext.mul(rhs, ext.inv(ext.sq(d)));
            if let Some(z) = roots[w as usize] {
                let y = ext.mul(d, z);
                vec![y, y ^ d]
            } else {
                vec![]
            }
        };
        for y in ys {
            tick();
            let p = Pt::A(x, y);
            assert!(ce.valid(p));
            if ce.times(p, 7) == Pt::O {
                points.push(p);
            }
        }
    }
    let mut seen = BTreeSet::new();
    let mut lines = BTreeSet::new();
    for p in points {
        tick();
        if seen.contains(&p) {
            continue;
        }
        let q = ce.times(p, 2);
        let r = ce.times(p, 4);
        seen.extend([p, q, r]);
        let fp = match p {
            Pt::A(x, y) => Pt::A(ext.pow(x, 1u64 << f.m), ext.pow(y, 1u64 << f.m)),
            _ => unreachable!(),
        };
        if ![p, q, r].contains(&fp) {
            continue;
        }
        let (Pt::A(x, y), Pt::A(u, v)) = (p, q) else {
            unreachable!()
        };
        let l = ext.mul(y ^ v, ext.inv(x ^ u));
        let n = y ^ ext.mul(l, x);
        let ll = em.iter().position(|v| *v == l).unwrap() as u32;
        let nn = em.iter().position(|v| *v == n).unwrap() as u32;
        lines.insert((ll, nn));
    }
    lines
}
pub fn run(dir: &Path) {
    let (p, q, ab) = circuit(false);
    let (t, _, _) = circuit(true);
    let mut records = vec![];
    for k in 1..=5 {
        let f = Field::find(k);
        let size = 1u32 << k;
        let mut out = vec![];
        let mut base = 0;
        let mut twist = 0;
        let mut direct_checks = 0;
        let mut twisted_direct_checks = 0;
        for b in 1..size {
            for c in 1..size {
                tick();
                let a = 1 ^ c;
                let dd = b ^ f.mul(c, f.mul(a, f.mul(a, a)));
                if dd == 0 {
                    continue;
                }
                let e = Curve {
                    f,
                    b,
                    c,
                    twist: false,
                };
                let pp = Pt::A(0, 0);
                assert!(e.valid(pp));
                if e.times(pp, 11) != Pt::O {
                    continue;
                }
                assert!(e.times(pp, 1) != Pt::O);
                base += 1;
                let mut lines = BTreeSet::new();
                let mut twisted = BTreeSet::new();
                for l in 0..size {
                    for n in 0..size {
                        let vars = [b, c, l, n];
                        if let Some(z) = eval(&p, f, vars) {
                            let delta = [b, c, l ^ a, n ^ b];
                            let zz = eval(&p, f, delta).unwrap();
                            assert_eq!([z[q[0]], z[q[1]], z[q[2]]], [zz[q[0]], zz[q[1]], zz[q[2]]]);
                            assert_eq!([z[ab[0]], z[ab[1]]], [a, b]);
                            lines.insert((l, n));
                        }
                        if eval(&t, f, vars).is_some() {
                            twisted.insert((l, n));
                        }
                    }
                }
                let other = independent_lines(f, b, c, false);
                assert_eq!(lines, other);
                direct_checks += 1;
                let other_twisted = independent_lines(f, b, c, true);
                assert_eq!(twisted, other_twisted);
                twisted_direct_checks += 1;
                assert!(out.len() + lines.len() <= 2000);
                for (l, n) in lines {
                    out.push(json!([b, c, l, n]));
                }
                twist += twisted.len();
            }
        }
        println!("CM17:F2^{k},Tate11 bases{base},oriented7 points{},quadratic twist{twist},direct torsion controls{direct_checks}",out.len());
        records.push(json!({"field_degree":k,"modulus":f.modulus,"Tate11_base_points":base,"untwisted_incidence_points":out,"twisted_incidence_count":twist,"independent_q3_torsion_comparisons":direct_checks,"independent_twisted_q3_torsion_comparisons":twisted_direct_checks}));
    }
    crate::round02::save(
        dir,
        "cm17-explicit-level77-incidence.json",
        &json!({"schema":"koblitz.round16/cm17-v1","level":77,"candidate_compact_genus_under_moduli_bridge":61,"primary_variables":["b","c","lambda","nu"],"Tate_curve":"y2+(1+c)xy+by=x3+bx2","P11":[0,0],"untwisted":{"nodes":p.nodes,"residuals":p.res,"units":p.units},"diamond_quadratic_twist":{"nodes":t.nodes,"residuals":t.res,"units":t.units},"diamond_involution":"lambda -> lambda+1+c;nu -> nu+b","finite_field_records":records,"conservative_field_multiplication_budget_charge":OPS.load(Ordering::Relaxed),"budget_accounting":"128 field multiplications charged per algebra visit;all fields degree<=15;this is a conservative cap,not a measured operation count","proper_normalization_constructed":false,"Koblitz_factor_correspondence_constructed":false,"subgroup_injection_evaluated":false,"scope":"explicit inverse-witness affine Tate incidence and its diamond twist;global compact moduli identification written;factor map open"}),
    );
}

#[path = "diamond18.rs"]
pub(crate) mod diamond18;
#[path = "maps18.rs"]
pub(crate) mod maps18;

#[path = "holdout18.rs"]
pub(crate) mod holdout18;
