//! Joint mixed-image S5 construction. Candidate input excludes reference data.
//! Process durations in this module are not admitted performance measurements.
use super::*;
#[path = "../../../joint_image_elimination/native/external_check.rs"]
mod external_check;
#[path = "../../../joint_image_elimination/native/five_point_replay.rs"]
mod five_point_replay;
#[path = "../../../joint_image_elimination/native/five_zero.rs"]
mod five_zero;
#[path = "../../../joint_image_elimination/native/fp_pruning_audit.rs"]
mod fp_pruning_audit;
#[path = "joint_proof.rs"]
mod proof;
#[path = "joint_report.rs"]
mod report;
#[path = "joint_solver.rs"]
mod solver;
#[path = "../../../joint_image_elimination/native/uniform.rs"]
mod uniform;
#[path = "../../../joint_image_elimination/native/variant_report.rs"]
mod variant_report;
#[path = "joint_variants.rs"]
mod variants;
#[path = "../../../zero_fiber_collector/native/collector.rs"]
mod zero_collector;
#[path = "../../../joint_image_elimination/native/zero_identity.rs"]
mod zero_identity;

pub(crate) fn zero_five_collect(public: &Path, sha: &str, arm: &str, out: &Path) -> bool {
    zero_collector::collect(public, sha, arm, out)
}
pub(crate) fn zero_five_audit(
    public: &Path,
    sha: &str,
    candidate: &Path,
    reference: &Path,
    out: &Path,
) -> bool {
    zero_collector::audit(public, sha, candidate, reference, out)
}

pub(crate) fn zero_oracle(public: &Path, sha: &str, out: &Path) {
    uniform::zero(public, sha, out)
}
pub(crate) fn population_oracle(public: &Path, sha: &str, out: &Path) {
    uniform::population(public, sha, out)
}
pub(crate) fn population_summary(run: &Path, out: &Path) -> bool {
    uniform::summarize(run, out)
}
pub(crate) fn external_audit(
    public: &Path,
    public_sha: &str,
    system: &Path,
    system_sha: &str,
    basis: &Path,
    basis_sha: &str,
    reference: &Path,
    out: &Path,
) -> bool {
    external_check::audit(
        public, public_sha, system, system_sha, basis, basis_sha, reference, out,
    )
}
pub(crate) fn zero_identity_audit(run: &Path, public_root: &Path, out: &Path) -> bool {
    zero_identity::audit(run, public_root, out)
}
pub(crate) fn variant_construction_report(run: &Path, out: &Path) -> bool {
    variant_report::construction(run, out)
}
pub(crate) fn fp_pruning_audit(
    public: &Path,
    sha: &str,
    workload: &Path,
    refs: &Path,
    double_refs: &Path,
    out: &Path,
) -> bool {
    fp_pruning_audit::audit(public, sha, workload, refs, double_refs, out)
}
pub(crate) fn fp_pruning_summary(run: &Path, out: &Path) -> bool {
    fp_pruning_audit::summarize(run, out)
}
pub(crate) fn five_zero(public: &Path, sha: &str, replay: &Path, out: &Path) -> bool {
    five_zero::audit(public, sha, replay, out)
}
pub(crate) fn five_zero_summary(run: &Path, out: &Path) -> bool {
    five_zero::summarize(run, out)
}
pub(crate) fn five_point_replay(public: &Path, sha: &str, prior: &Path, out: &Path) -> bool {
    five_point_replay::audit(public, sha, prior, out)
}
pub(crate) fn five_point_summary(run: &Path, out: &Path) -> bool {
    five_point_replay::summarize(run, out)
}

pub(crate) fn construction_report(run: &Path, out: &Path) -> bool {
    report::construction(run, out)
}
pub(crate) fn orbit_proofs(out: &Path) {
    proof::write_proofs(out)
}
pub(crate) fn export_variant(public: &Path, expected: &str, arm: &str, qi: usize, out: &Path) {
    variants::export(public, expected, arm, qi, out)
}

const PUBLIC_KEYS: [&str; 13] = [
    "p",
    "seed",
    "n",
    "b0",
    "generator_x",
    "generator_y",
    "target_x",
    "target_y",
    "omega",
    "lambda_cubic",
    "lambda_frob",
    "base_coordinates",
    "query_trace",
];
const JOINT_ARMS: [&str; 3] = ["phi-l0-active", "double-l0-active", "l0-peel"];

pub(crate) fn solve_query(
    public: &Path,
    public_sha: &str,
    system: &Path,
    system_sha: &str,
    out: &Path,
) -> bool {
    solver::solve(public, public_sha, system, system_sha, out)
}
pub(crate) fn check_query(
    public: &Path,
    public_sha: &str,
    candidate: &Path,
    reference: &Path,
    out: &Path,
) -> bool {
    solver::check(public, public_sha, candidate, reference, out)
}
pub(crate) fn solve_orbit_query(
    public: &Path,
    public_sha: &str,
    manifest: &Path,
    manifest_sha: &str,
    out: &Path,
) -> bool {
    solver::solve_orbits(public, public_sha, manifest, manifest_sha, out)
}
type Exponent = [u16; 5];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Polynomial(BTreeMap<Exponent, E>);

