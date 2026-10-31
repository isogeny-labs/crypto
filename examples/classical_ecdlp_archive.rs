//! Native checks for the historical classical-ECDLP replication archive.
//!
//! This executes no Python and takes no new timings. The existing W12
//! implementation supplies the finite-field, matrix, and zero-minor engine.
//! Only the six committed public known-answer fixtures are admitted here.
//! Historical wall times remain unqualified diagnostics, not benchmark claims.
//!
//! cargo run --release --example classical_ecdlp_archive -- check
//! cargo run --release --example classical_ecdlp_archive -- summary
//! cargo run --release --example classical_ecdlp_archive -- solve

#[allow(dead_code)]
#[path = "lit2026_w12_zero_minor.rs"]
mod engine;

use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, GroupOps, PrimeCurve, PrimeInstance, PrimePoint,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const STUDY: &str = "research/classical_ecdlp_20261028";
const FIXTURES: [(u64, u64, u64, u64, [u64; 2]); 6] = [
    (251, 1, 4, 271, [0, 2]),
    (1021, 1, 6, 991, [2, 4]),
    (4093, 1, 22, 4049, [4, 1121]),
    (16381, 2, 16, 16567, [0, 4]),
    (65521, 1, 35, 65761, [28357, 64561]),
    (1048573, 1, 14, 1048193, [992029, 718933]),
];

macro_rules! require {
    ($condition:expr, $message:expr) => {
        if !$condition {
            return Err($message.to_string());
        }
    };
}

#[derive(Clone, Deserialize)]
struct Archive {
    seed: u64,
    cases: Vec<Case>,
}

#[derive(Clone, Deserialize)]
struct Case {
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    #[serde(rename = "P")]
    generator: [u64; 2],
    #[serde(rename = "Q")]
    target: [u64; 2],
    trial: usize,
    expected_log: u64,
    n: usize,
    ell: usize,
    defect: usize,
    guess_determine: Arm,
    pollard_rho: Arm,
}

#[derive(Clone, Deserialize)]
struct Arm {
    status: String,
    log: u64,
    seconds: f64,
    #[serde(default)]
    stats: BTreeMap<String, u64>,
    witness: Option<Witness>,
}

#[derive(Clone, Deserialize)]
struct Record {
    side: String,
    scalar: u64,
    point: [u64; 2],
}

#[derive(Clone, Deserialize)]
struct Witness {
    n: usize,
    defect: usize,
    #[serde(rename = "K")]
    kernel: Vec<Vec<u64>>,
    records: Vec<Record>,
    a: Vec<usize>,
    b: Vec<usize>,
    remaining_columns: Vec<usize>,
    #[serde(rename = "X")]
    x: Vec<usize>,
    #[serde(rename = "Y")]
    y: Vec<usize>,
    normalized_kernel: Vec<u64>,
    reduced_minor_columns: Vec<usize>,
    zero_minor_columns: Vec<usize>,
    point_sum_indices: Vec<usize>,
    alpha: u64,
    beta: u64,
}

fn point(v: [u64; 2]) -> PrimePoint {
    PrimePoint::affine(v[0], v[1])
}

fn prime(n: u64) -> bool {
    n >= 2
        && (2..)
            .take_while(|d| d * d <= n)
            .all(|d| !n.is_multiple_of(d))
}

fn exact_order(c: &PrimeCurve) -> u64 {
    let mut roots = vec![0u8; c.p as usize];
    for y in 0..c.p {
        roots[(y * y % c.p) as usize] += 1;
    }
    1 + (0..c.p)
        .map(|x| u64::from(roots[c.rhs(x) as usize]))
        .sum::<u64>()
}

fn subset(indices: &[usize], length: usize, bound: usize) -> bool {
    indices.len() == length
        && indices.iter().all(|&i| i < bound)
        && indices.iter().copied().collect::<BTreeSet<_>>().len() == length
}

fn cols(m: &[Vec<u64>], indices: &[usize]) -> Vec<Vec<u64>> {
    m.iter()
        .map(|r| indices.iter().map(|&i| r[i]).collect())
        .collect()
}

fn transpose(m: &[Vec<u64>]) -> Vec<Vec<u64>> {
    (0..m[0].len())
        .map(|i| m.iter().map(|r| r[i]).collect())
        .collect()
}

fn rank(p: u64, matrix: &[Vec<u64>]) -> usize {
    let mut copy = matrix.to_vec();
    engine::rref_order(
        &mut engine::Fp::new(p),
        &mut copy,
        &(0..matrix[0].len()).collect::<Vec<_>>(),
    )
    .len()
}

