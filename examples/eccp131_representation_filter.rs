//! Prospective exact-Certicom-ECCp-131 representation-filter sweep.
//!
//! This is the native instrument frozen by
//! `research/eccp131_representation_filter_20261102/protocol.json`.  It does
//! not recover the published logarithm.  It selects coordinate predicates on
//! a training cohort, evaluates the winner once on a disjoint held-out cohort,
//! and exercises exact pair-table root and modular-solver positive controls.

use crypto_lib::cryptanalysis::ec_index_calculus::gaussian_eliminate_mod_n_particular;
use crypto_lib::cryptanalysis::prime_small_x::{
    small_x_base, Curve, MElem, MontField, PrimeField, Pt,
};
use crypto_lib::cryptanalysis::secg_standards::{SecgCurve, SecgField};
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

type F = MontField<3>;
type E = MElem<3>;
type Point = Pt<E>;
type Words = [u64; 3];

const SOURCES: [&str; 5] = [
    "pair_sum_x",
    "pair_sum_y",
    "endpoint_field_add",
    "endpoint_xor",
    "endpoint_field_mul",
];
const COMBINERS: [&str; 5] = ["xor", "add", "sub_either", "equal", "target_equal"];
const FIELD_BITS: usize = 131;
const TRAIN: usize = 4096;
const TEST: usize = 8192;
const SIZES: [usize; 3] = [64, 128, 256];

#[derive(Clone)]
struct Sample {
    left: [Words; 5],
    right: [Words; 5],
    target_x: Words,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Config {
    source: usize,
    combiner: usize,
    width: usize,
    offset: usize,
}

#[derive(Clone, Debug, Serialize)]
struct Counts {
    relation_hits: usize,
    null_hits: usize,
    samples_each: usize,
    relation_rate: f64,
    null_rate: f64,
    log2_enrichment: f64,
    ratio_lower95: f64,
    relation_ci95: [f64; 2],
    null_ci95: [f64; 2],
}

#[derive(Clone, Debug, Serialize)]
struct Selected {
    config: Config,
    source_name: &'static str,
    combiner_name: &'static str,
    training: Counts,
    test: Counts,
    aa_identical: bool,
    pre_add: bool,
    enumeration_class: &'static str,
    configurations_evaluated: usize,
}

#[derive(Clone, Debug, Serialize)]
struct ArmResult {
    base_design: &'static str,
    size: usize,
    train_seed: u64,
    test_seed: u64,
    generation_ms: f64,
    sweep_ms: f64,
    selected: Selected,
}

#[derive(Clone)]
struct PairEntry {
    point: Point,
    terms: [(usize, i8); 2],
}

#[derive(Debug, Serialize)]
struct MitmResult {
    base_size: usize,
    signed_points: usize,
    pair_entries: usize,
    distinct_pair_points: usize,
    setup_ms: f64,
    planted_query_ms: f64,
    published_query_ms: f64,
    planted_found: bool,
    planted_replayed: bool,
    published_found: bool,
    planted_terms: Option<Vec<(usize, i8)>>,
}

#[derive(Debug, Serialize)]
struct SolverControl {
    columns: usize,
    rows: usize,
    rank: usize,
    all_determined: bool,
    recovered_exactly: bool,
    elapsed_ms: f64,
}

fn exact_curve() -> Result<(Curve<F>, Point, Point), String> {
    let p = CurveParams::eccp131();
    let record = SecgCurve {
        name: p.name.into(),
        oid: String::new(),
        field: SecgField::Prime { p: p.p.clone() },
        a: p.a.clone(),
        b: p.b.clone(),
        gx: p.gx.clone(),
        gy: p.gy.clone(),
        order: p.n.clone(),
        cofactor: p.h as u64,
    };
    let c = Curve::<F>::from_secg(&record)?;
    let g = c.secg_generator(&record)?;
    let (qx, qy) = CurveParams::certicom_eccp131_target();
    let q = Point {
        x: c.field.embed_big(&qx)?,
        y: c.field.embed_big(&qy)?,
    };
    let mut f = c.field.clone();
    if !c.on_curve(&mut f, &q) || c.mul_big(&mut f, &q, &c.order).is_some() {
        return Err("published ECCp-131 target failed validation".into());
    }
    Ok((c, g, q))
}

fn point_key(f: &F, p: &Point) -> (Words, Words) {
    (f.key(p.x), f.key(p.y))
}

fn random_scalar(rng: &mut StdRng, n: &BigUint) -> BigUint {
    let bytes_len = (n.bits() as usize).div_ceil(8);
    loop {
        let mut bytes = vec![0u8; bytes_len];
        rng.fill(bytes.as_mut_slice());
        if !n.bits().is_multiple_of(8) {
            *bytes.last_mut().unwrap() &= (1u8 << (n.bits() % 8)) - 1;
        }
        let k = BigUint::from_bytes_le(&bytes);
        if !k.is_zero() && &k < n {
            return k;
        }
    }
}

fn random_base(c: &Curve<F>, g: &Point, size: usize, seed: u64) -> Vec<Point> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut f = c.field.clone();
    let mut seen = HashSet::new();
    let mut out = Vec::with_capacity(size);
    while out.len() < size {
        let k = random_scalar(&mut rng, &c.order);
        let p = c.mul_big(&mut f, g, &k).expect("nonzero scalar");
        if seen.insert(f.key(p.x)) {
            out.push(p);
        }
    }
    out
}