impl Polynomial {
    fn constant(c: E) -> Self {
        if c == [0; 4] {
            Self::default()
        } else {
            Self(BTreeMap::from([([0; 5], c)]))
        }
    }
    fn scalar(f: Field, c: u64) -> Self {
        Self::constant(f.scalar(c % f.p))
    }
    fn var(f: Field, i: usize) -> Self {
        let mut e = [0; 5];
        e[i] = 1;
        Self(BTreeMap::from([(e, f.scalar(1))]))
    }
    fn term(&mut self, f: Field, e: Exponent, c: E) {
        let v = f.add(self.0.get(&e).copied().unwrap_or([0; 4]), c);
        if v == [0; 4] {
            self.0.remove(&e);
        } else {
            self.0.insert(e, v);
        }
    }
    fn add(&self, f: Field, b: &Self) -> Self {
        let mut r = self.clone();
        for (&e, &c) in &b.0 {
            r.term(f, e, c);
        }
        r
    }
    fn scale(&self, f: Field, c: E) -> Self {
        let mut r = Self::default();
        for (&e, &v) in &self.0 {
            r.term(f, e, f.mul(v, c));
        }
        r
    }
    fn neg(&self, f: Field) -> Self {
        self.scale(f, f.neg(f.scalar(1)))
    }
    fn sub(&self, f: Field, b: &Self) -> Self {
        self.add(f, &b.neg(f))
    }
    fn mul(&self, f: Field, b: &Self) -> Self {
        let mut r = Self::default();
        for (&e, &a) in &self.0 {
            for (&d, &c) in &b.0 {
                let t = std::array::from_fn(|i| e[i].checked_add(d[i]).unwrap());
                r.term(f, t, f.mul(a, c));
            }
        }
        r
    }
    fn pow(&self, f: Field, k: u16) -> Self {
        (0..k).fold(Self::scalar(f, 1), |a, _| a.mul(f, self))
    }
    fn eval(&self, f: Field, x: &[E; 5]) -> E {
        self.0.iter().fold([0; 4], |sum, (e, &c)| {
            let term = (0..5).fold(c, |a, i| f.mul(a, f.pow(x[i], e[i] as u64)));
            f.add(sum, term)
        })
    }
    fn components(&self, f: Field, variables: usize) -> Vec<f4_fp::Poly> {
        assert!((1..=5).contains(&variables));
        assert!(self
            .0
            .keys()
            .all(|e| e[variables..].iter().all(|&v| v == 0)));
        (0..4)
            .map(|i| {
                f4_fp::normalise(
                    &self
                        .0
                        .iter()
                        .filter(|(_, c)| c[i] != 0)
                        .map(|(e, c)| (e[..variables].iter().map(|&v| v as u32).collect(), c[i]))
                        .collect::<Vec<_>>(),
                    f.p,
                    f4_fp::Ordering::Grevlex,
                )
            })
            .collect()
    }
    fn stats(&self) -> Value {
        json!({"extension_terms":self.0.len(),"total_degree":self.0.keys().map(|e|e.iter().map(|&v|v as usize).sum::<usize>()).max().unwrap_or(0),
            "variable_degrees":(0..5).map(|i|self.0.keys().map(|e|e[i]).max().unwrap_or(0)).collect::<Vec<_>>()})
    }
    fn fp_equation(&self, f: Field, variables: usize) -> f4_fp::Poly {
        let mut components = self.components(f, variables);
        assert!(
            components[1..].iter().all(Vec::is_empty),
            "constraint left Fp"
        );
        components.remove(0)
    }
}

fn arm_index(name: &str) -> usize {
    assert!(JOINT_ARMS.contains(&name), "unregistered joint arm");
    ARMS.iter().position(|&a| a == name).unwrap()
}

fn public_record(raw: &Value) -> Value {
    let mut r = serde_json::Map::new();
    for k in PUBLIC_KEYS {
        r.insert(k.into(), raw[k].clone());
    }
    r.insert(
        "query_trace".into(),
        json!(raw["query_trace"]
            .as_array()
            .unwrap()
            .iter()
            .map(|q| vec![q[0].clone(), q[1].clone()])
            .collect::<Vec<_>>()),
    );
    r.insert(
        "base_coordinates".into(),
        json!(raw["base_coordinates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| {
                let p = point(p);
                json!([p.x, p.y])
            })
            .collect::<Vec<_>>()),
    );
    Value::Object(r)
}

fn validate_public(raw: &Value) {
    let keys: BTreeSet<_> = raw
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        PUBLIC_KEYS.into_iter().collect(),
        "candidate input contains unapproved fields"
    );
    assert!(PRIMES.contains(&raw["p"].as_u64().unwrap()));
    assert!(raw["seed"].as_u64().unwrap() < 3);
    let n = raw["n"].as_u64().unwrap();
    assert!(raw["query_trace"].as_array().unwrap().iter().all(|q| q
        .as_array()
        .is_some_and(|a| a.len() == 2 && a.iter().all(|v| v.as_u64().is_some_and(|v| v < n)))));
    assert!(raw["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .all(|p| p.as_array().is_some_and(|a| a.len() == 2)));
}

pub(crate) fn prepare(tape: &Path, out: &Path) {
    assert!(!out.exists());
    std::fs::create_dir_all(out.join("public")).unwrap();
    let mut records = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            let raw = load(tape, INPUT_SHA, p, seed);
            let public = public_record(&raw);
            validate_public(&public);
            let name = format!("p{p}-seed{seed}.json");
            let bytes = serde_json::to_vec_pretty(&public).unwrap();
            let sha = hex::encode(hash::sha256::sha256(&bytes));
            std::fs::write(out.join("public").join(&name), &bytes).unwrap();
            let m = public["query_trace"].as_array().unwrap().len();
            let fixed: BTreeSet<_> = [0, m / 3, 2 * m / 3, m - 1].into_iter().collect();
            records.push(json!({"p":p,"seed":seed,"path":format!("public/{name}"),"sha256":sha,"bytes":bytes.len(),"queries":m,"fixed_queries":fixed}));
        }
    }
    write(
        &out.join("manifest.json"),
        &json!({"schema":"joint-image-public-inputs/v1","source_sha256":INPUT_SHA,"keys":PUBLIC_KEYS,"records":records,"scope":"Public file separation, not a formal noninterference proof"}),
    );
}

