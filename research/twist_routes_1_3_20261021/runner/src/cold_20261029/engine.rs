use crate::{
    arithmetic::*,
    linear::*,
    pari::{point, points, Pari, Roi},
    Result,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub id: String,
    pub fixture: Fixture,
    pub target: Point,
    pub target_label: u64,
    pub method: String,
    pub family: String,
    pub b: usize,
    pub shift: u64,
    pub alpha: String,
    pub evaluation: String,
    pub compression: bool,
    pub queries: usize,
    pub seed: u64,
}
type Witness = [i64; 5];
type Pair = (usize, usize, u8);
type Table = BTreeMap<Point, Vec<Pair>>;
fn gp_group(f: &Fixture, gp: &mut Pari) -> Result<u64> {
    let t = f.twist();
    let v = gp.call(&format!(
        "print(ellgroup(ellinit([{},{}],{})))",
        t.a, t.b, t.p
    ))?;
    Ok(v[0].as_u64().ok_or("empty twist structure")?)
}
#[derive(Clone, Copy, Debug, Serialize)]
struct Seed {
    point: Point,
    order: u64,
    x_trials: u64,
    order_tests: u64,
}
fn maximal(f: &Fixture, gp: &mut Pari) -> Result<Seed> {
    let tw = f.twist();
    let exponent = gp_group(f, gp)?;
    assert!(f.tn() % exponent == 0);
    let mut tests = 0;
    for x in 0..f.p {
        if let Some(p) = tw.lift(x).filter(|p| p[1] != 0) {
            tests += 1;
            let h = Some(p);
            if tw.order(h, f.tn()) == exponent {
                return Ok(Seed {
                    point: h,
                    order: exponent,
                    x_trials: x + 1,
                    order_tests: tests,
                });
            }
        }
    }
    Err("maximal-seed scan exhausted despite group structure".into())
}
fn maximal_scan(f: &Fixture) -> Seed {
    let tw = f.twist();
    let mut best = Seed {
        point: None,
        order: 0,
        x_trials: 0,
        order_tests: 0,
    };
    for x in 0..f.p {
        best.x_trials = x + 1;
        if let Some(p) = tw.lift(x).filter(|p| p[1] != 0) {
            best.order_tests += 1;
            let h = Some(p);
            let n = tw.order(h, f.tn());
            if n > best.order {
                best.point = h;
                best.order = n;
            }
            if n == f.tn() {
                break;
            }
        }
    }
    assert!(best.point.is_some());
    best
}
fn first(f: &Fixture) -> Seed {
    let tw = f.twist();
    for x in 0..f.p {
        if let Some(p) = tw.lift(x).filter(|p| p[1] != 0) {
            let h = Some(p);
            return Seed {
                point: h,
                order: tw.order(h, f.tn()),
                x_trials: x + 1,
                order_tests: 1,
            };
        }
    }
    panic!("no twist seed")
}
fn member(f: &Fixture, p: Point) -> bool {
    p.is_some_and(|q| q[1] != 0) && (f.r == f.order || f.curve().mul(p, f.r).is_none())
}
fn seed_base(t: &Task, requested: usize, hm: Option<Seed>) -> (Vec<Point>, Value) {
    let f = &t.fixture;
    let e = f.curve();
    let seed = match t.family.as_str() {
        "twist-max" | "twist-max-scan" => hm.unwrap(),
        "twist-first" => first(f),
        _ => Seed {
            point: f.g(),
            order: f.r,
            x_trials: 0,
            order_tests: 0,
        },
    };
    let h = seed.point;
    let hn = seed.order;
    let twist = t.family.starts_with("twist");
    let orbit = twist || t.family == "curve-orbit";
    let model = if twist { f.twist() } else { e };
    let di = if twist { inv(f.d, f.p) } else { 1 };
    let cap = if orbit {
        hn.saturating_sub(1)
    } else if t.family == "uniform" {
        8 * f.p
    } else {
        f.p
    };
    let mut accepted = BTreeMap::new();
    accepted.insert(f.generator[0], f.g());
    let mut p = h;
    let mut state = 202610290031u64 ^ f.p ^ (f.b << 16) ^ t.shift;
    let mut terms = 0;
    for i in 0..cap {
        if accepted.len() == requested {
            break;
        }
        if orbit && p.is_none() {
            break;
        }
        let raw = if orbit {
            p.unwrap()[0]
        } else if t.family == "uniform" {
            mix(&mut state) % f.p
        } else {
            i
        };
        let shift = if t.family == "low-x" { 0 } else { t.shift };
        let x = (raw * di + t.fixture.p + shift) % f.p;
        terms += 1;
        if !accepted.contains_key(&x) {
            let q = e.lift(x);
            if member(f, q) {
                accepted.insert(x, q);
            }
        }
        if orbit {
            p = model.add(p, h);
        }
    }
    let mut out = vec![f.g()];
    out.extend(
        accepted
            .into_iter()
            .filter(|(x, _)| *x != f.generator[0])
            .map(|(_, p)| p),
    );
    (
        out,
        json!({"seed":h,"seed_order":hn,"seed_search_x_trials":seed.x_trials,"seed_order_tests":seed.order_tests,"scan_terms":terms,"candidate_cap":cap,"requested":requested}),
    )
}
fn alpha_image(a: &Alpha, f: &Fixture, p: Point, mode: &str) -> Point {
    if mode == "scalar" {
        f.curve().mul(p, a.lambda)
    } else {
        a.eval(f.curve(), p)
    }
}
fn known_rows(t: &Task, base: &[Point], alpha: Option<&Alpha>) -> (Vec<Vec<u64>>, Vec<Value>) {
    let r = t.fixture.r;
    let b = base.len();
    let mut rows = vec![];
    let mut anchor = vec![0; b + 1];
    anchor[0] = 1;
    anchor[b] = 1;
    rows.push(anchor);
    let mut edges = vec![];
    if let Some(a) = alpha {
        let index: BTreeMap<_, _> = base
            .iter()
            .enumerate()
            .map(|(i, p)| (p.unwrap()[0], i))
            .collect();
        for (i, p) in base.iter().enumerate() {
            let q = alpha_image(a, &t.fixture, *p, &t.evaluation);
            if let Some([x, y]) = q {
                if let Some(&j) = index.get(&x) {
                    let sign = if y == base[j].unwrap()[1] {
                        a.lambda
                    } else {
                        r - a.lambda
                    };
                    let mut row = vec![0; b + 1];
                    row[j] = 1;
                    row[i] = (row[i] + r - sign) % r;
                    rows.push(row);
                    edges.push(json!({"i":i,"j":j,"multiplier":sign}));
                }
            }
        }
    }
    (rows, edges)
}
fn catalog(e: Curve, base: &[Point]) -> Table {
    let mut m: Table = BTreeMap::new();
    for i in 0..base.len() {
        for j in i..base.len() {
            for s in 0..4 {
                let a = if s & 1 == 0 { base[i] } else { e.neg(base[i]) };
                let b = if s & 2 == 0 { base[j] } else { e.neg(base[j]) };
                m.entry(e.add(a, b)).or_default().push((i, j, s));
            }
        }
    }
    m
}
fn group_collect(e: Curve, base: &[Point], table: &Table, targets: &[Point]) -> Vec<Vec<Witness>> {
    targets
        .iter()
        .map(|q| {
            let mut out = vec![];
            if let Some(v) = table.get(q) {
                for (i, j, s) in v {
                    out.push([2, *i as i64, *j as i64, -1, *s as i64]);
                }
            }
            for (k, p) in base.iter().enumerate() {
                for s in 0..2 {
                    let p = if s == 0 { *p } else { e.neg(*p) };
                    if let Some(v) = table.get(&e.add(*q, e.neg(p))) {
                        for (i, j, mask) in v {
                            out.push([3, *i as i64, *j as i64, k as i64, (*mask as i64) + 4 * s]);
                        }
                    }
                }
            }
            out.sort();
            out
        })
        .collect()
}
fn polynomial_roots(a: u64, b: u64, c: u64, p: u64, base: &[Point]) -> Vec<u64> {
    if a == 0 {
        if b == 0 {
            return if c == 0 {
                base.iter().map(|p| p.unwrap()[0]).collect()
            } else {
                vec![]
            };
        }
        return vec![(p - c) * inv(b, p) % p];
    }
    let disc = (b * b % p + p - 4 * a % p * c % p) % p;
    let Some(s) = sqrt(disc, p) else {
        return vec![];
    };
    let ia = inv(2 * a % p, p);
    let x = ((p - b) + s) * ia % p;
    let y = (2 * p - b - s) * ia % p;
    if x == y {
        vec![x]
    } else {
        vec![x, y]
    }
}
fn poly_pairs(e: Curve, base: &[Point], index: &BTreeMap<u64, usize>, q: Point) -> Vec<Pair> {
    let p = e.p;
    let mut out = vec![];
    let Some([z, _]) = q else {
        return (0..base.len())
            .flat_map(|i| [(i, i, 1), (i, i, 2)])
            .collect();
    };
    for (i, pt) in base.iter().enumerate() {
        let x = pt.unwrap()[0];
        let d = (x + p - z) % p;
        let a = d * d % p;
        let b = (p - 2 * ((x + z) % p * ((x * z + e.a) % p) % p + 2 * e.b) % p) % p;
        let tmp = (x * z % p + p - e.a) % p;
        let c = (tmp * tmp % p + p - 4 * e.b % p * ((x + z) % p) % p) % p;
        for root in polynomial_roots(a, b, c, p, base) {
            if let Some(&j) = index.get(&root) {
                if j < i {
                    continue;
                }
                for s in 0..4 {
                    let pi = if s & 1 == 0 { *pt } else { e.neg(*pt) };
                    let pj = if s & 2 == 0 { base[j] } else { e.neg(base[j]) };
                    if e.add(pi, pj) == q {
                        out.push((i, j, s));
                    }
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}
fn poly_collect(
    e: Curve,
    base: &[Point],
    index: &BTreeMap<u64, usize>,
    targets: &[Point],
) -> Vec<Vec<Witness>> {
    targets
        .iter()
        .map(|q| {
            let mut out = vec![];
            for (i, j, s) in poly_pairs(e, base, index, *q) {
                out.push([2, i as i64, j as i64, -1, s as i64]);
            }
            for (k, p) in base.iter().enumerate() {
                for s in 0..2 {
                    let v = if s == 0 { *p } else { e.neg(*p) };
                    for (i, j, mask) in poly_pairs(e, base, index, e.add(*q, e.neg(v))) {
                        out.push([3, i as i64, j as i64, k as i64, mask as i64 + 4 * s]);
                    }
                }
            }
            out.sort();
            out
        })
        .collect()
}
fn query_rows(
    f: &Fixture,
    base: &[Point],
    targets: &[Point],
    ws: &[Vec<Witness>],
) -> Vec<Vec<u64>> {
    let e = f.curve();
    let r = f.r;
    let b = base.len();
    let mut rows = BTreeSet::new();
    for (j, witnesses) in ws.iter().enumerate() {
        for w in witnesses {
            let mut row = vec![0; b + 2];
            let mut point = None;
            for k in 0..w[0] as usize {
                let i = w[k + 1] as usize;
                let neg = w[4] & (1 << k) != 0;
                let pt = if neg { e.neg(base[i]) } else { base[i] };
                point = e.add(point, pt);
                row[i] = (row[i] + if neg { r - 1 } else { 1 }) % r;
            }
            assert_eq!(point, targets[j]);
            row[b] = r - 1;
            row[b + 1] = j as u64 % r;
            rows.insert(row);
        }
    }
    rows.into_iter().collect()
}
fn validate_fiber(rows: &[Vec<u64>], fiber: &Fiber, p: u64) {
    assert!(!fiber.inconsistent);
    for row in rows {
        assert!(satisfies(row, &fiber.particular, p));
        for v in &fiber.kernel {
            let mut a = row.clone();
            *a.last_mut().unwrap() = 0;
            assert!(satisfies(&a, v, p));
        }
    }
}
pub fn ic(t: &Task, gp: &mut Pari) -> Result<Value> {
    let f = &t.fixture;
    let e = f.curve();
    let mut roi = Roi::new();
    let start = gp.calls;
    roi.begin();
    assert!(
        prime(f.p)
            && prime(f.r)
            && f.r > 17
            && f.order % f.r == 0
            && e.on(f.g())
            && f.g().is_some()
    );
    assert!(e.mul(f.g(), f.r).is_none());
    assert_eq!(e.canon(f.g()), f.g());
    let wants_alpha = t.alpha != "none";
    let mut hm = None;
    let map_possible = f.tn() % 17 == 0 && {
        let m = isqrt(f.tn() / 17);
        m > 1 && 17 * m * m == f.tn()
    };
    if t.family == "twist-max-scan" {
        hm = Some(maximal_scan(f));
    } else if t.family == "twist-max" || (wants_alpha && map_possible) {
        hm = Some(maximal(f, gp)?);
    }
    let alpha = if wants_alpha {
        hm.and_then(|s| Alpha::construct(f, s.point, s.order))
    } else {
        None
    };
    let (mut base, provenance) = seed_base(t, if t.alpha == "augment" { t.b / 2 } else { t.b }, hm);
    let unsupported = wants_alpha && alpha.is_none();
    if t.alpha == "augment" && !unsupported {
        let a = alpha.as_ref().unwrap();
        let mut all: BTreeMap<_, _> = base.iter().map(|p| (p.unwrap()[0], *p)).collect();
        for p in &base {
            let q = e.canon(alpha_image(a, f, *p, &t.evaluation));
            if let Some(v) = q {
                assert!(member(f, q));
                all.insert(v[0], q);
            }
        }
        base = vec![f.g()];
        base.extend(
            all.into_iter()
                .filter(|(x, _)| *x != f.generator[0])
                .map(|(_, p)| p),
        );
    }
    let (known, edges) = known_rows(t, &base, alpha.as_ref());
    let b = base.len();
    let mut known_full = vec![];
    for row in &known {
        let mut r = row[..b].to_vec();
        r.push(0);
        r.push(row[b]);
        known_full.push(r);
    }
    let comp = if t.compression && !unsupported && b == t.b {
        Some(Compression::new(f.r, b, &known))
    } else {
        None
    };
    let mut full_known = Rref::new(f.r, b + 1);
    for r in &known_full {
        full_known.add(r.clone());
    }
    let known_rank = full_known.fiber().rank;
    let table = if t.method == "group" && !unsupported && b == t.b {
        Some(catalog(e, &base))
    } else {
        None
    };
    let index: BTreeMap<_, _> = base
        .iter()
        .enumerate()
        .map(|(i, p)| (p.unwrap()[0], i))
        .collect();
    roi.end("setup", start, gp.calls);
    if unsupported || b != t.b {
        return Ok(
            json!({"schema":"twist.cold-cell/v1","task":t,"status":if unsupported{"unsupported-alpha"}else{"underfilled"},"base":base,"construction":provenance,"alpha":alpha,"known_edges":edges,"known_rows":known_full,"known_rank":known_rank,"markers":roi.marks,"measured_under_valgrind":crate::pari::running()}),
        );
    }
    let start = gp.calls;
    roi.begin();
    assert!(e.on(t.target) && e.mul(t.target, f.r).is_none());
    let mut q = t.target;
    let mut targets = vec![];
    for _ in 0..t.queries {
        targets.push(q);
        q = e.add(q, f.g());
    }
    roi.end("query-generation", start, gp.calls);
    let start = gp.calls;
    roi.begin();
    let witnesses = match t.method.as_str() {
        "group" => group_collect(e, &base, table.as_ref().unwrap(), &targets),
        "poly" => poly_collect(e, &base, &index, &targets),
        "gp-poly" => serde_json::from_value(gp.call(&format!(
            "print(coldcollect({},{},{},{},{}))",
            f.p,
            f.a,
            f.b,
            points(&base),
            points(&targets)
        ))?)?,
        _ => return Err("unknown collector".into()),
    };
    roi.end("decomposition", start, gp.calls);
    let start = gp.calls;
    roi.begin();
    let rows = query_rows(f, &base, &targets, &witnesses);
    roi.end("signs-deduplication", start, gp.calls);
    let start = gp.calls;
    roi.begin();
    let mut m = if let Some(c) = &comp {
        Rref::new(f.r, c.columns.len() + 1)
    } else {
        full_known.clone()
    };
    for row in &rows {
        m.add(if let Some(c) = &comp {
            c.row(row)
        } else {
            row.clone()
        });
    }
    let raw_fiber = m.fiber();
    let fiber = if let Some(c) = &comp {
        c.fiber(&raw_fiber, known_rank)
    } else {
        raw_fiber.clone()
    };
    let target_fixed = fiber.kernel.iter().all(|v| v[b] == 0);
    let scalar = if target_fixed {
        Some(fiber.particular[b])
    } else {
        None
    };
    roi.end("linear-algebra", start, gp.calls);
    let start = gp.calls;
    roi.begin();
    let mut all = known_full.clone();
    all.extend(rows.iter().cloned());
    validate_fiber(&all, &fiber, f.r);
    let alternative = if let Some(k) = scalar {
        assert_eq!(e.mul(f.g(), k), t.target);
        None
    } else {
        let v = fiber.kernel.iter().find(|v| v[b] != 0).unwrap();
        let alt: Vec<_> = fiber
            .particular
            .iter()
            .zip(v)
            .map(|(a, b)| (a + b) % f.r)
            .collect();
        assert_ne!(alt[b], fiber.particular[b]);
        for row in &all {
            assert!(satisfies(row, &alt, f.r));
        }
        Some(alt)
    };
    roi.end("recovery-verification", start, gp.calls);
    Ok(
        json!({"schema":"twist.cold-cell/v1","task":t,"status":if target_fixed{"recovered"}else{"ambiguous"},"base":base,"construction":provenance,"alpha":alpha,"known_edges":edges,"known_rows":known_full,"known_rank":known_rank,"query_targets":targets,"witnesses":witnesses,"query_rows":rows,"compression":comp,"reduced_fiber":raw_fiber,"fiber":fiber,"recovered_scalar":scalar,"ambiguity_alternative":alternative,"markers":roi.marks,"measured_under_valgrind":crate::pari::running()}),
    )
}
#[derive(Clone, Copy, Debug)]
struct Walk {
    p: Point,
    a: u64,
    b: u64,
}
fn canon(e: Curve, n: u64, mut w: Walk, fold: bool) -> Walk {
    if fold && w.p.is_some_and(|p| p[1] > e.p / 2) {
        w.p = e.neg(w.p);
        w.a = (n - w.a) % n;
        w.b = (n - w.b) % n;
    }
    w
}
fn rho(f: &Fixture, q: Point, seed: u64, fold: bool) -> (Option<u64>, Value) {
    let e = f.curve();
    let n = f.r;
    let mut rng = seed;
    let cap = 128 * (isqrt(n) + 1) + 16384;
    let (mut steps, mut restarts, mut escapes) = (0, 0, 0);
    while steps < cap {
        let jumps: Vec<_> = (0..16)
            .map(|_| {
                let a = mix(&mut rng) % n;
                let b = mix(&mut rng) % n;
                Walk {
                    p: e.add(e.mul(f.g(), a), e.mul(q, b)),
                    a,
                    b,
                }
            })
            .collect();
        let a = mix(&mut rng) % n;
        let b = mix(&mut rng) % n;
        let mut w = canon(
            e,
            n,
            Walk {
                p: e.add(e.mul(f.g(), a), e.mul(q, b)),
                a,
                b,
            },
            fold,
        );
        let mut anchor = w;
        let (mut power, mut length) = (1, 0);
        let mut history = vec![w];
        loop {
            if steps >= cap {
                break;
            }
            if power == length {
                anchor = w;
                power *= 2;
                length = 0;
            }
            let part = w.p.map_or(0, |p| (p[0] as usize) & 15);
            let j = jumps[part];
            w = canon(
                e,
                n,
                Walk {
                    p: e.add(w.p, j.p),
                    a: (w.a + j.a) % n,
                    b: (w.b + j.b) % n,
                },
                fold,
            );
            steps += 1;
            length += 1;
            if w.p == anchor.p {
                let d = (w.b + n - anchor.b) % n;
                if d != 0 {
                    let k = (anchor.a + n - w.a) % n * inv(d, n) % n;
                    if e.mul(f.g(), k) == q {
                        return (
                            Some(k),
                            json!({"steps":steps,"restarts":restarts,"cycle_escapes":escapes,"cap":cap}),
                        );
                    }
                }
                break;
            }
            if fold && history.iter().rev().skip(1).any(|h| h.p == w.p) {
                let best = *history
                    .iter()
                    .chain(std::iter::once(&w))
                    .min_by_key(|z| z.p)
                    .unwrap();
                w = canon(
                    e,
                    n,
                    Walk {
                        p: e.add(best.p, best.p),
                        a: 2 * best.a % n,
                        b: 2 * best.b % n,
                    },
                    true,
                );
                escapes += 1;
                history.clear();
            }
            history.push(w);
            if history.len() > 8 {
                history.remove(0);
            }
            if w.p.is_none() {
                break;
            }
        }
        restarts += 1;
    }
    (
        None,
        json!({"steps":steps,"restarts":restarts,"cycle_escapes":escapes,"cap":cap}),
    )
}
pub fn reference(t: &Task, gp: &mut Pari) -> Result<Value> {
    let f = &t.fixture;
    let e = f.curve();
    let mut roi = Roi::new();
    let start = gp.calls;
    roi.begin();
    assert!(prime(f.p) && prime(f.r) && e.on(f.g()) && e.mul(f.g(), f.r).is_none());
    let size = isqrt(f.r) + 1;
    let mut baby = BTreeMap::new();
    let mut folded_baby = BTreeMap::new();
    let mut p = None;
    let mut step = None;
    if t.method == "bsgs" {
        for i in 0..size {
            baby.entry(p).or_insert(i);
            p = e.add(p, f.g());
        }
        step = e.neg(e.mul(f.g(), size));
    }
    if t.method == "bsgs-negation" {
        for i in 0..=size {
            let sign = if e.canon(p) == p { 1 } else { f.r - 1 };
            folded_baby.entry(e.canon(p)).or_insert((i, sign));
            p = e.add(p, f.g());
        }
        step = e.neg(e.mul(f.g(), 2 * size));
    }
    if t.method == "gp-elllog" {
        gp.call(&format!("ce=ellinit([{},{}],{});print(1)", f.a, f.b, f.p))?;
    }
    roi.end("setup", start, gp.calls);
    let start = gp.calls;
    roi.begin();
    assert!(e.on(t.target) && e.mul(t.target, f.r).is_none());
    roi.end("query-generation", start, gp.calls);
    let start = gp.calls;
    roi.begin();
    let (mut scalar, mut details) = (None, Value::Null);
    match t.method.as_str() {
        "bsgs" => {
            let mut q = t.target;
            for j in 0..=size {
                if let Some(i) = baby.get(&q) {
                    let k = (j * size + i) % f.r;
                    if e.mul(f.g(), k) == t.target {
                        scalar = Some(k);
                        break;
                    }
                }
                q = e.add(q, step);
            }
        }
        "bsgs-negation" => {
            let mut q = t.target;
            for j in 0..=(f.r / (2 * size) + 1) {
                let sq = if e.canon(q) == q { 1 } else { f.r - 1 };
                if let Some((i, si)) = folded_baby.get(&e.canon(q)) {
                    let k = (2 * size * j + sq * si % f.r * i) % f.r;
                    if e.mul(f.g(), k) == t.target {
                        scalar = Some(k);
                        break;
                    }
                }
                q = e.add(q, step);
            }
        }
        "rho" | "rho-negation" => {
            (scalar, details) = rho(f, t.target, t.seed, t.method == "rho-negation");
        }
        "gp-elllog" => {
            scalar = gp
                .call(&format!(
                    "print(lift(elllog(ce,{},{},{})))",
                    point(t.target),
                    point(f.g()),
                    f.r
                ))?
                .as_u64();
        }
        _ => return Err("unknown DLP reference".into()),
    }
    roi.end("walk-collision", start, gp.calls);
    let start = gp.calls;
    roi.begin();
    if let Some(k) = scalar {
        assert_eq!(e.mul(f.g(), k), t.target);
    }
    roi.end("recovery-verification", start, gp.calls);
    Ok(
        json!({"schema":"twist.cold-reference/v1","task":t,"status":if scalar.is_some(){"recovered"}else{"capped"},"recovered_scalar":scalar,"details":details,"markers":roi.marks,"measured_under_valgrind":crate::pari::running()}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_root_and_mitm_signed_witnesses_equal() {
        let e = Curve { p: 101, a: 2, b: 3 };
        let base: Vec<_> = (0..101)
            .filter_map(|x| e.lift(x))
            .filter(|p| p[1] != 0)
            .take(7)
            .map(Some)
            .collect();
        let table = catalog(e, &base);
        let idx = base
            .iter()
            .enumerate()
            .map(|(i, p)| (p.unwrap()[0], i))
            .collect();
        let targets: Vec<_> = std::iter::once(None)
            .chain((0..101).filter_map(|x| e.lift(x)).map(Some))
            .collect();
        assert_eq!(
            group_collect(e, &base, &table, &targets),
            poly_collect(e, &base, &idx, &targets)
        );
    }
    #[test]
    fn rho_references_recover_public_toy() {
        let f = Fixture {
            id: "control".into(),
            p: 59,
            a: 1,
            b: 13,
            order: 67,
            r: 67,
            d: 2,
            generator: [1, 29],
            family: "test".into(),
            role: "test".into(),
        };
        for fold in [false, true] {
            for k in [1, 2, 17, 37, 66] {
                assert_eq!(rho(&f, f.curve().mul(f.g(), k), 20261029, fold).0, Some(k));
            }
        }
    }
}