fn signed(c: &Curve<F>, f: &mut F, p: Point, sign: i8) -> Point {
    if sign == 1 {
        p
    } else {
        c.negate(f, &p)
    }
}

fn pair_features(c: &Curve<F>, f: &mut F, a: Point, b: Point) -> Option<([Words; 5], Point)> {
    let sum = c.add_points(f, &a, &b)?;
    let ax = f.key(a.x);
    let bx = f.key(b.x);
    let endpoint_xor = [ax[0] ^ bx[0], ax[1] ^ bx[1], ax[2] ^ bx[2]];
    let endpoint_add_element = f.add(a.x, b.x);
    let endpoint_mul_element = f.mul(a.x, b.x);
    let endpoint_add = f.key(endpoint_add_element);
    let endpoint_mul = f.key(endpoint_mul_element);
    Some((
        [
            f.key(sum.x),
            f.key(sum.y),
            endpoint_add,
            endpoint_xor,
            endpoint_mul,
        ],
        sum,
    ))
}

fn samples(c: &Curve<F>, base: &[Point], count: usize, seed: u64) -> Vec<Sample> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut f = c.field.clone();
    let mut out = Vec::with_capacity(count);
    while out.len() < count {
        let mut ps = [base[0]; 4];
        for p in &mut ps {
            let i = rng.gen_range(0..base.len());
            let s = if rng.gen::<bool>() { 1 } else { -1 };
            *p = signed(c, &mut f, base[i], s);
        }
        let Some((left, lsum)) = pair_features(c, &mut f, ps[0], ps[1]) else {
            continue;
        };
        let Some((right, rsum)) = pair_features(c, &mut f, ps[2], ps[3]) else {
            continue;
        };
        let Some(target) = c.add_points(&mut f, &lsum, &rsum) else {
            continue;
        };
        out.push(Sample {
            left,
            right,
            target_x: f.key(target.x),
        });
    }
    out
}

fn bit_slice(words: Words, offset: usize, width: usize) -> u64 {
    debug_assert!(width <= 8 && offset + width <= FIELD_BITS);
    let limb = offset / 64;
    let shift = offset % 64;
    let mut value = words[limb] >> shift;
    if shift + width > 64 {
        value |= words[limb + 1] << (64 - shift);
    }
    value & ((1u64 << width) - 1)
}