fn checked_public(path: &Path, expected: &str) -> Value {
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(
        hex::encode(hash::sha256::sha256(&bytes)),
        expected,
        "public input hash mismatch before geometry"
    );
    let raw: Value = serde_json::from_slice(&bytes).unwrap();
    validate_public(&raw);
    raw
}

pub(crate) fn workload(prepared: &Path, refs: &Path, double_refs: &Path, out: &Path) {
    assert!(!out.exists());
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(prepared.join("manifest.json")).unwrap()).unwrap();
    let mut tasks = Vec::new();
    for rec in manifest["records"].as_array().unwrap() {
        let p = rec["p"].as_u64().unwrap();
        let seed = rec["seed"].as_u64().unwrap();
        let public = prepared
            .join(rec["path"].as_str().unwrap())
            .canonicalize()
            .unwrap();
        checked_public(&public, rec["sha256"].as_str().unwrap());
        let fixed: BTreeSet<usize> = serde_json::from_value(rec["fixed_queries"].clone()).unwrap();
        for arm in JOINT_ARMS {
            let root = if arm == "double-l0-active" {
                double_refs
            } else {
                refs
            };
            let reference = root.join(format!("p{p}-seed{seed}/{arm}.json"));
            let bytes = std::fs::read(&reference).unwrap();
            let v: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(v["p"], p);
            assert_eq!(v["seed"], seed);
            assert_eq!(v["arm"], arm);
            let first = v["trace"]
                .as_array()
                .unwrap()
                .iter()
                .position(|r| !r["signed_relations"].as_array().unwrap().is_empty());
            let mut queries = fixed.clone();
            if p == 37 && seed == 0 {
                queries.insert(2);
            }
            if let Some(i) = first {
                queries.insert(i);
            }
            for qi in queries {
                tasks.push(json!({"id":format!("p{p}-s{seed}-{arm}-q{qi}"),"p":p,"seed":seed,"arm":arm,"query_index":qi,"public_path":public,"public_sha256":rec["sha256"],"fixed_position":fixed.contains(&qi),"first_positive_control":first==Some(qi),"historical_s5_regression":p==37 && seed==0 && qi==2,"reference_sha256":hex::encode(hash::sha256::sha256(&bytes))}));
            }
        }
    }
    write(
        out,
        &json!({"schema":"joint-image-workload/v1","tasks":tasks,"selection":"Four fixed ordinal positions plus first previously independently recorded positive, deduplicated. No solver timing used; positive controls are conditioned fixtures."}),
    );
    println!(
        "{}",
        json!({"tasks":tasks.len(),"cases":manifest["records"].as_array().unwrap().len()})
    );
}

fn h_coefficients(f: Field, b: E, h: &integer::ZPoly) -> [Polynomial; 5] {
    let mut a: [Polynomial; 5] = std::array::from_fn(|_| Polynomial::default());
    for (&e, &c) in &h.0 {
        let coefficient = f.mul(
            f.scalar(c.rem_euclid(f.p as i128) as u64),
            f.pow(b, e[4] as u64),
        );
        a[e[3] as usize].term(
            f,
            [e[0] as u16, e[1] as u16, e[2] as u16, 0, 0],
            coefficient,
        );
    }
    a
}

fn s3_coefficients(f: Field, b: E, z: &Polynomial, r: E) -> [Polynomial; 3] {
    let r = Polynomial::constant(r);
    let b = Polynomial::constant(b);
    let sum = z.add(f, &r);
    let product = z.mul(f, &r);
    [
        product
            .pow(f, 2)
            .sub(f, &b.mul(f, &sum).scale(f, f.scalar(4))),
        sum.mul(f, &product)
            .add(f, &b.scale(f, f.scalar(2)))
            .scale(f, f.neg(f.scalar(2))),
        z.sub(f, &r).pow(f, 2),
    ]
}

fn sylvester(a: &[Polynomial], b: &[Polynomial]) -> Vec<Vec<Polynomial>> {
    let m = a.len() - 1;
    let n = b.len() - 1;
    let mut matrix = vec![vec![Polynomial::default(); m + n]; m + n];
    for i in 0..n {
        for j in 0..=m {
            matrix[i][i + j] = a[j].clone();
        }
    }
    for i in 0..m {
        for j in 0..=n {
            matrix[n + i][i + j] = b[j].clone();
        }
    }
    matrix
}

fn determinant(f: Field, matrix: &[Vec<Polynomial>]) -> Polynomial {
    let n = matrix.len();
    assert!(n > 0 && n <= 8 && matrix.iter().all(|r| r.len() == n));
    let mut dp = vec![Polynomial::default(); 1 << n];
    dp[0] = Polynomial::scalar(f, 1);
    for mask in 0usize..1 << n {
        let row = mask.count_ones() as usize;
        let current = std::mem::take(&mut dp[mask]);
        if row == n {
            return current;
        }
        if current.0.is_empty() {
            continue;
        }
        for col in 0..n {
            if mask & (1 << col) == 0 {
                let mut term = current.mul(f, &matrix[row][col]);
                if (mask >> (col + 1)).count_ones() % 2 == 1 {
                    term = term.neg(f);
                }
                let next = mask | 1 << col;
                dp[next] = dp[next].add(f, &term);
            }
        }
    }
    unreachable!()
}

fn gaussian_det(f: Field, mut a: Vec<Vec<E>>) -> E {
    let n = a.len();
    let mut d = f.scalar(1);
    for j in 0..n {
        let Some(i) = (j..n).find(|&i| a[i][j] != [0; 4]) else {
            return [0; 4];
        };
        if i != j {
            a.swap(i, j);
            d = f.neg(d);
        }
        let pivot = a[j][j];
        d = f.mul(d, pivot);
        let inverse = f.pow(pivot, f.p.pow(4) - 2);
        let row: Vec<_> = (j..n).map(|k| f.mul(a[j][k], inverse)).collect();
        for i in j + 1..n {
            let scale = a[i][j];
            for k in j..n {
                a[i][k] = f.sub(a[i][k], f.mul(scale, row[k - j]));
            }
        }
    }
    d
}

