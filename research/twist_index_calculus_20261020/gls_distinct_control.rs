//! Post hoc flat control: one representative from each of twelve GLS orbits.
//! Compile with `rustc -D warnings -O gls_distinct_control.rs -o gls_distinct_control`.
//! It reuses the sealed v2 arithmetic and exact oracle without altering v2.
#[allow(dead_code)]
mod original {
// Native exact toy experiment for twist-induced Frobenius orbit folding.
// rustc -O gls_symmetry_ic.rs -o /tmp/gls_symmetry_ic
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct F {
    a: u64,
    b: u64,
}
const Z: F = F { a: 0, b: 0 };
const ONE: F = F { a: 1, b: 0 };
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct P {
    x: F,
    y: F,
    inf: bool,
}
const O: P = P {
    x: Z,
    y: Z,
    inf: true,
};
#[derive(Clone, Copy)]
struct C {
    p: u64,
    d: u64,
    r: u64,
    cof: u64,
    c: F,
    a: F,
    b: F,
    alpha2: F,
    alpha3: F,
}
#[derive(Clone, Default)]
struct Ops {
    add: u64,
    fmul: u64,
    finv: u64,
    psi: u64,
    probes: u64,
    row: u64,
    minv: u64,
    msquare: u64,
}
fn m(x: i128, p: u64) -> u64 {
    x.rem_euclid(p as i128) as u64
}
fn pmul(x: u64, y: u64, p: u64) -> u64 {
    ((x as u128 * y as u128) % p as u128) as u64
}
fn mpow(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n & 1 == 1 {
            z = pmul(z, x, p);
        }
        x = pmul(x, x, p);
        n >>= 1;
    }
    z
}
fn minv(x: u64, p: u64) -> u64 {
    assert_ne!(x, 0);
    mpow(x, p - 2, p)
}
fn prime(n: u64) -> bool {
    n >= 2 && (2..=((n as f64).sqrt() as u64)).all(|v| n % v != 0)
}
fn fa(x: F, y: F, p: u64) -> F {
    F {
        a: (x.a + y.a) % p,
        b: (x.b + y.b) % p,
    }
}
fn fn_eg(x: F, p: u64) -> F {
    F {
        a: m(-(x.a as i128), p),
        b: m(-(x.b as i128), p),
    }
}
fn fs(x: F, y: F, p: u64) -> F {
    fa(x, fn_eg(y, p), p)
}
fn fm(x: F, y: F, p: u64, d: u64, o: &mut Ops) -> F {
    o.fmul += 1;
    F {
        a: m(
            x.a as i128 * y.a as i128 + d as i128 * x.b as i128 * y.b as i128,
            p,
        ),
        b: m(x.a as i128 * y.b as i128 + x.b as i128 * y.a as i128, p),
    }
}
fn fi(x: F, p: u64, d: u64, o: &mut Ops) -> F {
    assert_ne!(x, Z);
    o.finv += 1;
    let norm = m(
        x.a as i128 * x.a as i128 - d as i128 * x.b as i128 * x.b as i128,
        p,
    );
    let z = minv(norm, p);
    F {
        a: pmul(x.a, z, p),
        b: pmul(m(-(x.b as i128), p), z, p),
    }
}
fn fpow(mut x: F, mut n: u64, p: u64, d: u64, o: &mut Ops) -> F {
    let mut z = ONE;
    while n > 0 {
        if n & 1 == 1 {
            z = fm(z, x, p, d, o);
        }
        x = fm(x, x, p, d, o);
        n >>= 1;
    }
    z
}
fn ffrob(x: F, p: u64) -> F {
    F {
        a: x.a,
        b: m(-(x.b as i128), p),
    }
}
fn pn(x: P, p: u64) -> P {
    if x.inf {
        x
    } else {
        P {
            x: x.x,
            y: fn_eg(x.y, p),
            inf: false,
        }
    }
}
fn on(x: P, c: &C, o: &mut Ops) -> bool {
    if x.inf {
        return true;
    }
    let x2 = fm(x.x, x.x, c.p, c.d, o);
    let x3 = fm(x2, x.x, c.p, c.d, o);
    fm(x.y, x.y, c.p, c.d, o) == fa(fa(x3, fm(c.a, x.x, c.p, c.d, o), c.p), c.b, c.p)
}
fn pa(x: P, y: P, c: &C, o: &mut Ops) -> P {
    if x.inf {
        return y;
    }
    if y.inf {
        return x;
    }
    o.add += 1;
    if x.x == y.x && fa(x.y, y.y, c.p) == Z {
        return O;
    }
    let lam = if x == y {
        let xx = fm(x.x, x.x, c.p, c.d, o);
        let num = fa(fm(F { a: 3, b: 0 }, xx, c.p, c.d, o), c.a, c.p);
        fm(
            num,
            fi(fm(F { a: 2, b: 0 }, x.y, c.p, c.d, o), c.p, c.d, o),
            c.p,
            c.d,
            o,
        )
    } else {
        fm(
            fs(y.y, x.y, c.p),
            fi(fs(y.x, x.x, c.p), c.p, c.d, o),
            c.p,
            c.d,
            o,
        )
    };
    let xx = fs(fs(fm(lam, lam, c.p, c.d, o), x.x, c.p), y.x, c.p);
    let yy = fs(fm(lam, fs(x.x, xx, c.p), c.p, c.d, o), x.y, c.p);
    P {
        x: xx,
        y: yy,
        inf: false,
    }
}
fn pm(mut n: u64, mut x: P, c: &C, o: &mut Ops) -> P {
    let mut z = O;
    while n > 0 {
        if n & 1 == 1 {
            z = pa(z, x, c, o);
        }
        n >>= 1;
        if n > 0 {
            x = pa(x, x, c, o);
        }
    }
    z
}
fn psi(x: P, c: &C, o: &mut Ops) -> P {
    o.psi += 1;
    if x.inf {
        return x;
    }
    P {
        x: fm(c.alpha2, ffrob(x.x, c.p), c.p, c.d, o),
        y: fm(c.alpha3, ffrob(x.y, c.p), c.p, c.d, o),
        inf: false,
    }
}
fn make_curve(p: u64, a0: u64, b0: u64, r: u64, cof: u64) -> C {
    assert!(prime(p) && prime(r));
    let d = (2..p).find(|&x| mpow(x, (p - 1) / 2, p) == p - 1).unwrap();
    let mut o = Ops::default();
    let c = (0..p)
        .flat_map(|a| (1..p).map(move |b| F { a, b }))
        .find(|&x| fpow(x, (p * p - 1) / 2, p, d, &mut o) != ONE)
        .unwrap();
    let c2 = fm(c, c, p, d, &mut o);
    let c3 = fm(c2, c, p, d, &mut o);
    let alpha = fpow(c, p * p - 1 - (p - 1) / 2, p, d, &mut o);
    let alpha2 = fm(alpha, alpha, p, d, &mut o);
    let alpha3 = fm(alpha2, alpha, p, d, &mut o);
    C {
        p,
        d,
        r,
        cof,
        c,
        a: fm(F { a: a0, b: 0 }, c2, p, d, &mut o),
        b: fm(F { a: b0, b: 0 }, c3, p, d, &mut o),
        alpha2,
        alpha3,
    }
}
fn enumerate(c: &C, o: &mut Ops) -> Vec<P> {
    let mut roots: HashMap<F, Vec<F>> = HashMap::new();
    for a in 0..c.p {
        for b in 0..c.p {
            let y = F { a, b };
            let y2 = fm(y, y, c.p, c.d, o);
            roots.entry(y2).or_default().push(y);
        }
    }
    let mut out = Vec::new();
    for a in 0..c.p {
        for b in 0..c.p {
            let x = F { a, b };
            let x2 = fm(x, x, c.p, c.d, o);
            let rhs = fa(
                fa(fm(x2, x, c.p, c.d, o), fm(c.a, x, c.p, c.d, o), c.p),
                c.b,
                c.p,
            );
            if let Some(ys) = roots.get(&rhs) {
                for &y in ys {
                    out.push(P { x, y, inf: false });
                }
            }
        }
    }
    out.sort();
    out
}
fn generator(c: &C, points: &[P], o: &mut Ops) -> P {
    for &q in points {
        let g = pm(c.cof, q, c, o);
        if !g.inf && pm(c.r, g, c, o) == O {
            return g;
        }
    }
    panic!("no selected subgroup generator")
}
fn lambda(c: &C, g: P, o: &mut Ops) -> u64 {
    let pg = psi(g, c, o);
    for x in 1..c.r {
        o.msquare += 1;
        if pmul(x, x, c.r) == c.r - 1 && pm(x, g, c, o) == pg {
            return x;
        }
    }
    panic!("psi has no scalar action on selected subgroup")
}
fn orbit_base(c: &C, points: &[P], b: usize, o: &mut Ops) -> Vec<P> {
    let mut got = HashSet::new();
    let mut out = Vec::new();
    for &q in points {
        let p = pm(c.cof, q, c, o);
        if p.inf {
            continue;
        }
        let pp = psi(p, c, o);
        let orbit = [p, pp, pn(p, c.p), pn(pp, c.p)];
        if orbit.iter().any(|x| got.contains(x)) {
            continue;
        }
        for x in orbit {
            got.insert(x);
        }
        out.push(p);
        out.push(pp);
        if out.len() == b {
            break;
        }
    }
    assert_eq!(out.len(), b);
    out
}
// Same lexicographic base as orbit_base(enumerate(...)), stopping the x scan
// once all requested orbits have been found. The square table remains charged.
fn first_orbit_base(c: &C, b: usize, o: &mut Ops) -> Vec<P> {
    let mut roots: HashMap<F, Vec<F>> = HashMap::new();
    for a in 0..c.p {
        for bb in 0..c.p {
            let y = F { a, b: bb };
            let y2 = fm(y, y, c.p, c.d, o);
            roots.entry(y2).or_default().push(y);
        }
    }
    let mut got = HashSet::new();
    let mut out = Vec::new();
    for a in 0..c.p {
        for bb in 0..c.p {
            let x = F { a, b: bb };
            let x2 = fm(x, x, c.p, c.d, o);
            let rhs = fa(
                fa(fm(x2, x, c.p, c.d, o), fm(c.a, x, c.p, c.d, o), c.p),
                c.b,
                c.p,
            );
            if let Some(ys) = roots.get(&rhs) {
                for &y in ys {
                    let q = P { x, y, inf: false };
                    let p = pm(c.cof, q, c, o);
                    if p.inf {
                        continue;
                    }
                    let pp = psi(p, c, o);
                    let orbit = [p, pp, pn(p, c.p), pn(pp, c.p)];
                    if orbit.iter().any(|z| got.contains(z)) {
                        continue;
                    }
                    for z in orbit {
                        got.insert(z);
                    }
                    out.push(p);
                    out.push(pp);
                    if out.len() == b {
                        return out;
                    }
                }
            }
        }
    }
    panic!("insufficient orbit points")
}
#[derive(Clone, Copy)]
struct S {
    q: P,
    i: usize,
    sign: i128,
}
fn signed(base: &[P], c: &C) -> Vec<S> {
    let mut out = Vec::new();
    for (i, &q) in base.iter().enumerate() {
        out.push(S { q, i, sign: 1 });
        out.push(S {
            q: pn(q, c.p),
            i,
            sign: -1,
        });
    }
    out
}
#[derive(Clone, Copy)]
struct Pair {
    a: usize,
    b: usize,
}
struct Table {
    map: HashMap<P, Vec<Pair>>,
    entries: usize,
}
fn table(ss: &[S], c: &C, o: &mut Ops) -> Table {
    let mut map: HashMap<P, Vec<Pair>> = HashMap::new();
    let mut entries = 0;
    for a in 0..ss.len() {
        for b in (a + 1)..ss.len() {
            if ss[a].i == ss[b].i {
                continue;
            }
            let sum = pa(ss[a].q, ss[b].q, c, o);
            map.entry(sum).or_default().push(Pair { a, b });
            entries += 1;
        }
    }
    Table { map, entries }
}
fn modulus(x: i128, r: u64) -> u64 {
    m(x, r)
}
fn row(ids: [S; 3], b: usize, fold: bool, lambda: u64, r: u64) -> Vec<u64> {
    let n = if fold { b / 2 } else { b };
    let mut out = vec![0; n];
    for s in ids {
        let j = if fold { s.i / 2 } else { s.i };
        let weight = if fold && s.i % 2 == 1 { lambda } else { 1 };
        out[j] = modulus(out[j] as i128 + s.sign * weight as i128, r);
    }
    out
}
fn hits(q: P, ss: &[S], tab: &Table, c: &C, o: &mut Ops) -> Vec<[S; 3]> {
    let mut out = Vec::new();
    for &s in ss {
        let need = pa(q, pn(s.q, c.p), c, o);
        o.probes += 1;
        if let Some(pairs) = tab.map.get(&need) {
            for &ab in pairs {
                let x = ss[ab.a];
                let y = ss[ab.b];
                if x.i < y.i && y.i < s.i {
                    out.push([x, y, s]);
                }
            }
        }
    }
    out
}
struct Basis {
    rows: Vec<Option<Vec<u64>>>,
    rank: usize,
    r: u64,
}
impl Basis {
    fn new(n: usize, r: u64) -> Self {
        Self {
            rows: vec![None; n],
            rank: 0,
            r,
        }
    }
    fn insert(&mut self, coeff: Vec<u64>, rhs: u64, o: &mut Ops) -> bool {
        let n = self.rows.len();
        assert_eq!(coeff.len(), n);
        let mut v = coeff;
        v.push(rhs);
        for j in 0..n {
            if v[j] == 0 {
                continue;
            }
            if let Some(pivot) = &self.rows[j] {
                let f = v[j];
                for k in j..=n {
                    v[k] = modulus(v[k] as i128 - f as i128 * pivot[k] as i128, self.r);
                    o.row += 1;
                }
            } else {
                o.minv += 1;
                let inv = minv(v[j], self.r);
                for k in j..=n {
                    v[k] = pmul(v[k], inv, self.r);
                    o.row += 1;
                }
                self.rows[j] = Some(v);
                self.rank += 1;
                return true;
            }
        }
        assert_eq!(v[n], 0, "inconsistent exact group relation");
        false
    }
    fn solve(&self, o: &mut Ops) -> Vec<u64> {
        assert_eq!(self.rank, self.rows.len());
        let n = self.rows.len();
        let mut x = vec![0; n];
        for j in (0..n).rev() {
            let row = self.rows[j].as_ref().unwrap();
            let mut v = row[n];
            for k in j + 1..n {
                v = modulus(v as i128 - row[k] as i128 * x[k] as i128, self.r);
                o.row += 1;
            }
            x[j] = v;
        }
        x
    }
}
fn scalar_of_row(coeff: &[u64], logs: &[u64], r: u64, o: &mut Ops) -> u64 {
    assert_eq!(coeff.len(), logs.len());
    let mut z = 0u64;
    for (&a, &b) in coeff.iter().zip(logs) {
        z = modulus(z as i128 + a as i128 * b as i128, r);
        o.row += 1;
    }
    z
}
fn validate_base(c: &C, g: P, base: &[P], lambda: u64, o: &mut Ops) -> bool {
    if base.len() % 2 != 0 {
        return false;
    }
    for pair in base.chunks_exact(2) {
        if pair[0].inf || pair[1].inf || !on(pair[0], c, o) || !on(pair[1], c, o) {
            return false;
        }
        if pm(c.r, pair[0], c, o) != O || pm(c.r, pair[1], c, o) != O {
            return false;
        }
        if psi(pair[0], c, o) != pair[1] {
            return false;
        }
        if pm(lambda, pair[0], c, o) != pair[1] {
            return false;
        }
        if psi(pair[1], c, o) != pn(pair[0], c.p) {
            return false;
        }
    }
    pm(lambda, g, c, o) == psi(g, c, o)
}
fn validate_geometry(c: &C, base: &[P], o: &mut Ops) -> bool {
    if base.len() % 2 != 0 {
        return false;
    }
    for pair in base.chunks_exact(2) {
        if pair[0].inf || pair[1].inf || !on(pair[0], c, o) || !on(pair[1], c, o) {
            return false;
        }
        if pm(c.r, pair[0], c, o) != O || pm(c.r, pair[1], c, o) != O {
            return false;
        }
        if psi(pair[0], c, o) != pair[1] || psi(pair[1], c, o) != pn(pair[0], c.p) {
            return false;
        }
    }
    true
}
fn census(c: &C, g: P, base: &[P], lambda: u64) {
    let b = base.len();
    let mut o = Ops::default();
    let mut logs = HashMap::new();
    let mut q = O;
    for k in 1..c.r {
        q = pa(q, g, c, &mut o);
        assert!(logs.insert(q, k).is_none());
    }
    assert_eq!(pa(q, g, c, &mut o), O);
    let mut covered = HashSet::new();
    let mut unique_flat = HashSet::new();
    let mut unique_fold = HashSet::new();
    let mut flat = Basis::new(b, c.r);
    let mut fold = Basis::new(b / 2, c.r);
    let mut raw = 0u64;
    let mut zero = 0u64;
    for i in 0..b {
        for j in i + 1..b {
            for k in j + 1..b {
                for si in [1i128, -1] {
                    for sj in [1i128, -1] {
                        for sk in [1i128, -1] {
                            let xi = S {
                                q: if si == 1 { base[i] } else { pn(base[i], c.p) },
                                i,
                                sign: si,
                            };
                            let xj = S {
                                q: if sj == 1 { base[j] } else { pn(base[j], c.p) },
                                i: j,
                                sign: sj,
                            };
                            let xk = S {
                                q: if sk == 1 { base[k] } else { pn(base[k], c.p) },
                                i: k,
                                sign: sk,
                            };
                            let sum = pa(pa(xi.q, xj.q, c, &mut o), xk.q, c, &mut o);
                            if sum.inf {
                                zero += 1;
                                continue;
                            }
                            let rhs = *logs.get(&sum).expect("triple not in selected subgroup");
                            raw += 1;
                            covered.insert(rhs);
                            let rf = row([xi, xj, xk], b, false, lambda, c.r);
                            let rq = row([xi, xj, xk], b, true, lambda, c.r);
                            if unique_flat.insert(rf.clone()) {
                                flat.insert(rf, rhs, &mut o);
                            }
                            if unique_fold.insert(rq.clone()) {
                                fold.insert(rq, rhs, &mut o);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{{\"kind\":\"census\",\"p\":{},\"r\":{},\"b\":{},\"targets\":{},\"covered\":{},\"raw_hits\":{},\"zero_sums\":{},\"flat_rows\":{},\"folded_rows\":{},\"flat_rank\":{},\"folded_rank\":{},\"flat_columns\":{},\"folded_columns\":{}}}",
       c.p,c.r,b,c.r-1,covered.len(),raw,zero,unique_flat.len(),unique_fold.len(),flat.rank,fold.rank,b,b/2);
}
fn scalar_json(v: Option<u64>) -> String {
    v.map(|x| x.to_string())
        .unwrap_or_else(|| "null".to_string())
}
fn solve(c: &C, g: P, expected: &[P], b: usize, seed: usize, target: P, known: u64, fold: bool) {
    let started = Instant::now();
    let mut base_ops = Ops::default();
    let st = Instant::now();
    let base = first_orbit_base(c, b, &mut base_ops);
    assert_eq!(base, expected);
    let eigen = if fold { lambda(c, g, &mut base_ops) } else { 0 };
    assert!(validate_geometry(c, &base, &mut base_ops));
    let base_ns = st.elapsed().as_nanos();
    let mut tab_ops = Ops::default();
    let st = Instant::now();
    let ss = signed(&base, c);
    let tab = table(&ss, c, &mut tab_ops);
    let table_ns = st.elapsed().as_nanos();
    let n = if fold { b / 2 } else { b };
    let mut basis = Basis::new(n, c.r);
    let mut relation_ops = Ops::default();
    let st = Instant::now();
    let mut q = O;
    let mut trials = 0u64;
    let mut raw_hits = 0u64;
    let mut seen = HashSet::new();
    let mut novel = 0u64;
    for a in 1..c.r {
        q = pa(q, g, c, &mut relation_ops);
        trials += 1;
        let rr = hits(q, &ss, &tab, c, &mut relation_ops);
        raw_hits += rr.len() as u64;
        for ids in rr {
            let v = row(ids, b, fold, eigen, c.r);
            if seen.insert(v.clone()) {
                if basis.insert(v, a, &mut relation_ops) {
                    novel += 1;
                }
            }
        }
        if basis.rank == n {
            break;
        }
    }
    let collect_ns = st.elapsed().as_nanos();
    let mut la_ops = Ops::default();
    let st = Instant::now();
    let logs = if basis.rank == n {
        Some(basis.solve(&mut la_ops))
    } else {
        None
    };
    let linear_ns = st.elapsed().as_nanos();
    let mut online_ops = Ops::default();
    let mut target_query_ns = 0u128;
    let mut target_pdp_ns = 0u128;
    let mut target_descent_ns = 0u128;
    let mut target_recovery_check_ns = 0u128;
    let mut online_queries = 0u64;
    let mut recovered = None;
    if let Some(logs) = logs {
        let mut qq = target;
        for shift in 0..c.r {
            online_queries += 1;
            let st = Instant::now();
            let rr = hits(qq, &ss, &tab, c, &mut online_ops);
            target_pdp_ns += st.elapsed().as_nanos();
            if let Some(ids) = rr.first().copied() {
                let st = Instant::now();
                let v = row(ids, b, fold, eigen, c.r);
                let k = modulus(
                    scalar_of_row(&v, &logs, c.r, &mut online_ops) as i128 - shift as i128,
                    c.r,
                );
                target_descent_ns += st.elapsed().as_nanos();
                let st = Instant::now();
                if pm(k, g, c, &mut online_ops) == target {
                    recovered = Some(k);
                }
                target_recovery_check_ns += st.elapsed().as_nanos();
                break;
            }
            let st = Instant::now();
            qq = pa(qq, g, c, &mut online_ops);
            target_query_ns += st.elapsed().as_nanos();
        }
    }
    let online_ns = target_query_ns + target_pdp_ns + target_descent_ns + target_recovery_check_ns;
    let total_ns = started.elapsed().as_nanos();
    assert_eq!(recovered, Some(known), "one-target IC scalar failed");
    let total_add = base_ops.add + tab_ops.add + relation_ops.add + la_ops.add + online_ops.add;
    let total_fmul =
        base_ops.fmul + tab_ops.fmul + relation_ops.fmul + la_ops.fmul + online_ops.fmul;
    let total_finv =
        base_ops.finv + tab_ops.finv + relation_ops.finv + la_ops.finv + online_ops.finv;
    let total_psi = base_ops.psi + tab_ops.psi + relation_ops.psi + la_ops.psi + online_ops.psi;
    let total_probes =
        base_ops.probes + tab_ops.probes + relation_ops.probes + la_ops.probes + online_ops.probes;
    let total_row = base_ops.row + tab_ops.row + relation_ops.row + la_ops.row + online_ops.row;
    let total_minv =
        base_ops.minv + tab_ops.minv + relation_ops.minv + la_ops.minv + online_ops.minv;
    let total_msquare = base_ops.msquare
        + tab_ops.msquare
        + relation_ops.msquare
        + la_ops.msquare
        + online_ops.msquare;
    let overhead_ns =
        total_ns.saturating_sub(base_ns + table_ns + collect_ns + linear_ns + online_ns);
    println!("{{\"kind\":\"solve\",\"p\":{},\"r\":{},\"b\":{},\"seed\":{},\"target\":[{},{},{},{}],\"arm\":\"{}\",\"lambda\":{},\"columns\":{},\"rank\":{},\"trials\":{},\"raw_hits\":{},\"unique_rows\":{},\"novel_rows\":{},\"table_entries\":{},\"online_queries\":{},\"recovered\":{},\"correct\":{},\"base_add\":{},\"table_add\":{},\"relation_add\":{},\"online_add\":{},\"group_add\":{},\"f2_mul\":{},\"f2_inv\":{},\"psi_eval\":{},\"probes\":{},\"row_ops\":{},\"mod_inv\":{},\"mod_square_tests\":{},\"base_ns\":{},\"table_ns\":{},\"collect_ns\":{},\"linear_ns\":{},\"target_query_ns\":{},\"target_pdp_ns\":{},\"target_descent_ns\":{},\"target_recovery_check_ns\":{},\"online_ns\":{},\"overhead_ns\":{},\"total_ns\":{}}}",
       c.p,c.r,b,seed,target.x.a,target.x.b,target.y.a,target.y.b,
       if fold{"psi-folded"}else{"signed-flat"},scalar_json(if fold{Some(eigen)}else{None}),
       n,basis.rank,trials,raw_hits,seen.len(),novel,tab.entries,online_queries,
       scalar_json(recovered),recovered==Some(known),base_ops.add,tab_ops.add,relation_ops.add,
       online_ops.add,total_add,total_fmul,total_finv,total_psi,total_probes,total_row,total_minv,
       total_msquare,base_ns,table_ns,collect_ns,linear_ns,target_query_ns,target_pdp_ns,
       target_descent_ns,target_recovery_check_ns,online_ns,overhead_ns,total_ns);
}
fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
fn rho(c: &C, g: P, target: P, known: u64, b: usize, seed: usize) {
    let started = Instant::now();
    let mut o = Ops::default();
    let mut found = None;
    let mut steps = 0u64;
    let mut canonical = 0u64;
    let mut collisions = 0u64;
    let mut restarts = 0u64;
    'again: for restart in 0..64u64 {
        restarts = restart;
        let mut seen = HashMap::<P, (u64, u64)>::new();
        let salt = mix(c.p ^ ((b as u64) << 20) ^ seed as u64 ^ restart);
        let jumps: Vec<(P, u64, u64)> = (0..16u64)
            .map(|j| {
                let a = 1 + mix(salt ^ j ^ 0xa11ce) % (c.r - 1);
                let d = 1 + mix(salt ^ j ^ 0xbadc0de) % (c.r - 1);
                (
                    pa(pm(a, g, c, &mut o), pm(d, target, c, &mut o), c, &mut o),
                    a,
                    d,
                )
            })
            .collect();
        let mut a = mix(salt ^ 0x1234) % c.r;
        let mut d = mix(salt ^ 0x5678) % c.r;
        let mut x = pa(pm(a, g, c, &mut o), pm(d, target, c, &mut o), c, &mut o);
        for _ in 0..(8 * c.r) {
            let neg = pn(x, c.p);
            canonical += 1;
            if neg < x {
                x = neg;
                a = modulus(-(a as i128), c.r);
                d = modulus(-(d as i128), c.r);
            }
            steps += 1;
            o.probes += 1;
            if let Some(&(aa, dd)) = seen.get(&x) {
                collisions += 1;
                let den = modulus(d as i128 - dd as i128, c.r);
                if den != 0 {
                    o.minv += 1;
                    let k = modulus((aa as i128 - a as i128) * minv(den, c.r) as i128, c.r);
                    if pm(k, g, c, &mut o) == target {
                        found = Some(k);
                        break 'again;
                    }
                }
                break;
            }
            seen.insert(x, (a, d));
            let idx =
                (mix(x.x.a ^ (x.x.b << 16) ^ (x.y.a << 32) ^ (x.y.b << 48) ^ salt) & 15) as usize;
            let (jump, ja, jd) = jumps[idx];
            x = pa(x, jump, c, &mut o);
            a = (a + ja) % c.r;
            d = (d + jd) % c.r;
        }
    }
    assert_eq!(found, Some(known), "rho scalar failed");
    println!("{{\"kind\":\"rho\",\"p\":{},\"r\":{},\"b\":{},\"seed\":{},\"target\":[{},{},{},{}],\"arm\":\"rho-negation\",\"steps\":{},\"canonicalisations\":{},\"collisions\":{},\"restarts\":{},\"recovered\":{},\"correct\":true,\"group_add\":{},\"f2_mul\":{},\"f2_inv\":{},\"probes\":{},\"mod_inv\":{},\"total_ns\":{}}}",
       c.p,c.r,b,seed,target.x.a,target.x.b,target.y.a,target.y.b,steps,canonical,
       collisions,restarts,found.unwrap(),o.add,o.fmul,o.finv,o.probes,o.minv,started.elapsed().as_nanos());
}
fn run() {
    let panel = [
        (101, 3, 17, 401, 25),
        (211, 1, 1, 44221, 1),
        (307, 1, 6, 509, 185),
    ];
    for (p, a, b0, r, cof) in panel {
        let c = make_curve(p, a, b0, r, cof);
        let mut o = Ops::default();
        let points = enumerate(&c, &mut o);
        let order = points.len() as u64 + 1;
        assert_eq!(order, r * cof);
        let g = generator(&c, &points, &mut o);
        assert!(on(g, &c, &mut o));
        let eig = lambda(&c, g, &mut o);
        assert_eq!(pmul(eig, eig, r), r - 1);
        assert_eq!(psi(psi(g, &c, &mut o), &c, &mut o), pn(g, p));
        println!("{{\"kind\":\"curve\",\"p\":{},\"a\":{},\"b_curve\":{},\"d\":{},\"c\":[{},{}],\"r\":{},\"cofactor\":{},\"order\":{},\"generator\":[{},{},{},{}],\"lambda\":{},\"lambda_other\":{}}}",
           p,a,b0,c.d,c.c.a,c.c.b,r,cof,order,g.x.a,g.x.b,g.y.a,g.y.b,eig,r-eig);
        for b in [12usize, 24, 36] {
            let base = orbit_base(&c, &points, b, &mut o);
            assert!(validate_base(&c, g, &base, eig, &mut o));
            census(&c, g, &base, eig);
            let mut fake = base.clone();
            fake[1] = g;
            if fake[1] == base[1] {
                fake[1] = pm(2, g, &c, &mut o);
            }
            assert!(!validate_base(&c, g, &fake, eig, &mut o));
            println!("{{\"kind\":\"negative_control\",\"p\":{},\"b\":{},\"corrupted_orbit_rejected\":true}}",p,b);
            for (seed, &k) in [37u64, 113, 271].iter().enumerate() {
                let target = pm(k, g, &c, &mut o);
                solve(&c, g, &base, b, seed, target, k, false);
                solve(&c, g, &base, b, seed, target, k, true);
                rho(&c, g, target, k, b, seed);
            }
        }
    }
}
fn main() {
    run();
}


    fn distinct_census(c: &C, g: P, base: &[P]) {
        assert_eq!(base.len(), 12);
        let mut ops = Ops::default();
        let mut logs = HashMap::new();
        let mut q = O;
        for k in 1..c.r {
            q = pa(q, g, c, &mut ops);
            assert!(logs.insert(q, k).is_none());
        }
        assert_eq!(pa(q, g, c, &mut ops), O);
        let mut covered = HashSet::new();
        let mut rows = HashSet::new();
        let mut basis = Basis::new(12, c.r);
        let mut hits_count = 0u64;
        let mut zero = 0u64;
        for i in 0..12 {
            for j in i + 1..12 {
                for k in j + 1..12 {
                    for si in [1i128, -1] {
                        for sj in [1i128, -1] {
                            for sk in [1i128, -1] {
                                let factors = [(i, si), (j, sj), (k, sk)].map(|(idx, sign)| S {
                                    q: if sign == 1 { base[idx] } else { pn(base[idx], c.p) },
                                    i: idx,
                                    sign,
                                });
                                let sum = pa(pa(factors[0].q, factors[1].q, c, &mut ops), factors[2].q, c, &mut ops);
                                if sum.inf {
                                    zero += 1;
                                    continue;
                                }
                                hits_count += 1;
                                covered.insert(sum);
                                let rhs = *logs.get(&sum).expect("sum outside selected subgroup");
                                let coefficients = row(factors, 12, false, 0, c.r);
                                if rows.insert(coefficients.clone()) {
                                    basis.insert(coefficients, rhs, &mut ops);
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("{{\"kind\":\"distinct_census\",\"p\":{},\"r\":{},\"b\":12,\"columns\":12,\"targets\":{},\"covered\":{},\"raw_hits\":{},\"zero_sums\":{},\"unique_rows\":{},\"rank\":{}}}",
            c.p,c.r,c.r-1,covered.len(),hits_count,zero,rows.len(),basis.rank);
    }

    fn distinct_solve(c: &C, g: P, expected: &[P], seed: usize, target: P, known: u64) {
        let started = Instant::now();
        let mut base_ops = Ops::default();
        let st = Instant::now();
        let full = first_orbit_base(c, 24, &mut base_ops);
        assert_eq!(full, expected);
        assert!(validate_geometry(c, &full, &mut base_ops));
        let base: Vec<P> = full.iter().step_by(2).copied().collect();
        let base_ns = st.elapsed().as_nanos();

        let mut table_ops = Ops::default();
        let st = Instant::now();
        let ss = signed(&base, c);
        let tab = table(&ss, c, &mut table_ops);
        let table_ns = st.elapsed().as_nanos();

        let mut basis = Basis::new(12, c.r);
        let mut relation_ops = Ops::default();
        let st = Instant::now();
        let mut q = O;
        let mut trials = 0u64;
        let mut raw_hits = 0u64;
        let mut seen = HashSet::new();
        for a in 1..c.r {
            q = pa(q, g, c, &mut relation_ops);
            trials += 1;
            let relations = hits(q, &ss, &tab, c, &mut relation_ops);
            raw_hits += relations.len() as u64;
            for factors in relations {
                let v = row(factors, 12, false, 0, c.r);
                if seen.insert(v.clone()) {
                    basis.insert(v, a, &mut relation_ops);
                }
            }
            if basis.rank == 12 { break; }
        }
        let collect_ns = st.elapsed().as_nanos();
        assert_eq!(basis.rank, 12, "distinct baseline failed full rank");

        let mut la_ops = Ops::default();
        let st = Instant::now();
        let logs = basis.solve(&mut la_ops);
        let linear_ns = st.elapsed().as_nanos();
        let mut online_ops = Ops::default();
        let st = Instant::now();
        let mut qq = target;
        let mut online_queries = 0u64;
        let mut recovered = None;
        for shift in 0..c.r {
            online_queries += 1;
            if let Some(factors) = hits(qq, &ss, &tab, c, &mut online_ops).first().copied() {
                let v = row(factors, 12, false, 0, c.r);
                let k = modulus(
                    scalar_of_row(&v, &logs, c.r, &mut online_ops) as i128 - shift as i128,
                    c.r,
                );
                if pm(k, g, c, &mut online_ops) == target { recovered = Some(k); }
                break;
            }
            qq = pa(qq, g, c, &mut online_ops);
        }
        let online_ns = st.elapsed().as_nanos();
        assert_eq!(recovered, Some(known), "distinct baseline scalar failed");

        let parts = [&base_ops, &table_ops, &relation_ops, &la_ops, &online_ops];
        let total = |select: fn(&Ops) -> u64| parts.iter().map(|x| select(x)).sum::<u64>();
        println!("{{\"kind\":\"distinct_solve\",\"p\":{},\"r\":{},\"b\":12,\"seed\":{},\"target\":[{},{},{},{}],\"columns\":12,\"rank\":12,\"trials\":{},\"raw_hits\":{},\"unique_rows\":{},\"table_entries\":{},\"online_queries\":{},\"recovered\":{},\"correct\":true,\"group_add\":{},\"f2_mul\":{},\"f2_inv\":{},\"psi_eval\":{},\"probes\":{},\"row_ops\":{},\"mod_inv\":{},\"mod_square_tests\":{},\"base_ns\":{},\"table_ns\":{},\"collect_ns\":{},\"linear_ns\":{},\"online_ns\":{},\"total_ns\":{}}}",
            c.p,c.r,seed,target.x.a,target.x.b,target.y.a,target.y.b,
            trials,raw_hits,seen.len(),tab.entries,online_queries,recovered.unwrap(),
            total(|x| x.add),total(|x| x.fmul),total(|x| x.finv),total(|x| x.psi),
            total(|x| x.probes),total(|x| x.row),total(|x| x.minv),total(|x| x.msquare),
            base_ns,table_ns,collect_ns,linear_ns,online_ns,started.elapsed().as_nanos());
    }

    pub fn run_distinct() {
        for (p, a, b0, r, cof) in [(101, 3, 17, 401, 25), (211, 1, 1, 44221, 1), (307, 1, 6, 509, 185)] {
            let c = make_curve(p, a, b0, r, cof);
            let mut ops = Ops::default();
            let points = enumerate(&c, &mut ops);
            assert_eq!(points.len() as u64 + 1, r * cof);
            let g = generator(&c, &points, &mut ops);
            let full = orbit_base(&c, &points, 24, &mut ops);
            let eigen = lambda(&c, g, &mut ops);
            assert!(validate_base(&c, g, &full, eigen, &mut ops));
            let base: Vec<P> = full.iter().step_by(2).copied().collect();
            distinct_census(&c, g, &base);
            for (seed, &k) in [37u64, 113, 271].iter().enumerate() {
                let target = pm(k, g, &c, &mut ops);
                distinct_solve(&c, g, &full, seed, target, k);
            }
        }
    }
}

fn main() { original::run_distinct(); }