fn dot(a: &[u64], b: &[u64], p: u64) -> u64 {
    a.iter().zip(b).fold(0, |acc, (&a, &b)| (acc + a * b) % p)
}

fn pow(mut a: u64, mut n: usize, p: u64) -> u64 {
    let mut answer = 1;
    while n != 0 {
        if n & 1 == 1 {
            answer = answer * a % p;
        }
        a = a * a % p;
        n >>= 1;
    }
    answer
}

fn check_witness(case: &Case, w: &Witness) -> Result<(), String> {
    let (p, ell, h, d) = (case.p, case.ell, case.ell / 2, case.defect);
    let curve = PrimeCurve {
        p,
        a: case.a,
        b: case.b,
    };
    let generator = point(case.generator);
    let target = point(case.target);
    require!(w.n == case.n && w.defect == d, "witness parameter mismatch");
    require!(w.records.len() == 2 * ell, "wrong point-record count");
    require!(
        w.kernel.len() == ell
            && w.kernel
                .iter()
                .all(|r| r.len() == 2 * ell && r.iter().all(|&x| x < p)),
        "invalid kernel shape or entries"
    );
    let mut ops = GroupOps::default();
    let mut points = BTreeSet::new();
    for (i, r) in w.records.iter().enumerate() {
        require!(
            r.scalar > 0 && r.scalar < case.r,
            "scalar outside subgroup range"
        );
        require!(
            r.point.iter().all(|&x| x < p) && curve.is_on_curve(point(r.point)),
            "invalid recorded point"
        );
        require!(points.insert(r.point), "duplicate point");
        let base = if i < ell {
            require!(r.side == "P", "wrong P-side label");
            generator
        } else {
            require!(r.side == "Q", "wrong Q-side label");
            curve.neg(target)
        };
        require!(
            curve.mul(&mut ops, base, r.scalar) == point(r.point),
            "label does not generate recorded point"
        );
    }
    let powers: Vec<_> = (0..=w.n)
        .flat_map(|a| (0..=w.n - a).map(move |b| (a, b)))
        .collect();
    let m: Vec<Vec<u64>> = w
        .records
        .iter()
        .map(|r| {
            powers
                .iter()
                .map(|&(a, b)| pow(r.point[0], a, p) * pow(r.point[1], b, p) % p)
                .collect()
        })
        .collect();
    require!(
        rank(p, &m) == ell && rank(p, &w.kernel) == ell,
        "incorrect evaluation/kernel rank"
    );
    let mt = transpose(&m);
    require!(
        w.kernel
            .iter()
            .all(|row| mt.iter().all(|column| dot(row, column, p) == 0)),
        "K*M is nonzero"
    );
    for (i, row) in w.kernel.iter().enumerate() {
        require!(
            (ell..2 * ell).all(|j| row[j] == u64::from(i + j == 2 * ell - 1)),
            "kernel is not anti-diagonal systematic"
        );
    }
    require!(subset(&w.a, h, ell), "invalid outer guess");
    require!(subset(&w.b, h - d, h), "invalid inner guess");
    require!(
        subset(&w.x, d - 1, h + d) && subset(&w.y, d - 1, h + d) && w.x != w.y,
        "invalid collision subsets"
    );
    let remaining: Vec<_> = (0..ell).filter(|i| !w.b.contains(i)).collect();
    require!(
        w.remaining_columns == remaining,
        "incorrect signature-column map"
    );
    let union: BTreeSet<_> = w.x.iter().chain(&w.y).copied().collect();
    require!(union.len() == d, "collision union does not have size d");
    let expected_i: BTreeSet<_> =
        w.b.iter()
            .copied()
            .chain(union.iter().map(|&i| remaining[i]))
            .collect();
    require!(
        subset(&w.reduced_minor_columns, h, ell)
            && w.reduced_minor_columns
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                == expected_i,
        "incorrect reduced minor"
    );

    let mut reduced = transpose(&cols(&w.kernel, &w.a));
    let pivot_order: Vec<_> = (h..ell).rev().chain(0..h).collect();
    let pivots = engine::rref_order(&mut engine::Fp::new(p), &mut reduced, &pivot_order);
    require!(pivots == pivot_order[..h], "singular guess-reduction block");
    let t = engine::left_kernel(&mut engine::Fp::new(p), &cols(&reduced, &w.b), h, h - d);
    require!(
        t.len() == d
            && w.normalized_kernel.len() == d
            && w.normalized_kernel.iter().all(|&x| x < p),
        "invalid intersection kernel"
    );
    require!(
        w.normalized_kernel.iter().find(|&&x| x != 0) == Some(&1),
        "intersection kernel is not normalized"
    );
    let mut f = engine::Fp::new(p);
    let signatures: Vec<Vec<u64>> = remaining
        .iter()
        .map(|&j| {
            t.iter()
                .map(|row| (0..h).fold(0, |acc, k| (acc + row[k] * reduced[k][j]) % p))
                .collect()
        })
        .collect();
    for indices in [&w.x, &w.y] {
        let rows: Vec<_> = indices.iter().map(|&i| signatures[i].clone()).collect();
        require!(
            engine::right_kernel(&mut f, &rows, d).len() == 1,
            "intersection is not one-dimensional"
        );
        require!(
            rows.iter()
                .all(|row| dot(row, &w.normalized_kernel, p) == 0),
            "saved collision vector is not in both kernels"
        );
    }

    let expected_c: BTreeSet<_> =
        w.a.iter()
            .copied()
            .chain(
                (0..ell)
                    .filter(|i| !expected_i.contains(i))
                    .map(|i| 2 * ell - 1 - i),
            )
            .collect();
    require!(
        subset(&w.zero_minor_columns, ell, 2 * ell)
            && w.zero_minor_columns
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                == expected_c,
        "incorrect lifted zero-minor map"
    );
    require!(
        engine::det_uncounted(p, &cols(&w.kernel, &w.zero_minor_columns)) == 0,
        "selected minor is nonsingular"
    );
    let support: Vec<_> = (0..2 * ell).filter(|i| !expected_c.contains(i)).collect();
    require!(
        w.point_sum_indices == support,
        "point sum is not the complement of the zero minor"
    );
    let mut sum = curve.identity();
    for &i in &support {
        sum = curve.add(&mut ops, sum, point(w.records[i].point));
    }
    require!(sum.infinity, "selected points do not sum to identity");
    let alpha = support
        .iter()
        .filter(|&&i| i < ell)
        .map(|&i| w.records[i].scalar)
        .sum::<u64>()
        % case.r;
    let beta = support
        .iter()
        .filter(|&&i| i >= ell)
        .map(|&i| w.records[i].scalar)
        .sum::<u64>()
        % case.r;
    require!(
        alpha == w.alpha && beta == w.beta && beta != 0,
        "incorrect or uninformative label sums"
    );
    require!(
        alpha * engine::inv_mod(beta, case.r) % case.r == case.expected_log,
        "label equation gives the wrong known answer"
    );
    Ok(())
}