fn generic(
    f: Field,
    b: E,
    h: &integer::ZPoly,
    r: Point,
) -> (Polynomial, Vec<Vec<Polynomial>>, u16) {
    let a = h_coefficients(f, b, h);
    let z = Polynomial::var(f, 3);
    if r.inf {
        let value = a
            .iter()
            .enumerate()
            .fold(Polynomial::default(), |s, (i, c)| {
                s.add(f, &c.mul(f, &z.pow(f, i as u16)))
            });
        return (value, Vec::new(), 4);
    }
    let q = s3_coefficients(f, b, &z, r.x);
    let matrix = sylvester(&a, &q);
    (determinant(f, &matrix), matrix, 8)
}

fn maps(f: Field, b: E, omega: u64, arm: usize) -> (Polynomial, Polynomial) {
    let u = Polynomial::var(f, 3);
    let b = Polynomial::constant(b);
    match arm {
        7 => {
            let w2 = f.scalar(omega * omega % f.p);
            (
                u.pow(f, 3)
                    .add(f, &b.scale(f, f.scalar(4)))
                    .scale(f, f.neg(w2)),
                u.pow(f, 2).scale(f, f.scalar(3)),
            )
        }
        9 => (
            u.pow(f, 4).sub(f, &b.mul(f, &u).scale(f, f.scalar(8))),
            u.pow(f, 3).add(f, &b).scale(f, f.scalar(4)),
        ),
        8 => (u, Polynomial::scalar(f, 1)),
        _ => panic!("unregistered image map"),
    }
}

fn compose(f: Field, p: &Polynomial, n: &Polynomial, d: &Polynomial, power: u16) -> Polynomial {
    let np: Vec<_> = (0..=power).map(|i| n.pow(f, i)).collect();
    let dp: Vec<_> = (0..=power).map(|i| d.pow(f, i)).collect();
    let mut out = Polynomial::default();
    for (&e, &c) in &p.0 {
        assert!(e[3] <= power && e[4] == 0);
        let mut coefficient = Polynomial::default();
        coefficient.term(f, [e[0], e[1], e[2], 0, 0], c);
        out = out.add(
            f,
            &coefficient
                .mul(f, &np[e[3] as usize])
                .mul(f, &dp[(power - e[3]) as usize]),
        );
    }
    out
}

fn reduce_orbit(f: Field, polynomial: &Polynomial, orbit: u64) -> Polynomial {
    let mut result = Polynomial::default();
    for (&powers, &coefficient) in &polynomial.0 {
        let mut reduced = powers;
        reduced[3] %= 12;
        result.term(
            f,
            reduced,
            f.mul(
                coefficient,
                f.scalar(fpow(orbit, (powers[3] / 12) as u64, f.p)),
            ),
        );
    }
    result
}

fn compose_orbit(
    f: Field,
    polynomial: &Polynomial,
    z: &Polynomial,
    power: u16,
    orbit: u64,
) -> Polynomial {
    let mut powers = vec![Polynomial::scalar(f, 1)];
    for i in 1..=power {
        powers.push(reduce_orbit(f, &powers[i as usize - 1].mul(f, z), orbit));
    }
    let mut result = Polynomial::default();
    for (&e, &c) in &polynomial.0 {
        assert!(e[3] <= power && e[4] == 0);
        let mut coefficient = Polynomial::default();
        coefficient.term(f, [e[0], e[1], e[2], 0, 0], c);
        result = result.add(f, &coefficient.mul(f, &powers[e[3] as usize]));
    }
    result
}

fn source_orbits(case: &Case) -> BTreeMap<u64, Vec<usize>> {
    let mut orbits = BTreeMap::<u64, Vec<usize>>::new();
    for (i, p) in case.pool[..case.l0].iter().enumerate() {
        assert_ne!(p.p.x[0], 0);
        assert_eq!(p.p.x[1..], [0, 0, 0]);
        orbits
            .entry(fpow(p.p.x[0], 12, case.p))
            .or_default()
            .push(i);
    }
    let mut product = vec![1u64];
    for (&t, ids) in &orbits {
        assert_ne!(t, 0);
        assert_eq!(ids.len(), 12, "registered source orbit has wrong size");
        let mut next = vec![0; product.len() + 12];
        for (i, &c) in product.iter().enumerate() {
            next[i] = (next[i] + case.p - c * t % case.p) % case.p;
            next[i + 12] = (next[i + 12] + c) % case.p;
        }
        product = next;
    }
    assert_eq!(
        product,
        domain(case),
        "source-orbit factorization does not equal full domain"
    );
    orbits
}

fn orbit_map(case: &Case, arm: usize, t: u64) -> Polynomial {
    let f = case.c.f;
    let u = Polynomial::var(f, 3);
    let b = case.c.b;
    match arm {
        7 => {
            let den = 3 * t % f.p;
            assert_ne!(den, 0);
            let scale =
                f.neg(f.scalar(case.omega * case.omega % f.p * fpow(den, f.p - 2, f.p) % f.p));
            u.scale(f, f.mul(scale, f.scalar(t))).add(
                f,
                &u.pow(f, 10).scale(f, f.mul(scale, f.mul(f.scalar(4), b))),
            )
        }
        9 => {
            let c = f.pow(b, 4);
            assert_eq!(c[1..], [0, 0, 0]);
            let den = 4 * (t + f.p - c[0]) % f.p;
            assert_ne!(den, 0);
            let inverse = f.scalar(fpow(den, f.p - 2, f.p));
            u.scale(f, f.mul(f.add(f.scalar(t), f.mul(f.scalar(8), c)), inverse))
                .add(
                    f,
                    &u.pow(f, 10)
                        .scale(f, f.mul(f.neg(f.mul(f.scalar(9), b)), inverse)),
                )
                .add(
                    f,
                    &u.pow(f, 7)
                        .scale(f, f.mul(f.mul(f.scalar(9), f.pow(b, 2)), inverse)),
                )
                .add(
                    f,
                    &u.pow(f, 4)
                        .scale(f, f.mul(f.neg(f.mul(f.scalar(9), f.pow(b, 3))), inverse)),
                )
        }
        8 => u,
        _ => panic!("unregistered orbit map"),
    }
}