fn predicate(sample: &Sample, target: Words, cfg: Config) -> bool {
    let mask = (1u64 << cfg.width) - 1;
    let a = bit_slice(sample.left[cfg.source], cfg.offset, cfg.width);
    let b = bit_slice(sample.right[cfg.source], cfg.offset, cfg.width);
    let r = bit_slice(target, cfg.offset, cfg.width);
    match cfg.combiner {
        0 => (a ^ b) == r,
        1 => a.wrapping_add(b) & mask == r,
        2 => (a.wrapping_sub(b) & mask == r) || (b.wrapping_sub(a) & mask == r),
        3 => a == b,
        4 => a == r || b == r,
        _ => unreachable!(),
    }
}

fn hits(data: &[Sample], cfg: Config) -> (usize, usize) {
    let n = data.len();
    let relation = data
        .iter()
        .filter(|s| predicate(s, s.target_x, cfg))
        .count();
    let null = data
        .iter()
        .enumerate()
        .filter(|(i, s)| predicate(s, data[(i + 1) % n].target_x, cfg))
        .count();
    (relation, null)
}

fn wilson(k: usize, n: usize) -> [f64; 2] {
    let z = 1.959_963_984_540_054;
    let p = k as f64 / n as f64;
    let den = 1.0 + z * z / n as f64;
    let center = (p + z * z / (2.0 * n as f64)) / den;
    let half = z * ((p * (1.0 - p) / n as f64 + z * z / (4.0 * (n * n) as f64)).sqrt()) / den;
    [(center - half).max(0.0), (center + half).min(1.0)]
}

fn counts(relation: usize, null: usize, n: usize) -> Counts {
    let pr = relation as f64 / n as f64;
    let pn = null as f64 / n as f64;
    let smooth_r = (relation as f64 + 0.5) / (n as f64 + 1.0);
    let smooth_n = (null as f64 + 0.5) / (n as f64 + 1.0);
    let rci = wilson(relation, n);
    let nci = wilson(null, n);
    Counts {
        relation_hits: relation,
        null_hits: null,
        samples_each: n,
        relation_rate: pr,
        null_rate: pn,
        log2_enrichment: (smooth_r / smooth_n).log2(),
        ratio_lower95: if nci[1] > 0.0 { rci[0] / nci[1] } else { 0.0 },
        relation_ci95: rci,
        null_ci95: nci,
    }
}

fn offsets(width: usize) -> Vec<usize> {
    let last = FIELD_BITS - width;
    let mut v: Vec<usize> = (0..=last).step_by(8).collect();
    if v.last().copied() != Some(last) {
        v.push(last);
    }
    v
}

fn select(train: &[Sample], test: &[Sample]) -> Selected {
    let mut best: Option<(Config, Counts)> = None;
    let mut evaluated = 0usize;
    for source in 0..SOURCES.len() {
        for combiner in 0..COMBINERS.len() {
            for width in 1..=8 {
                for offset in offsets(width) {
                    let cfg = Config {
                        source,
                        combiner,
                        width,
                        offset,
                    };
                    let (rh, nh) = hits(train, cfg);
                    evaluated += 1;
                    if nh < 32 {
                        continue;
                    }
                    let c = counts(rh, nh, train.len());
                    let replace = best.as_ref().is_none_or(|(old, oldc)| {
                        c.log2_enrichment > oldc.log2_enrichment
                            || (c.log2_enrichment == oldc.log2_enrichment
                                && (cfg.source, cfg.combiner, cfg.width, cfg.offset)
                                    < (old.source, old.combiner, old.width, old.offset))
                    });
                    if replace {
                        best = Some((cfg, c));
                    }
                }
            }
        }
    }
    let (cfg, training) = best.expect("at least width-one candidates have null support");
    let (rh, nh) = hits(test, cfg);
    let test_counts = counts(rh, nh, test.len());
    let repeat = hits(test, cfg);
    let pre_add = cfg.source >= 2;
    Selected {
        config: cfg,
        source_name: SOURCES[cfg.source],
        combiner_name: COMBINERS[cfg.combiner],
        training,
        test: test_counts,
        aa_identical: repeat == (rh, nh),
        pre_add,
        enumeration_class: if pre_add {
            "endpoint-residue buckets: O(B + residue-pair classes + output) before EC additions"
        } else {
            "post-add coordinate: requires all Theta(B^2) EC pair sums"
        },
        configurations_evaluated: evaluated,
    }
}

