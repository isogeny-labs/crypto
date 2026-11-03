//! Image-base geometry, independent full-stream reference and polynomial collector.
use super::*;
use crate::fx_hash::FxMap;
use serde::{Deserialize, Serialize};
#[path = "cold.rs"]
pub(super) mod cold;
#[path = "cold_catalog.rs"]
pub(super) mod cold_catalog;
#[path = "external.rs"]
pub(super) mod external;
#[path = "joint.rs"]
pub(super) mod joint;
#[path = "report.rs"]
pub(super) mod report;
type Terms = Vec<(usize, i8)>;
type Key = (E, E, bool);
const ARMS: [&str; 10] = [
    "original",
    "psi-control",
    "phi-l2",
    "phi-l0",
    "union-l2",
    "union-all",
    "union-l0",
    "phi-l0-active",
    "l0-peel",
    "double-l0-active",
];
const PRIMES: [u64; 7] = [37, 73, 97, 157, 229, 241, 337];
const INPUT_SHA: &str = "d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c";
fn key(p: Point) -> Key {
    (p.x, p.y, p.inf)
}
fn negn(x: u64, n: u64) -> u64 {
    if x == 0 {
        0
    } else {
        n - x
    }
}
fn mm(a: u64, b: u64, n: u64) -> u64 {
    meter::modular(n, 1);
    ((a as u128 * b as u128) % n as u128) as u64
}
fn signed(c: &Curve, p: Point, s: i8) -> Point {
    if s == 1 {
        p
    } else {
        c.neg(p)
    }
}
fn write(p: &Path, v: &Value) {
    std::fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Factor {
    p: PointRecord,
    col: usize,
    coef: u64,
    source_id: usize,
    image: bool,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct PointRecord {
    x: E,
    y: E,
    inf: bool,
}
impl From<Point> for PointRecord {
    fn from(p: Point) -> Self {
        Self {
            x: p.x,
            y: p.y,
            inf: p.inf,
        }
    }
}
impl From<PointRecord> for Point {
    fn from(p: PointRecord) -> Self {
        Self {
            x: p.x,
            y: p.y,
            inf: p.inf,
        }
    }
}
struct Case {
    raw: Value,
    c: Curve,
    g: Point,
    q: Point,
    n: u64,
    p: u64,
    seed: u64,
    omega: u64,
    lc: u64,
    lf: u64,
    l0: usize,
    original_len: usize,
    reps: Vec<usize>,
    anchor: u64,
    pool: Vec<Factor>,
    peels: Vec<Vec<usize>>,
    zero_triples: BTreeSet<Terms>,
    geometry: Value,
}
fn psi(c: &Curve, p: Point, w: u64) -> Point {
    if p.inf {
        p
    } else {
        Point {
            x: c.f.mul(p.x, c.f.scalar(w)),
            ..p
        }
    }
}
fn frob(c: &Curve, p: Point) -> Point {
    if p.inf {
        return p;
    }
    let t = [0, 1, 0, 0];
    let f = c.f;
    Point {
        x: f.mul(f.inv(f.pow(t, f.p - 1)), f.pow(p.x, f.p)),
        y: f.mul(f.inv(f.pow(t, 3 * (f.p - 1) / 2)), f.pow(p.y, f.p)),
        inf: false,
    }
}
fn matches_sign(c: &Curve, stored: Point, image: Point) -> i8 {
    if stored == image {
        1
    } else {
        assert_eq!(stored, c.neg(image), "same x without sign equivalence");
        -1
    }
}
fn load(path: &Path, sha: &str, p: u64, seed: u64) -> Value {
    assert_eq!(sha, INPUT_SHA, "caller changed the frozen digest");
    assert!(PRIMES.contains(&p) && seed < 3, "unregistered instance");
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(
        hex::encode(hash::sha256::sha256(&bytes)),
        sha,
        "input SHA mismatch before geometry or solving"
    );
    let records: Vec<Value> = std::str::from_utf8(&bytes)
        .unwrap()
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    records
        .into_iter()
        .find(|r| r["p"] == p && r["seed"] == seed && r["line_powers"] == json!([0, 2]))
        .unwrap()
}
impl Case {
    fn new(raw: Value, out: &Path) -> Self {
        let p = raw["p"].as_u64().unwrap();
        let seed = raw["seed"].as_u64().unwrap();
        let n = raw["n"].as_u64().unwrap();
        assert!(
            n > 3 && n % 2 == 1 && (3..=n.isqrt()).step_by(2).all(|d| n % d != 0),
            "registered subgroup order must be prime"
        );
        meter::configure(p, n);
        let c = Curve {
            f: Field::new(p),
            b: [0, 0, 0, raw["b0"].as_u64().unwrap()],
        };
        let g = Point {
            x: element(&raw["generator_x"]),
            y: element(&raw["generator_y"]),
            inf: false,
        };
        let q = Point {
            x: element(&raw["target_x"]),
            y: element(&raw["target_y"]),
            inf: false,
        };
        assert!(c.on_curve(g) && c.on_curve(q));
        assert_eq!(c.mul(g, n), INF);
        assert_eq!(c.mul(q, n), INF);
        let omega = raw["omega"].as_u64().unwrap();
        let lc = raw["lambda_cubic"].as_u64().unwrap();
        let lf = raw["lambda_frob"].as_u64().unwrap();
        assert_ne!(omega, 1);
        assert_eq!(fpow(omega, 3, p), 1);
        assert_eq!(psi(&c, g, omega), c.mul(g, lc));
        assert_eq!(frob(&c, g), c.mul(g, lf));
        let negative = psi(&c, g, omega) != c.mul(g, (lc + 1) % n);
        assert!(negative, "corrupt eigenvalue did not reject");
        let base: Vec<_> = raw["base_coordinates"]
            .as_array()
            .unwrap()
            .iter()
            .map(point)
            .collect();
        assert!(base
            .iter()
            .copied()
            .all(|v| !v.inf && c.on_curve(v) && v.x != [0; 4]));
        let original_len = base.len();
        let l0 = base.iter().take_while(|v| v.x[2] == 0).count();
        assert!(l0 > 0 && l0 < base.len());
        assert!(base[..l0].iter().all(|v| v.x[1..] == [0, 0, 0]));
        assert!(base[l0..]
            .iter()
            .all(|v| v.x[0] == 0 && v.x[1] == 0 && v.x[3] == 0));
        let by_x: BTreeMap<_, _> = base.iter().enumerate().map(|(i, v)| (v.x, i)).collect();
        assert_eq!(by_x.len(), base.len());
        let anchor = if base[0] == g {
            1
        } else {
            assert_eq!(
                base[0],
                c.neg(g),
                "no registered first-factor generator anchor"
            );
            n - 1
        };
        let mut columns = vec![usize::MAX; base.len()];
        let mut coefficients = vec![0; base.len()];
        let mut reps = Vec::new();
        for (i, &rep) in base.iter().enumerate() {
            if columns[i] != usize::MAX {
                continue;
            }
            let col = reps.len();
            reps.push(i);
            let mut fr = rep;
            for j in 0..4 {
                let mut image = fr;
                for k in 0..3 {
                    let &idx = by_x.get(&image.x).expect("automorphism left original base");
                    let sign = matches_sign(&c, base[idx], image);
                    let scalar = mm(fpow(lf, j, n), fpow(lc, k, n), n);
                    let scalar = if sign == 1 { scalar } else { negn(scalar, n) };
                    if columns[idx] == usize::MAX {
                        columns[idx] = col;
                        coefficients[idx] = scalar;
                    } else {
                        assert_eq!((columns[idx], coefficients[idx]), (col, scalar));
                    }
                    image = psi(&c, image, omega);
                }
                fr = frob(&c, fr);
            }
        }
        assert!(columns.iter().all(|&v| v != usize::MAX));
        assert_eq!(reps.len() * 12, base.len());
        for (id, &pt) in base.iter().enumerate() {
            assert_eq!(c.mul(base[reps[columns[id]]], coefficients[id]), pt);
        }
        let mut pool: Vec<_> = base
            .iter()
            .enumerate()
            .map(|(id, &pt)| Factor {
                p: pt.into(),
                col: columns[id],
                coef: coefficients[id],
                source_id: id,
                image: false,
            })
            .collect();
        let mut all_by_x = by_x.clone();
        let delta = (n + 1 - lc) % n;
        assert_ne!(delta, 0);
        let mut images = Vec::new();
        let mut exceptions = Vec::new();
        let mut formula_matches = 0;
        for (id, &pt) in base.iter().enumerate() {
            let image = c.add(pt, c.neg(psi(&c, pt, omega)));
            if image.inf {
                exceptions.push(json!({"kind":"infinity_image","source_id":id}));
                continue;
            }
            assert!(c.on_curve(image));
            assert_eq!(image, c.mul(pt, delta));
            let f = c.f;
            let lhs = f.mul(f.scalar(3), f.mul(f.mul(pt.x, pt.x), image.x));
            let rhs = f.neg(f.mul(
                f.scalar(omega * omega % p),
                f.add(f.mul(f.mul(pt.x, pt.x), pt.x), f.mul(f.scalar(4), c.b)),
            ));
            assert_eq!(lhs, rhs, "image coordinate formula mismatch");
            formula_matches += 1;
            if let Some(&prior) = all_by_x.get(&image.x) {
                exceptions.push(json!({"kind":"coincident_image_abscissa","source_id":id,"prior":prior,"point":PointRecord::from(image)}));
                continue;
            }
            if image.x[3] == 0 {
                exceptions.push(json!({"kind":"missing_new_component","source_id":id,"point":PointRecord::from(image)}));
                continue;
            }
            let idx = pool.len();
            all_by_x.insert(image.x, idx);
            images.push(idx);
            pool.push(Factor {
                p: image.into(),
                col: columns[id],
                coef: mm(delta, coefficients[id], n),
                source_id: id,
                image: true,
            });
        }
        // The frozen panel must discharge exceptions explicitly, before any census.
        if !exceptions.is_empty() {
            write(
                &out.join("exceptional-images.json"),
                &json!({"status":"unsupported","exceptions":exceptions}),
            );
        }
        assert!(
            exceptions.is_empty(),
            "unsupported exceptional image case: {exceptions:?}"
        );
        assert_eq!(images.len(), base.len());
        let original: Vec<_> = (l0..base.len()).collect();
        let mut psi_control = BTreeSet::new();
        for &id in &original {
            let im = psi(&c, base[id], omega);
            let &j = by_x.get(&im.x).unwrap();
            let sign = matches_sign(&c, base[j], im);
            let coef = mm(lc, coefficients[id], n);
            assert_eq!(
                (columns[j], coefficients[j]),
                (columns[id], if sign == 1 { coef } else { negn(coef, n) })
            );
            psi_control.insert(j);
        }
        assert_eq!(psi_control.iter().copied().collect::<Vec<_>>(), original);
        let phi_l2 = images[l0..].to_vec();
        let phi_l0 = images[..l0].to_vec();
        let mut union_l2 = original.clone();
        union_l2.extend(&phi_l2);
        let mut union_all = union_l2.clone();
        union_all.extend(&phi_l0);
        union_all.sort();
        let mut union_l0 = original.clone();
        union_l0.extend(&phi_l0);
        union_l0.sort();
        let mut peels = vec![
            original,
            psi_control.into_iter().collect(),
            phi_l2,
            phi_l0.clone(),
            union_l2,
            union_all,
            union_l0,
            phi_l0,
            (0..l0).collect(),
        ];
        let mut doubled = Vec::new();
        for (id, &pt) in base[..l0].iter().enumerate() {
            let im = c.add(pt, pt);
            assert!(!im.inf && c.on_curve(im));
            assert_eq!(im, c.mul(pt, 2));
            if let Some(&prior) = all_by_x.get(&im.x) {
                write(
                    &out.join("exceptional-doubling.json"),
                    &json!({"status":"unsupported","source_id":id,"prior":prior,"point":PointRecord::from(im)}),
                );
                panic!("doubling image coincides with an existing factor");
            }
            assert_ne!(im.x[1], 0, "doubling image lacks its separating component");
            let j = pool.len();
            all_by_x.insert(im.x, j);
            doubled.push(j);
            pool.push(Factor {
                p: im.into(),
                col: columns[id],
                coef: mm(2, coefficients[id], n),
                source_id: id,
                image: true,
            });
        }
        peels.push(doubled.clone());
        let mut action_checks = 0;
        for &id in &images {
            let factor = &pool[id];
            let mut fr: Point = factor.p.into();
            for j in 0..4 {
                let mut im = fr;
                for k in 0..3 {
                    assert!(c.on_curve(im));
                    let &other = all_by_x.get(&im.x).expect("action left expanded base");
                    let sign = matches_sign(&c, pool[other].p.into(), im);
                    let expected = mm(mm(factor.coef, fpow(lf, j, n), n), fpow(lc, k, n), n);
                    assert_eq!(pool[other].col, factor.col);
                    assert_eq!(
                        pool[other].coef,
                        if sign == 1 {
                            expected
                        } else {
                            negn(expected, n)
                        }
                    );
                    action_checks += 1;
                    im = psi(&c, im, omega);
                }
                fr = frob(&c, fr);
            }
        }
        let mut doubling_action_checks = 0;
        for &id in &doubled {
            let factor = &pool[id];
            let mut fr: Point = factor.p.into();
            for j in 0..4 {
                let mut im = fr;
                for k in 0..3 {
                    let &other = all_by_x.get(&im.x).expect("action left doubling base");
                    let sign = matches_sign(&c, pool[other].p.into(), im);
                    let coef = mm(mm(factor.coef, fpow(lf, j, n), n), fpow(lc, k, n), n);
                    assert_eq!(pool[other].col, factor.col);
                    assert_eq!(
                        pool[other].coef,
                        if sign == 1 { coef } else { negn(coef, n) }
                    );
                    doubling_action_checks += 1;
                    im = psi(&c, im, omega);
                }
                fr = frob(&c, fr);
            }
        }
        let mut zero_triples = BTreeSet::new();
        for i in 0..l0 {
            for outer in [-1, 1] {
                let mut im = base[i];
                let mut terms = Vec::new();
                let mut sum = INF;
                for _ in 0..3 {
                    let j = by_x[&im.x];
                    assert!(j < l0);
                    let sign = matches_sign(&c, base[j], im) * outer;
                    terms.push((j, sign));
                    sum = c.add(sum, signed(&c, base[j], sign));
                    im = psi(&c, im, omega);
                }
                assert_eq!(sum, INF);
                terms.sort();
                zero_triples.insert(terms);
            }
        }
        let geometry = json!({"p":p,"seed":seed,"n":n,"b0":raw["b0"],"d":c.f.d,"omega":omega,"lambda_cubic":lc,"lambda_frob":lf,"delta":delta,
            "original_physical_factors":base.len(),"l0_factors":l0,"l2_factors":base.len()-l0,"image_factors":images.len(),"expanded_physical_factors":pool.len(),"log_columns":reps.len(),
            "formula_checks":formula_matches,"image_action_checks":action_checks,"doubling_factors":doubled.len(),"doubling_action_checks":doubling_action_checks,"corrupt_eigenvalue_rejected":negative,"exceptions":exceptions,
            "pool":pool,"peel_sets":ARMS.iter().zip(&peels).map(|(name,ids)|json!({"arm":name,"ids":ids,"physical_peels":ids.len()})).collect::<Vec<_>>(),
            "public_geometry_and_validation_work":meter::snapshot()});
        Self {
            raw,
            c,
            g,
            q,
            n,
            p,
            seed,
            omega,
            lc,
            lf,
            l0,
            original_len,
            reps,
            anchor,
            pool,
            peels,
            zero_triples,
            geometry,
        }
    }
    fn residual(&self, item: &Value) -> Point {
        self.c.add(
            self.c.mul(self.g, item[0].as_u64().unwrap()),
            self.c.mul(self.q, item[1].as_u64().unwrap()),
        )
    }
    fn columns(&self, arm: usize) -> usize {
        if arm == 7 || arm == 8 || arm == 9 {
            self.l0 / 12
        } else {
            self.reps.len()
        }
    }
    fn usable(&self, arm: usize) -> BTreeSet<usize> {
        (0..self.l0)
            .chain(self.peels[arm].iter().copied())
            .collect()
    }
    fn row(&self, terms: &Terms, item: &Value, arm: usize) -> Vec<u64> {
        let mut sum = vec![0; self.reps.len()];
        for &(id, s) in terms {
            let f = &self.pool[id];
            sum[f.col] = (sum[f.col] + if s == 1 { f.coef } else { negn(f.coef, self.n) }) % self.n;
        }
        let cols = self.columns(arm);
        assert!(
            sum[cols..].iter().all(|&v| v == 0),
            "active-column projection lost a term"
        );
        let mut row = sum[1..cols].to_vec();
        row.push(negn(item[1].as_u64().unwrap(), self.n));
        row.push((item[0].as_u64().unwrap() + self.n - mm(self.anchor, sum[0], self.n)) % self.n);
        row
    }
    fn verify_terms(&self, terms: &Terms, r: Point) {
        assert_eq!(terms.len(), 4);
        assert!(terms.iter().all(|&(_, s)| s == 1 || s == -1));
        assert_eq!(terms.iter().map(|v| v.0).collect::<BTreeSet<_>>().len(), 4);
        let sum = terms.iter().fold(INF, |s, &(id, sign)| {
            self.c.add(s, signed(&self.c, self.pool[id].p.into(), sign))
        });
        assert_eq!(sum, r, "foreign signed relation");
    }
    fn verify_solution(&self, logs: &[u64], arm: usize) -> bool {
        let cols = self.columns(arm);
        let mut reps = vec![self.anchor];
        reps.extend(&logs[..cols - 1]);
        self.c.mul(self.g, logs[cols - 1]) == self.q
            && self.usable(arm).iter().all(|&id| {
                let f = &self.pool[id];
                self.c.mul(self.g, mm(reps[f.col], f.coef, self.n)) == f.p.into()
            })
    }
}
#[derive(Default)]
struct Matrix {
    rows: Vec<Vec<u64>>,
    n: u64,
    cols: usize,
}
impl Matrix {
    fn new(n: u64, cols: usize) -> Self {
        Self {
            n,
            cols,
            ..Self::default()
        }
    }
    fn rank(&self) -> usize {
        self.rows.len()
    }
    fn insert(&mut self, mut row: Vec<u64>) -> bool {
        assert_eq!(row.len(), self.cols + 1);
        for prior in &self.rows {
            let col = prior[..self.cols].iter().position(|&v| v != 0).unwrap();
            let a = row[col];
            if a != 0 {
                for j in col..=self.cols {
                    row[j] = (row[j] + self.n - mm(a, prior[j], self.n)) % self.n;
                }
            }
        }
        if let Some(col) = row[..self.cols].iter().position(|&v| v != 0) {
            let inv = fpow(row[col], self.n - 2, self.n);
            for v in &mut row[col..] {
                *v = mm(*v, inv, self.n);
            }
            self.rows.push(row);
            self.rows
                .sort_by_key(|r| r[..self.cols].iter().position(|&v| v != 0).unwrap());
            true
        } else {
            assert_eq!(row[self.cols], 0, "inconsistent candidate matrix");
            false
        }
    }
    fn solution(&self) -> Option<Vec<u64>> {
        if self.rank() != self.cols {
            return None;
        }
        let mut out = vec![0; self.cols];
        for i in (0..self.cols).rev() {
            let mut rhs = self.rows[i][self.cols];
            for j in i + 1..self.cols {
                rhs = (rhs + self.n - mm(self.rows[i][j], out[j], self.n)) % self.n;
            }
            out[i] = rhs;
        }
        Some(out)
    }
}
fn reference_logs(case: &Case) -> Vec<u64> {
    let c = &case.c;
    let n = case.n;
    let mut m = n.isqrt();
    if m * m < n {
        m += 1;
    }
    let mut baby = FxMap::<Key, u64>::default();
    baby.reserve(m as usize);
    let mut p = INF;
    for j in 0..m {
        baby.entry(key(p)).or_insert(j);
        p = c.add(p, case.g);
    }
    let step = c.neg(c.mul(case.g, m));
    let mut rep_logs = Vec::new();
    for (col, &id) in case.reps.iter().enumerate() {
        if col == 0 {
            rep_logs.push(case.anchor);
            continue;
        }
        let target: Point = case.pool[id].p.into();
        let mut v = target;
        let mut found = None;
        for i in 0..=m {
            if let Some(&j) = baby.get(&key(v)) {
                let k = i * m + j;
                if k < n && c.mul(case.g, k) == target {
                    found = Some(k);
                    break;
                }
            }
            v = c.add(v, step);
        }
        rep_logs.push(found.expect("reference BSGS failed"));
    }
    let logs: Vec<_> = case
        .pool
        .iter()
        .map(|f| mm(rep_logs[f.col], f.coef, n))
        .collect();
    for (&log, f) in logs.iter().zip(&case.pool) {
        assert_eq!(
            c.mul(case.g, log),
            f.p.into(),
            "reference physical log failed"
        );
    }
    logs
}
fn reference_sets(
    case: &Case,
    logs: &[u64],
    peels: &[usize],
    scalar: u64,
    pairs: &FxMap<u64, Vec<[(usize, i8); 2]>>,
) -> BTreeSet<Terms> {
    let mut out = BTreeSet::new();
    let n = case.n;
    for &j in peels {
        for sj in [-1, 1] {
            let u = (scalar + n - if sj == 1 { logs[j] } else { negn(logs[j], n) }) % n;
            for i in 0..case.l0 {
                for si in [-1, 1] {
                    let need = (u + n - if si == 1 { logs[i] } else { negn(logs[i], n) }) % n;
                    if let Some(ps) = pairs.get(&need) {
                        for pair in ps {
                            if pair.iter().any(|v| v.0 == i || v.0 == j) || i == j {
                                continue;
                            }
                            let mut terms = vec![pair[0], pair[1], (i, si), (j, sj)];
                            terms.sort();
                            out.insert(terms);
                        }
                    }
                }
            }
        }
    }
    out
}
fn census(case: &Case, out: &Path, selected: Option<usize>) {
    let known = case.raw["known_scalar"].as_u64().unwrap();
    assert_eq!(case.c.mul(case.g, known), case.q);
    let logs = reference_logs(case);
    let n = case.n;
    let mut pairs = FxMap::<u64, Vec<[(usize, i8); 2]>>::default();
    for j in 1..case.l0 {
        for i in 0..j {
            for sj in [-1, 1] {
                for si in [-1, 1] {
                    let a = if si == 1 { logs[i] } else { negn(logs[i], n) };
                    let b = if sj == 1 { logs[j] } else { negn(logs[j], n) };
                    pairs
                        .entry((a + b) % n)
                        .or_default()
                        .push([(i, si), (j, sj)]);
                }
            }
        }
    }
    let mut arms = Vec::new();
    for (ai, arm) in ARMS.iter().enumerate() {
        if selected.is_some_and(|v| v != ai) {
            continue;
        }
        let mut matrix = Matrix::new(n, case.columns(ai));
        let mut trace = Vec::new();
        let mut first = None;
        let mut attempts = 0u64;
        let mut at_rank = None;
        let mut total = 0;
        let mut unique = BTreeSet::new();
        for (qi, item) in case.raw["query_trace"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            let scalar = (item[0].as_u64().unwrap() + mm(known, item[1].as_u64().unwrap(), n)) % n;
            let r = case.c.mul(case.g, scalar);
            assert_eq!(r, case.residual(item));
            let got = reference_sets(case, &logs, &case.peels[ai], scalar, &pairs);
            let mut finite = 0;
            let mut infinities = 0;
            for &id in &case.peels[ai] {
                for s in [-1, 1] {
                    if r == signed(&case.c, case.pool[id].p.into(), s) {
                        infinities += 1;
                    } else {
                        finite += 1;
                    }
                }
            }
            attempts += finite;
            for terms in &got {
                case.verify_terms(terms, r);
                unique.insert(terms.clone());
                if matrix.rank() < matrix.cols {
                    matrix.insert(case.row(terms, item, ai));
                }
            }
            total += got.len();
            if matrix.rank() == matrix.cols && first.is_none() {
                first = Some(qi);
                at_rank = Some(attempts);
            }
            trace.push(json!({"query_index":qi,"a":item[0],"b":item[1],"signed_relations":got,"rank_after":matrix.rank(),"finite_signed_peels":finite,"infinity_peels":infinities}));
        }
        let solution = matrix.solution();
        let verified = solution.as_ref().map(|s| case.verify_solution(s, ai));
        let recovered = solution.as_ref().map(|v| v[matrix.cols - 1]);
        let record = json!({"p":case.p,"seed":case.seed,"arm":arm,"queries":trace.len(),"physical_peels":case.peels[ai].len(),"usable_physical_factors":case.usable(ai).len(),"signed_relations":total,"unique_signed_tuples_across_stream":unique.len(),"finite_signed_peels":attempts,"first_full_rank_query":first,"finite_peels_through_rank":at_rank,"rank":matrix.rank(),"columns":matrix.cols,"recovered":recovered,"all_target_and_factor_logs_verified":verified,"censored":first.is_none(),"trace":trace});
        write(&out.join(format!("{arm}.json")), &record);
        arms.push(json!({"arm":arm,"queries":record["queries"],"physical_peels":record["physical_peels"],"signed_relations":total,"finite_signed_peels":attempts,"first_full_rank_query":first,"finite_peels_through_rank":at_rank,"rank":matrix.rank(),"columns":matrix.cols,"recovered":recovered,"censored":first.is_none()}));
        eprintln!(
            "reference p={} seed={} arm={arm} rank={}/{} first={first:?}",
            case.p,
            case.seed,
            matrix.rank(),
            matrix.cols
        );
    }
    write(
        &out.join("summary.json"),
        &json!({"schema":"quartic-twist-image-reference/v1","status":"pass","p":case.p,"seed":case.seed,"input_sha256":INPUT_SHA,"arms":arms,"reference_work":meter::snapshot()}),
    );
}
pub(super) fn run(mode: &str, path: &Path, sha: &str, p: u64, seed: u64, out: &Path) {
    let raw = load(path, sha, p, seed);
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    write(&out.join("geometry.json"), &case.geometry);
    meter::reset();
    if mode == "--image-census" {
        census(&case, out, None);
    } else {
        collect(&case, out);
    }
}

fn fm(a: u64, b: u64, p: u64) -> u64 {
    meter::fp(1);
    a * b % p
}
fn polynomial_value(poly: &Poly, x: &[u64], p: u64) -> u64 {
    poly.iter().fold(0, |sum, (e, &c)| {
        let mut v = c;
        for i in 0..3 {
            v = fm(v, fpow(x[i], e[i] as u64, p), p);
        }
        (sum + v) % p
    })
}
fn polynomial_query(
    case: &Case,
    h: &integer::ZPoly,
    arm: usize,
    r: Point,
) -> (BTreeSet<Terms>, Value) {
    let c = &case.c;
    let f = c.f;
    let p = f.p;
    let mut got = BTreeSet::new();
    let mut systems = 0;
    let mut infinities = 0;
    let mut f4_muls = 0;
    let mut fields = 0;
    let mut split_failures = 0;
    let mut foreign_or_repeat = 0;
    let mut undetermined = Vec::new();
    let mut retained_roots = Vec::new();
    let mut max_rows = 0;
    let mut max_cols = 0;
    let by_x: BTreeMap<_, _> = case.pool[..case.l0]
        .iter()
        .enumerate()
        .map(|(id, v)| (v.p.x, id))
        .collect();
    let mut peel_work = meter::Counts::default();
    let mut restriction_work = meter::Counts::default();
    for &id in &case.peels[arm] {
        for sign in [-1, 1] {
            let before = meter::snapshot();
            let u = c.add(r, c.neg(signed(c, case.pool[id].p.into(), sign)));
            let work = meter::difference(before, meter::snapshot());
            peel_work.fp_muls += work.fp_muls;
            peel_work.subgroup_muls += work.subgroup_muls;
            peel_work.group_add_calls += work.group_add_calls;
            if u.inf {
                infinities += 1;
                for triple in &case.zero_triples {
                    if triple.iter().any(|v| v.0 == id) {
                        continue;
                    }
                    let mut terms = triple.clone();
                    terms.push((id, sign));
                    terms.sort();
                    case.verify_terms(&terms, r);
                    got.insert(terms);
                }
                continue;
            }
            systems += 1;
            let before = meter::snapshot();
            let equations = restrict_exact(f, c.b, h, u.x);
            let work = meter::difference(before, meter::snapshot());
            restriction_work.fp_muls += work.fp_muls;
            restriction_work.subgroup_muls += work.subgroup_muls;
            restriction_work.group_add_calls += work.group_add_calls;
            let input: Vec<_> = equations
                .iter()
                .map(|poly| {
                    f4_fp::normalise(
                        &poly
                            .iter()
                            .map(|(e, &v)| (e.iter().map(|&x| x as u32).collect(), v))
                            .collect::<Vec<_>>(),
                        p,
                        f4_fp::Ordering::Grevlex,
                    )
                })
                .collect();
            let opts = f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, 12)
                .with_budget(Duration::from_secs(1200));
            let solved = f4_fp::solve(&input, 3, p, &opts);
            f4_muls += solved.field_ops;
            max_rows = max_rows.max(solved.max_rows);
            max_cols = max_cols.max(solved.max_cols);
            let mut roots = match solved.verdict {
                f4_fp::Verdict::Solutions(v) => v,
                f4_fp::Verdict::Inconsistent => Vec::new(),
                f4_fp::Verdict::Undetermined => {
                    undetermined.push(json!({"peel_id":id,"sign":sign,"remaining_x":u.x,"equations":equations.iter().map(|poly|poly.iter().map(|(e,v)|json!([e,v])).collect::<Vec<_>>()).collect::<Vec<_>>() }));
                    Vec::new()
                }
            };
            roots.sort();
            roots.dedup();
            fields += roots.len();
            if !roots.is_empty() {
                retained_roots.push(json!({"peel_id":id,"sign":sign,"roots":roots}));
            }
            for e in roots {
                assert_eq!(e.len(), 3);
                assert!(e.iter().all(|&v| v < p));
                assert!(
                    equations
                        .iter()
                        .all(|poly| polynomial_value(poly, &e, p) == 0),
                    "F4 returned a false field root"
                );
                let xs: Vec<_> = (0..p)
                    .filter(|&x| {
                        let a = (fm((x + p - e[0]) % p, x, p) + e[1]) % p;
                        (fm(a, x, p) + p - e[2]) % p == 0
                    })
                    .collect();
                if xs.len() != 3 {
                    split_failures += 1;
                    continue;
                }
                let ids: Option<Vec<_>> = xs
                    .iter()
                    .map(|&x| by_x.get(&f.scalar(x)).copied())
                    .collect();
                let Some(ids) = ids else {
                    foreign_or_repeat += 1;
                    continue;
                };
                if ids.contains(&id) {
                    foreign_or_repeat += 1;
                    continue;
                }
                for mask in 0..8 {
                    let mut sum = signed(c, case.pool[id].p.into(), sign);
                    let mut terms = vec![(id, sign)];
                    for (j, &k) in ids.iter().enumerate() {
                        let s = if mask & (1 << j) == 0 { 1 } else { -1 };
                        sum = c.add(sum, signed(c, case.pool[k].p.into(), s));
                        terms.push((k, s));
                    }
                    if sum == r {
                        terms.sort();
                        case.verify_terms(&terms, r);
                        got.insert(terms);
                    }
                }
            }
        }
    }
    (
        got,
        json!({"systems":systems,"infinity_peels":infinities,"f4_row_reduction_fp_muls":f4_muls,"field_roots":fields,"split_failures":split_failures,"foreign_or_repeat":foreign_or_repeat,"undetermined":undetermined,"nonempty_root_records":retained_roots,"max_rows":max_rows,"max_cols":max_cols,"peel_work":peel_work,"restriction_work":restriction_work}),
    )
}
fn collect_arm(case: &Case, arm: usize, out: &Path) {
    let (_, h) = integer::derive();
    let mut matrix = Matrix::new(case.n, case.columns(arm));
    let mut queries = 0;
    let mut total = 0;
    let mut systems = 0;
    let mut f4_muls = 0;
    let mut failures = 0;
    let mut first = None;
    let initial = meter::snapshot();
    for (qi, item) in case.raw["query_trace"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let before = meter::snapshot();
        let r = case.residual(item);
        let residual_work = meter::difference(before, meter::snapshot());
        let (got, mut record) = polynomial_query(case, &h, arm, r);
        let before = meter::snapshot();
        for terms in &got {
            if matrix.rank() < matrix.cols {
                matrix.insert(case.row(terms, item, arm));
            }
        }
        let matrix_work = meter::difference(before, meter::snapshot());
        queries += 1;
        total += got.len();
        systems += record["systems"].as_u64().unwrap();
        f4_muls += record["f4_row_reduction_fp_muls"].as_u64().unwrap();
        failures += record["undetermined"].as_array().unwrap().len();
        record["query_index"] = json!(qi);
        record["a"] = item[0].clone();
        record["b"] = item[1].clone();
        record["signed_relations"] = json!(got);
        record["rank_after"] = json!(matrix.rank());
        record["residual_work"] = json!(residual_work);
        record["matrix_work"] = json!(matrix_work);
        write(&out.join(format!("q{qi:05}.json")), &record);
        eprintln!(
            "candidate p={} seed={} arm={} query={qi} rank={}/{}",
            case.p,
            case.seed,
            ARMS[arm],
            matrix.rank(),
            matrix.cols
        );
        if matrix.rank() == matrix.cols {
            first = Some(qi);
            break;
        }
    }
    let before = meter::snapshot();
    let solution = matrix.solution();
    let verified = solution.as_ref().map(|v| case.verify_solution(v, arm));
    let answer_work = meter::difference(before, meter::snapshot());
    let recovered = solution.as_ref().map(|v| v[matrix.cols - 1]);
    if solution.is_some() {
        assert_eq!(
            verified,
            Some(true),
            "candidate matrix answer failed group checks"
        );
    }
    let status = if failures > 0 {
        "partial"
    } else if first.is_none() {
        "censored"
    } else {
        "pass"
    };
    write(
        &out.join("summary.json"),
        &json!({"schema":"quartic-twist-image-polynomial/v1","status":status,"candidate_id":null,"p":case.p,"seed":case.seed,"arm":ARMS[arm],"input_sha256":INPUT_SHA,
        "queries":queries,"stream_cap":case.raw["query_trace"].as_array().unwrap().len(),"physical_peels":case.peels[arm].len(),"usable_physical_factors":case.usable(arm).len(),"columns":matrix.cols,"rank":matrix.rank(),"first_full_rank_query":first,
        "signed_relations":total,"systems":systems,"undetermined":failures,"f4_row_reduction_fp_muls":f4_muls,"native_work":meter::difference(initial,meter::snapshot()),"answer_work":answer_work,
        "recovered":recovered,"solved_logs":solution,"all_usable_factor_logs_verified":verified,"censored":first.is_none(),
        "unpriced":["cold base/model construction","geometry and endomorphism verification replay inputs","F4 arithmetic outside row reduction","allocation/hash/tree CPU","calibrated GAE","isolated timing","external solver comparison"]}),
    );
}
fn collect(case: &Case, out: &Path) {
    for arm in 0..ARMS.len() {
        meter::reset();
        let dest = out.join(ARMS[arm]);
        std::fs::create_dir_all(&dest).unwrap();
        collect_arm(case, arm, &dest);
    }
}
pub(super) fn run_arm(path: &Path, sha: &str, p: u64, seed: u64, arm: &str, out: &Path) {
    let ai = ARMS
        .iter()
        .position(|&v| v == arm)
        .expect("unregistered arm");
    let raw = load(path, sha, p, seed);
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    write(&out.join("geometry.json"), &case.geometry);
    meter::reset();
    collect_arm(&case, ai, out);
}
pub(super) fn run_shard(path: &Path, sha: &str, index: usize, out: &Path) {
    // The encoded nine-arm mapping is immutable even when controls are added.
    const SHARD_ARMS: usize = 9;
    assert!(index < PRIMES.len() * 3 * SHARD_ARMS);
    let arm = index % SHARD_ARMS;
    let seed = (index / SHARD_ARMS) % 3;
    let p = PRIMES[index / (SHARD_ARMS * 3)];
    run_arm(path, sha, p, seed as u64, ARMS[arm], out);
}
pub(super) fn reference_arm(path: &Path, sha: &str, p: u64, seed: u64, arm: &str, out: &Path) {
    let ai = ARMS.iter().position(|&v| v == arm).unwrap();
    let raw = load(path, sha, p, seed);
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    write(&out.join("geometry.json"), &case.geometry);
    meter::reset();
    census(&case, out, Some(ai));
}
fn independent_rref(rows: &[Vec<u64>], cols: usize, n: u64) -> (usize, Option<Vec<u64>>) {
    let mut a = rows.to_vec();
    let mut rank = 0;
    for col in 0..cols {
        let Some(pivot) = (rank..a.len()).find(|&i| a[i][col] != 0) else {
            continue;
        };
        a.swap(rank, pivot);
        let inverse = fpow(a[rank][col], n - 2, n);
        for j in col..=cols {
            a[rank][j] = mm(a[rank][j], inverse, n);
        }
        for i in 0..a.len() {
            if i == rank {
                continue;
            }
            let scale = a[i][col];
            for j in col..=cols {
                a[i][j] = (a[i][j] + n - mm(scale, a[rank][j], n)) % n;
            }
        }
        rank += 1;
        if rank == a.len() {
            break;
        }
    }
    for row in a.iter().skip(rank) {
        assert!(row[..cols].iter().all(|&v| v == 0));
        assert_eq!(row[cols], 0, "inconsistent audited row");
    }
    (
        rank,
        if rank == cols {
            Some(a.iter().take(cols).map(|r| r[cols]).collect())
        } else {
            None
        },
    )
}
fn canonical_row(mut row: Vec<u64>, n: u64) -> Option<Vec<u64>> {
    let first = row.iter().position(|&v| v != 0)?;
    let inv = fpow(row[first], n - 2, n);
    for v in &mut row {
        *v = mm(*v, inv, n);
    }
    Some(row)
}
pub(super) fn audit(path: &Path, sha: &str, reference: &Path, candidate: &Path, out: &Path) {
    let rb = std::fs::read(reference).unwrap();
    let reference: Value = serde_json::from_slice(&rb).unwrap();
    let summary: Value =
        serde_json::from_slice(&std::fs::read(candidate.join("summary.json")).unwrap()).unwrap();
    let p = reference["p"].as_u64().unwrap();
    let seed = reference["seed"].as_u64().unwrap();
    let arm = reference["arm"].as_str().unwrap();
    let ai = ARMS.iter().position(|&v| v == arm).unwrap();
    assert_eq!(summary["p"], p);
    assert_eq!(summary["seed"], seed);
    assert_eq!(summary["arm"], arm);
    assert_eq!(summary["input_sha256"], sha);
    let raw = load(path, sha, p, seed);
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    let cols = case.columns(ai);
    assert_eq!(summary["columns"], cols);
    let queries = summary["queries"].as_u64().unwrap() as usize;
    let mut rows = Vec::new();
    let mut unique = BTreeSet::new();
    let mut zero = 0;
    let mut differences = Vec::new();
    let mut rank = 0;
    let mut solution = None;
    let mut first = None;
    let mut total = 0;
    let mut systems = 0;
    for qi in 0..queries {
        let rec: Value = serde_json::from_slice(
            &std::fs::read(candidate.join(format!("q{qi:05}.json"))).unwrap(),
        )
        .unwrap();
        let item = &case.raw["query_trace"][qi];
        assert_eq!(rec["query_index"], qi);
        assert_eq!(rec["a"], item[0]);
        assert_eq!(rec["b"], item[1]);
        let r = case.residual(item);
        let got: BTreeSet<Terms> = serde_json::from_value(rec["signed_relations"].clone()).unwrap();
        let expected: BTreeSet<Terms> =
            serde_json::from_value(reference["trace"][qi]["signed_relations"].clone()).unwrap();
        if got != expected {
            differences.push(json!({"query":qi,"missing":expected.difference(&got).collect::<Vec<_>>(),"extra":got.difference(&expected).collect::<Vec<_>>() }));
        }
        for terms in &got {
            case.verify_terms(terms, r);
            if ai == 8 {
                assert!(terms.iter().all(|v| v.0 < case.l0));
            } else {
                assert_eq!(terms.iter().filter(|v| v.0 < case.l0).count(), 3);
                assert_eq!(
                    terms
                        .iter()
                        .filter(|v| case.peels[ai].contains(&v.0))
                        .count(),
                    1
                );
            }
            let row = case.row(terms, item, ai);
            if let Some(canon) = canonical_row(row.clone(), case.n) {
                unique.insert(canon);
            } else {
                zero += 1;
            }
            if rank < cols {
                rows.push(row);
            }
        }
        if rank < cols {
            let checked = independent_rref(&rows, cols, case.n);
            rank = checked.0;
            solution = checked.1;
        }
        if rank == cols && first.is_none() {
            first = Some(qi);
        }
        assert_eq!(
            rec["rank_after"], rank,
            "candidate rank disagrees with independent RREF"
        );
        assert_eq!(
            reference["trace"][qi]["rank_after"], rank,
            "reference rank disagrees with independent RREF"
        );
        total += got.len();
        systems += rec["systems"].as_u64().unwrap();
        assert_eq!(
            rec["systems"],
            reference["trace"][qi]["finite_signed_peels"]
        );
    }
    assert_eq!(summary["rank"], rank);
    assert_eq!(summary["signed_relations"], total);
    assert_eq!(summary["systems"], systems);
    assert_eq!(summary["first_full_rank_query"], json!(first));
    let verified = solution.as_ref().map(|v| case.verify_solution(v, ai));
    if solution.is_some() {
        assert_eq!(verified, Some(true));
        assert_eq!(summary["solved_logs"], json!(solution));
    }
    let pass = differences.is_empty() && summary["undetermined"] == 0;
    write(
        &out.join("audit.json"),
        &json!({"schema":"quartic-twist-image-audit/v1","status":if pass{"pass"}else{"partial"},"p":p,"seed":seed,"arm":arm,"reference_sha256":hex::encode(hash::sha256::sha256(&rb)),"queries_checked":queries,"signed_relations":total,"unique_projective_augmented_rows":unique.len(),"zero_rows":zero,"rank":rank,"columns":cols,"first_full_rank_query":first,"recovered":solution.as_ref().map(|v|v[cols-1]),"all_usable_factor_logs_verified":verified,"differences":differences}),
    );
    assert!(
        pass,
        "complete-set or solver discrepancy retained in audit.json"
    );
}

pub(super) fn audit_mutations(
    path: &Path,
    sha: &str,
    reference: &Path,
    candidate: &Path,
    out: &Path,
) {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    audit(path, sha, reference, candidate, &out.join("positive"));
    let mut changed: Value = serde_json::from_slice(&std::fs::read(reference).unwrap()).unwrap();
    let qi = changed["trace"]
        .as_array()
        .unwrap()
        .iter()
        .position(|v| !v["signed_relations"].as_array().unwrap().is_empty())
        .unwrap();
    let omitted = changed["trace"][qi]["signed_relations"]
        .as_array_mut()
        .unwrap()
        .remove(0);
    let mutated = out.join("omitted-reference.json");
    write(&mutated, &changed);
    let rejected = std::panic::catch_unwind(|| {
        audit(path, sha, &mutated, candidate, &out.join("omission-audit"))
    })
    .is_err();
    assert!(rejected);
    let check: Value =
        serde_json::from_slice(&std::fs::read(out.join("omission-audit/audit.json")).unwrap())
            .unwrap();
    assert_eq!(check["status"], "partial");
    assert_eq!(check["differences"].as_array().unwrap().len(), 1);
    assert_eq!(check["differences"][0]["query"], qi);
    assert_eq!(check["differences"][0]["extra"], json!([omitted]));
    let mut bad_bytes = std::fs::read(path).unwrap();
    bad_bytes.push(b' ');
    let bad_tape = out.join("hash-mutated-tape.ndjson");
    std::fs::write(&bad_tape, bad_bytes).unwrap();
    let hash_rejected = std::panic::catch_unwind(|| {
        audit(
            &bad_tape,
            sha,
            reference,
            candidate,
            &out.join("hash-audit"),
        )
    })
    .is_err();
    assert!(hash_rejected && !out.join("hash-audit").exists());
    write(
        &out.join("negative-controls.json"),
        &json!({"status":"pass","positive_audit_passed":true,"omitted_reference_relation_rejected":true,"omission_query":qi,"omitted_relation":omitted,"input_hash_mutation_rejected_before_output_directory":true}),
    );
}

pub(super) fn baseline_parity(reference_root: &Path, old_root: &Path, out: &Path) {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let mut cases = Vec::new();
    let mut total = 0;
    for (p, seed) in [
        (37, 0),
        (73, 0),
        (73, 1),
        (73, 2),
        (97, 0),
        (97, 1),
        (97, 2),
    ] {
        let stem = format!("p{p}-seed{seed}");
        let reference: Value = serde_json::from_slice(
            &std::fs::read(reference_root.join(&stem).join("original.json")).unwrap(),
        )
        .unwrap();
        let mut queries = 0;
        let mut relations = 0;
        loop {
            let path = old_root.join(&stem).join(format!("q{queries:04}.json"));
            if !path.exists() {
                break;
            }
            let old: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
            let new = &reference["trace"][queries];
            for field in ["query_index", "a", "b", "signed_relations", "rank_after"] {
                assert_eq!(
                    old[field], new[field],
                    "old S4 parity mismatch {stem} q{queries} {field}"
                );
            }
            assert_eq!(old["systems"], new["finite_signed_peels"]);
            relations += old["signed_relations"].as_array().unwrap().len();
            queries += 1;
        }
        assert!(queries > 0);
        total += queries;
        cases.push(json!({"p":p,"seed":seed,"queries":queries,"signed_relations":relations,"all_sets_and_rank_equal":true}));
    }
    write(
        &out.join("baseline-parity.json"),
        &json!({"status":"pass","baseline":"c7cecbd07f437b600b5893be61ad0eedf38d2368","queries":total,"cases":cases}),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_reorders_pivots_and_solves() {
        meter::configure(37, 101);
        let mut m = Matrix::new(101, 3);
        assert!(m.insert(vec![0, 0, 1, 4]));
        assert!(m.insert(vec![1, 2, 3, 26]));
        assert!(m.insert(vec![0, 1, 1, 9]));
        assert_eq!(m.solution(), Some(vec![4, 5, 4]));
        assert!(!m.insert(vec![1, 0, 0, 4]));
    }
    #[test]
    fn modular_products_remain_exact_above_32_bits() {
        let n = 12896490241u64;
        meter::configure(337, n);
        assert_eq!(mm(n - 1, n - 1, n), 1);
        assert_eq!(mm(n - 2, n - 3, n), 6);
    }
    #[test]
    fn independent_rref_matches_sparse_pivot_insert_over_wide_prime() {
        let n = 4_294_967_311u64;
        assert!((3..=n.isqrt()).step_by(2).all(|d| n % d != 0));
        meter::configure(337, n);
        let answer = [n - 2, n - 3, 5];
        let mut matrix = Matrix::new(n, 3);
        let mut rows = Vec::new();
        for left in [[0, 0, 1], [n - 1, 2, 3], [4, n - 1, 6], [7, 8, n - 1]] {
            let rhs = left
                .iter()
                .zip(answer)
                .fold(0, |s, (&x, y)| (s + mm(x, y, n)) % n);
            let mut row = left.to_vec();
            row.push(rhs);
            matrix.insert(row.clone());
            rows.push(row);
            let r = independent_rref(&rows, 3, n);
            assert_eq!(matrix.rank(), r.0);
            assert_eq!(matrix.solution(), r.1);
        }
        assert_eq!(matrix.solution(), Some(answer.to_vec()));
    }
}
