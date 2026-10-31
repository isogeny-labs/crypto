//! Exact mixed-component E(Fp²) index-calculus control.
//! Compile: rustc -O -C debuginfo=0 mixed_extension_ic.rs -o mixed_extension_ic
//! No external crates or planted logarithms enter the solver.
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pt {
    x: u64,
    y: u64,
    inf: bool,
}
const O: Pt = Pt {
    x: 0,
    y: 0,
    inf: true,
};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pair {
    e: Pt,
    t: Pt,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct F2 {
    a: u64,
    b: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct P2 {
    x: F2,
    y: F2,
    inf: bool,
}
const Z2: F2 = F2 { a: 0, b: 0 };
const O2: P2 = P2 {
    x: Z2,
    y: Z2,
    inf: true,
};
#[derive(Clone, Copy)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
    d: u64,
    r: u64,
    nt: u64,
}

#[derive(Default, Clone, Copy)]
struct Ops {
    e_add: u64,
    t_add: u64,
    e_mul: u64,
    t_mul: u64,
    probes: u64,
    hits: u64,
    row_ops: u64,
    field_inverses: u64,
    modular_inverses: u64,
}
impl Ops {
    fn plus(&mut self, rhs: Ops) {
        self.e_add += rhs.e_add;
        self.t_add += rhs.t_add;
        self.e_mul += rhs.e_mul;
        self.t_mul += rhs.t_mul;
        self.probes += rhs.probes;
        self.hits += rhs.hits;
        self.row_ops += rhs.row_ops;
        self.field_inverses += rhs.field_inverses;
        self.modular_inverses += rhs.modular_inverses;
    }
}
fn m(v: i128, p: u64) -> u64 {
    v.rem_euclid(p as i128) as u64
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n & 1 == 1 {
            z = (z as u128 * a as u128 % p as u128) as u64;
        }
        a = (a as u128 * a as u128 % p as u128) as u64;
        n >>= 1;
    }
    z
}
fn inv(a: u64, p: u64) -> u64 {
    assert!(a != 0);
    pow(a, p - 2, p)
}
fn prime(n: u64) -> bool {
    n >= 2 && (2..=((n as f64).sqrt() as u64)).all(|i| n % i != 0)
}
fn rhs(c: Curve, x: u64) -> u64 {
    m(
        x as i128 * x as i128 * x as i128 + c.a as i128 * x as i128 + c.b as i128,
        c.p,
    )
}
fn neg(q: Pt, c: Curve) -> Pt {
    if q.inf {
        q
    } else {
        Pt {
            x: q.x,
            y: m(-(q.y as i128), c.p),
            inf: false,
        }
    }
}
fn add(q: Pt, s: Pt, c: Curve, d: u64, ops: &mut Ops) -> Pt {
    if q.inf {
        return s;
    }
    if s.inf {
        return q;
    }
    if d == 1 {
        ops.e_add += 1;
    } else {
        ops.t_add += 1;
    }
    if q.x == s.x && (q.y + s.y) % c.p == 0 {
        return O;
    }
    ops.field_inverses += 1;
    let lam = if q == s {
        m(
            (3 * q.x * q.x + c.a) as i128 * inv(2 * d % c.p * q.y % c.p, c.p) as i128,
            c.p,
        )
    } else {
        m(
            (s.y as i128 - q.y as i128) * inv(m(s.x as i128 - q.x as i128, c.p), c.p) as i128,
            c.p,
        )
    };
    let x = m(
        d as i128 * lam as i128 * lam as i128 - q.x as i128 - s.x as i128,
        c.p,
    );
    let y = m(lam as i128 * (q.x as i128 - x as i128) - q.y as i128, c.p);
    let out = Pt { x, y, inf: false };
    debug_assert_eq!(d * y * y % c.p, rhs(c, x));
    out
}
fn mul(mut n: u64, mut q: Pt, c: Curve, d: u64, ops: &mut Ops) -> Pt {
    if d == 1 {
        ops.e_mul += 1
    } else {
        ops.t_mul += 1
    }
    let mut z = O;
    while n > 0 {
        if n & 1 == 1 {
            z = add(z, q, c, d, ops)
        }
        n >>= 1;
        if n > 0 {
            q = add(q, q, c, d, ops)
        }
    }
    z
}
fn sum(q: Pair, s: Pair, c: Curve, ops: &mut Ops) -> Pair {
    Pair {
        e: add(q.e, s.e, c, 1, ops),
        t: add(q.t, s.t, c, c.d, ops),
    }
}
fn minus(q: Pair, c: Curve) -> Pair {
    Pair {
        e: neg(q.e, c),
        t: neg(q.t, c),
    }
}
fn fadd(x: F2, y: F2, c: Curve) -> F2 {
    F2 {
        a: (x.a + y.a) % c.p,
        b: (x.b + y.b) % c.p,
    }
}
fn fneg(x: F2, c: Curve) -> F2 {
    F2 {
        a: m(-(x.a as i128), c.p),
        b: m(-(x.b as i128), c.p),
    }
}
fn fsub(x: F2, y: F2, c: Curve) -> F2 {
    fadd(x, fneg(y, c), c)
}
fn fmul(x: F2, y: F2, c: Curve) -> F2 {
    F2 {
        a: m(
            x.a as i128 * y.a as i128 + c.d as i128 * x.b as i128 * y.b as i128,
            c.p,
        ),
        b: m(x.a as i128 * y.b as i128 + x.b as i128 * y.a as i128, c.p),
    }
}
fn finv(x: F2, c: Curve) -> F2 {
    let norm = m(
        x.a as i128 * x.a as i128 - c.d as i128 * x.b as i128 * x.b as i128,
        c.p,
    );
    let z = inv(norm, c.p);
    F2 {
        a: x.a * z % c.p,
        b: m(-(x.b as i128), c.p) * z % c.p,
    }
}
fn p2neg(q: P2, c: Curve) -> P2 {
    if q.inf {
        q
    } else {
        P2 {
            x: q.x,
            y: fneg(q.y, c),
            inf: false,
        }
    }
}
fn p2add(q: P2, s: P2, c: Curve) -> P2 {
    if q.inf {
        return s;
    }
    if s.inf {
        return q;
    }
    if q.x == s.x && fadd(q.y, s.y, c) == Z2 {
        return O2;
    }
    let lam = if q == s {
        let x2 = fmul(q.x, q.x, c);
        fmul(
            fadd(fmul(F2 { a: 3, b: 0 }, x2, c), F2 { a: c.a, b: 0 }, c),
            finv(fmul(F2 { a: 2, b: 0 }, q.y, c), c),
            c,
        )
    } else {
        fmul(fsub(s.y, q.y, c), finv(fsub(s.x, q.x, c), c), c)
    };
    let x = fsub(fsub(fmul(lam, lam, c), q.x, c), s.x, c);
    let y = fsub(fmul(lam, fsub(q.x, x, c), c), q.y, c);
    P2 { x, y, inf: false }
}
fn p2_on_curve(q: P2, c: Curve) -> bool {
    if q.inf {
        return true;
    }
    let x2 = fmul(q.x, q.x, c);
    let x3 = fmul(x2, q.x, c);
    let ax = fmul(F2 { a: c.a, b: 0 }, q.x, c);
    fmul(q.y, q.y, c) == fadd(fadd(x3, ax, c), F2 { a: c.b, b: 0 }, c)
}
fn extension_order_exact(c: Curve) -> u64 {
    let mut squares = vec![false; c.p as usize];
    for y in 0..c.p {
        squares[(y * y % c.p) as usize] = true
    }
    let mut order = 1u64;
    for a in 0..c.p {
        for b in 0..c.p {
            let x = F2 { a, b };
            let x3 = fmul(fmul(x, x, c), x, c);
            let ax = fmul(F2 { a: c.a, b: 0 }, x, c);
            let v = fadd(fadd(x3, ax, c), F2 { a: c.b, b: 0 }, c);
            if v == Z2 {
                order += 1;
                continue;
            }
            let norm = m(
                v.a as i128 * v.a as i128 - c.d as i128 * v.b as i128 * v.b as i128,
                c.p,
            );
            assert_ne!(norm, 0);
            order += if squares[norm as usize] { 2 } else { 0 };
        }
    }
    order
}
fn embed(q: Pair, c: Curve) -> P2 {
    let e = if q.e.inf {
        O2
    } else {
        P2 {
            x: F2 { a: q.e.x, b: 0 },
            y: F2 { a: q.e.y, b: 0 },
            inf: false,
        }
    };
    let t = if q.t.inf {
        O2
    } else {
        P2 {
            x: F2 { a: q.t.x, b: 0 },
            y: F2 { a: 0, b: q.t.y },
            inf: false,
        }
    };
    let out = p2add(e, t, c);
    assert!(p2_on_curve(out, c));
    out
}
fn frob(q: P2, c: Curve) -> P2 {
    if q.inf {
        q
    } else {
        P2 {
            x: F2 {
                a: q.x.a,
                b: m(-(q.x.b as i128), c.p),
            },
            y: F2 {
                a: q.y.a,
                b: m(-(q.y.b as i128), c.p),
            },
            inf: false,
        }
    }
}
fn enumerate(c: Curve, d: u64) -> Vec<Pt> {
    let mut roots = vec![Vec::new(); c.p as usize];
    for y in 0..c.p {
        roots[(y * y % c.p) as usize].push(y)
    }
    let invd = inv(d, c.p);
    let mut out = Vec::new();
    for x in 0..c.p {
        for &y in &roots[(rhs(c, x) * invd % c.p) as usize] {
            out.push(Pt { x, y, inf: false });
        }
    }
    out
}
fn twist_colors(c: Curve, ts: &[Pt]) -> [Pt; 3] {
    let t1 = ts[0];
    let mut three = None;
    for &t2 in &ts[1..] {
        let t3 = add(t1, t2, c, c.d, &mut Ops::default());
        if t3 != O
            && [t1, neg(t1, c)].contains(&t2) == false
            && [t1, neg(t1, c), t2, neg(t2, c)].contains(&t3) == false
        {
            three = Some([t1, t2, t3]);
            break;
        }
    }
    three.expect("three independent twist colors")
}
fn choose_points(c: Curve) -> (Vec<Pt>, [Pt; 3], Pt) {
    let es = enumerate(c, 1);
    let ts = enumerate(c, c.d);
    assert_eq!(es.len() as u64 + 1, c.r);
    assert_eq!(ts.len() as u64 + 1, c.nt);
    assert!(prime(c.r));
    assert_eq!(c.r + c.nt, 2 * (c.p + 1));
    let g = es[0];
    let mut x_seen = HashSet::new();
    let aa: Vec<Pt> = es.into_iter().filter(|p| x_seen.insert(p.x)).collect();
    (aa, twist_colors(c, &ts), g)
}
fn color_zero_space(c: Curve, tt: [Pt; 3]) -> (usize, usize) {
    let mut vectors = HashSet::<Vec<u64>>::new();
    let mut basis = Basis::new(3, c.r);
    let mut ops = Ops::default();
    for i in 0..6usize {
        for j in 0..6usize {
            for k in 0..6usize {
                let choices = [i, j, k];
                let mut t = O;
                let mut counts = [0i64; 3];
                for h in choices {
                    let color = h / 2;
                    let sign = if h % 2 == 0 { 1 } else { -1 };
                    let q = if sign == 1 {
                        tt[color]
                    } else {
                        neg(tt[color], c)
                    };
                    t = add(t, q, c, c.d, &mut ops);
                    counts[color] += sign;
                }
                if t == O {
                    let row: Vec<u64> = counts.iter().map(|&v| m(v as i128, c.r)).collect();
                    vectors.insert(row);
                }
            }
        }
    }
    for row in &vectors {
        basis.insert(row.clone(), 0, &mut ops)
    }
    (vectors.len(), basis.rank)
}
#[derive(Clone)]
struct Base {
    name: &'static str,
    pts: Vec<Pair>,
    tr: Vec<usize>,
    ntr: usize,
    trace_unique: usize,
}
fn build_base(c: Curve, b: usize, aa: &[Pt], tt: [Pt; 3], name: &'static str) -> Base {
    assert!(b % 3 == 0 && b <= aa.len());
    let n = b / 3;
    let pure = |xs: &[Pt], name| Base {
        name,
        pts: xs.iter().map(|&e| Pair { e, t: O }).collect(),
        tr: (0..xs.len()).collect(),
        ntr: xs.len(),
        trace_unique: xs.len(),
    };
    if name == "native-B" || name == "native-mimic" {
        return pure(&aa[..b], name);
    }
    if name == "native-trace" {
        return pure(&aa[..n], name);
    }
    let mut base = Base {
        name,
        pts: Vec::new(),
        tr: Vec::new(),
        ntr: if name == "mixed-rect" { n } else { b },
        trace_unique: 0,
    };
    assert!(name == "mixed-rect" || name == "mixed-unique");
    for j in 0..3 {
        for i in 0..n {
            let ai = if name == "mixed-rect" { i } else { j * n + i };
            base.pts.push(Pair {
                e: aa[ai],
                t: tt[j],
            });
            base.tr.push(ai);
        }
    }
    let mut trace_seen = HashSet::new();
    for (i, q) in base.pts.iter().enumerate() {
        assert_ne!(q.e, O);
        assert_ne!(q.t, O);
        let x = embed(*q, c);
        let trace = p2add(x, frob(x, c), c);
        trace_seen.insert(trace);
        let twice = add(q.e, q.e, c, 1, &mut Ops::default());
        assert_eq!(trace, embed(Pair { e: twice, t: O }, c));
        let anti = p2add(x, p2neg(frob(x, c), c), c);
        let twot = add(q.t, q.t, c, c.d, &mut Ops::default());
        assert_eq!(anti, embed(Pair { e: O, t: twot }, c));
        assert_eq!(
            base.tr[i],
            if base.name == "mixed-rect" { i % n } else { i }
        );
    }
    base.trace_unique = trace_seen.len();
    assert_eq!(base.trace_unique, base.ntr);
    base
}
fn build_base_cold(c: Curve, b: usize, name: &'static str) -> Base {
    let es = enumerate(c, 1);
    assert_eq!(es.len() as u64 + 1, c.r);
    let mut seen = HashSet::new();
    let aa: Vec<Pt> = es.into_iter().filter(|p| seen.insert(p.x)).collect();
    let tt = if name.starts_with("mixed") {
        let ts = enumerate(c, c.d);
        assert_eq!(ts.len() as u64 + 1, c.nt);
        twist_colors(c, &ts)
    } else {
        [O; 3]
    };
    build_base(c, b, &aa, tt, name)
}
#[derive(Clone, Copy)]
struct Signed {
    q: Pair,
    col: usize,
    sgn: i64,
}
fn signed(base: &Base, c: Curve) -> Vec<Signed> {
    let mut out = Vec::new();
    for (i, &q) in base.pts.iter().enumerate() {
        out.push(Signed { q, col: i, sgn: 1 });
        out.push(Signed {
            q: minus(q, c),
            col: i,
            sgn: -1,
        });
    }
    out
}
struct Table {
    map: HashMap<Pair, Vec<(usize, usize)>>,
    entries: u64,
    ops: Ops,
}
fn table(ss: &[Signed], c: Curve) -> Table {
    let mut map: HashMap<Pair, Vec<(usize, usize)>> = HashMap::new();
    let mut ops = Ops::default();
    let mut entries = 0;
    for i in 0..ss.len() {
        for j in i..ss.len() {
            if ss[i].col == ss[j].col && ss[i].sgn != ss[j].sgn {
                continue;
            }
            let q = sum(ss[i].q, ss[j].q, c, &mut ops);
            map.entry(q).or_default().push((i, j));
            entries += 1;
        }
    }
    Table { map, entries, ops }
}
fn row(base: &Base, ss: &[Signed], ids: [usize; 3], r: u64) -> (Vec<u64>, Vec<u64>) {
    let mut raw = vec![0u64; base.pts.len()];
    for &i in &ids {
        let s = ss[i];
        raw[s.col] = m(raw[s.col] as i128 + s.sgn as i128, r);
    }
    let mut tr = vec![0u64; base.ntr];
    for (i, &v) in raw.iter().enumerate() {
        tr[base.tr[i]] = (tr[base.tr[i]] + v) % r
    }
    (raw, tr)
}
fn decompose<F: FnMut([usize; 3])>(
    target: Pair,
    ss: &[Signed],
    tab: &Table,
    c: Curve,
    mimic: bool,
    ops: &mut Ops,
    mut use_hit: F,
) {
    for k in 0..ss.len() {
        let need = sum(target, minus(ss[k].q, c), c, ops);
        ops.probes += 1;
        if let Some(pairs) = tab.map.get(&need) {
            for &(i, j) in pairs {
                if (ss[i].col == ss[k].col && ss[i].sgn != ss[k].sgn)
                    || (ss[j].col == ss[k].col && ss[j].sgn != ss[k].sgn)
                {
                    continue;
                }
                if mimic {
                    let width = ss.len() / 6;
                    let mut color_counts = [0i64; 3];
                    for index in [i, j, k] {
                        color_counts[ss[index].col / width] += ss[index].sgn;
                    }
                    if color_counts != [1, 1, -1] && color_counts != [-1, -1, 1] {
                        continue;
                    }
                }
                ops.hits += 1;
                use_hit([i, j, k]);
            }
        }
    }
}
fn modinv(v: u64, r: u64, ops: &mut Ops) -> u64 {
    ops.modular_inverses += 1;
    inv(v, r)
}
struct Basis {
    rows: Vec<Option<(Vec<u64>, u64)>>,
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
    fn insert(&mut self, mut a: Vec<u64>, mut rhs: u64, ops: &mut Ops) {
        for j in 0..self.rows.len() {
            if a[j] == 0 {
                continue;
            }
            if let Some((v, z)) = &self.rows[j] {
                let t = a[j];
                for h in j..a.len() {
                    a[h] = m(a[h] as i128 - t as i128 * v[h] as i128, self.r);
                    ops.row_ops += 1;
                }
                rhs = m(rhs as i128 - t as i128 * (*z) as i128, self.r);
            } else {
                let z = modinv(a[j], self.r, ops);
                for h in j..a.len() {
                    a[h] = a[h] * z % self.r;
                    ops.row_ops += 1
                }
                rhs = rhs * z % self.r;
                self.rows[j] = Some((a, rhs));
                self.rank += 1;
                return;
            }
        }
        assert_eq!(rhs, 0, "inconsistent relation");
    }
    fn solve(&self, ops: &mut Ops) -> Vec<u64> {
        assert_eq!(self.rank, self.rows.len());
        let mut x = vec![0; self.rows.len()];
        for j in (0..x.len()).rev() {
            let (v, z) = self.rows[j].as_ref().unwrap();
            let mut y = *z;
            for h in j + 1..x.len() {
                y = m(y as i128 - v[h] as i128 * x[h] as i128, self.r);
                ops.row_ops += 1
            }
            x[j] = y;
        }
        x
    }
    fn evaluate(&self, mut a: Vec<u64>, ops: &mut Ops) -> Option<u64> {
        let mut rhs = 0;
        for j in 0..self.rows.len() {
            if a[j] == 0 {
                continue;
            }
            let (v, z) = self.rows[j].as_ref()?;
            let t = a[j];
            for h in j..a.len() {
                a[h] = m(a[h] as i128 - t as i128 * v[h] as i128, self.r);
                ops.row_ops += 1;
            }
            rhs = m(rhs as i128 + t as i128 * (*z) as i128, self.r);
        }
        Some(rhs)
    }
}
fn mix(mut z: u64) -> u64 {
    z ^= z >> 30;
    z = z.wrapping_mul(0xbf58476d1ce4e5b9);
    z ^= z >> 27;
    z = z.wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
fn target_scalar(c: Curve, b: usize, seed: u64) -> u64 {
    1 + mix(c.p ^ ((b as u64) << 32) ^ seed) % (c.r - 1)
}
fn result_line(
    c: Curve,
    b: usize,
    seed: u64,
    base: &Base,
    base_ns: u128,
    p: Pt,
    q: Pt,
    known: u64,
    full_rank_control: bool,
) {
    let start = Instant::now();
    let ss = signed(base, c);
    let mut setup = Ops::default();
    let tab = table(&ss, c);
    setup.plus(tab.ops);
    let table_ns = start.elapsed().as_nanos();
    let mut projected = Basis::new(base.ntr, c.r);
    let mut raw = Basis::new(base.pts.len(), c.r);
    let mut current = O;
    let mut trials = 0u64;
    let mut unique = HashSet::new();
    let mut raw_unique = HashSet::new();
    let mut raw_hits = 0u64;
    let mut witness = 0u64;
    let mut relation_ops = Ops::default();
    let rel_start = Instant::now();
    let rank_goal =
        if (base.name == "mixed-unique" || base.name == "native-mimic") && !full_rank_control {
            base.ntr - 2
        } else {
            base.ntr
        };
    for a in 1..c.r {
        current = add(current, p, c, 1, &mut relation_ops);
        trials += 1;
        let target = Pair { e: current, t: O };
        let mut this = Vec::new();
        decompose(
            target,
            &ss,
            &tab,
            c,
            base.name == "native-mimic",
            &mut relation_ops,
            |ids| this.push(ids),
        );
        for ids in this {
            let (rr, tt) = row(base, &ss, ids, c.r);
            raw_hits += 1;
            if witness < 256 {
                let v = sum(
                    sum(ss[ids[0]].q, ss[ids[1]].q, c, &mut Ops::default()),
                    ss[ids[2]].q,
                    c,
                    &mut Ops::default(),
                );
                assert_eq!(v, target);
                let direct = p2add(
                    p2add(embed(ss[ids[0]].q, c), embed(ss[ids[1]].q, c), c),
                    embed(ss[ids[2]].q, c),
                    c,
                );
                assert_eq!(direct, embed(target, c));
                witness += 1;
            }
            if raw_unique.insert(rr.clone()) {
                raw.insert(rr, a, &mut relation_ops);
            }
            if unique.insert(tt.clone()) {
                projected.insert(tt, a, &mut relation_ops);
            }
        }
        if projected.rank == rank_goal {
            break;
        }
    }
    let relation_ns = rel_start.elapsed().as_nanos();
    let mut linear_ops = Ops::default();
    let lin_start = Instant::now();
    let logs = if projected.rank == base.ntr {
        Some(projected.solve(&mut linear_ops))
    } else {
        None
    };
    let linear_ns = lin_start.elapsed().as_nanos();
    let mut online = Ops::default();
    let on_start = Instant::now();
    let mut online_queries = 0;
    let mut recovered = None;
    let mut offset = O;
    let mut unspanned_hits = 0u64;
    let mut target_query_ns = 0u128;
    let mut target_pdp_ns = 0u128;
    let mut target_relation_check_ns = 0u128;
    let mut target_descent_ns = 0u128;
    let mut online_relation_checks = 0u64;
    if projected.rank == rank_goal {
        for offset_scalar in 0..c.r {
            let phase = Instant::now();
            if offset_scalar > 0 {
                offset = add(offset, p, c, 1, &mut online)
            }
            let query = Pair {
                e: add(q, offset, c, 1, &mut online),
                t: O,
            };
            online_queries += 1;
            target_query_ns += phase.elapsed().as_nanos();
            let phase = Instant::now();
            let mut found = None;
            decompose(
                query,
                &ss,
                &tab,
                c,
                base.name == "native-mimic",
                &mut online,
                |ids| {
                    if found.is_none() {
                        found = Some(ids)
                    }
                },
            );
            target_pdp_ns += phase.elapsed().as_nanos();
            if let Some(ids) = found {
                let phase = Instant::now();
                let direct = p2add(
                    p2add(embed(ss[ids[0]].q, c), embed(ss[ids[1]].q, c), c),
                    embed(ss[ids[2]].q, c),
                    c,
                );
                assert_eq!(direct, embed(query, c));
                online_relation_checks += 1;
                target_relation_check_ns += phase.elapsed().as_nanos();
                let phase = Instant::now();
                let (_, tt) = row(base, &ss, ids, c.r);
                let value = if let Some(logs) = &logs {
                    let mut v = 0u64;
                    for (i, &z) in tt.iter().enumerate() {
                        v = m(v as i128 + z as i128 * logs[i] as i128, c.r);
                        online.row_ops += 1;
                    }
                    Some(v)
                } else {
                    projected.evaluate(tt, &mut online)
                };
                target_descent_ns += phase.elapsed().as_nanos();
                if let Some(v) = value {
                    let k = m(v as i128 - offset_scalar as i128, c.r);
                    if mul(k, p, c, 1, &mut online) == q {
                        recovered = Some(k);
                        break;
                    }
                    panic!("target descent gave wrong scalar");
                } else {
                    unspanned_hits += 1
                }
            }
        }
    }
    let online_ns = on_start.elapsed().as_nanos();
    let total_ns = base_ns + start.elapsed().as_nanos();
    let accounted = target_query_ns + target_pdp_ns + target_relation_check_ns + target_descent_ns;
    assert!(accounted <= online_ns);
    let target_recovery_check_ns = online_ns - accounted;
    if let Some(k) = recovered {
        assert_eq!(k, known)
    }
    println!("{{\"kind\":\"solve\",\"policy\":\"{}\",\"p\":{},\"a\":{},\"b_curve\":{},\"r\":{},\"twist_order\":{},\"base_size\":{},\"seed\":{},\"target_x\":{},\"target_y\":{},\"arm\":\"{}\",\"raw_columns\":{},\"trace_columns\":{},\"unique_traces\":{},\"rank_goal\":{},\"full_log_solution\":{},\"table_entries\":{},\"table_keys\":{},\"trials\":{},\"raw_hits\":{},\"unique_raw_rows\":{},\"unique_trace_rows\":{},\"raw_rank\":{},\"trace_rank\":{},\"witness_checks\":{},\"online_queries\":{},\"online_relation_checks\":{},\"unspanned_hits\":{},\"recovered\":{},\"correct\":{},\"setup_e_add\":{},\"setup_t_add\":{},\"setup_field_inverses\":{},\"relation_e_add\":{},\"relation_t_add\":{},\"relation_field_inverses\":{},\"relation_modular_inverses\":{},\"relation_probes\":{},\"relation_row_ops\":{},\"online_e_add\":{},\"online_t_add\":{},\"online_field_inverses\":{},\"online_probes\":{},\"online_row_ops\":{},\"base_ns\":{},\"table_ns\":{},\"relation_ns\":{},\"linear_ns\":{},\"online_ns\":{},\"target_query_ns\":{},\"target_PDP_ns\":{},\"target_relation_check_ns\":{},\"target_descent_ns\":{},\"target_recovery_check_ns\":{},\"total_ns\":{}}}",
        if full_rank_control{"full-rank"}else{"row-span"},
        c.p,c.a,c.b,c.r,c.nt,b,seed,q.x,q.y,base.name,base.pts.len(),base.ntr,base.trace_unique,
        rank_goal,logs.is_some(),tab.entries,tab.map.len(),trials,raw_hits,raw_unique.len(),unique.len(),
        raw.rank,projected.rank,witness,online_queries,online_relation_checks,unspanned_hits,
        recovered.map(|k|k.to_string()).unwrap_or_else(||"null".into()),recovered==Some(known),
        setup.e_add,setup.t_add,setup.field_inverses,
        relation_ops.e_add,relation_ops.t_add,relation_ops.field_inverses,
        relation_ops.modular_inverses+linear_ops.modular_inverses,relation_ops.probes,
        relation_ops.row_ops+linear_ops.row_ops,online.e_add,online.t_add,online.field_inverses,online.probes,
        online.row_ops,base_ns,table_ns,relation_ns,linear_ns,online_ns,
        target_query_ns,target_pdp_ns,target_relation_check_ns,target_descent_ns,
        target_recovery_check_ns,total_ns);
}
fn census(c: Curve, b: usize, base: &Base, p: Pt) -> HashSet<Vec<u64>> {
    let ss = signed(base, c);
    let tab = table(&ss, c);
    let mut all = HashSet::new();
    let mut raw_all = HashSet::new();
    let mut rank = Basis::new(base.ntr, c.r);
    let mut raw_rank = Basis::new(base.pts.len(), c.r);
    let mut hits = 0u64;
    let mut coverage = 0u64;
    let mut point = O;
    let mut ops = Ops::default();
    for a in 1..c.r {
        point = add(point, p, c, 1, &mut ops);
        let mut found = false;
        let mut ids_list = Vec::new();
        decompose(
            Pair { e: point, t: O },
            &ss,
            &tab,
            c,
            base.name == "native-mimic",
            &mut ops,
            |ids| {
                found = true;
                ids_list.push(ids)
            },
        );
        coverage += found as u64;
        for ids in ids_list {
            hits += 1;
            let (rr, tt) = row(base, &ss, ids, c.r);
            if raw_all.insert(rr.clone()) {
                raw_rank.insert(rr, a, &mut ops)
            }
            if all.insert(tt.clone()) {
                rank.insert(tt, a, &mut ops)
            }
        }
    }
    let mut support = [0u64; 4];
    for row in &all {
        let n = row.iter().filter(|&&x| x != 0).count();
        support[n.min(3)] += 1
    }
    println!("{{\"kind\":\"census\",\"p\":{},\"base_size\":{},\"arm\":\"{}\",\"targets\":{},\"covered\":{},\"raw_hits\":{},\"unique_raw_rows\":{},\"unique_trace_rows\":{},\"raw_rank\":{},\"trace_rank\":{},\"support_0\":{},\"support_1\":{},\"support_2\":{},\"support_3plus\":{},\"probes\":{}}}",
        c.p,b,base.name,c.r-1,coverage,hits,raw_all.len(),all.len(),raw_rank.rank,
        rank.rank,support[0],support[1],support[2],support[3],ops.probes);
    all
}
fn rho(c: Curve, p: Pt, q: Pt, known: u64, seed: u64, b: usize) {
    let st = Instant::now();
    let mut ops = Ops::default();
    let mut seen = HashMap::<Pt, (u64, u64)>::new();
    let mut found = None;
    let mut steps = 0u64;
    let mut restarts = 0u64;
    'outer: for restart in 0..64u64 {
        restarts = restart;
        let aa = mix(c.p ^ seed ^ restart) % c.r;
        let bb = mix(c.p ^ seed ^ restart ^ 0xfeedface) % c.r;
        let jumps: Vec<(Pt, u64, u64)> = (0..16u64)
            .map(|j| {
                let ja = 1 + mix(c.p ^ seed ^ restart ^ j ^ 0xa11ce) % (c.r - 1);
                let jb = 1 + mix(c.p ^ seed ^ restart ^ j ^ 0xbadc0de) % (c.r - 1);
                let jq = add(
                    mul(ja, p, c, 1, &mut ops),
                    mul(jb, q, c, 1, &mut ops),
                    c,
                    1,
                    &mut ops,
                );
                (jq, ja, jb)
            })
            .collect();
        let mut x = add(
            mul(aa, p, c, 1, &mut ops),
            mul(bb, q, c, 1, &mut ops),
            c,
            1,
            &mut ops,
        );
        let mut a = aa;
        let mut bq = bb;
        seen.clear();
        for _ in 0..(c.r * 8) {
            // Negation quotient; the coefficient pair is negated with the point.
            if !x.inf && x.y > c.p / 2 {
                x = neg(x, c);
                a = m(-(a as i128), c.r);
                bq = m(-(bq as i128), c.r)
            }
            steps += 1;
            ops.probes += 1;
            if let Some(&(ap, bp)) = seen.get(&x) {
                let den = m(bq as i128 - bp as i128, c.r);
                if den != 0 {
                    let k = m((ap as i128 - a as i128) * inv(den, c.r) as i128, c.r);
                    if mul(k, p, c, 1, &mut ops) == q {
                        found = Some(k);
                        break 'outer;
                    }
                }
                break;
            }
            seen.insert(x, (a, bq));
            let idx = (mix(x.x ^ (x.y << 32) ^ restart ^ seed) & 15) as usize;
            let (jq, ja, jb) = jumps[idx];
            x = add(x, jq, c, 1, &mut ops);
            a = (a + ja) % c.r;
            bq = (bq + jb) % c.r;
        }
    }
    assert_eq!(found, Some(known));
    println!("{{\"kind\":\"rho\",\"p\":{},\"base_size\":{},\"seed\":{},\"target_x\":{},\"target_y\":{},\"steps\":{},\"restarts\":{},\"e_add\":{},\"probes\":{},\"total_ns\":{},\"recovered\":{}}}",
       c.p,b,seed,q.x,q.y,steps,restarts,ops.e_add,ops.probes,st.elapsed().as_nanos(),found.unwrap());
}
fn main() {
    let mode = std::env::args().nth(1);
    assert!(
        mode.is_none()
            || matches!(
                mode.as_deref(),
                Some("--full-rank-control" | "--all-target-cost" | "--native-mimic-control")
            )
    );
    let full_rank_control = mode.as_deref() == Some("--full-rank-control");
    let all_target_cost = mode.as_deref() == Some("--all-target-cost");
    let mimic_control = mode.as_deref() == Some("--native-mimic-control");
    let fixtures = [
        (101, 3, 17, 107, 97),
        (211, 1, 1, 223, 201),
        (307, 1, 6, 331, 285),
        (509, 1, 17, 503, 517),
        (1009, 1, 14, 1013, 1007),
        (2003, 2, 15, 1933, 2075),
    ];
    if mimic_control {
        println!("{{\"kind\":\"metadata\",\"schema\":\"mixed-extension-ic/native-mimic/v1\",\"policy\":\"row-span\",\"bases\":[12,24,36],\"target_schedule\":\"every_nonzero_scalar\"}}");
    } else if all_target_cost {
        println!("{{\"kind\":\"metadata\",\"schema\":\"mixed-extension-ic/all-target-cost/v1\",\"policy\":\"row-span\",\"bases\":[12,24,36],\"target_schedule\":\"every_nonzero_scalar\"}}");
    } else {
        println!("{{\"kind\":\"metadata\",\"schema\":\"mixed-extension-ic/v1\",\"policy\":\"{}\",\"bases\":[12,24,36],\"seeds\":[1,2,3]}}",
            if full_rank_control{"full-rank"}else{"row-span"});
    }
    for (p, a, b, r, nt) in fixtures {
        let d = (2..p).find(|&x| pow(x, (p - 1) / 2, p) == p - 1).unwrap();
        let c = Curve { p, a, b, d, r, nt };
        let (aa, tt, g) = choose_points(c);
        let mut check = Ops::default();
        assert_eq!(mul(r, g, c, 1, &mut check), O);
        let n2 = extension_order_exact(c);
        assert_eq!(n2, r * nt);
        let (color_patterns, color_rank) = color_zero_space(c, tt);
        assert_eq!(color_rank, 1);
        println!("{{\"kind\":\"curve\",\"p\":{},\"a\":{},\"b\":{},\"r\":{},\"twist_order\":{},\"d\":{},\"generator_x\":{},\"generator_y\":{}}}",
                p,a,b,r,nt,d,g.x,g.y);
        println!("{{\"kind\":\"extension_order\",\"p\":{},\"enumerated_x\":{},\"order\":{},\"order_product\":{}}}",
                p,p*p,n2,r*nt);
        println!("{{\"kind\":\"color_space\",\"p\":{},\"zero_color_vectors\":{},\"rank\":{},\"codimension\":{}}}",
                p,color_patterns,color_rank,3-color_rank);
        for size in [12, 24, 36] {
            if mimic_control {
                let mimic = build_base(c, size, &aa, tt, "native-mimic");
                let mixed = build_base(c, size, &aa, tt, "mixed-unique");
                let mimic_rows = census(c, size, &mimic, g);
                let mixed_rows = census(c, size, &mixed, g);
                assert_eq!(mimic_rows, mixed_rows, "projected relation sets differ");
                println!("{{\"kind\":\"mimic_equality\",\"p\":{},\"base_size\":{},\"rows\":{},\"equal\":true}}",
                    p,size,mimic_rows.len());
                for k in 1..c.r {
                    let q = mul(k, g, c, 1, &mut Ops::default());
                    for name in ["native-mimic", "mixed-unique"] {
                        let st = Instant::now();
                        let base = build_base_cold(c, size, name);
                        let base_ns = st.elapsed().as_nanos();
                        result_line(c, size, k, &base, base_ns, g, q, k, false);
                    }
                }
                continue;
            }
            let all: Vec<Base> = ["native-B", "native-trace", "mixed-rect", "mixed-unique"]
                .iter()
                .map(|&name| build_base(c, size, &aa, tt, name))
                .collect();
            if !full_rank_control && !all_target_cost {
                let rows: Vec<HashSet<Vec<u64>>> =
                    all.iter().map(|base| census(c, size, base, g)).collect();
                for (mixed, native) in [(2usize, 1usize), (3, 0)] {
                    let missing: Vec<&Vec<u64>> = rows[mixed].difference(&rows[native]).collect();
                    let short = missing
                        .iter()
                        .filter(|r| r.iter().filter(|&&v| v != 0).count() <= 2)
                        .count();
                    assert_eq!(
                        short,
                        missing.len(),
                        "a full-width mixed trace row is absent in native base"
                    );
                    println!("{{\"kind\":\"dominance\",\"p\":{},\"base_size\":{},\"arm\":\"{}\",\"mixed_rows\":{},\"native_width3_rows\":{},\"shared_rows\":{},\"shorter_native_rows\":{},\"violations\":0}}",
                        p,size,all[mixed].name,rows[mixed].len(),rows[native].len(),
                        rows[mixed].len()-missing.len(),short);
                }
            }
            for seed in 1..=if all_target_cost { c.r - 1 } else { 3 } {
                let k = if all_target_cost {
                    seed
                } else {
                    target_scalar(c, size, seed)
                };
                let q = mul(k, g, c, 1, &mut Ops::default());
                for name in ["native-B", "native-trace", "mixed-rect", "mixed-unique"] {
                    let st = Instant::now();
                    let base = build_base_cold(c, size, name);
                    let base_ns = st.elapsed().as_nanos();
                    result_line(c, size, seed, &base, base_ns, g, q, k, full_rank_control)
                }
                if !full_rank_control {
                    rho(c, g, q, k, seed, size);
                }
            }
        }
    }
}