fn run_arm(
    c: &Curve<F>,
    base: &[Point],
    design: &'static str,
    size: usize,
    train_seed: u64,
    test_seed: u64,
) -> ArmResult {
    let started = Instant::now();
    let train = samples(c, base, TRAIN, train_seed);
    let test = samples(c, base, TEST, test_seed);
    let generation_ms = started.elapsed().as_secs_f64() * 1000.0;
    let sweep = Instant::now();
    let selected = select(&train, &test);
    let sweep_ms = sweep.elapsed().as_secs_f64() * 1000.0;
    ArmResult {
        base_design: design,
        size,
        train_seed,
        test_seed,
        generation_ms,
        sweep_ms,
        selected,
    }
}

fn add_many(c: &Curve<F>, f: &mut F, terms: &[Point]) -> Option<Point> {
    let mut acc = None;
    for p in terms {
        acc = match acc {
            None => Some(*p),
            Some(a) => c.add_points(f, &a, p),
        };
    }
    acc
}

fn mitm_probe(c: &Curve<F>, base: &[Point], q: &Point) -> MitmResult {
    let mut f = c.field.clone();
    let setup = Instant::now();
    let mut signed_points = Vec::with_capacity(2 * base.len());
    for (i, p) in base.iter().enumerate() {
        signed_points.push((*p, i, 1i8));
        signed_points.push((c.negate(&mut f, p), i, -1i8));
    }
    let mut entries = Vec::new();
    let mut index = HashMap::new();
    for i in 0..signed_points.len() {
        for j in i..signed_points.len() {
            let Some(p) = c.add_points(&mut f, &signed_points[i].0, &signed_points[j].0) else {
                continue;
            };
            let entry = PairEntry {
                point: p,
                terms: [
                    (signed_points[i].1, signed_points[i].2),
                    (signed_points[j].1, signed_points[j].2),
                ],
            };
            let pos = entries.len();
            index.entry(point_key(&f, &p)).or_insert(pos);
            entries.push(entry);
        }
    }
    let setup_ms = setup.elapsed().as_secs_f64() * 1000.0;
    let planted_terms = [(0usize, 1i8), (1, -1), (2, 1), (3, 1)];
    let planted_points: Vec<Point> = planted_terms
        .iter()
        .map(|&(i, s)| signed(c, &mut f, base[i], s))
        .collect();
    let target = add_many(c, &mut f, &planted_points).expect("fixed planted target is finite");

    let query = |target: &Point, f: &mut F| -> Option<Vec<(usize, i8)>> {
        for e in &entries {
            let minus = c.negate(f, &e.point);
            let Some(rest) = c.add_points(f, target, &minus) else {
                continue;
            };
            if let Some(&j) = index.get(&point_key(f, &rest)) {
                let other = &entries[j];
                return Some(vec![e.terms[0], e.terms[1], other.terms[0], other.terms[1]]);
            }
        }
        None
    };
    let started = Instant::now();
    let witness = query(&target, &mut f);
    let planted_query_ms = started.elapsed().as_secs_f64() * 1000.0;
    let replayed = witness.as_ref().is_some_and(|terms| {
        let ps: Vec<Point> = terms
            .iter()
            .map(|&(i, s)| signed(c, &mut f, base[i], s))
            .collect();
        add_many(c, &mut f, &ps).as_ref() == Some(&target)
    });
    let started = Instant::now();
    let published = query(q, &mut f);
    let published_query_ms = started.elapsed().as_secs_f64() * 1000.0;
    MitmResult {
        base_size: base.len(),
        signed_points: signed_points.len(),
        pair_entries: entries.len(),
        distinct_pair_points: index.len(),
        setup_ms,
        planted_query_ms,
        published_query_ms,
        planted_found: witness.is_some(),
        planted_replayed: replayed,
        published_found: published.is_some(),
        planted_terms: witness,
    }
}

