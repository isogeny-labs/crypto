//! Cold public-input reconstruction and complete elapsed execution boundary.
//! Original fixtures and the nine-arm collector are not rewritten.
use super::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering as AO};
use std::time::Instant;

struct ObservedAllocator;
static ALLOCS: AtomicU64 = AtomicU64::new(0);
static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);
static LIVE: AtomicU64 = AtomicU64::new(0);
static PEAK: AtomicU64 = AtomicU64::new(0);
unsafe impl GlobalAlloc for ObservedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = System.alloc(layout);
        if !p.is_null() {
            ALLOCS.fetch_add(1, AO::Relaxed);
            ALLOC_BYTES.fetch_add(layout.size() as u64, AO::Relaxed);
            let live = LIVE.fetch_add(layout.size() as u64, AO::Relaxed) + layout.size() as u64;
            PEAK.fetch_max(live, AO::Relaxed);
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size() as u64, AO::Relaxed);
        System.dealloc(p, layout)
    }
    unsafe fn realloc(&self, p: *mut u8, old: Layout, size: usize) -> *mut u8 {
        let q = System.realloc(p, old, size);
        if !q.is_null() {
            ALLOCS.fetch_add(1, AO::Relaxed);
            ALLOC_BYTES.fetch_add(size as u64, AO::Relaxed);
            let live = if size >= old.size() {
                LIVE.fetch_add((size - old.size()) as u64, AO::Relaxed) + (size - old.size()) as u64
            } else {
                LIVE.fetch_sub((old.size() - size) as u64, AO::Relaxed) - (old.size() - size) as u64
            };
            PEAK.fetch_max(live, AO::Relaxed);
        }
        q
    }
}
#[global_allocator]
static GLOBAL: ObservedAllocator = ObservedAllocator;

pub(super) fn allocation_begin() -> (u64, u64) {
    PEAK.store(LIVE.load(AO::Relaxed), AO::Relaxed);
    (ALLOCS.load(AO::Relaxed), ALLOC_BYTES.load(AO::Relaxed))
}

pub(super) fn allocation_end(start: (u64, u64)) -> Value {
    let calls = ALLOCS.load(AO::Relaxed) - start.0;
    let bytes = ALLOC_BYTES.load(AO::Relaxed) - start.1;
    let peak = PEAK.load(AO::Relaxed);
    json!({"allocation_calls":calls,"allocated_bytes":bytes,"peak_live_rust_bytes":peak,
        "scope":"Allocation calls and requested bytes during the charged interval; peak live Rust bytes includes retained decoded input. Process RSS is separate."})
}

fn ns(t: Instant) -> u64 {
    t.elapsed().as_nanos().try_into().unwrap()
}

/// Deterministic Tonelli--Shanks in the actual quartic field.
fn square_root(f: Field, a: E, non_square: E) -> Option<E> {
    if a == [0; 4] {
        return Some(a);
    }
    let order = f.p.pow(4) - 1;
    let one = f.scalar(1);
    if f.pow(a, order / 2) != one {
        return None;
    }
    let mut m = order.trailing_zeros();
    let odd = order >> m;
    let mut z = f.pow(non_square, odd);
    let mut t = f.pow(a, odd);
    let mut r = f.pow(a, odd.div_ceil(2));
    while t != one {
        let mut i = 0;
        let mut tt = t;
        while tt != one {
            tt = f.mul(tt, tt);
            i += 1;
            assert!(i < m, "bad Tonelli--Shanks invariant");
        }
        let mut b = z;
        for _ in 0..m - i - 1 {
            b = f.mul(b, b);
        }
        r = f.mul(r, b);
        z = f.mul(b, b);
        t = f.mul(t, z);
        m = i;
    }
    assert_eq!(f.mul(r, r), a);
    Some(r)
}

pub(super) fn build(raw: Value, arm: usize) -> Case {
    build_inner(raw, arm, true)
}

/// Target-independent arithmetic setup for reusable zero precomputation.
/// The public Q encoding is retained, but its validation is charged online.
pub(super) fn build_reusable(raw: Value, arm: usize) -> Case {
    build_inner(raw, arm, false)
}