pub(crate) fn export_orbits(public: &Path, expected: &str, arm_name: &str, qi: usize, out: &Path) {
    let raw = checked_public(public, expected);
    let arm = arm_index(arm_name);
    assert!(qi < raw["query_trace"].as_array().unwrap().len());
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    let f = case.c.f;
    let r = case.residual(&case.raw["query_trace"][qi]);
    let (_, h) = integer::derive();
    let (generic, matrix, power) = generic(f, case.c.b, &h, r);
    let checks = probes(f, &generic, &matrix);
    let mut blocks = Vec::new();
    for (t, source_ids) in source_orbits(&case) {
        let label = format!("orbit-{t}");
        let directory = out.join(&label);
        std::fs::create_dir(&directory).unwrap();
        let z = orbit_map(&case, arm, t);
        for &source in &source_ids {
            let x = [[0; 4], [0; 4], [0; 4], case.pool[source].p.x, [0; 4]];
            let image = case.peels[arm]
                .iter()
                .find(|&&id| case.pool[id].source_id == source)
                .unwrap();
            assert_eq!(
                z.eval(f, &x),
                case.pool[*image].p.x,
                "orbit formula changed the image point"
            );
        }
        let joined = compose_orbit(f, &generic, &z, power, t);
        assert!(joined.0.keys().all(|e| e[3] < 12));
        let mut equations = joined.components(f, 5);
        let guard = Polynomial::var(f, 4)
            .mul(f, &geometric_guard(f, arm))
            .sub(f, &Polynomial::scalar(f, 1));
        equations.push(guard.fp_equation(f, 5));
        let constraint = Polynomial::var(f, 3)
            .pow(f, 12)
            .sub(f, &Polynomial::scalar(f, t));
        equations.push(constraint.fp_equation(f, 5));
        let record = json!({"schema":"joint-image-system/v1","p":case.p,"seed":case.seed,"arm":arm_name,"query_index":qi,"orbit_value":t,"source_ids":source_ids,"variables":5,"equations":input_json(&equations)});
        write(&directory.join("system.json"), &record);
        std::fs::write(
            directory.join("system.ms"),
            msolve_input(&equations, case.p, 5),
        )
        .unwrap();
        let bytes = std::fs::read(directory.join("system.json")).unwrap();
        blocks.push(json!({"id":label,"orbit_value":t,"source_ids":source_ids,"system_path":format!("{label}/system.json"),"system_sha256":hex::encode(hash::sha256::sha256(&bytes)),"relation_stats":joined.stats(),"equations":equations.len()}));
    }
    write(
        &out.join("manifest.json"),
        &json!({"schema":"joint-image-orbit-inputs/v1","p":case.p,"seed":case.seed,"arm":arm_name,"query_index":qi,"public_input_sha256":expected,"determinant_evaluation_checks":checks,"blocks":blocks,"source_orbit_factorization_verified":true,"status":"exported-not-solved"}),
    );
    println!(
        "{}",
        json!({"p":case.p,"seed":case.seed,"arm":arm_name,"query":qi,"blocks":blocks.len(),"degree_u_bound_after_reduction":11})
    );
}

fn discriminant(f: Field) -> Polynomial {
    let a = Polynomial::var(f, 0);
    let b = Polynomial::var(f, 1);
    let c = Polynomial::var(f, 2);
    a.pow(f, 2)
        .mul(f, &b.pow(f, 2))
        .sub(f, &b.pow(f, 3).scale(f, f.scalar(4)))
        .sub(f, &a.pow(f, 3).mul(f, &c).scale(f, f.scalar(4)))
        .sub(f, &c.pow(f, 2).scale(f, f.scalar(27)))
        .add(f, &a.mul(f, &b).mul(f, &c).scale(f, f.scalar(18)))
}

fn geometric_guard(f: Field, arm: usize) -> Polynomial {
    let delta = discriminant(f);
    if arm != 8 {
        return delta;
    }
    let u = Polynomial::var(f, 3);
    let at_u = u
        .pow(f, 3)
        .sub(f, &Polynomial::var(f, 0).mul(f, &u.pow(f, 2)))
        .add(f, &Polynomial::var(f, 1).mul(f, &u))
        .sub(f, &Polynomial::var(f, 2));
    delta.mul(f, &at_u)
}

fn domain(case: &Case) -> Vec<u64> {
    let p = case.p;
    let mut g = vec![1];
    for factor in &case.pool[..case.l0] {
        let x = factor.p.x;
        assert_eq!(x[1..], [0, 0, 0]);
        let mut next = vec![0; g.len() + 1];
        for (i, &c) in g.iter().enumerate() {
            next[i] = (next[i] + p - c * x[0] % p) % p;
            next[i + 1] = (next[i + 1] + c) % p;
        }
        g = next;
    }
    g
}

fn remainder(f: Field, g: &[u64], divisor: &[Polynomial]) -> Vec<Polynomial> {
    let degree = divisor.len() - 1;
    assert_eq!(divisor[degree], Polynomial::scalar(f, 1));
    let mut r: Vec<_> = g.iter().map(|&c| Polynomial::scalar(f, c)).collect();
    for i in (degree..r.len()).rev() {
        let lead = std::mem::take(&mut r[i]);
        for j in 0..degree {
            r[i - degree + j] = r[i - degree + j].sub(f, &lead.mul(f, &divisor[j]));
        }
    }
    r.truncate(degree);
    r
}