fn solver_control(n: &BigUint) -> SolverControl {
    let started = Instant::now();
    let cols = 16usize;
    let known: Vec<BigUint> = (0..cols)
        .map(|i| BigUint::from((i * i + 17 * i + 3) as u64) % n)
        .collect();
    let mut matrix = vec![vec![BigUint::zero(); cols]; cols];
    for i in 0..cols {
        matrix[i][i] = BigUint::one();
        if i + 1 < cols {
            matrix[i][i + 1] = BigUint::from(3u8);
        }
    }
    let mut rhs: Vec<BigUint> = matrix
        .iter()
        .map(|row| {
            row.iter()
                .zip(&known)
                .fold(BigUint::zero(), |a, (m, x)| (a + m * x) % n)
        })
        .collect();
    let sol =
        gaussian_eliminate_mod_n_particular(&mut matrix, &mut rhs, n).expect("prime-order system");
    SolverControl {
        columns: cols,
        rows: cols,
        rank: sol.rank,
        all_determined: sol.determined.iter().all(|d| *d),
        recovered_exactly: sol.values == known,
        elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
    }
}

fn curve_digest(c: &Curve<F>, g: &Point, q: &Point) -> String {
    let f = c.field.clone();
    let mut h = Sha256::new();
    h.update(b"ECCP131-REPRESENTATION-FILTER-CURVE-v1\n");
    h.update(c.name.as_bytes());
    h.update(format!("\n{:x}\n{:x}\n", f.modulus(), c.order));
    h.update(format!(
        "{:?}\n{:?}\n{:?}\n{:?}\n",
        f.key(g.x),
        f.key(g.y),
        f.key(q.x),
        f.key(q.y)
    ));
    format!("{:x}", h.finalize())
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn write_svg(path: &Path, arms: &[ArmResult]) -> Result<(), String> {
    let zero_y = 280.0;
    let scale = 65.0;
    let mut svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1040\" height=\"520\" viewBox=\"0 0 1040 520\"><rect width=\"1040\" height=\"520\" fill=\"#f7f4ed\"/><text x=\"48\" y=\"48\" font-family=\"sans-serif\" font-size=\"26\" fill=\"#152238\">Exact ECCp-131 held-out representation-filter enrichment</text><text x=\"48\" y=\"76\" font-family=\"sans-serif\" font-size=\"14\" fill=\"#52606d\">Training-selected configuration; 8,192 relation and 8,192 independent-null samples per cell</text><line x1=\"60\" y1=\"{zero_y}\" x2=\"1000\" y2=\"{zero_y}\" stroke=\"#263238\" stroke-width=\"2\"/><text x=\"10\" y=\"{}\" font-family=\"sans-serif\" font-size=\"13\">0 bits</text>", zero_y + 5.0);
    for (i, arm) in arms.iter().enumerate() {
        let x = 105.0 + i as f64 * 145.0;
        let v = arm.selected.test.log2_enrichment.clamp(-3.5, 3.5);
        let y = zero_y - v * scale;
        let color = if arm.base_design == "small_x" {
            "#00796b"
        } else {
            "#d2691e"
        };
        let top = y.min(zero_y);
        let h = (y - zero_y).abs().max(1.0);
        svg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"72\" height=\"{}\" fill=\"{}\"/><text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"13\" fill=\"#152238\">{:.3}</text><text x=\"{}\" y=\"430\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"13\" fill=\"#152238\">{}</text><text x=\"{}\" y=\"450\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"12\" fill=\"#52606d\">B={}</text>", x, top, h, color, x + 36.0, if v >= 0.0 { top - 8.0 } else { top + h + 18.0 }, arm.selected.test.log2_enrichment, x + 36.0, escape(arm.base_design), x + 36.0, arm.size));
    }
    svg.push_str("<rect x=\"680\" y=\"88\" width=\"310\" height=\"92\" rx=\"8\" fill=\"#ffffff\" stroke=\"#c7c7c7\"/><rect x=\"700\" y=\"110\" width=\"20\" height=\"14\" fill=\"#00796b\"/><text x=\"730\" y=\"122\" font-family=\"sans-serif\" font-size=\"14\">small-x candidate</text><rect x=\"700\" y=\"142\" width=\"20\" height=\"14\" fill=\"#d2691e\"/><text x=\"730\" y=\"154\" font-family=\"sans-serif\" font-size=\"14\">matched scalar-random base</text><text x=\"48\" y=\"495\" font-family=\"sans-serif\" font-size=\"13\" fill=\"#52606d\">Bars are descriptive effect sizes; acceptance also requires ratio lower95 &gt; 1 and a subquadratic pre-add enumerator.</text></svg>");
    fs::write(path, svg).map_err(|e| e.to_string())
}