fn build_inner(raw: Value, arm: usize, validate_target: bool) -> Case {
    let p = raw["p"].as_u64().unwrap();
    let seed = raw["seed"].as_u64().unwrap();
    let n = raw["n"].as_u64().unwrap();
    meter::configure(p, n);
    assert!(n > 4 * p * p && n % 2 == 1);
    assert!((3..=n.isqrt()).step_by(2).all(|d| n % d != 0));
    let c = Curve {
        f: Field::new(p),
        b: [0, 0, 0, raw["b0"].as_u64().unwrap()],
    };
    let f = c.f;
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
    assert!(c.on_curve(g));
    if validate_target {
        assert!(c.on_curve(q));
    }
    assert_eq!(c.mul(g, n), INF);
    if validate_target {
        assert_eq!(c.mul(q, n), INF);
    }
    let omega = raw["omega"].as_u64().unwrap();
    let lc = raw["lambda_cubic"].as_u64().unwrap();
    let lf = raw["lambda_frob"].as_u64().unwrap();
    assert_ne!(omega, 1);
    assert_eq!(fpow(omega, 3, p), 1);
    assert_eq!(psi(&c, g, omega), c.mul(g, lc));
    assert_eq!(frob(&c, g), c.mul(g, lf));
    let expected: Vec<Point> = raw["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    let original_len = expected.len();
    let l0 = expected.iter().take_while(|v| v.x[2] == 0).count();
    let active = matches!(arm, 7 | 8 | 9);
    let required = if active { l0 } else { original_len };
    let non_square = [0, 1, 0, 0];
    assert_ne!(f.pow(non_square, (p.pow(4) - 1) / 2), f.scalar(1));
    let mut base = Vec::new();
    let mut lift_attempts = 0;
    for line in if active {
        &[0usize][..]
    } else {
        &[0usize, 2][..]
    } {
        for scalar in 1..p {
            lift_attempts += 1;
            let mut x = [0; 4];
            x[*line] = scalar;
            let rhs = f.add(f.mul(f.mul(x, x), x), c.b);
            if let Some(y) = square_root(f, rhs, non_square) {
                let mut pt = Point { x, y, inf: false };
                let encoded = expected[base.len()];
                assert_eq!(pt.x, encoded.x);
                // Only the public sign encoding is read; the lift is computed above.
                if pt.y != encoded.y {
                    pt = c.neg(pt);
                }
                assert_eq!(pt, encoded);
                assert!(c.on_curve(pt));
                base.push(pt);
            }
        }
    }
    assert_eq!(base.len(), required);
    let anchor = if base[0] == g {
        1
    } else {
        assert_eq!(base[0], c.neg(g));
        n - 1
    };
    let by_x: BTreeMap<_, _> = base.iter().enumerate().map(|(i, pt)| (pt.x, i)).collect();
    assert_eq!(by_x.len(), base.len());
    let mut columns = vec![usize::MAX; required];
    let mut coefficients = vec![0; required];
    let mut reps = Vec::new();
    for (i, &rep) in base.iter().enumerate() {
        if columns[i] != usize::MAX {
            continue;
        }
        let col = reps.len();
        reps.push(i);
        let mut fr = rep;
        for j in 0..4 {
            let mut im = fr;
            for k in 0..3 {
                let idx = by_x[&im.x];
                let sign = matches_sign(&c, base[idx], im);
                let a = mm(fpow(lf, j, n), fpow(lc, k, n), n);
                let coef = if sign == 1 { a } else { negn(a, n) };
                if columns[idx] == usize::MAX {
                    columns[idx] = col;
                    coefficients[idx] = coef;
                } else {
                    assert_eq!((columns[idx], coefficients[idx]), (col, coef));
                }
                im = psi(&c, im, omega);
            }
            fr = frob(&c, fr);
        }
    }
    assert_eq!(reps.len() * 12, required);
    for id in 0..required {
        assert_eq!(c.mul(base[reps[columns[id]]], coefficients[id]), base[id]);
    }
    // Preserve frozen physical IDs. Unused entries are inert placeholders.
    let dummy = Factor {
        p: INF.into(),
        col: usize::MAX,
        coef: 0,
        source_id: usize::MAX,
        image: false,
    };
    let mut pool = vec![dummy; 2 * original_len + l0];
    for id in 0..required {
        pool[id] = Factor {
            p: base[id].into(),
            col: columns[id],
            coef: coefficients[id],
            source_id: id,
            image: false,
        };
    }
    let mut peels = vec![Vec::new(); ARMS.len()];
    let mut source_images = Vec::new();
    match arm {
        0 | 1 => peels[arm].extend(l0..original_len),
        2 => source_images.extend(l0..original_len),
        3 | 7 => source_images.extend(0..l0),
        4 => {
            peels[arm].extend(l0..original_len);
            source_images.extend(l0..original_len);
        }
        5 => {
            peels[arm].extend(l0..original_len);
            source_images.extend(0..original_len);
        }
        6 => {
            peels[arm].extend(l0..original_len);
            source_images.extend(0..l0);
        }
        8 => peels[arm].extend(0..l0),
        9 => {}
        _ => unreachable!(),
    }
    let delta = (n + 1 - lc) % n;
    let mut all_by_x = by_x.clone();
    let mut image_checks = 0;
    for id in source_images {
        let pt = base[id];
        let im = c.add(pt, c.neg(psi(&c, pt, omega)));
        assert!(!im.inf && c.on_curve(im));
        assert_eq!(im, c.mul(pt, delta));
        let lhs = f.mul(f.scalar(3), f.mul(f.mul(pt.x, pt.x), im.x));
        let rhs = f.neg(f.mul(
            f.scalar(omega * omega % p),
            f.add(f.mul(f.mul(pt.x, pt.x), pt.x), f.mul(f.scalar(4), c.b)),
        ));
        assert_eq!(lhs, rhs);
        let idx = original_len + id;
        assert!(all_by_x.insert(im.x, idx).is_none());
        pool[idx] = Factor {
            p: im.into(),
            col: columns[id],
            coef: mm(delta, coefficients[id], n),
            source_id: id,
            image: true,
        };
        peels[arm].push(idx);
        image_checks += 1;
    }
    if arm == 9 {
        for id in 0..l0 {
            let im = c.add(base[id], base[id]);
            assert!(!im.inf && c.on_curve(im));
            assert_eq!(im, c.mul(base[id], 2));
            let idx = 2 * original_len + id;
            assert!(all_by_x.insert(im.x, idx).is_none());
            pool[idx] = Factor {
                p: im.into(),
                col: columns[id],
                coef: mm(2, coefficients[id], n),
                source_id: id,
                image: true,
            };
            peels[arm].push(idx);
            image_checks += 1;
        }
    }
    peels[arm].sort();
    // Verify closure and signs for precisely this candidate's peeled factors.
    for &id in &peels[arm] {
        let factor = &pool[id];
        let mut fr: Point = factor.p.into();
        for j in 0..4 {
            let mut im = fr;
            for k in 0..3 {
                let other = all_by_x[&im.x];
                let s = matches_sign(&c, pool[other].p.into(), im);
                let a = mm(mm(factor.coef, fpow(lf, j, n), n), fpow(lc, k, n), n);
                assert_eq!(
                    (pool[other].col, pool[other].coef),
                    (factor.col, if s == 1 { a } else { negn(a, n) })
                );
                im = psi(&c, im, omega);
            }
            fr = frob(&c, fr);
        }
    }
    let mut zero_triples = BTreeSet::new();
    for &pt in &base[..l0] {
        for outer in [-1, 1] {
            let mut im = pt;
            let mut terms = Vec::new();
            let mut sum = INF;
            for _ in 0..3 {
                let id = by_x[&im.x];
                let s = matches_sign(&c, base[id], im) * outer;
                terms.push((id, s));
                sum = c.add(sum, signed(&c, base[id], s));
                im = psi(&c, im, omega);
            }
            assert_eq!(sum, INF);
            terms.sort();
            zero_triples.insert(terms);
        }
    }
    let geometry = json!({"lift_attempts":lift_attempts,"lifted_original_factors":required,"constructed_images":image_checks,"columns":reps.len(),"setup_work":meter::snapshot()});
    Case {
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

pub(crate) fn execute(path: &Path, sha: &str, p: u64, seed: u64, arm: &str, out: &Path) {
    let raw = load(path, sha, p, seed);
    let ai = ARMS.iter().position(|&x| x == arm).unwrap();
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let alloc0 = ALLOCS.load(AO::Relaxed);
    let bytes0 = ALLOC_BYTES.load(AO::Relaxed);
    PEAK.store(LIVE.load(AO::Relaxed), AO::Relaxed);
    let cold_clock = Instant::now();
    let case = build(raw, ai);
    let (_, h) = integer::derive();
    let mut matrix = Matrix::new(case.n, case.columns(ai));
    let setup_work = meter::snapshot();
    meter::reset();
    let setup_ns = ns(cold_clock);
    let mut phases = [0u64; 5];
    let mut traces = Vec::new();
    let mut total = 0;
    let mut systems = 0;
    let mut failures = 0;
    let mut first = None;
    let mut rows = BTreeSet::new();
    let mut tuples = BTreeSet::new();
    for (qi, item) in case.raw["query_trace"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let t = Instant::now();
        let r = case.residual(item);
        phases[0] += ns(t);
        let t = Instant::now();
        let (got, mut rec) = polynomial_query(&case, &h, ai, r);
        phases[1] += ns(t);
        let t = Instant::now();
        for terms in &got {
            tuples.insert(terms.clone());
            let row = case.row(terms, item, ai);
            if let Some(canon) = canonical_row(row.clone(), case.n) {
                rows.insert(canon);
            }
            if matrix.rank() < matrix.cols {
                matrix.insert(row);
            }
        }
        phases[2] += ns(t);
        total += got.len();
        systems += rec["systems"].as_u64().unwrap();
        failures += rec["undetermined"].as_array().unwrap().len();
        rec["query_index"] = json!(qi);
        rec["rank_after"] = json!(matrix.rank());
        rec["signed_relations"] = json!(got);
        traces.push(rec);
        if matrix.rank() == matrix.cols {
            first = Some(qi);
            break;
        }
    }
    let t = Instant::now();
    let solution = matrix.solution();
    phases[3] = ns(t);
    let t = Instant::now();
    let verified = solution.as_ref().map(|s| case.verify_solution(s, ai));
    if solution.is_some() {
        assert_eq!(verified, Some(true));
    }
    phases[4] = ns(t);
    let cold_ns = ns(cold_clock);
    let online_ns = cold_ns - setup_ns;
    let measured_sum: u64 = phases.iter().sum();
    assert!(online_ns >= measured_sum);
    phases[0] += online_ns - measured_sum;
    let online_work = meter::snapshot();
    let allocations = ALLOCS.load(AO::Relaxed) - alloc0;
    let allocated_bytes = ALLOC_BYTES.load(AO::Relaxed) - bytes0;
    let peak_live = PEAK.load(AO::Relaxed);
    let record = json!({"schema":"quartic-twist-image-cold/v1",
        "deterministic":{"p":p,"seed":seed,"arm":arm,"input_sha256":sha,"n":case.n,"status":if failures>0 {"partial"} else if first.is_none() {"censored"} else {"pass"},
        "geometry":case.geometry,"queries":traces.len(),"signed_relations":total,"unique_signed_tuples":tuples.len(),"unique_projective_augmented_rows":rows.len(),"systems":systems,"undetermined":failures,"rank":matrix.rank(),"columns":matrix.cols,"first_full_rank_query":first,"solved_logs":solution,"all_usable_factor_logs_verified":verified,"setup_work":setup_work,"online_work":online_work,"trace":traces},
        "timing":{"setup_ns":setup_ns,"online_ns":online_ns,"cold_ns":cold_ns,"point_queries_ns":phases[0],"decomposition_ns":phases[1],"matrix_ns":phases[2],"recovery_ns":phases[3],"verification_ns":phases[4]},
        "memory":{"allocation_calls":allocations,"allocated_bytes":allocated_bytes,"peak_live_rust_bytes":peak_live},
        "boundary":"public parameters to verified answer; setup rebuilt from abscissae; evidence writes excluded; capped arms report consumed cost only"});
    write(&out.join("cold.json"), &record);
}

pub(crate) fn check(first: &Path, second: &Path, candidate: &Path, out: &Path) {
    let a: Value = serde_json::from_slice(&std::fs::read(first).unwrap()).unwrap();
    let b: Value = serde_json::from_slice(&std::fs::read(second).unwrap()).unwrap();
    assert_eq!(
        a["deterministic"], b["deterministic"],
        "cold deterministic A/A mismatch"
    );
    let d = &a["deterministic"];
    let s: Value =
        serde_json::from_slice(&std::fs::read(candidate.join("summary.json")).unwrap()).unwrap();
    for field in [
        "p",
        "seed",
        "arm",
        "queries",
        "signed_relations",
        "systems",
        "rank",
        "columns",
        "first_full_rank_query",
        "solved_logs",
        "undetermined",
    ] {
        assert_eq!(d[field], s[field], "cold/collector mismatch at {field}");
    }
    for (qi, rec) in d["trace"].as_array().unwrap().iter().enumerate() {
        let r: Value = serde_json::from_slice(
            &std::fs::read(candidate.join(format!("q{qi:05}.json"))).unwrap(),
        )
        .unwrap();
        for field in [
            "signed_relations",
            "systems",
            "rank_after",
            "f4_row_reduction_fp_muls",
        ] {
            assert_eq!(
                rec[field], r[field],
                "cold/collector query {qi} field {field}"
            );
        }
    }
    write(
        out,
        &json!({"schema":"quartic-twist-image-cold-check/v1","status":"pass","p":d["p"],"seed":d["seed"],"arm":d["arm"],"queries":d["queries"],"deterministic_aa_equal":true,"all_collector_sets_rank_and_answers_equal":true}),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tonelli_shanks_quartic_and_rejection() {
        for p in [37, 73, 97] {
            let f = Field::new(p);
            let z = [0, 1, 0, 0];
            assert!(square_root(f, z, z).is_none());
            for a in [[1, 2, 3, 4], [0, 0, 1, 0], [7, 0, 0, 8], [0; 4]] {
                let sq = f.mul(a, a);
                let root = square_root(f, sq, z).unwrap();
                assert!(root == a || root == f.neg(a));
            }
        }
    }
}