fn read_archive(root: &Path) -> Result<(Archive, String), String> {
    let bytes = std::fs::read(root.join("historical/guess_determine/results.json"))
        .map_err(|e| e.to_string())?;
    let digest = format!("{:x}", Sha256::digest(&bytes));
    let archive = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    Ok((archive, digest))
}

fn check_archive(archive: &Archive) -> Result<usize, String> {
    require!(
        archive.seed == 20261027 && archive.cases.len() == 60,
        "wrong archive population or seed"
    );
    let mut identities = BTreeSet::new();
    let mut curves = BTreeSet::new();
    let mut witnessed = BTreeSet::new();
    for case in &archive.cases {
        require!(
            FIXTURES.contains(&(case.p, case.a, case.b, case.r, case.generator)),
            "not one of the six public fixtures"
        );
        require!(
            case.trial < 10 && identities.insert((case.p, case.trial)),
            "duplicate or invalid trial"
        );
        require!(
            case.n >= 2
                && case.n <= 8
                && case.n.is_multiple_of(2)
                && case.ell == 3 * case.n
                && (2..case.ell / 2).contains(&case.defect),
            "invalid solve parameters"
        );
        let curve = PrimeCurve {
            p: case.p,
            a: case.a,
            b: case.b,
        };
        if curves.insert(case.p) {
            require!(
                prime(case.p) && prime(case.r) && case.p != case.r && exact_order(&curve) == case.r,
                "invalid, composite-order or anomalous curve"
            );
        }
        let mut ops = GroupOps::default();
        require!(
            case.target.iter().all(|&x| x < case.p) && curve.is_on_curve(point(case.target)),
            "invalid target"
        );
        require!(
            case.expected_log > 0
                && case.expected_log < case.r
                && curve.mul(&mut ops, point(case.generator), case.expected_log)
                    == point(case.target),
            "fixture known answer is inconsistent"
        );
        for arm in [&case.guess_determine, &case.pollard_rho] {
            require!(
                arm.status == "solved"
                    && arm.log == case.expected_log
                    && arm.seconds.is_finite()
                    && arm.seconds > 0.0,
                "unsolved or invalid historical arm"
            );
            require!(
                curve.mul(&mut ops, point(case.generator), arm.log) == point(case.target),
                "recorded answer fails group replay"
            );
        }
        require!(
            case.guess_determine
                .stats
                .get("signature_collisions")
                .copied()
                .unwrap_or(0)
                > 0,
            "no recorded signature collision"
        );
        if let Some(witness) = &case.guess_determine.witness {
            require!(
                witnessed.insert(case.p),
                "more than one witness for a curve"
            );
            check_witness(case, witness)?;
        }
    }
    require!(
        curves.len() == 6 && witnessed.len() == 6,
        "missing curve or full witness"
    );
    Ok(witnessed.len())
}

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    fn visit(n: usize, k: usize, start: usize, prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if k == 0 {
            out.push(prefix.clone());
            return;
        }
        for i in start..=n - k {
            prefix.push(i);
            visit(n, k - 1, i + 1, prefix, out);
            prefix.pop();
        }
    }
    let mut out = Vec::new();
    visit(n, k, 0, &mut Vec::new(), &mut out);
    out
}