fn finite_constraints(f: Field, arm: usize, g: &[u64]) -> Vec<Polynomial> {
    let a = Polynomial::var(f, 0);
    let b = Polynomial::var(f, 1);
    let c = Polynomial::var(f, 2);
    let u = Polynomial::var(f, 3);
    if arm == 8 {
        remainder(
            f,
            g,
            &[
                u.mul(f, &c),
                c.neg(f).sub(f, &u.mul(f, &b)),
                b.add(f, &u.mul(f, &a)),
                a.neg(f).sub(f, &u),
                Polynomial::scalar(f, 1),
            ],
        )
    } else {
        let mut constraints = remainder(f, g, &[c.neg(f), b, a.neg(f), Polynomial::scalar(f, 1)]);
        let gu = g.iter().rev().fold(Polynomial::default(), |s, &c| {
            s.mul(f, &u).add(f, &Polynomial::scalar(f, c))
        });
        constraints.push(gu);
        constraints
    }
}

fn input_json(input: &[f4_fp::Poly]) -> Value {
    json!(input
        .iter()
        .map(|p| p.iter().map(|(e, c)| json!([e, c])).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn msolve_input(input: &[f4_fp::Poly], p: u64, n: usize) -> String {
    assert!(
        input.iter().flatten().all(|(e, _)| e.len() == n),
        "exported exponent dimension mismatch"
    );
    let names = ["e1", "e2", "e3", "u", "v", "s"];
    let polys = input
        .iter()
        .map(|poly| {
            if poly.is_empty() {
                return "0".to_string();
            }
            poly.iter()
                .map(|(e, c)| {
                    let mut term = c.to_string();
                    for i in 0..n {
                        if e[i] > 0 {
                            term.push_str(&format!("*{}^{}", names[i], e[i]));
                        }
                    }
                    term
                })
                .collect::<Vec<_>>()
                .join("+")
        })
        .collect::<Vec<_>>()
        .join(",\n");
    format!("{}\n{p}\n{polys}\n", names[..n].join(","))
}

fn probes(f: Field, p: &Polynomial, matrix: &[Vec<Polynomial>]) -> usize {
    if matrix.is_empty() {
        return 0;
    }
    for j in 0..16u64 {
        let x = std::array::from_fn(|i| {
            std::array::from_fn(|k| (17 * j + 13 * i as u64 + 7 * k as u64 + 3 * j * j + 1) % f.p)
        });
        let numeric: Vec<_> = matrix
            .iter()
            .map(|r| r.iter().map(|p| p.eval(f, &x)).collect())
            .collect();
        assert_eq!(
            p.eval(f, &x),
            gaussian_det(f, numeric),
            "symbolic/numeric determinant mismatch"
        );
    }
    16
}

pub(crate) fn export(public: &Path, expected: &str, arm_name: &str, qi: usize, out: &Path) {
    let raw = checked_public(public, expected);
    let arm = arm_index(arm_name);
    assert!(qi < raw["query_trace"].as_array().unwrap().len());
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    let f = case.c.f;
    let r = case.residual(&case.raw["query_trace"][qi]);
    let (_, h) = integer::derive();
    let (generic, matrix, power) = generic(f, case.c.b, &h, r);
    let evaluations = probes(f, &generic, &matrix);
    let (n, d) = maps(f, case.c.b, case.omega, arm);
    let mut map_checks = 0;
    for &id in &case.peels[arm] {
        let u = case.pool[case.pool[id].source_id].p.x;
        let x = [[0; 4], [0; 4], [0; 4], u, [0; 4]];
        let den = d.eval(f, &x);
        assert_ne!(den, [0; 4]);
        let z = f.mul(n.eval(f, &x), f.pow(den, f.p.pow(4) - 2));
        assert_eq!(z, case.pool[id].p.x);
        map_checks += 1;
    }
    let joined = compose(f, &generic, &n, &d, power);
    let g = domain(&case);
    let mut geometric = joined.components(f, 5);
    let saturation = Polynomial::var(f, 4)
        .mul(f, &geometric_guard(f, arm))
        .sub(f, &Polynomial::scalar(f, 1));
    geometric.push(saturation.fp_equation(f, 5));
    let mut finite = joined.components(f, 4);
    for c in finite_constraints(f, arm, &g) {
        finite.push(c.fp_equation(f, 4));
    }
    let domain_points: Vec<_> = case.pool[..case.l0].iter().map(|f| f.p.x[0]).collect();
    let mut systems = Vec::new();
    let mut encodings = vec![("geometric", geometric, 5), ("finite-domain", finite, 4)];
    if arm == 8 {
        let mut under = joined.components(f, 5);
        let guard = Polynomial::var(f, 4)
            .mul(f, &discriminant(f))
            .sub(f, &Polynomial::scalar(f, 1));
        under.push(guard.fp_equation(f, 5));
        encodings.push(("geometric-cubic-only-diagnostic", under, 5));
    }
    for (name, input, vars) in encodings {
        let equations = input_json(&input);
        let bytes = serde_json::to_vec(&equations).unwrap();
        let stats:Vec<_>=input.iter().map(|poly|json!({"terms":poly.len(),"degree":poly.iter().map(|(e,_)|e.iter().sum::<u32>()).max().unwrap_or(0)})).collect();
        write(
            &out.join(format!("{name}.json")),
            &json!({"schema":"joint-image-system/v1","p":case.p,"seed":case.seed,"arm":arm_name,"query_index":qi,"variables":vars,"equations":equations,"input_equations_sha256":hex::encode(hash::sha256::sha256(&bytes)),"stats":stats}),
        );
        std::fs::write(
            out.join(format!("{name}.ms")),
            msolve_input(&input, case.p, vars),
        )
        .unwrap();
        systems
            .push(json!({"encoding":name,"variables":vars,"equations":input.len(),"stats":stats}));
    }
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-export/v1","status":"exported-not-solved","p":case.p,"seed":case.seed,"arm":arm_name,"query_index":qi,"public_input_sha256":expected,"residual":PointRecord::from(r),"generic_resultant":generic.stats(),"composed_resultant":joined.stats(),"denominator_clearing_power":power,"map_coordinate_checks":map_checks,"determinant_evaluation_checks":evaluations,"source_abscissae":domain_points,"domain_coefficients":g,"systems":systems,"cost_scope":"Construction and correctness diagnostics only; no admitted performance interval"}),
    );
    println!(
        "{}",
        json!({"p":case.p,"seed":case.seed,"arm":arm_name,"query":qi,"stats":joined.stats(),"systems":systems})
    );
}

fn numeric_s5(case: &Case, h: &integer::ZPoly, e: [u64; 3], z: E, r: Point) -> E {
    let f = case.c.f;
    let x = [
        f.scalar(e[0]),
        f.scalar(e[1]),
        f.scalar(e[2]),
        [0; 4],
        [0; 4],
    ];
    let a = h_coefficients(f, case.c.b, h);
    if r.inf {
        return a.iter().enumerate().fold([0; 4], |sum, (i, a)| {
            f.add(sum, f.mul(a.eval(f, &x), f.pow(z, i as u64)))
        });
    }
    let q = s3_coefficients(f, case.c.b, &Polynomial::constant(z), r.x);
    let matrix = sylvester(&a, &q)
        .iter()
        .map(|row| row.iter().map(|p| p.eval(f, &x)).collect())
        .collect();
    gaussian_det(f, matrix)
}

pub(crate) fn audit(tape: &Path, refs: &Path, double_refs: &Path, out: &Path) {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let (_, h) = integer::derive();
    let mut results = Vec::new();
    let mut total = 0;
    let mut presentations = 0;
    for p in PRIMES {
        for seed in 0..3 {
            let raw = load(tape, INPUT_SHA, p, seed);
            let case = Case::new(raw, out);
            let f = case.c.f;
            for arm_name in JOINT_ARMS {
                let arm = arm_index(arm_name);
                let root = if arm == 9 { double_refs } else { refs };
                let path = root.join(format!("p{p}-seed{seed}/{arm_name}.json"));
                let reference: Value =
                    serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
                let mut count = 0;
                let mut incidence = 0;
                for (qi, rec) in reference["trace"].as_array().unwrap().iter().enumerate() {
                    let r = case.residual(&case.raw["query_trace"][qi]);
                    let relations: BTreeSet<Terms> =
                        serde_json::from_value(rec["signed_relations"].clone()).unwrap();
                    for terms in relations {
                        case.verify_terms(&terms, r);
                        count += 1;
                        let positions: Vec<_> = terms
                            .iter()
                            .enumerate()
                            .filter(|(_, v)| case.peels[arm].contains(&v.0))
                            .map(|(i, _)| i)
                            .collect();
                        assert_eq!(positions.len(), if arm == 8 { 4 } else { 1 });
                        for position in positions {
                            let xs: Vec<_> = terms
                                .iter()
                                .enumerate()
                                .filter(|(i, _)| *i != position)
                                .map(|(_, v)| {
                                    let x = case.pool[v.0].p.x;
                                    assert_eq!(x[1..], [0, 0, 0]);
                                    x[0]
                                })
                                .collect();
                            let e = [
                                xs.iter().sum::<u64>() % p,
                                (xs[0] * xs[1] + xs[0] * xs[2] + xs[1] * xs[2]) % p,
                                xs[0] * xs[1] % p * xs[2] % p,
                            ];
                            assert_ne!(
                                discriminant(f).eval(
                                    f,
                                    &[
                                        f.scalar(e[0]),
                                        f.scalar(e[1]),
                                        f.scalar(e[2]),
                                        [0; 4],
                                        [0; 4]
                                    ]
                                ),
                                [0; 4]
                            );
                            assert_eq!(
                                numeric_s5(&case, &h, e, case.pool[terms[position].0].p.x, r),
                                [0; 4],
                                "known physical relation fails S5"
                            );
                            incidence += 1;
                        }
                    }
                }
                total += count;
                presentations += incidence;
                results.push(json!({"p":p,"seed":seed,"arm":arm_name,"reference":path,"signed_relations":count,"presentations_checked":incidence,"status":"pass"}));
            }
        }
    }
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-full-stream-incidence/v1","status":"pass","cases":results,"signed_relations":total,"presentations_checked":presentations,"scope":"Independent numerical resultant incidence on every existing full-stream signed relation, not a polynomial solver or performance result"}),
    );
    println!(
        "{}",
        json!({"status":"pass","cases":results.len(),"signed_relations":total,"presentations_checked":presentations})
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_registered_source_orbit_preserves_all_three_maps() {
        let tape =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../quartic_twist_mixed_v1.ndjson");
        let out = std::env::temp_dir().join(format!("joint-orbit-check-{}", std::process::id()));
        std::fs::create_dir(&out).unwrap();
        for p in PRIMES {
            for seed in 0..3 {
                let raw = public_record(&load(&tape, INPUT_SHA, p, seed));
                validate_public(&raw);
                let case = Case::new(raw, &out);
                let f = case.c.f;
                let orbits = source_orbits(&case);
                for arm in [7, 8, 9] {
                    for (&t, sources) in &orbits {
                        let z = orbit_map(&case, arm, t);
                        for &source in sources {
                            let image = case.peels[arm]
                                .iter()
                                .find(|&&i| case.pool[i].source_id == source)
                                .unwrap();
                            let x = [[0; 4], [0; 4], [0; 4], case.pool[source].p.x, [0; 4]];
                            assert_eq!(z.eval(f, &x), case.pool[*image].p.x);
                        }
                    }
                }
                if p == 37 && seed == 0 {
                    let (_, h) = integer::derive();
                    let r = case.residual(&case.raw["query_trace"][2]);
                    let (generic, _, power) = generic(f, case.c.b, &h, r);
                    for arm in [7, 8, 9] {
                        for (&t, sources) in &orbits {
                            let z = orbit_map(&case, arm, t);
                            let reduced = compose_orbit(f, &generic, &z, power, t);
                            let mut nonzero = 0;
                            for &source in sources {
                                let x = [
                                    f.scalar(3),
                                    f.scalar(7),
                                    f.scalar(11),
                                    case.pool[source].p.x,
                                    [0; 4],
                                ];
                                let mut value = x;
                                value[3] = z.eval(f, &x);
                                let direct = generic.eval(f, &value);
                                assert_eq!(reduced.eval(f, &x), direct);
                                if direct != [0; 4] {
                                    nonzero += 1;
                                }
                            }
                            assert!(
                                nonzero > 0,
                                "orbit reduction test accidentally checked only zeros"
                            );
                        }
                    }
                }
            }
        }
        std::fs::remove_dir(&out).unwrap();
    }
    #[test]
    fn determinant_matches_gaussian_including_singular_inputs() {
        let f = Field::new(37);
        let a = [
            Polynomial::var(f, 0),
            Polynomial::var(f, 1),
            Polynomial::var(f, 2),
            Polynomial::scalar(f, 3),
            Polynomial::scalar(f, 1),
        ];
        let q = [
            Polynomial::var(f, 3),
            Polynomial::scalar(f, 2),
            Polynomial::scalar(f, 1),
        ];
        let matrix = sylvester(&a, &q);
        let p = determinant(f, &matrix);
        assert_eq!(probes(f, &p, &matrix), 16);
        assert_eq!(gaussian_det(f, vec![vec![f.scalar(1); 2]; 2]), [0; 4]);
    }
    #[test]
    fn finite_domain_divisibility_excludes_repeated_roots() {
        let f = Field::new(37);
        let a = Polynomial::var(f, 0);
        let b = Polynomial::var(f, 1);
        let c = Polynomial::var(f, 2);
        let g = vec![31, 11, 31, 1]; // (T-1)(T-2)(T-3)
        let rem = remainder(f, &g, &[c.neg(f), b, a.neg(f), Polynomial::scalar(f, 1)]);
        let good = [f.scalar(6), f.scalar(11), f.scalar(6), [0; 4], [0; 4]];
        let repeated = [f.scalar(4), f.scalar(5), f.scalar(2), [0; 4], [0; 4]]; // 1,1,2
        assert!(rem.iter().all(|p| p.eval(f, &good) == [0; 4]));
        assert!(rem.iter().any(|p| p.eval(f, &repeated) != [0; 4]));
    }
    #[test]
    fn public_allowlist_rejects_answer_injection() {
        let mut raw = json!({"p":37,"seed":0,"n":1873633,"b0":2,"generator_x":[1,0,0,0],"generator_y":[1,0,0,0],"target_x":[1,0,0,0],"target_y":[1,0,0,0],"omega":10,"lambda_cubic":1,"lambda_frob":1,"base_coordinates":[[[1,0,0,0],[1,0,0,0]]],"query_trace":[[1,2]]});
        validate_public(&raw);
        raw.as_object_mut()
            .unwrap()
            .insert("known_scalar".into(), json!(271));
        assert!(std::panic::catch_unwind(|| validate_public(&raw)).is_err());
    }
    #[test]
    fn quartic_domain_excludes_the_repeated_fourth_factor() {
        let f = Field::new(37);
        let g = vec![24, 24, 35, 27, 1]; // (T-1)(T-2)(T-3)(T-4)
        let equations = finite_constraints(f, 8, &g);
        let x = [f.scalar(6), f.scalar(11), f.scalar(6), f.scalar(4), [0; 4]];
        assert!(equations.iter().all(|p| p.eval(f, &x) == [0; 4]));
        let repeated = [f.scalar(6), f.scalar(11), f.scalar(6), f.scalar(3), [0; 4]];
        assert!(equations.iter().any(|p| p.eval(f, &repeated) != [0; 4]));
        assert_eq!(geometric_guard(f, 8).eval(f, &repeated), [0; 4]);
        assert_ne!(geometric_guard(f, 7).eval(f, &repeated), [0; 4]);
        assert_ne!(geometric_guard(f, 9).eval(f, &repeated), [0; 4]);
    }
    #[test]
    fn rational_composition_keeps_the_exact_clearing_power() {
        let f = Field::new(37);
        let b = [0, 0, 0, 2];
        let (_, h) = integer::derive();
        let r = Point {
            x: [2, 3, 4, 5],
            y: [0; 4],
            inf: false,
        };
        let (polynomial, matrix, power) = generic(f, b, &h, r);
        assert_eq!(probes(f, &polynomial, &matrix), 16);
        for arm in [7, 8, 9] {
            let (n, d) = maps(f, b, 10, arm);
            let joined = compose(f, &polynomial, &n, &d, power);
            for u in 1..=4 {
                let x = [f.scalar(3), f.scalar(7), f.scalar(11), f.scalar(u), [0; 4]];
                let denominator = d.eval(f, &x);
                assert_ne!(denominator, [0; 4]);
                let mut z = x;
                z[3] = f.mul(n.eval(f, &x), f.pow(denominator, f.p.pow(4) - 2));
                assert_eq!(
                    joined.eval(f, &x),
                    f.mul(f.pow(denominator, power as u64), polynomial.eval(f, &z))
                );
            }
        }
    }
}
