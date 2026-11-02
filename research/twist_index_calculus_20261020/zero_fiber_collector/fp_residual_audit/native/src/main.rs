//! Immutable-record counterfactual audit. No F4, collector, secret or timer.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const WORKLOAD_SHA: &str = "17ecb4c302526bc58e592466071e1c2704eaaf43a0a0ca1efe563e1bd69b3c10";
const PRIMES: [u64; 7] = [37, 73, 97, 157, 229, 241, 337];
type E = [u64; 4];
type Terms = Vec<(usize, i8)>;
fn mm(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn neg(a: u64, n: u64) -> u64 {
    (n - a) % n
}
fn pow(mut a: u64, mut k: u64, n: u64) -> u64 {
    let mut v = 1;
    while k != 0 {
        if k & 1 != 0 {
            v = mm(v, a, n);
        }
        a = mm(a, a, n);
        k >>= 1;
    }
    v
}
fn digest(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn load(path: &Path, sha: &str) -> Value {
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(digest(&bytes), sha, "input hash: {}", path.display());
    serde_json::from_slice(&bytes).unwrap()
}
fn u(v: &Value) -> u64 {
    v.as_u64().unwrap()
}
fn s(v: &Value) -> &str {
    v.as_str().unwrap()
}
fn arr(v: &Value) -> E {
    serde_json::from_value(v.clone()).unwrap()
}
fn write(path: &Path, v: &Value) {
    std::fs::write(path, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}

#[derive(Clone, Copy)]
struct Field {
    p: u64,
    d: u64,
}
impl Field {
    fn new(p: u64) -> Self {
        assert!(PRIMES.contains(&p));
        Self {
            p,
            d: (2..p).find(|&d| pow(d, (p - 1) / 2, p) == p - 1).unwrap(),
        }
    }
    fn scalar(self, x: u64) -> E {
        [x % self.p, 0, 0, 0]
    }
    fn add(self, a: E, b: E) -> E {
        std::array::from_fn(|i| (a[i] + b[i]) % self.p)
    }
    fn neg(self, a: E) -> E {
        std::array::from_fn(|i| neg(a[i], self.p))
    }
    fn sub(self, a: E, b: E) -> E {
        self.add(a, self.neg(b))
    }
    fn mul(self, a: E, b: E) -> E {
        let mut c = [0u64; 7];
        for i in 0..4 {
            for j in 0..4 {
                c[i + j] = (c[i + j] + a[i] * b[j]) % self.p;
            }
        }
        for k in (4..7).rev() {
            c[k - 4] = (c[k - 4] + self.d * c[k]) % self.p;
        }
        [c[0], c[1], c[2], c[3]]
    }
    fn pow(self, mut a: E, mut k: u64) -> E {
        let mut v = self.scalar(1);
        while k != 0 {
            if k & 1 != 0 {
                v = self.mul(v, a);
            }
            a = self.mul(a, a);
            k >>= 1;
        }
        v
    }
    fn inv(self, a: E) -> E {
        assert_ne!(a, [0; 4]);
        self.pow(a, self.p.pow(4) - 2)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Point {
    x: E,
    y: E,
    inf: bool,
}
const O: Point = Point {
    x: [0; 4],
    y: [0; 4],
    inf: true,
};
fn reject(p: Point) -> bool {
    !p.inf && p.x[1..] == [0, 0, 0]
}
struct Curve {
    f: Field,
    b: E,
}
impl Curve {
    fn neg(&self, p: Point) -> Point {
        if p.inf {
            O
        } else {
            Point {
                y: self.f.neg(p.y),
                ..p
            }
        }
    }
    fn signed(&self, p: Point, sign: i8) -> Point {
        assert!([-1, 1].contains(&sign));
        if sign == 1 {
            p
        } else {
            self.neg(p)
        }
    }
    fn on(&self, p: Point) -> bool {
        p.inf || self.f.mul(p.y, p.y) == self.f.add(self.f.mul(self.f.mul(p.x, p.x), p.x), self.b)
    }
    fn add(&self, a: Point, b: Point) -> Point {
        if a.inf {
            return b;
        }
        if b.inf {
            return a;
        }
        let f = self.f;
        let m = if a.x == b.x {
            if a.y != b.y || a.y == [0; 4] {
                return O;
            }
            f.mul(
                f.mul(f.scalar(3), f.mul(a.x, a.x)),
                f.inv(f.mul(f.scalar(2), a.y)),
            )
        } else {
            f.mul(f.sub(b.y, a.y), f.inv(f.sub(b.x, a.x)))
        };
        let x = f.sub(f.sub(f.mul(m, m), a.x), b.x);
        Point {
            x,
            y: f.sub(f.mul(m, f.sub(a.x, x)), a.y),
            inf: false,
        }
    }
    fn mul(&self, mut a: Point, mut k: u64) -> Point {
        let mut v = O;
        while k != 0 {
            if k & 1 != 0 {
                v = self.add(v, a);
            }
            a = self.add(a, a);
            k >>= 1;
        }
        v
    }
}
#[derive(Clone)]
struct Factor {
    point: Point,
    col: usize,
    coefficient: u64,
}
struct Case {
    c: Curve,
    pool: Vec<Factor>,
    n: u64,
    original_len: usize,
    l0: usize,
    active: usize,
    anchor: u64,
    g: Point,
    actions: Vec<Vec<(usize, i8)>>,
}
impl Case {
    fn new(raw: &Value) -> Self {
        let p = u(&raw["p"]);
        let n = u(&raw["n"]);
        assert!(n > 3 && n % 2 == 1 && (3..=n.isqrt()).step_by(2).all(|d| n % d != 0));
        let c = Curve {
            f: Field::new(p),
            b: [0, 0, 0, u(&raw["b0"])],
        };
        let g = Point {
            x: arr(&raw["generator_x"]),
            y: arr(&raw["generator_y"]),
            inf: false,
        };
        assert!(c.on(g));
        assert_eq!(c.mul(g, n), O);
        let base: Vec<Point> = raw["base_coordinates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| Point {
                x: arr(&v[0]),
                y: arr(&v[1]),
                inf: false,
            })
            .collect();
        assert!(base.iter().all(|&p| c.on(p) && p.x != [0; 4]));
        let l0 = base.iter().take_while(|p| p.x[1..] == [0, 0, 0]).count();
        assert!(l0 > 0 && l0 < base.len() && l0 % 12 == 0);
        let anchor = if base[0] == g {
            1
        } else {
            assert_eq!(base[0], c.neg(g));
            n - 1
        };
        let mut scalars = BTreeSet::new();
        for j in 0..4 {
            for k in 0..3 {
                let t = mm(
                    pow(u(&raw["lambda_frob"]), j, n),
                    pow(u(&raw["lambda_cubic"]), k, n),
                    n,
                );
                scalars.extend([t, neg(t, n)]);
            }
        }
        assert_eq!(scalars.len(), 24);
        assert!(!scalars.contains(&0));
        for &a in &scalars {
            for &b in &scalars {
                assert!(scalars.contains(&mm(a, b, n)));
            }
        }
        let byx: BTreeMap<E, usize> = base.iter().enumerate().map(|(i, p)| (p.x, i)).collect();
        assert_eq!(byx.len(), base.len());
        let mut cols = vec![usize::MAX; base.len()];
        let mut coefs = vec![0; base.len()];
        let mut next = 0;
        for i in 0..base.len() {
            if cols[i] != usize::MAX {
                continue;
            }
            let col = next;
            next += 1;
            for &scalar in &scalars {
                let im = c.mul(base[i], scalar);
                let id = byx[&im.x];
                let coef = if im == base[id] {
                    scalar
                } else {
                    assert_eq!(im, c.neg(base[id]));
                    neg(scalar, n)
                };
                if cols[id] == usize::MAX {
                    cols[id] = col;
                    coefs[id] = coef;
                } else {
                    assert_eq!((cols[id], coefs[id]), (col, coef));
                }
            }
        }
        assert_eq!(next * 12, base.len());
        let mut pool: Vec<Factor> = base
            .iter()
            .enumerate()
            .map(|(i, &point)| Factor {
                point,
                col: cols[i],
                coefficient: coefs[i],
            })
            .collect();
        let omega = u(&raw["omega"]);
        let delta = (n + 1 - u(&raw["lambda_cubic"])) % n;
        assert!(omega != 1 && pow(omega, 3, p) == 1 && delta != 0);
        for (i, &pt) in base.iter().enumerate() {
            let psi = Point {
                x: c.f.mul(c.f.scalar(omega), pt.x),
                ..pt
            };
            assert_eq!(psi, c.mul(pt, u(&raw["lambda_cubic"])));
            let point = c.add(pt, c.neg(psi));
            assert_eq!(point, c.mul(pt, delta));
            assert!(!point.inf && c.on(point));
            pool.push(Factor {
                point,
                col: cols[i],
                coefficient: mm(delta, coefs[i], n),
            });
        }
        for (i, &pt) in base[..l0].iter().enumerate() {
            let point = c.add(pt, pt);
            assert!(!point.inf && c.on(point));
            pool.push(Factor {
                point,
                col: cols[i],
                coefficient: mm(2, coefs[i], n),
            });
        }
        let all: BTreeMap<(E, E), (usize, i8)> = pool
            .iter()
            .enumerate()
            .flat_map(|(i, f)| {
                [
                    ((f.point.x, f.point.y), (i, 1)),
                    ((f.point.x, c.neg(f.point).y), (i, -1)),
                ]
            })
            .collect();
        assert_eq!(all.len(), 2 * pool.len());
        let actions: Vec<Vec<(usize, i8)>> = scalars
            .iter()
            .map(|&scalar| {
                pool.iter()
                    .map(|f| {
                        let im = c.mul(f.point, scalar);
                        let to = all[&(im.x, im.y)];
                        assert_eq!(pool[to.0].col, f.col);
                        let coeff = mm(f.coefficient, scalar, n);
                        assert_eq!(
                            pool[to.0].coefficient,
                            if to.1 == 1 { coeff } else { neg(coeff, n) }
                        );
                        to
                    })
                    .collect()
            })
            .collect();
        // Degree-four power basis for B, not an assumption imported from a name.
        let mut axes = BTreeSet::new();
        for k in 0..4 {
            let b = c.f.pow(c.b, k);
            let a: Vec<_> = b
                .iter()
                .enumerate()
                .filter(|(_, v)| **v != 0)
                .map(|(i, _)| i)
                .collect();
            assert_eq!(a.len(), 1);
            axes.insert(a[0]);
        }
        assert_eq!(axes.len(), 4);
        Self {
            c,
            pool,
            n,
            original_len: base.len(),
            l0,
            active: l0 / 12,
            anchor,
            g,
            actions,
        }
    }
    fn physical(&self, terms: &Terms, double: bool) -> bool {
        let first = self.original_len * if double { 2 } else { 1 };
        terms.len() == 5
            && terms.windows(2).all(|w| w[0] < w[1])
            && terms
                .iter()
                .all(|&(i, t)| i < self.pool.len() && [-1, 1].contains(&t))
            && terms.iter().map(|t| t.0).collect::<BTreeSet<_>>().len() == 5
            && terms.iter().filter(|t| t.0 < self.l0).count() == 4
            && terms
                .iter()
                .filter(|t| t.0 >= first && t.0 < first + self.l0)
                .count()
                == 1
            && terms
                .iter()
                .map(|t| self.pool[t.0].point.x)
                .collect::<BTreeSet<_>>()
                .len()
                == 5
            && terms.iter().fold(O, |sum, &(id, t)| {
                self.c.add(sum, self.c.signed(self.pool[id].point, t))
            }) == O
    }
    fn row(&self, terms: &Terms) -> Vec<u64> {
        let mut v = vec![0; self.active];
        for &(id, t) in terms {
            let f = &self.pool[id];
            assert!(f.col < self.active);
            v[f.col] = (v[f.col]
                + if t == 1 {
                    f.coefficient
                } else {
                    neg(f.coefficient, self.n)
                })
                % self.n;
        }
        let mut row = v[1..].to_vec();
        row.push(neg(mm(self.anchor, v[0], self.n), self.n));
        row
    }
    fn rows(&self, physical: &BTreeSet<Terms>) -> Value {
        let rows: Vec<_> = physical.iter().map(|t| self.row(t)).collect();
        let zeros = rows.iter().filter(|r| r.iter().all(|&a| a == 0)).count();
        let projective: BTreeSet<_> = rows.iter().filter_map(|r| canonical(r, self.n)).collect();
        let (rank, solution) = rref(rows, self.active - 1, self.n);
        let checked = solution
            .as_ref()
            .map(|x| {
                let mut logs = vec![self.anchor];
                logs.extend(x);
                self.pool
                    .iter()
                    .filter(|f| f.col < self.active)
                    .all(|f| self.c.mul(self.g, mm(logs[f.col], f.coefficient, self.n)) == f.point)
            })
            .unwrap_or(false);
        assert_eq!(checked, rank == self.active - 1);
        json!({"rank":rank,"projective_rows":projective.len(),"projective_augmented_rows":projective,"zero_rows":zeros,"factor_logs":solution,"full_rank_point_verified":checked})
    }
}
fn canonical(row: &[u64], n: u64) -> Option<Vec<u64>> {
    row.iter().find(|&&x| x != 0).map(|&x| {
        let inv = pow(x, n - 2, n);
        row.iter().map(|&a| mm(a, inv, n)).collect()
    })
}
fn rref(mut rows: Vec<Vec<u64>>, cols: usize, n: u64) -> (usize, Option<Vec<u64>>) {
    assert!(rows
        .iter()
        .all(|r| r.len() == cols + 1 && r.iter().all(|&x| x < n)));
    let mut rank = 0;
    for col in 0..cols {
        let Some(pivot) = (rank..rows.len()).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(rank, pivot);
        let inv = pow(rows[rank][col], n - 2, n);
        for x in &mut rows[rank] {
            *x = mm(*x, inv, n);
        }
        let pivotrow = rows[rank].clone();
        for (i, row) in rows.iter_mut().enumerate() {
            if i == rank {
                continue;
            }
            let m = row[col];
            for j in col..=cols {
                row[j] = (row[j] + n - mm(m, pivotrow[j], n)) % n;
            }
        }
        rank += 1;
    }
    assert!(
        rows.iter()
            .all(|r| r[..cols].iter().any(|&x| x != 0) || r[cols] == 0),
        "inconsistent factor row"
    );
    let solution = if rank == cols {
        Some(rows[..cols].iter().map(|r| r[cols]).collect())
    } else {
        None
    };
    (rank, solution)
}
fn eval(equation: &Value, root: [u64; 3], p: u64) -> u64 {
    let mut exps = BTreeSet::new();
    equation.as_array().unwrap().iter().fold(0, |sum, t| {
        let e: [u32; 3] = serde_json::from_value(t[0].clone()).unwrap();
        let c = u(&t[1]);
        assert!(c > 0 && c < p && exps.insert(e));
        let mut v = c;
        for i in 0..3 {
            v = mm(v, pow(root[i], e[i] as u64, p), p);
        }
        (sum + v) % p
    })
}
fn component_values(root: [u64; 3], x: u64, p: u64) -> ([u64; 4], u64) {
    let a = (root[0] + x) % p;
    let b = (root[1] + mm(root[0], x, p)) % p;
    let c = (root[2] + mm(root[1], x, p)) % p;
    let d = mm(root[2], x, p);
    let a = a as i128;
    let b = b as i128;
    let c = c as i128;
    let d = d as i128;
    let reduced = |v: i128| v.rem_euclid(p as i128) as u64;
    (
        [
            reduced(-64 * d.pow(3) + c.pow(4) - 8 * b * c * c * d + 16 * b * b * d * d),
            reduced(
                -24 * c.pow(3) + 32 * b * c * d - 192 * a * d * d - 8 * a * b * c * c
                    + 32 * a * b * b * d,
            ),
            reduced(
                192 * c * c + 256 * b * d - 64 * b.pow(3) + 32 * a * b * c - 192 * a * a * d
                    + 16 * a * a * b * b,
            ),
            reduced(-512 * c + 256 * a * b - 64 * a.pow(3)),
        ],
        reduced(64 * d - (4 * b - a * a).pow(2)),
    )
}
fn cubic_roots(e: [u64; 3], p: u64) -> Vec<u64> {
    (0..p)
        .filter(|&x| {
            (pow(x, 3, p) + p - mm(e[0], mm(x, x, p), p) + mm(e[1], x, p) + p - e[2]) % p == 0
        })
        .collect()
}
#[derive(Default, Serialize)]
struct Counts {
    systems: u64,
    rejected_finite_fp: u64,
    retained_infinity: u64,
    retained_non_fp: u64,
    f4_row_multiplications: u64,
    rejected_f4_row_multiplications: u64,
    roots_in_rejected: u64,
    accepted_in_rejected: u64,
}
fn arm_audit(row: &Value, out: &Path) -> Value {
    let public = load(Path::new(s(&row["public_path"])), s(&row["public_sha256"]));
    let reference = load(
        Path::new(s(&row["reference_path"])),
        s(&row["reference_sha256"]),
    );
    let audit = load(Path::new(s(&row["audit_path"])), s(&row["audit_sha256"]));
    assert_eq!(audit["status"], "pass");
    assert_eq!(audit["candidate_sha256"], row["reference_sha256"]);
    assert_eq!(audit["public_input_sha256"], row["public_sha256"]);
    assert_eq!(reference["status"], "complete-polynomial-enumeration");
    assert_eq!(reference["public_input_sha256"], row["public_sha256"]);
    for key in ["p", "seed", "arm"] {
        assert_eq!(reference[key], row[key]);
    }
    assert_eq!(public["p"], row["p"]);
    assert_eq!(public["seed"], row["seed"]);
    let case = Case::new(&public);
    let double = s(&row["arm"]) == "double-l0-active";
    let p = case.c.f.p;
    let expected: BTreeSet<Terms> =
        serde_json::from_value(reference["signed_physical_set"].clone()).unwrap();
    assert_eq!(
        expected.len() as u64,
        u(&reference["signed_physical_relations"])
    );
    assert_eq!(
        expected.len(),
        reference["signed_physical_set"].as_array().unwrap().len()
    );
    assert_eq!(
        digest(&serde_json::to_vec(&expected).unwrap()),
        s(&reference["signed_set_sha256"])
    );
    assert!(expected.iter().all(|t| case.physical(t, double)));
    let mut kept = BTreeSet::new();
    let mut original = BTreeSet::new();
    let mut violations = Vec::new();
    let mut systems = Vec::new();
    let mut counts = Counts::default();
    for (index, record) in reference["systems"].as_array().unwrap().iter().enumerate() {
        let pair: (usize, i8, usize, i8) = serde_json::from_value(record["pair"].clone()).unwrap();
        assert!(pair.0 < case.l0);
        let start = case.original_len * if double { 2 } else { 1 };
        assert!(pair.2 >= start && pair.2 < start + case.l0);
        let point = case.c.neg(case.c.add(
            case.c.signed(case.pool[pair.0].point, pair.1),
            case.c.signed(case.pool[pair.2].point, pair.3),
        ));
        let saved: Point = serde_json::from_value(record["remaining_point"].clone()).unwrap();
        assert_eq!(point, saved);
        assert!(case.c.on(point));
        let rejected = reject(point);
        let work = u(&record["f4_row_reduction_fp_muls"]);
        counts.systems += 1;
        counts.f4_row_multiplications += work;
        if rejected {
            counts.rejected_finite_fp += 1;
            counts.rejected_f4_row_multiplications += work;
        } else if point.inf {
            counts.retained_infinity += 1;
        } else {
            counts.retained_non_fp += 1;
        }
        let accepted: BTreeSet<Terms> =
            serde_json::from_value(record["accepted_representative_relations"].clone()).unwrap();
        assert_eq!(
            accepted.len(),
            record["accepted_representative_relations"]
                .as_array()
                .unwrap()
                .len()
        );
        let expected_representatives: BTreeSet<_> = expected
            .iter()
            .filter(|t| t.contains(&(pair.0, pair.1)) && t.contains(&(pair.2, pair.3)))
            .cloned()
            .collect();
        assert_eq!(accepted, expected_representatives);
        if rejected {
            counts.accepted_in_rejected += accepted.len() as u64;
            for relation in &accepted {
                violations.push(json!({"kind":"accepted_relation_rejected","index":index,"pair":pair,"relation":relation}));
            }
        }
        for terms in &accepted {
            for map in &case.actions {
                let mut image: Terms = terms
                    .iter()
                    .map(|&(id, t)| {
                        let to = map[id];
                        (to.0, to.1 * t)
                    })
                    .collect();
                image.sort();
                assert!(case.physical(&image, double));
                original.insert(image.clone());
                if !rejected {
                    kept.insert(image);
                }
            }
        }
        let roots: Vec<[u64; 3]> = serde_json::from_value(record["field_roots"].clone()).unwrap();
        assert!(roots.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(record["equations"].as_array().unwrap().len(), 4);
        let mut witnesses = Vec::new();
        for &root in &roots {
            assert!(root.iter().all(|&x| x < p));
            assert!(record["equations"]
                .as_array()
                .unwrap()
                .iter()
                .all(|eq| eval(eq, root, p) == 0));
            if rejected {
                let (components, defect) = component_values(root, point.x[0], p);
                let distinct = cubic_roots(root, p);
                counts.roots_in_rejected += 1;
                if components != [0; 4] || defect != 0 || distinct.len() >= 3 {
                    violations.push(json!({"kind":"root_component_or_nonrepetition_violation","index":index,"root":root,"components":components,"defect":defect,"distinct_roots":distinct}));
                }
                witnesses.push(json!({"root":root,"components":components,"defect":defect,"distinct_roots":distinct}));
            }
        }
        systems.push(json!({"index":index,"pair":pair,"point":point,"reject_finite_fp":rejected,"f4_row_multiplications":work,"field_root_count":roots.len(),"accepted_representatives":accepted,"rejected_root_witnesses":witnesses}));
    }
    assert_eq!(counts.systems, u(&reference["systems_attempted"]));
    assert_eq!(
        counts.f4_row_multiplications,
        u(&reference["f4_row_reduction_fp_muls"])
    );
    assert_eq!(original, expected);
    let before = case.rows(&original);
    let after = case.rows(&kept);
    assert_eq!(before["rank"], reference["factor_log_rank"]);
    assert_eq!(
        before["projective_rows"],
        reference["unique_projective_augmented_rows"]
    );
    assert_eq!(before["zero_rows"], reference["zero_augmented_rows"]);
    assert_eq!(before["factor_logs"], reference["factor_log_solution"]);
    if kept != expected {
        violations.push(json!({"kind":"physical_set_changed","lost":expected.difference(&kept).collect::<Vec<_>>()}));
    }
    if before != after {
        violations.push(json!({"kind":"row_or_rank_changed","before":before,"after":after}));
    }
    let result = json!({"id":row["id"],"p":row["p"],"seed":row["seed"],"arm":row["arm"],"status":if violations.is_empty(){"pass"}else{"counterexample"},"counts":counts,"physical_before":original.len(),"physical_after":kept.len(),"physical_before_sha256":digest(&serde_json::to_vec(&original).unwrap()),"physical_after_sha256":digest(&serde_json::to_vec(&kept).unwrap()),"physical_sets":{"before":original,"after":kept},"before":before,"after":after,"violations":violations,"systems":systems,"source_bindings":row});
    write(&out.join(format!("{}.json", s(&row["id"]))), &result);
    result
}
fn run(workload: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    std::fs::create_dir_all(out.join("cases")).unwrap();
    let catalog = load(workload, WORKLOAD_SHA);
    assert_eq!(catalog["schema"], "twist-collision-workload/v1");
    assert_eq!(catalog["case_arms"], 42);
    let rows = catalog["workloads"].as_array().unwrap();
    assert_eq!(rows.len(), 42);
    let expected: BTreeSet<_> = PRIMES
        .iter()
        .flat_map(|&p| {
            (0..3).flat_map(move |seed| {
                ["phi-l0-active", "double-l0-active"].map(|a| (p, seed, a.to_string()))
            })
        })
        .collect();
    let actual: BTreeSet<_> = rows
        .iter()
        .map(|r| (u(&r["p"]), u(&r["seed"]), s(&r["arm"]).to_string()))
        .collect();
    assert_eq!(actual, expected);
    let mut cases = Vec::new();
    for row in rows {
        let v = arm_audit(row, &out.join("cases"));
        let mut brief = v.as_object().unwrap().clone();
        brief.remove("systems");
        brief.remove("physical_sets");
        cases.push(Value::Object(brief));
    }
    let mut totals = Vec::new();
    for arm in ["phi-l0-active", "double-l0-active"] {
        let mut counts = BTreeMap::<String, u64>::new();
        let mut physical = 0;
        for case in cases.iter().filter(|c| c["arm"] == arm) {
            for (k, v) in case["counts"].as_object().unwrap() {
                *counts.entry(k.clone()).or_default() += u(v);
            }
            physical += u(&case["physical_after"]);
        }
        totals.push(json!({"arm":arm,"counts":counts,"physical_relations":physical}));
    }
    let ok = cases.iter().all(|c| c["status"] == "pass");
    let result = json!({"schema":"twist-zero-fp-residual-audit/v1","status":if ok{"pass"}else{"counterexample"},"workload_sha256":WORKLOAD_SHA,"case_arms":cases.len(),"cases":cases,"arm_totals":totals,"admitted_timing":false,"complete_cost":null,"filter_deployed":false,"scope":"Counterfactual classification of immutable complete zero-fiber records, independent group/action/fold/RREF replay and finite-root component checks. Historical F4 row multiplications are partial counts. No new collector, full curve/group formal proof, extra relations, target recovery or complete-cost/twist-specific gain is established."});
    write(&out.join("summary.json"), &result);
    std::fs::copy(workload, out.join("workload.json")).unwrap();
    println!(
        "{}",
        json!({"status":result["status"],"case_arms":42,"arm_totals":result["arm_totals"]})
    );
    ok
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 4);
    assert_eq!(a[1], "audit");
    if !run(Path::new(&a[2]), Path::new(&a[3])) {
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn infinity_is_retained() {
        assert!(!reject(O));
        assert!(reject(Point { inf: false, ..O }));
        assert!(!reject(Point {
            x: [1, 0, 2, 0],
            inf: false,
            ..O
        }));
    }
    #[test]
    fn field_basis_inverse_and_frobenius() {
        for p in PRIMES {
            let f = Field::new(p);
            let t = [0, 1, 0, 0];
            assert_eq!(f.pow(t, 4), f.scalar(f.d));
            for a in [t, [1, 2, 3, 4]] {
                assert_eq!(f.mul(a, f.inv(a)), f.scalar(1));
                assert_eq!(f.pow(a, p.pow(4)), a);
            }
        }
    }
    #[test]
    fn group_cancellation_and_signed_orientation() {
        let c = Curve {
            f: Field::new(37),
            b: [0, 0, 0, 2],
        };
        let a = Point {
            x: [1, 0, 0, 0],
            y: [36, 11, 4, 6],
            inf: false,
        };
        assert!(c.on(a));
        assert_eq!(c.add(a, c.neg(a)), O);
        assert_eq!(c.add(c.add(a, a), c.neg(a)), a);
        assert_eq!(c.signed(c.signed(a, -1), -1), a);
        assert!(c.on(c.mul(a, 17)));
    }
    #[test]
    fn cubic_distinct_and_repeated() {
        assert_eq!(cubic_roots([6, 11, 6], 37), vec![1, 2, 3]);
        assert_eq!(cubic_roots([28, 16, 26], 37), vec![1, 26]);
    }
    #[test]
    fn repeated_square_components() {
        assert_eq!(component_values([0, 0, 0], 0, 37), ([0; 4], 0));
    }
    #[test]
    fn projective_and_rank_controls() {
        assert_eq!(canonical(&[2, 4, 6], 37), Some(vec![1, 2, 3]));
        assert_eq!(canonical(&[0, 0], 37), None);
        assert_eq!(
            rref(vec![vec![1, 2, 3], vec![2, 4, 6], vec![0, 0, 0]], 2, 37),
            (1, None)
        );
        assert_eq!(
            rref(vec![vec![0, 1, 5], vec![1, 0, 4]], 2, 37),
            (2, Some(vec![4, 5]))
        );
    }
    #[test]
    #[should_panic(expected = "inconsistent factor row")]
    fn inconsistent_rank_refuses() {
        rref(vec![vec![0, 1]], 1, 37);
    }
    #[test]
    fn rejection_witness_is_detectable() {
        let accepted: BTreeSet<Terms> = [vec![(0, -1), (1, 1)]].into_iter().collect();
        let point = Point {
            x: [3, 0, 0, 0],
            inf: false,
            ..O
        };
        assert!(reject(point) && !accepted.is_empty());
        assert!(!(reject(O) && !accepted.is_empty()));
    }
}
