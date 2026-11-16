//! Complete finite fixture audit. Running `audit` is an experiment, never a unit test.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub type Point = [u64; 2];
type Mono = [u32; 3];
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Arm {
    pub name: String,
    pub fb_id: String,
    pub x_values: Vec<u64>,
    pub h_low_to_high: Vec<u64>,
    pub include_explicit_membership: bool,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    pub schema: String,
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub group_order: u64,
    pub generator: Point,
    pub curve_slug: String,
    pub max_wall_seconds: u64,
    pub max_address_space_bytes: u64,
    pub logical_cpus: u64,
    pub arms: Vec<Arm>,
}
#[derive(Default)]
struct Counter {
    adds: Cell<u64>,
    muls: Cell<u64>,
    inversions: Cell<u64>,
    group_adds: Cell<u64>,
    group_doubles: Cell<u64>,
    polynomial_evaluations: Cell<u64>,
    terms: Cell<u64>,
}
impl Counter {
    fn snapshot(&self) -> Value {
        json!({"field_adds":self.adds.get(),"field_muls":self.muls.get(),"field_inversions":self.inversions.get(),"group_additions_including_doublings":self.group_adds.get(),"group_doublings_subset":self.group_doubles.get(),"polynomial_evaluations":self.polynomial_evaluations.get(),"monomial_terms_evaluated":self.terms.get()})
    }
}
fn bump(c: &Cell<u64>) {
    c.set(c.get() + 1);
}
struct F<'a>(&'a Counter);
impl F<'_> {
    fn add(&self, a: u64, b: u64) -> u64 {
        bump(&self.0.adds);
        (a + b) % 17
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        self.add(a, (17 - b) % 17)
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        bump(&self.0.muls);
        a * b % 17
    }
    fn pow(&self, mut a: u64, mut n: u32) -> u64 {
        let mut out = 1;
        while n != 0 {
            if n & 1 != 0 {
                out = self.mul(out, a);
            }
            n >>= 1;
            if n != 0 {
                a = self.mul(a, a);
            }
        }
        out
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0);
        bump(&self.0.inversions);
        self.pow(a, 15)
    }
    fn rhs(&self, x: u64) -> u64 {
        self.add(self.add(self.mul(self.mul(x, x), x), self.mul(2, x)), 2)
    }
    fn h(&self, h: &[u64], x: u64) -> u64 {
        h.iter()
            .rev()
            .fold(0, |out, c| self.add(self.mul(out, x), *c))
    }
    fn on_curve(&self, p: Point) -> bool {
        p[0] < 17 && p[1] < 17 && self.mul(p[1], p[1]) == self.rhs(p[0])
    }
    // Independent affine chord/tangent reference (MIT 18.783 lecture 2).
    fn group_add(&self, p: Point, q: Point) -> Option<Point> {
        bump(&self.0.group_adds);
        let [x, y] = p;
        let [z, w] = q;
        if x == z && self.add(y, w) == 0 {
            return None;
        }
        let slope = if x == z {
            bump(&self.0.group_doubles);
            self.mul(
                self.add(self.mul(3, self.mul(x, x)), 2),
                self.inv(self.mul(2, y)),
            )
        } else {
            self.mul(self.sub(w, y), self.inv(self.sub(z, x)))
        };
        let r = self.sub(self.sub(self.mul(slope, slope), x), z);
        Some([r, self.sub(self.mul(slope, self.sub(x, r)), y)])
    }
    fn slope(&self, p: Point, q: Point) -> u64 {
        if p[0] == q[0] {
            self.mul(
                self.add(self.mul(3, self.mul(p[0], p[0])), 2),
                self.inv(self.mul(2, p[1])),
            )
        } else {
            self.mul(self.sub(q[1], p[1]), self.inv(self.sub(q[0], p[0])))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Term {
    pub exponents: Mono,
    pub coefficient: u64,
}
pub type Poly = Vec<Term>;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Branch {
    pub signs: [i8; 2],
    pub polynomials: Vec<Poly>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct System {
    pub schema: String,
    pub p: u64,
    pub variable_order: Vec<String>,
    pub target: Point,
    pub arm: String,
    pub branches: Vec<Branch>,
    pub dense_polynomials: Vec<Poly>,
}
fn polynomial(terms: impl IntoIterator<Item = (Mono, i64)>) -> Poly {
    let mut out = BTreeMap::<Mono, i64>::new();
    for (m, c) in terms {
        *out.entry(m).or_default() += c;
    }
    out.into_iter()
        .filter_map(|(exponents, c)| {
            let coefficient = c.rem_euclid(17) as u64;
            (coefficient != 0).then_some(Term {
                exponents,
                coefficient,
            })
        })
        .collect()
}
fn membership(xs: &[u64], slot: usize) -> Poly {
    let mut coefficients = vec![1i64];
    for x in xs {
        let mut next = vec![0; coefficients.len() + 1];
        for (i, c) in coefficients.iter().enumerate() {
            next[i] -= *c * *x as i64;
            next[i + 1] += *c;
        }
        coefficients = next.into_iter().map(|c| c.rem_euclid(17)).collect();
    }
    polynomial(coefficients.into_iter().enumerate().map(|(i, c)| {
        let mut m = [0; 3];
        m[slot] = i as u32;
        (m, c)
    }))
}
pub fn build_system(arm: &Arm, target: Point) -> System {
    let [r, s] = target;
    let (r, s) = (r as i64, s as i64);
    let g1 = polynomial([
        ([0, 0, 2], 1),
        ([1, 0, 0], -1),
        ([0, 1, 0], -1),
        ([0, 0, 0], -r),
    ]);
    let g2 = polynomial([
        ([1, 1, 0], 1),
        ([0, 0, 2], -r),
        ([0, 0, 1], -2 * s),
        ([0, 0, 0], -r * r - 2),
    ]);
    let mut branches = Vec::new();
    for signs in [[1, 1], [1, -1], [-1, 1], [-1, -1]] {
        let mut polynomials = vec![g1.clone(), g2.clone()];
        for (slot, sign) in signs.iter().enumerate() {
            let mut terms: Vec<_> = arm
                .h_low_to_high
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let mut m = [0; 3];
                    m[slot] = i as u32;
                    (m, i64::from(*sign) * *c as i64)
                })
                .collect();
            let mut cross = [0; 3];
            cross[slot] = 1;
            cross[2] = 1;
            terms.extend([(cross, -1), ([0, 0, 1], r), ([0, 0, 0], s)]);
            polynomials.push(polynomial(terms));
            if arm.include_explicit_membership {
                polynomials.push(membership(&arm.x_values, slot));
            }
        }
        branches.push(Branch { signs, polynomials });
    }
    // Expanded S3, independently from the three-variable secant generators.
    let s3 = polynomial([
        ([2, 2, 0], 1),
        ([1, 1, 0], -4 - 2 * r * r),
        ([0, 0, 0], 4 - 8 * r),
        ([2, 1, 0], -2 * r),
        ([1, 2, 0], -2 * r),
        ([1, 0, 0], -4 * r - 8),
        ([0, 1, 0], -4 * r - 8),
        ([2, 0, 0], r * r),
        ([0, 2, 0], r * r),
    ]);
    System {
        schema: "secant-audit-complete-system/v1".into(),
        p: 17,
        variable_order: vec!["x".into(), "z".into(), "m".into()],
        target,
        arm: arm.name.clone(),
        branches,
        dense_polynomials: vec![
            membership(&arm.x_values, 0),
            membership(&arm.x_values, 1),
            s3,
        ],
    }
}
fn evaluate(poly: &Poly, at: [u64; 3], f: &F<'_>) -> u64 {
    bump(&f.0.polynomial_evaluations);
    poly.iter().fold(0, |out, t| {
        bump(&f.0.terms);
        let value = at
            .iter()
            .zip(t.exponents)
            .fold(t.coefficient, |v, (x, e)| f.mul(v, f.pow(*x, e)));
        f.add(out, value)
    })
}
fn all_zero(polys: &[Poly], at: [u64; 3], f: &F<'_>) -> bool {
    // Evaluate every equation: operation accounting is not short-circuit dependent.
    polys
        .iter()
        .map(|p| evaluate(p, at, f) == 0)
        .fold(true, |a, b| a & b)
}
fn points(f: &F<'_>) -> Vec<Point> {
    (0..17)
        .flat_map(|x| (0..17).filter_map(move |y| f.on_curve([x, y]).then_some([x, y])))
        .collect()
}
pub fn fixed_spec() -> Spec {
    serde_json::from_str(include_str!("../protocol.json")).expect("fixed protocol")
}
pub fn validate(spec: &Spec) -> Result<(), String> {
    if *spec != fixed_spec() {
        Err(
            "scope is the exact frozen p17 protocol; changed inputs require a new protocol/version"
                .into(),
        )
    } else {
        Ok(())
    }
}
fn base_points(arm: &Arm, f: &F<'_>) -> Result<Vec<Point>, String> {
    let mut out = BTreeSet::new();
    for x in &arm.x_values {
        let y = f.h(&arm.h_low_to_high, *x);
        if y == 0 || !f.on_curve([*x, y]) {
            return Err("fixture base is not the expected distinct signed rational set".into());
        }
        out.insert([*x, y]);
        out.insert([*x, 17 - y]);
    }
    if out.len() != 8 {
        return Err("base cardinality mismatch".into());
    }
    Ok(out.into_iter().collect())
}
pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn source_digest() -> String {
    digest(
        &[
            include_bytes!("lib.rs").as_slice(),
            include_bytes!("main.rs").as_slice(),
            include_bytes!("../Cargo.toml").as_slice(),
            include_bytes!("../Cargo.lock").as_slice(),
            include_bytes!("../protocol.json").as_slice(),
        ]
        .concat(),
    )
}
fn write_new<T: Serialize>(path: &Path, value: &T) -> Result<u64, String> {
    let mut bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    f.write_all(&bytes).map_err(|e| e.to_string())?;
    Ok(bytes.len() as u64)
}
fn read_peak_rss() -> Option<u64> {
    let text = fs::read_to_string("/proc/self/status").ok()?;
    text.lines().find_map(|s| {
        s.strip_prefix("VmHWM:")
            .and_then(|v| v.split_whitespace().next())
            .and_then(|v| v.parse::<u64>().ok())
            .map(|k| k * 1024)
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct Root {
    signs: [i8; 2],
    x: u64,
    z: u64,
    m: u64,
    p: Point,
    q: Point,
}
fn orientation(arm: &Arm, p: Point, f: &F<'_>) -> i8 {
    if f.h(&arm.h_low_to_high, p[0]) == p[1] {
        1
    } else {
        -1
    }
}
fn matrix_row(arm: &Arm, p: Point, q: Point) -> Vec<u64> {
    let mut out = vec![0; arm.x_values.len()];
    for v in [p, q] {
        let i = arm
            .x_values
            .iter()
            .position(|x| *x == v[0])
            .expect("base point");
        out[i] = (out[i] + if v[1] < 17 - v[1] { 1 } else { 18 }) % 19;
    }
    out
}
fn pairs_ref(
    arm: &Arm,
    base: &[Point],
    f: &F<'_>,
) -> (BTreeMap<Point, BTreeSet<Root>>, Vec<[Point; 2]>) {
    let mut map = BTreeMap::<Point, BTreeSet<Root>>::new();
    let mut infinity = Vec::new();
    for p in base {
        for q in base {
            if let Some(r) = f.group_add(*p, *q) {
                map.entry(r).or_default().insert(Root {
                    signs: [orientation(arm, *p, f), orientation(arm, *q, f)],
                    x: p[0],
                    z: q[0],
                    m: f.slope(*p, *q),
                    p: *p,
                    q: *q,
                });
            } else {
                infinity.push([*p, *q]);
            }
        }
    }
    (map, infinity)
}
fn ratio(n: usize, d: usize) -> Value {
    json!({"numerator":n,"denominator":d})
}

pub fn f4_systems(arm: &Arm, target: Point) -> Result<Vec<Value>, String> {
    let count = Counter::default();
    let f = F(&count);
    let base = base_points(arm, &f)?;
    let system = build_system(arm, target);
    let prefix = if arm.name == "split-graph" {
        "candidate"
    } else {
        "control"
    };
    let mut out = Vec::new();
    for branch in &system.branches {
        let id = format!(
            "p17-{prefix}-r{}-s{}-secant-{}-{}",
            target[0], target[1], branch.signs[0], branch.signs[1]
        );
        out.push(json!({"id":id,"p":17,"variables":["x","z","m"],"polynomials":branch.polynomials,"curve":{"p":17,"a":2,"b":2},"target":target,"base_x":arm.x_values,"base_points":base,"arm":format!("{prefix}-secant"),"signs":branch.signs,"h_low_to_high":arm.h_low_to_high}));
    }
    let dense: Vec<Vec<_>> = system
        .dense_polynomials
        .iter()
        .map(|p| {
            p.iter()
                .map(|t| {
                    assert_eq!(t.exponents[2], 0);
                    json!({"exponents":&t.exponents[..2],"coefficient":t.coefficient})
                })
                .collect()
        })
        .collect();
    out.push(json!({"id":format!("p17-{prefix}-r{}-s{}-dense",target[0],target[1]),"p":17,"variables":["x","z"],"polynomials":dense,"curve":{"p":17,"a":2,"b":2},"target":target,"base_x":arm.x_values,"base_points":base,"arm":format!("{prefix}-dense")}));
    Ok(out)
}

pub fn export_f4(spec: &Spec, output: &Path, revision: &str, spec_sha: &str) -> Result<(), String> {
    validate(spec)?;
    valid_revision(revision)?;
    fs::create_dir(output).map_err(|e| e.to_string())?;
    let count = Counter::default();
    let f = F(&count);
    let population = points(&f);
    let mut systems = Vec::new();
    for arm in &spec.arms {
        for target in &population {
            systems.extend(f4_systems(arm, *target)?);
        }
    }
    let bytes = write_new(
        &output.join("f4-inputs.json"),
        &json!({"schema":"secant-f4-input-bundle/v1","systems":systems}),
    )?;
    write_new(
        &output.join("manifest.json"),
        &json!({"schema":"secant-audit-f4-export/v1","source_revision":revision,"native_source_sha256":source_digest(),"spec_sha256":spec_sha,"target_population":population,"systems":systems.len(),"inputs_sha256":digest(&fs::read(output.join("f4-inputs.json")).map_err(|e|e.to_string())?),"payload_bytes_written_excluding_manifest":bytes,"expected_roots_omitted":true,"roots_enumerated":false,"f4_executed":false}),
    )?;
    Ok(())
}

pub fn export_inputs(
    spec: &Spec,
    output: &Path,
    revision: &str,
    spec_sha: &str,
) -> Result<(), String> {
    validate(spec)?;
    valid_revision(revision)?;
    fs::create_dir(output).map_err(|e| e.to_string())?;
    let count = Counter::default();
    let f = F(&count);
    let population = points(&f);
    let inputs: Vec<_> = spec
        .arms
        .iter()
        .flat_map(|a| population.iter().map(|r| build_system(a, *r)))
        .collect();
    let bytes = write_new(&output.join("inputs.json"), &inputs)?;
    write_new(
        &output.join("manifest.json"),
        &json!({"schema":"secant-audit-input-export/v1","source_revision":revision,"native_source_sha256":source_digest(),"spec_sha256":spec_sha,"target_population":population,"input_cells":inputs.len(),"inputs_sha256":digest(&fs::read(output.join("inputs.json")).map_err(|e|e.to_string())?),"payload_bytes_written_excluding_manifest":bytes,"roots_enumerated":false,"experiment_completed":false}),
    )?;
    Ok(())
}
fn valid_revision(rev: &str) -> Result<(), String> {
    if rev.len() == 40 && rev.bytes().all(|b| b.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err("source revision must be a full 40-hex commit".into())
    }
}

pub fn audit(spec: &Spec, output: &Path, revision: &str, spec_sha: &str) -> Result<bool, String> {
    validate(spec)?;
    valid_revision(revision)?;
    fs::create_dir(output).map_err(|e| e.to_string())?;
    fs::create_dir(output.join("cells")).map_err(|e| e.to_string())?;
    let start = Instant::now();
    let setup = Counter::default();
    let sf = F(&setup);
    let population = points(&sf);
    if population.len() != 18 {
        return Err("curve order failed independent full point count".into());
    }
    let mut payload = write_new(
        &output.join("RUNNING.json"),
        &json!({"source_revision":revision,"native_source_sha256":source_digest(),"spec_sha256":spec_sha,"planned_cells":36,"planned_targets":population,"scope":"all_x_z_m_all_signs_fixed_F17","admission":"external_caller_must_record_checkpoint/backend_admission"}),
    )?;
    let mut summaries = Vec::new();
    let mut reference_files = Vec::new();
    let mut failures = 0usize;
    let mut cell_index = 0;
    for arm in &spec.arms {
        let base = base_points(arm, &sf)?;
        let ref_count = Counter::default();
        let rf = F(&ref_count);
        let (references, infinity) = pairs_ref(arm, &base, &rf);
        let mut sat = 0usize;
        let mut ordered = 0usize;
        let mut model_roots_total = 0usize;
        let mut double_total = 0usize;
        let mut unordered_total = 0usize;
        let mut root_counts = BTreeMap::<Point, usize>::new();
        for target in &population {
            if start.elapsed().as_secs() >= spec.max_wall_seconds {
                return Err("global wall deadline reached; completed cell files retained".into());
            }
            let system = build_system(arm, *target);
            let input_bytes = serde_json::to_vec(&system).map_err(|e| e.to_string())?;
            let expected = references.get(target).cloned().unwrap_or_default();
            let root_count = Counter::default();
            let mf = F(&root_count);
            let replay_count = Counter::default();
            let vf = F(&replay_count);
            let mut actual = BTreeSet::new();
            let mut branch_records = Vec::new();
            let mut roots_before_dedup = 0usize;
            let mut replay_failures = Vec::new();
            for branch in &system.branches {
                let mut branch_roots = Vec::new();
                for x in 0..17 {
                    for z in 0..17 {
                        for m in 0..17 {
                            if !all_zero(&branch.polynomials, [x, z, m], &mf) {
                                continue;
                            }
                            let p = [
                                x,
                                (i64::from(branch.signs[0]) * mf.h(&arm.h_low_to_high, x) as i64)
                                    .rem_euclid(17) as u64,
                            ];
                            let q = [
                                z,
                                (i64::from(branch.signs[1]) * mf.h(&arm.h_low_to_high, z) as i64)
                                    .rem_euclid(17) as u64,
                            ];
                            let root = Root {
                                signs: branch.signs,
                                x,
                                z,
                                m,
                                p,
                                q,
                            };
                            let replay = vf.on_curve(p)
                                && vf.on_curve(q)
                                && base.contains(&p)
                                && base.contains(&q)
                                && vf.group_add(p, q) == Some(*target);
                            if !replay {
                                replay_failures.push(root.clone());
                            }
                            actual.insert(root.clone());
                            branch_roots.push(root);
                            roots_before_dedup += 1;
                        }
                    }
                }
                branch_records.push(json!({"signs":branch.signs,"assignments":4913,"roots":branch_roots,"survival":ratio(branch_roots.len(),4913)}));
            }
            let extra: Vec<_> = actual.difference(&expected).cloned().collect();
            let missing: Vec<_> = expected.difference(&actual).cloned().collect();
            let expected_x: BTreeSet<_> = expected.iter().map(|v| [v.x, v.z]).collect();
            let dc = Counter::default();
            let df = F(&dc);
            let mut dense = BTreeSet::new();
            let mut dense_witnesses = Vec::new();
            let mut dense_replay_failures = 0usize;
            for x in 0..17 {
                for z in 0..17 {
                    if all_zero(&system.dense_polynomials, [x, z, 0], &df) {
                        dense.insert([x, z]);
                        let witness = expected.iter().find(|v| v.x == x && v.z == z);
                        let replay =
                            witness.is_some_and(|w| vf.group_add(w.p, w.q) == Some(*target));
                        dense_replay_failures += usize::from(!replay);
                        dense_witnesses.push(
                            json!({"x":x,"z":z,"group_witness":witness,"group_replay":replay}),
                        );
                    }
                }
            }
            let dense_extra: Vec<_> = dense.difference(&expected_x).copied().collect();
            let dense_missing: Vec<_> = expected_x.difference(&dense).copied().collect();
            let actual_x: BTreeSet<_> = actual.iter().map(|v| [v.x, v.z]).collect();
            let pairs: BTreeSet<_> = actual.iter().map(|v| [v.p, v.q]).collect();
            let unordered: BTreeSet<_> = actual
                .iter()
                .map(|v| if v.p <= v.q { [v.p, v.q] } else { [v.q, v.p] })
                .collect();
            let rows: BTreeSet<_> = actual
                .iter()
                .filter(|v| base.contains(&v.p) && base.contains(&v.q))
                .map(|v| matrix_row(arm, v.p, v.q))
                .collect();
            let mismatch = extra.len()
                + missing.len()
                + dense_extra.len()
                + dense_missing.len()
                + dense_replay_failures
                + replay_failures.len()
                + usize::from(actual_x != dense);
            failures += mismatch;
            ordered += pairs.len();
            model_roots_total += roots_before_dedup;
            double_total += actual.iter().filter(|r| r.p == r.q).count();
            unordered_total += unordered.len();
            root_counts.insert(*target, pairs.len());
            sat += usize::from(!expected.is_empty());
            let record = json!({"schema":"secant-complete-audit-cell/v1","curve_slug":spec.curve_slug,"fb_id":arm.fb_id,"arm":arm.name,"target":target,"input_sha256":digest(&input_bytes),"complete":true,"branch_records":branch_records,"reference_roots":expected,"reported_roots":actual,"dense_witnesses":dense_witnesses,"dense_assignments_checked":289,"structured_assignments_checked":19652,"false_positive_roots":extra,"false_negative_roots":missing,"dense_false_positive_x_pairs":dense_extra,"dense_false_negative_x_pairs":dense_missing,"dense_group_replay_failures":dense_replay_failures,"group_replay_failures":replay_failures,"model_dense_x_projection_equal":actual_x==dense,"roots_before_dedup":roots_before_dedup,"distinct_ordered_point_pairs":pairs.len(),"unordered_pairs":unordered.len(),"matrix_rows":rows.len(),"matrix_row_vectors":rows,"duplicates":{"model_roots":roots_before_dedup-actual.len(),"ordered_pairs_to_unordered":pairs.len()-unordered.len(),"ordered_pairs_to_matrix_rows":pairs.len()-rows.len()},"doubles":actual.iter().filter(|r|r.p==r.q).count(),"operation_count_scope":{"input_polynomial_construction_counted":false,"reference_setup":"recorded_per_arm_separately","structured_evaluation_includes":"all_equation_terms_and_h_evaluations_for_materialized_roots","unaccounted":"polynomial_construction_integer_loop_hash_serialization_io_costs"},"operation_counts":{"structured_evaluation":root_count.snapshot(),"dense_evaluation":dc.snapshot(),"fresh_group_replay":replay_count.snapshot()},"survival":ratio(roots_before_dedup,19652),"mismatch_count":mismatch});
            let relative = format!("cells/{cell_index:03}.json");
            let n = write_new(&output.join(&relative), &record)?;
            payload += n;
            cell_index += 1;
            summaries.push(json!({"path":relative,"sha256":digest(&fs::read(output.join(&relative)).map_err(|e|e.to_string())?),"bytes":n,"arm":arm.name,"target":target,"roots":pairs.len(),"mismatch_count":mismatch}));
        }
        let negation_symmetric = root_counts
            .iter()
            .all(|(r, n)| root_counts.get(&[r[0], 17 - r[1]]) == Some(n));
        let invariants = json!({"finite_ordered_relations_56":ordered==56,"finite_model_roots_56":model_roots_total==56,"tangent_roots_8":double_total==8,"finite_unordered_pairs_32":unordered_total==32,"infinity_inverse_pairs_8":infinity.len()==8,"negation_target_counts_symmetric":negation_symmetric,"sat_target_count_even":sat.is_multiple_of(2)});
        failures += invariants
            .as_object()
            .unwrap()
            .values()
            .filter(|v| **v == false)
            .count();
        let reference_name = format!("{}-reference.json", arm.name);
        let reference_bytes = write_new(
            &output.join(format!("{}-reference.json", arm.name)),
            &json!({"arm":arm.name,"base_points":base,"infinity_target":{"method":"exact_inverse_pairs_separate_from_finite_secant_system","relations":infinity,"every_pair_group_checked":true},"reference_setup_operations":ref_count.snapshot(),"finite_target_population":18,"finite_sat_targets":sat,"finite_ordered_relations":ordered,"finite_model_roots":model_roots_total,"finite_tangent_roots":double_total,"finite_unordered_pairs":unordered_total,"independent_aggregation_invariants":invariants,"natural_target_success_ratio":ratio(sat,18),"top_ideal_degree":"not_measured_by_this_audit","f4_degree":null}),
        )?;
        payload += reference_bytes;
        reference_files.push(json!({"path":reference_name,"bytes":reference_bytes,"sha256":digest(&fs::read(output.join(&reference_name)).map_err(|e|e.to_string())?)}));
    }
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_bytes = fs::read(executable).map_err(|e| e.to_string())?;
    write_new(
        &output.join("manifest.json"),
        &json!({"schema":"secant-complete-audit/v1","spec_sha256":spec_sha,"source_revision":revision,"native_source_sha256":source_digest(),"binary_sha256":digest(&exe_bytes),"complete":true,"cells":summaries,"reference_files":reference_files,"target_count_per_arm":18,"arms":2,"sign_branches_per_target":4,"structured_assignments_checked":36*19652u64,"dense_assignments_checked":36*289u64,"mismatch_count":failures,"passes":failures==0,"setup_operations":setup.snapshot(),"setup_operation_scope":"point_inventory_and_base_lift_checks_only;input_polynomial_construction_uncounted","process_peak_rss_bytes":read_peak_rss(),"elapsed_wall_ns_unqualified":start.elapsed().as_nanos().to_string(),"comparative_performance_claim":false,"disk_payload":{"written_bytes_excluding_manifest":payload,"binary_bytes_read_for_hash":exe_bytes.len(),"cell_readback_bytes":summaries.iter().filter_map(|s|s["bytes"].as_u64()).sum::<u64>(),"reference_readback_bytes":reference_files.iter().filter_map(|s|s["bytes"].as_u64()).sum::<u64>(),"spec_read_bytes":"caller_recorded_not_charged_here","operating_system_disk_traffic_measured":false},"requested_resources":{"max_address_space_bytes":spec.max_address_space_bytes,"logical_cpus":spec.logical_cpus,"external_enforcement_required":true,"observed_available_parallelism":std::thread::available_parallelism().ok().map(|n|n.get())},"f4_solving_degree":null,"no_probabilistic_branches_discarded":true}),
    )?;
    Ok(failures == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn group_reference_handles_secant_tangent_and_inverse() {
        let c = Counter::default();
        let f = F(&c);
        assert_eq!(f.group_add([5, 1], [5, 1]), Some([6, 3]));
        assert_eq!(f.group_add([5, 1], [6, 3]), Some([10, 6]));
        assert_eq!(f.group_add([5, 1], [5, 16]), None);
    }
    #[test]
    fn equation_export_has_all_signs_and_complete_control_membership() {
        let spec = fixed_spec();
        let s = build_system(&spec.arms[0], [10, 6]);
        assert_eq!(s.branches.len(), 4);
        assert!(all_zero(
            &s.branches[0].polynomials,
            [5, 6, 2],
            &F(&Counter::default())
        ));
        let control = build_system(&spec.arms[1], [10, 6]);
        assert_eq!(control.branches[0].polynomials.len(), 6);
    }
    #[test]
    fn provided_exporter_fixture_matches_independent_builder() {
        let input: Value =
            serde_json::from_str(include_str!("../../../secant/target-10-6.json")).unwrap();
        let spec = fixed_spec();
        for (i, arm) in spec.arms.iter().enumerate() {
            let s = build_system(arm, [10, 6]);
            for (j, b) in s.branches.iter().enumerate() {
                assert_eq!(
                    serde_json::to_value(&b.polynomials).unwrap(),
                    input["arms"][i]["branches"][j]["polynomials"]
                );
            }
        }
    }
    #[test]
    fn fixture_mutations_are_refused() {
        let mut s = fixed_spec();
        s.arms[0].x_values.pop();
        assert!(validate(&s).is_err());
    }
    #[test]
    fn membership_and_s3_preserve_known_double() {
        let spec = fixed_spec();
        let s = build_system(&spec.arms[0], [6, 3]);
        assert!(all_zero(
            &s.dense_polynomials,
            [5, 5, 0],
            &F(&Counter::default())
        ));
    }
    #[test]
    fn f4_export_has_four_signed_secants_and_two_variable_dense_input() {
        let spec = fixed_spec();
        let s = f4_systems(&spec.arms[0], [10, 6]).unwrap();
        assert_eq!(s.len(), 5);
        for entry in &s[..4] {
            assert_eq!(entry["variables"].as_array().unwrap().len(), 3);
            assert_eq!(entry["signs"].as_array().unwrap().len(), 2);
        }
        assert_eq!(s[4]["variables"].as_array().unwrap().len(), 2);
        for p in s[4]["polynomials"].as_array().unwrap() {
            for t in p.as_array().unwrap() {
                assert_eq!(t["exponents"].as_array().unwrap().len(), 2);
            }
        }
    }
}