fn finite_counts(d: usize) -> (usize, usize) {
    let h = 6;
    let mut sets = BTreeSet::new();
    let mut occurrences = 0;
    for b in combinations(h, h - d) {
        let remaining: Vec<_> = (0..2 * h).filter(|i| !b.contains(i)).collect();
        for extension in combinations(remaining.len(), d) {
            let mut set = b.clone();
            set.extend(extension.iter().map(|&i| remaining[i]));
            set.sort_unstable();
            sets.insert(set);
            occurrences += 1;
        }
    }
    (occurrences, sets.len())
}

fn aps_check() -> Result<usize, String> {
    let curve = PrimeCurve { p: 17, a: 2, b: 2 };
    let mut points = BTreeMap::new();
    for x in 0..17 {
        for y in 0..17 {
            let r = PrimePoint::affine(x, y);
            if curve.is_on_curve(r) {
                points.insert(x, r);
            }
        }
    }
    let values: Vec<_> = points.values().copied().collect();
    let mut checked = 0;
    for triple in combinations(values.len(), 3) {
        let [u, v, w] = [values[triple[0]], values[triple[1]], values[triple[2]]];
        let (x, y, z) = (i128::from(u.x), i128::from(v.x), i128::from(w.x));
        let polynomial = ((x - y).pow(2) * z * z - 2 * ((x + y) * (x * y + 2) + 4) * z
            + (x * y - 2).pow(2)
            - 8 * (x + y))
            .rem_euclid(17);
        let mut signed_zero = false;
        for vv in [v, curve.neg(v)] {
            for ww in [w, curve.neg(w)] {
                let mut ops = GroupOps::default();
                let uv = curve.add(&mut ops, u, vv);
                signed_zero |= curve.add(&mut ops, uv, ww).infinity;
            }
        }
        require!(
            (polynomial == 0) == signed_zero,
            "APS S3/sign equivalence failed"
        );
        checked += 1;
    }
    require!(checked == 84, "unexpected APS population");
    Ok(checked)
}

fn native_solve(archive: &Archive) -> Result<usize, String> {
    let mut count = 0;
    for case in archive.cases.iter().filter(|c| c.trial == 0) {
        let inst = PrimeInstance {
            name: format!("public-archive-p{}", case.p),
            curve: PrimeCurve {
                p: case.p,
                a: case.a,
                b: case.b,
            },
            group_order: case.r,
            r: case.r,
            cofactor: 1,
            generator: (case.generator[0], case.generator[1]),
        };
        let report = engine::zero_minor_solve(
            &inst,
            point(case.target),
            20261028 + case.p,
            engine::ZmParams {
                nprime: case.n,
                d: case.defect,
                cap_covered: 64.0,
            },
        );
        require!(
            report.recovered == Some(case.expected_log),
            "native engine failed a known-answer fixture"
        );
        let mut ops = GroupOps::default();
        require!(
            inst.curve
                .mul(&mut ops, inst.generator_point(), report.recovered.unwrap())
                == point(case.target),
            "native answer fails independent group check"
        );
        count += 1;
    }
    require!(count == 6, "incomplete native fixture sample");
    Ok(count)
}