fn write_report(
    path: &Path,
    value: &serde_json::Value,
    arms: &[ArmResult],
    mitm: &MitmResult,
    solver: &SolverControl,
) -> Result<(), String> {
    let qualifying = arms.iter().any(|a| {
        a.base_design == "small_x" && a.selected.pre_add && a.selected.test.ratio_lower95 > 1.0
    }) && !arms
        .iter()
        .any(|a| a.base_design == "random_base" && a.selected.test.ratio_lower95 > 1.0);
    let verdict = if qualifying {
        "qualifying_filter_observed"
    } else {
        "no_qualifying_filter_in_frozen_family"
    };
    let mut out = format!("# Exact ECCp-131 representation-filter sweep\n\n## Result\n\nVerdict: `{verdict}`. The planted four-sum root was found and replayed: `{}`. The published-target bounded MITM probe found a four-sum: `{}`. The 16-column modular solver control recovered its known solution: `{}`.\n\n", mitm.planted_replayed, mitm.published_found, solver.recovered_exactly);
    out.push_str("## Held-out cells\n\n| Base | B | Selected source | Combiner | w | offset | relation hits | null hits | log2 enrichment | ratio lower95 | pre-add |\n| --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |\n");
    for a in arms {
        let s = &a.selected;
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {:.6} | {:.6} | {} |\n",
            a.base_design,
            a.size,
            s.source_name,
            s.combiner_name,
            s.config.width,
            s.config.offset,
            s.test.relation_hits,
            s.test.null_hits,
            s.test.log2_enrichment,
            s.test.ratio_lower95,
            s.pre_add
        ));
    }
    out.push_str(&format!("\n## Phase ledger\n\n| Phase | State | Evidence |\n| --- | --- | --- |\n| Exact curve/target validation | completed | subgroup and curve checks in native instrument |\n| Factor-base generation | completed | small-x and matched scalar-random arms |\n| Hyperparameter search | completed | training-only exhaustive frozen grid |\n| Held-out relation filter | completed | 8,192 relation and null samples per cell |\n| F4 algebraic solve | unavailable | no exact 131-bit F4 decomposition backend is dispatched |\n| F5 algebraic solve | unavailable | no exact 131-bit F5 decomposition backend is dispatched |\n| F6 geometric pair index | completed | exact signed MITM root probe |\n| Root finding | completed | planted control plus published target probe |\n| Relation solving | control completed; natural rows separate | exact modular known-answer control; `ic prime` raw runs carry natural rank |\n| Published discrete logarithm | not completed | no target logarithm inferred from a bounded null |\n\n## Flow\n\nThe local native sweep planned, attempted, and completed six filter cells, one planted root control, one published-target probe, and one modular-solver control. Local clocks are descriptive and invalid for performance claims. Exact arity-2/3 natural relation/rank runs are retained separately. Cloud L2 execution is excluded by the repository cost gate until recorded budget headroom exists.\n\n## Reproduction\n\n```sh\ncargo run --release --example eccp131_representation_filter -- research/eccp131_representation_filter_20261102/raw/local\n```\n\nProtocol SHA-256: `{}`. Source revision: `{}`.\n", value["protocol_sha256"].as_str().unwrap_or("unknown"), value["source_revision"].as_str().unwrap_or("unknown")));
    fs::write(path, out).map_err(|e| e.to_string())
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn main() -> Result<(), String> {
    let all = Instant::now();
    let output = PathBuf::from(
        env::args()
            .nth(1)
            .unwrap_or_else(|| "research/eccp131_representation_filter_20261102/raw/local".into()),
    );
    fs::create_dir_all(&output).map_err(|e| e.to_string())?;
    let (curve, g, q) = exact_curve()?;
    let digest = curve_digest(&curve, &g, &q);
    let mut arms = Vec::new();
    for &size in &SIZES {
        let base = small_x_base(&curve, size)?;
        arms.push(run_arm(&curve, &base, "small_x", size, 131001, 131002));
        let rb = random_base(&curve, &g, size, 131003);
        arms.push(run_arm(&curve, &rb, "random_base", size, 131004, 131005));
    }
    let mitm_base = small_x_base(&curve, 128)?;
    let mitm = mitm_probe(&curve, &mitm_base, &q);
    if !mitm.planted_found || !mitm.planted_replayed {
        return Err("planted MITM root control failed".into());
    }
    if arms.iter().any(|a| !a.selected.aa_identical) {
        return Err("A/A count replay failed".into());
    }
    let solver = solver_control(&curve.order);
    if !solver.all_determined || !solver.recovered_exactly {
        return Err("modular solver control failed".into());
    }
    let protocol = Path::new("research/eccp131_representation_filter_20261102/protocol.json");
    let protocol_sha256 = sha256_file(protocol).unwrap_or_else(|_| "unavailable".into());
    let source_revision =
        env::var("SOURCE_REVISION").unwrap_or_else(|_| "working-tree-development-run".into());
    let qualifying = arms.iter().any(|a| {
        a.base_design == "small_x" && a.selected.pre_add && a.selected.test.ratio_lower95 > 1.0
    }) && !arms
        .iter()
        .any(|a| a.base_design == "random_base" && a.selected.test.ratio_lower95 > 1.0);
    let value = json!({
        "schema": "eccp131.representation-filter/v1",
        "status": "complete",
        "curve": {"name": curve.name, "field_bits": curve.field.modulus().bits(), "subgroup_bits": curve.order.bits(), "cofactor": curve.cofactor, "digest_sha256": digest},
        "protocol_sha256": protocol_sha256,
        "source_revision": source_revision,
        "samples": {"training_relation_and_null_each": TRAIN, "heldout_relation_and_null_each": TEST},
        "arms": &arms,
        "mitm": &mitm,
        "solver_control": &solver,
        "verdict": if qualifying { "qualifying_filter_observed" } else { "no_qualifying_filter_in_frozen_family" },
        "elapsed_seconds": all.elapsed().as_secs_f64(),
        "limitations": [
            "The frozen family contains simple bit-slice coordinate predicates only; a null does not rule out other algebraic filters.",
            "Local wall times are development diagnostics and are not admitted performance measurements.",
            "The published-target MITM probe is bounded to B=128 and does not infer a zero logarithm from a miss.",
            "F4 and F5 are unavailable at exact 131-bit width in the current dispatched backend and are not relabelled as measured."
        ]
    });
    let result_path = output.join("result.json");
    fs::write(
        &result_path,
        serde_json::to_vec_pretty(&value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    write_svg(&output.join("filter-enrichment.svg"), &arms)?;
    write_report(&output.join("REPORT.md"), &value, &arms, &mitm, &solver)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?
    );
    Ok(())
}
