#[allow(dead_code)]
#[path = "../hec_g4_four_torsion_20261118/field.rs"]
mod field;
use field::{scalar, E, F, Z};
fn nums(s: &str) -> Vec<u64> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
fn sqroot(f: &F, a: E, z: E) -> E {
    if a == Z {
        return Z;
    }
    assert_eq!(f.pow(a, (f.q - 1) / 2), scalar(1));
    let mut q = f.q - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1
    }
    let (mut c, mut x, mut t, mut m) = (f.pow(z, q), f.pow(a, (q + 1) / 2), f.pow(a, q), s);
    while t != scalar(1) {
        let mut i = 0;
        let mut tt = t;
        while tt != scalar(1) {
            tt = f.mul(tt, tt);
            i += 1;
            assert!(i < m)
        }
        let b = f.pow(c, 1u64 << (m - i - 1));
        x = f.mul(x, b);
        c = f.mul(b, b);
        t = f.mul(t, c);
        m = i
    }
    assert_eq!(f.mul(x, x), a);
    x.min(f.sub(Z, x))
}
fn sign(f: &F, a: E, m: usize) -> E {
    if m.count_ones() % 2 == 1 {
        f.sub(Z, a)
    } else {
        a
    }
}
fn array(s: &str) -> Vec<E> {
    nums(s)
        .chunks_exact(9)
        .map(|v| v.try_into().unwrap())
        .collect()
}
fn clifford(f: &F, roots: &[E], basis: &[usize], plane: &[field::P]) -> Vec<Vec<E>> {
    let mut z = Z;
    z[1] = 1;
    let mut rows = vec![vec![Z; 16]; 64];
    for (i, &r) in roots.iter().enumerate() {
        let wi = if i == 8 { 255 } else { 1 << i };
        let coord = (0usize..256)
            .find(|&s| {
                basis
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| s & (1 << j) != 0)
                    .fold(0, |a, (_, b)| a ^ b)
                    == wi
            })
            .unwrap();
        let (x, b) = (coord & 15, coord >> 4);
        let derivative = roots
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .fold(scalar(1), |a, (_, &s)| f.mul(a, f.sub(r, s)));
        let weight = sqroot(&f, sign(&f, f.inv(derivative), x & b), z);
        for k in 0..4 {
            for col in 0..16 {
                let entry = &mut rows[k * 16 + (col ^ x)][col];
                *entry = f.add(
                    *entry,
                    sign(&f, f.mul(weight, f.eval(&plane[k], r)), b & col),
                );
            }
        }
    }
    rows
}
#[allow(dead_code)]
#[path = "../hec_g4_four_torsion_20261118/jac.rs"]
mod jac;
use jac::{C, D};
fn plane(c: &C, d: &D) -> Vec<field::P> {
    let f = c.f;
    let m = d.u.len() - 1;
    let v = c.exact(&f.ps(&c.ff, &f.pm(&d.v, &d.v)), &d.u);
    let mut rows = Vec::new();
    for i in 0..4 - m {
        let mut p = vec![Z; i];
        p.extend(&d.u);
        rows.push(p);
    }
    for i in 0..m {
        let mut xv = vec![Z; i];
        xv.extend(&v);
        let mut xr = vec![Z; i];
        xr.extend(&d.v);
        let (_, a) = f.div(&xv, &d.u);
        let (q, _) = f.div(&c.scale(&xr, scalar(29)), &d.u);
        rows.push(f.pa(&f.pa(&c.scale(&xv, scalar(29)), &a), &f.pm(&q, &d.v)));
    }
    assert_eq!(rows.len(), 4);
    rows
}
fn null(f: &F, mut a: Vec<Vec<E>>, n: usize) -> Vec<Vec<E>> {
    let original = a.clone();
    let mut piv = Vec::new();
    for j in 0..n {
        let k = piv.len();
        let Some(i) = (k..a.len()).find(|&i| a[i][j] != Z) else {
            continue;
        };
        a.swap(k, i);
        let iv = f.inv(a[k][j]);
        let row: Vec<_> = a[k].iter().map(|&x| f.mul(x, iv)).collect();
        for (i, r) in a.iter_mut().enumerate() {
            if i == k {
                continue;
            }
            let t = r[j];
            if t == Z {
                continue;
            }
            for l in j..n {
                r[l] = f.sub(r[l], f.mul(t, row[l]));
            }
        }
        a[k] = row;
        piv.push(j);
    }
    (0..n)
        .filter(|j| !piv.contains(j))
        .map(|j| {
            let mut v = vec![Z; n];
            v[j] = scalar(1);
            for (i, &k) in piv.iter().enumerate() {
                v[k] = f.sub(Z, a[i][j]);
            }
            for r in &original {
                assert_eq!(dot(f, r, &v), Z);
            }
            v
        })
        .collect()
}
fn dot(f: &F, x: &[E], y: &[E]) -> E {
    x.iter().zip(y).fold(Z, |a, (&b, &c)| f.add(a, f.mul(b, c)))
}
fn normalize(f: &F, v: &mut [E]) {
    let iv = f.inv(*v.iter().find(|&&x| x != Z).unwrap());
    for x in v {
        *x = f.mul(*x, iv);
    }
}
fn shifted(v: &field::P, i: usize) -> field::P {
    let mut w = vec![Z; i];
    w.extend(v);
    field::trim(w)
}
fn jet(c: &C, d: &D, j: usize) -> Vec<field::P> {
    let f = c.f;
    let u = &d.u;
    let r = &d.v;
    assert_eq!(u.len(), 5);
    let v = c.exact(&f.ps(&c.ff, &f.pm(r, r)), u);
    let h = u[j + 1..].to_vec();
    let du = f.rem(&c.scale(&f.pm(r, &h), scalar(29)), u);
    let dr = f.rem(&f.pm(&v, &h), u);
    let dv = c.exact(
        &f.ps(&c.scale(&f.pm(r, &dr), scalar(29)), &f.pm(&v, &du)),
        u,
    );
    for k in 0..4 {
        let residue = f.rem(&shifted(&h, k), u);
        assert_eq!(*residue.get(3).unwrap_or(&Z), scalar((k == j) as u64));
    }
    (0..4)
        .map(|i| {
            let (p, _) = f.div(&shifted(&v, i), u);
            let (q, _) = f.div(&c.scale(&shifted(r, i), scalar(29)), u);
            let dxv = shifted(&dv, i);
            let da = f.rem(&f.ps(&dxv, &f.pm(&p, &du)), u);
            let (dq, _) = f.div(
                &f.ps(&c.scale(&shifted(&dr, i), scalar(29)), &f.pm(&q, &du)),
                u,
            );
            f.pa(
                &f.pa(&c.scale(&dxv, scalar(29)), &da),
                &f.pa(&f.pm(&dq, r), &f.pm(&q, &dr)),
            )
        })
        .collect()
}
fn fourier(f: &F, v: &[E], b: usize) -> E {
    v.iter()
        .enumerate()
        .fold(Z, |s, (i, &x)| f.add(s, sign(f, x, i & b)))
}
fn gradients(c: &C, roots: &[E], basis: &[usize], d: &D) -> (Vec<E>, Vec<Vec<E>>) {
    let f = c.f;
    let rows = clifford(f, roots, basis, &plane(c, d));
    let ker = null(f, rows.clone(), 16);
    assert_eq!(ker.len(), 1);
    let v = ker[0].clone();
    let fix = v.iter().position(|&x| x != Z).unwrap();
    let grad = (0..4)
        .map(|j| {
            let dm = clifford(f, roots, basis, &jet(c, d, j));
            let mut eq = rows.clone();
            for (r, m) in eq.iter_mut().zip(dm) {
                r.push(dot(f, &m, &v));
            }
            let mut gauge = vec![Z; 17];
            gauge[fix] = scalar(1);
            eq.push(gauge);
            let sol = null(f, eq, 17);
            assert_eq!(sol.len(), 1);
            let scale = f.inv(sol[0][16]);
            sol[0][..16].iter().map(|&x| f.mul(x, scale)).collect()
        })
        .collect();
    (v, grad)
}
use hec_native_ic::{
    instance::pick_instance,
    jac::{Curve, Div},
    rng::Rng,
};
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;
const L: u64 = 239753;
fn inv_scalar(mut x: u64) -> u64 {
    assert!(x != 0);
    let mut r = 1;
    let mut e = L - 2;
    while e != 0 {
        if e % 2 == 1 {
            r = r * x % L;
        }
        x = x * x % L;
        e /= 2;
    }
    r
}
fn lift(p: &Div) -> D {
    D {
        u: field::poly(&p.u.c[..p.u.n]),
        v: field::poly(&p.v.c[..p.v.n]),
    }
}
struct Chart {
    roots: Vec<E>,
    basis: Vec<usize>,
    points: Vec<([u64; 4], u64)>,
    canonical: u64,
}
impl Chart {
    fn new() -> Self {
        let raw = include_str!("../hec_g4_four_torsion_20261118/CONTROL.txt")
            .lines()
            .next()
            .unwrap();
        let roots = array(raw.split("roots=").nth(1).unwrap());
        let basis = nums(
            include_str!("../hec_g4_spinor_marking_20261118/panel/CONTROL.txt")
                .lines()
                .find(|s| s.starts_with("KERNEL id=2 "))
                .unwrap()
                .split("basis=")
                .nth(1)
                .unwrap()
                .split(']')
                .next()
                .unwrap(),
        )
        .into_iter()
        .map(|v| v as usize)
        .collect();
        let inv = nums(
            include_str!("../hec_g4_canonical_quadric_20261118/rulings/CONTROL.txt")
                .lines()
                .next()
                .unwrap()
                .split("inverse_basis=")
                .nth(1)
                .unwrap(),
        );
        let logs = include_str!("../hec_g4_subgroup_calibration_20261118/CONTROL.txt");
        let points = logs
            .lines()
            .filter(|s| s.starts_with("POINT "))
            .map(|s| {
                let v = nums(s);
                let pair = |x| if x == 31 { (0u64, 1u64) } else { (1, x) };
                let (u, t) = pair(v[1]);
                let (w, z) = pair(v[2]);
                let seg = [u * w, t * z, u * z, 30 * t * w];
                let x = std::array::from_fn(|i| {
                    (0..4).map(|k| seg[k] * inv[4 * k + i]).sum::<u64>() % 31
                });
                (x, v[3])
            })
            .collect();
        let canonical = logs
            .lines()
            .last()
            .unwrap()
            .split("canonical_minus_six_base=")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        Self {
            roots,
            basis,
            points,
            canonical,
        }
    }
    fn candidates(&self, c: &C, p: &Div) -> BTreeSet<u64> {
        let mut out = BTreeSet::new();
        if p.u.n != 5 {
            return out;
        }
        let d = lift(p);
        let m = clifford(c.f, &self.roots, &self.basis, &plane(c, &d));
        let v = null(c.f, m, 16);
        assert_eq!(v.len(), 1);
        let hits: Vec<_> = [0, 4, 10, 14]
            .into_iter()
            .filter(|&b| fourier(c.f, &v[0], b) == Z)
            .collect();
        if hits.is_empty() {
            return out;
        }
        let (_, g) = gradients(c, &self.roots, &self.basis, &d);
        for b in hits {
            let mut form: Vec<_> = g.iter().map(|r| fourier(c.f, r, b)).collect();
            if form.iter().all(|&y| y == Z) {
                continue;
            }
            normalize(c.f, &mut form);
            assert!(form.iter().all(|e| e[1..].iter().all(|&a| a == 0)));
            let pts: Vec<_> = self
                .points
                .iter()
                .filter(|(p, _)| (0..4).map(|i| form[i][0] * p[i]).sum::<u64>() % 31 == 0)
                .map(|(_, v)| *v)
                .collect();
            for i in 0..pts.len() {
                for j in 0..i {
                    for k in 0..j {
                        out.insert((2 * (pts[i] + pts[j] + pts[k]) + L - self.canonical) % L);
                    }
                }
            }
        }
        out
    }
}
fn descend(curve: &Curve, d: &Div, target: &Div) -> Option<u64> {
    let f = F::new(31, vec![17, 0, 30, 0, 5, 0, 6, 0, 1]);
    let c = C {
        f: &f,
        ff: field::poly(&[1, 16, 0, 1, 0, 0, 0, 0, 0, 1]),
        g: 4,
    };
    let chart = Chart::new();
    let mut source = curve.zero();
    let mut source_hits = BTreeMap::<u64, Vec<usize>>::new();
    let mut slopes = BTreeSet::new();
    let mut m_stop = 0;
    for m in 1..=512 {
        source = curve.add(&source, d);
        for y in chart.candidates(&c, &source) {
            let u = y * inv_scalar(m as u64) % L;
            if u == 0 {
                continue;
            }
            let h = source_hits.entry(u).or_default();
            h.push(m);
            if h.len() == 2 {
                slopes.insert(u);
            }
        }
        if !slopes.is_empty() {
            m_stop = m;
            break;
        }
    }
    println!("SOURCE stop={m_stop} slopes={slopes:?}");
    if slopes.is_empty() {
        return None;
    }
    let mut anchor = *target;
    let mut seen = BTreeMap::<(u64, u64), Vec<usize>>::new();
    let mut checks = 0;
    for a in 0..512 {
        for y in chart.candidates(&c, &anchor) {
            for &u in &slopes {
                let k = (y * inv_scalar(u) % L + L - a as u64) % L;
                let h = seen.entry((u, k)).or_default();
                h.push(a);
                if h.len() == 2 {
                    checks += 1;
                    if curve.mul(d, k) == *target {
                        println!("VERIFY source_slope={u} source_witness={:?} target_witness={h:?} candidates_checked={checks} source_anchors={m_stop} target_anchors={} k={k}",source_hits[&u],a+1);
                        return Some(k);
                    }
                }
            }
        }
        anchor = curve.add(&anchor, d);
    }
    println!("CENSORED target_anchors=512 candidates_checked={checks}");
    None
}
#[path = "../hec_residual_span_20261116/native-lazy-finisher/finish.rs"]
mod finish;
#[path = "../hec_residual_span_20261116/native-pencil-images/images.rs"]
mod images;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/linear.rs"]
mod linear;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/pencil.rs"]
mod pencil;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/prepared.rs"]
mod prepared;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/public.rs"]
mod public;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/small.rs"]
mod small;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/table_oracle.rs"]
mod table_oracle;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/tail.rs"]
mod tail;
#[path = "../hec_residual_span_20261116/native-deferred-membership/validate.rs"]
mod validate;
fn baseline(c: &Curve, d: &Div, t: &Div, case: usize) -> Option<u64> {
    assert!(validate::eligible(c, d, t, L));
    let prep = prepared::prepare(c, false);
    pencil::solve(
        c,
        d,
        t,
        L,
        512,
        false,
        &prep,
        pencil::Options {
            normalize: true,
            finish: false,
            seed: 2026112200 ^ 31 ^ (4u64 << 32) ^ case as u64,
        },
    )
    .k
}
fn main() {
    hec_native_ic::counted::disable();
    let inst = pick_instance(31, 4).unwrap();
    assert_eq!((inst.c, inst.ell), (16, L));
    let mut rng = Rng::new(2026111800 ^ 31 ^ (4u64 << 32));
    let c = &inst.curve;
    for case in 0..2 {
        let sm = if case == 0 { 1 } else { 17 };
        let d = c.mul(&inst.d1, sm);
        let planted = 1 + rng.below(L - 1);
        let t = c.mul(&d, planted);
        println!("FIXTURE case={case} source_multiple={sm} source={d:?} target={t:?}");
        for arm in if case == 0 { [0, 1] } else { [1, 0] } {
            let start = Instant::now();
            let answer = if arm == 0 {
                baseline(c, &d, &t, case)
            } else {
                descend(c, &d, &t)
            };
            let elapsed = start.elapsed().as_nanos();
            println!("RUN case={case} arm={arm} answer={answer:?} ns={elapsed}");
            if arm == 0 {
                assert_eq!(answer, Some(planted));
            }
            if let Some(k) = answer {
                assert_eq!(k, planted);
                assert_eq!(c.mul(&d, k), t);
            }
        }
    }
}