#[derive(Serialize)]
struct Summary {
    p: u64,
    r: u64,
    samples: usize,
    guess_median_ms: f64,
    guess_min_ms: f64,
    guess_max_ms: f64,
    rho_median_ms: f64,
    rho_min_ms: f64,
    rho_max_ms: f64,
    historical_ratio: f64,
}

fn summaries(archive: &Archive) -> Vec<Summary> {
    FIXTURES
        .iter()
        .map(|&(p, _, _, r, _)| {
            let mut a: Vec<_> = archive
                .cases
                .iter()
                .filter(|c| c.p == p)
                .map(|c| c.guess_determine.seconds * 1000.0)
                .collect();
            let mut b: Vec<_> = archive
                .cases
                .iter()
                .filter(|c| c.p == p)
                .map(|c| c.pollard_rho.seconds * 1000.0)
                .collect();
            a.sort_by(f64::total_cmp);
            b.sort_by(f64::total_cmp);
            let median = |v: &[f64]| (v[4] + v[5]) / 2.0;
            Summary {
                p,
                r,
                samples: a.len(),
                guess_median_ms: median(&a),
                guess_min_ms: a[0],
                guess_max_ms: a[9],
                rho_median_ms: median(&b),
                rho_min_ms: b[0],
                rho_max_ms: b[9],
                historical_ratio: median(&a) / median(&b),
            }
        })
        .collect()
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let mode = args.first().map(String::as_str).unwrap_or("check");
    let root = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(STUDY));
    let (archive, digest) = read_archive(&root)?;
    let witnessed = check_archive(&archive)?;
    match mode {
        "check" => {
            require!(
                finite_counts(2) == (420, 262) && finite_counts(3) == (1680, 662),
                "finite candidate counts differ"
            );
            println!("{}", serde_json::to_string_pretty(&json!({
                "schema":"classical-archive-replay/v1", "archive_sha256":digest,
                "known_answer_instances":60,"verified_arm_answers":120,"complete_witnesses":witnessed,
                "curve_orders_recomputed":6,"aps_triples_checked":aps_check()?,
                "finite_counts":[{"h":6,"d":2,"occurrences":420,"distinct":262},{"h":6,"d":3,"occurrences":1680,"distinct":662}],
                "historical_timings_remeasured":false,"wall_time_admission":false,
                "reference_quality":"historical Python three-part rho; not the strongest matched native reference",
                "benchmark_status":"historical diagnostic only"
            })).map_err(|e| e.to_string())?);
        }
        "summary" => println!(
            "{}",
            serde_json::to_string_pretty(&summaries(&archive)).map_err(|e| e.to_string())?
        ),
        "solve" => println!(
            "{}",
            json!({"native_public_fixtures_verified":native_solve(&archive)?,"timings_measured":false})
        ),
        _ => {
            return Err(
                "usage: classical_ecdlp_archive [check|summary|solve] [study-directory]".into(),
            )
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("archive check failed: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn archive() -> Archive {
        read_archive(&Path::new(env!("CARGO_MANIFEST_DIR")).join(STUDY))
            .unwrap()
            .0
    }
    #[test]
    fn committed_archive_replays() {
        assert_eq!(check_archive(&archive()).unwrap(), 6);
    }
    #[test]
    fn changed_kernel_is_rejected() {
        let mut a = archive();
        let w = a.cases[0].guess_determine.witness.as_mut().unwrap();
        w.kernel[0][0] = (w.kernel[0][0] + 1) % 251;
        assert!(check_archive(&a).is_err());
    }
    #[test]
    fn changed_label_is_rejected() {
        let mut a = archive();
        let w = a.cases[0].guess_determine.witness.as_mut().unwrap();
        w.records[0].scalar = (w.records[0].scalar % 270) + 1;
        assert!(check_archive(&a).is_err());
    }
    #[test]
    fn duplicate_minor_column_is_rejected() {
        let mut a = archive();
        let w = a.cases[0].guess_determine.witness.as_mut().unwrap();
        w.zero_minor_columns[0] = w.zero_minor_columns[1];
        assert!(check_archive(&a).is_err());
    }
    #[test]
    fn candidate_count_enumeration() {
        assert_eq!(finite_counts(2), (420, 262));
        assert_eq!(finite_counts(3), (1680, 662));
    }
    #[test]
    fn aps_polynomial_matches_group_signs() {
        assert_eq!(aps_check().unwrap(), 84);
    }
    #[test]
    fn native_engine_solves_same_public_fixtures() {
        assert_eq!(native_solve(&archive()).unwrap(), 6);
    }
}
