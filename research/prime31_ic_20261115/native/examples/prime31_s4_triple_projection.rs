//! EXP-R31-G: finite-rational S4 triple-arm projection on the inherited prime31 curve.
//!
//! Construct all roots of S4(x_i,x_j,x_k,u) for DISTINCT canonical base
//! indices i<j<k by TWO nested S3 quadratic root calculations in F_p.
//! This uses no EC group operation for root discovery. Existing Rust
//! Arithmetic/Base are used to validate every extracted root *afterward*.
//! The output is a target-INDEPENDENT component, NOT a complete IC oracle,
//! a cold ECDLP solve, a non-generic-complexity claim or a rho benchmark.
//! [pcc:unverified]
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, PRIME31};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const DEPTH: usize = 6;

fn add(x: u64, y: u64) -> u64 {
    let z = x + y;
    if z >= P { z - P } else { z }
}
fn sub(x: u64, y: u64) -> u64 {
    if x >= y { x - y } else { x + P - y }
}
fn mul(x: u64, y: u64) -> u64 {
    ((u128::from(x) * u128::from(y)) % u128::from(P)) as u64
}
fn pow(mut x: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e & 1 != 0 { r = mul(r, x); }
        x = mul(x, x);
        e >>= 1;
    }
    r
}
fn sqrt(z: u64) -> Result<u64, String> {
    let r = pow(z, (P + 1) / 4);
    if mul(r, r) == z { Ok(r) }
    else { Err(format!("S3 discriminant has no Fp square root: {z}")) }
}
fn coeff(x: u64, y: u64) -> (u64, u64, u64) {
    let sum = add(x, y);
    let pr = mul(x, y);
    let a = mul(sub(x, y), sub(x, y));
    let b = sub(0, mul(2, add(mul(sum, add(pr, A)), mul(2, B))));
    let c = sub(mul(sub(pr, A), sub(pr, A)), mul(4, mul(B, sum)));
    (a, b, c)
}
// Batched Montgomery inversions: every S3 root needs (2a)^(-1).
// In this fixed DISTINCT-index rational corpus a!=0 and both roots are
// rational and distinct. Fail closed rather than silently dropping cases.
fn batched_s3(jobs: &[(u64, u64)]) -> Result<Vec<[u64; 2]>, String> {
    if jobs.is_empty() { return Err("empty S3 batch".into()); }
    let mut pref = Vec::with_capacity(jobs.len() + 1);
    pref.push(1u64);
    let mut discr = Vec::with_capacity(jobs.len());
    let mut coeff_b = Vec::with_capacity(jobs.len());
    for &(x, y) in jobs {
        let (a, b, c) = coeff(x, y);
        if a == 0 { return Err(format!("diagonal S3 arm at x={x}")); }
        let root = sqrt(sub(mul(b, b), mul(4, mul(a, c))))?;
        if root == 0 { return Err("S3 double rational root".into()); }
        let denom = mul(2, a);
        pref.push(mul(*pref.last().unwrap(), denom));
        discr.push(root);
        coeff_b.push(b);
    }
    if *pref.last().unwrap() == 0 { return Err("zero batch product".into()); }
    let mut total_inverse = pow(*pref.last().unwrap(), P - 2);
    let mut inverses = vec![0u64; jobs.len()];
    for k in (0..jobs.len()).rev() {
        inverses[k] = mul(total_inverse, pref[k]);
        let (a, _, _) = coeff(jobs[k].0, jobs[k].1);
        total_inverse = mul(total_inverse, mul(2, a));
    }
    if total_inverse != 1 { return Err("batch inverse prefix failed".into()); }
    let mut ans = Vec::with_capacity(jobs.len());
    for k in 0..jobs.len() {
        let (b, root, iv) = (coeff_b[k], discr[k], inverses[k]);
        let u = mul(sub(root, b), iv);
        let v = mul(sub(sub(0, b), root), iv);
        if u == v { return Err("duplicate S3 roots".into()); }
        ans.push(if u < v { [u, v] } else { [v, u] });
    }
    Ok(ans)
}
// A separate scalar-inversion control for exact output matching.
// It is not an independent scientific verifier; replay_s4_triple_projection.py
// separately implements the field and EC formulas.
fn scalar_s3(x: u64, y: u64) -> Result<[u64; 2], String> {
    let (a, b, c) = coeff(x, y);
    if a == 0 { return Err("scalar S3 diagonal".into()); }
    let r = sqrt(sub(mul(b, b), mul(4, mul(a, c))))?;
    let iv = pow(mul(2, a), P - 2);
    let u = mul(sub(r, b), iv);
    let v = mul(sub(sub(0, b), r), iv);
    if u == v { return Err("scalar S3 merged".into()); }
    Ok(if u < v { [u, v] } else { [v, u] })
}
fn fnv_le32(roots: &BTreeSet<u64>) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &w in roots {
        for byte in (w as u32).to_le_bytes() {
            h = (h ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
        }
    }
    h
}
fn main() -> Result<(), String> {
    if std::env::args().len() != 1 {
        return Err("usage: prime31_s4_triple_projection".into());
    }
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    if PRIME31.p != P || PRIME31.a != A || PRIME31.b != B
        || PRIME31.n != 2_147_446_831 {
        return Err("inherited prime31 fixture changed".into());
    }
    let fb = Base::build(PRIME31, DEPTH, &mut ar)?;
    if fb.terminal != 933_677_148 || fb.points.len() != 31 {
        return Err("Dickson depth-six base drift".into());
    }
    let base_size = fb.points.len();
    let triples: Vec<(usize, usize, usize)> = (0..base_size)
        .flat_map(|i| (i + 1..base_size).flat_map(move |j|
            (j + 1..base_size).map(move |k| (i, j, k))))
        .collect();
    if triples.len() != 4_495 { return Err("triple count drift".into()); }
    let pair_indices: Vec<(usize, usize)> = (0..fb.points.len())
        .flat_map(|i| (i + 1..fb.points.len()).map(move |j| (i, j)))
        .collect();
    let pair_jobs: Vec<_> = pair_indices.iter()
        .map(|&(i,j)| (fb.points[i].0, fb.points[j].0)).collect();

    let algebraic_start = Instant::now();
    let pair_roots = batched_s3(&pair_jobs)?;
    let lookup: BTreeMap<(usize,usize), [u64;2]> =
        pair_indices.iter().copied().zip(pair_roots.iter().copied()).collect();
    let mut triple_jobs = Vec::with_capacity(8_990);
    for &(i,j,k) in &triples {
        let [u,v] = *lookup.get(&(i,j)).ok_or("missing pair")?;
        triple_jobs.push((u,fb.points[k].0));
        triple_jobs.push((v,fb.points[k].0));
    }
    let triple_roots = batched_s3(&triple_jobs)?;
    let algebraic_ms = algebraic_start.elapsed().as_secs_f64() * 1000.0;

    // Adversarial within-code scalar-inversion control: both passes must
    // return the same roots. Times are development diagnostics, not cold costs.
    let scalar_start = Instant::now();
    let mut scalar_checked = 0usize;
    for (job, expected) in pair_jobs.iter().zip(&pair_roots)
        .chain(triple_jobs.iter().zip(&triple_roots)) {
        if scalar_s3(job.0,job.1)? != *expected {
            return Err("batched/scalar S3 root mismatch".into());
        }
        scalar_checked += 1;
    }
    let scalar_ms = scalar_start.elapsed().as_secs_f64() * 1000.0;

    let mut all = BTreeSet::new();
    let mut negative_controls = 0usize;
    let validation_additions_before = ar.counts.group_additions;
    for (t,&(i,j,k)) in triples.iter().enumerate() {
        let roots = [
            triple_roots[2*t][0],triple_roots[2*t][1],
            triple_roots[2*t+1][0],triple_roots[2*t+1][1]
        ];
        if roots.iter().copied().collect::<BTreeSet<_>>().len() != 4 {
            return Err(format!("within-triple S4 collision at {i}/{j}/{k}"));
        }
        for &u in &roots {
            if !all.insert(u) {
                return Err(format!("cross-triple S4 root collision at {u}"));
            }
        }
        // Independent elliptic group semantics AFTER Fp-only discovery.
        let pi = Some(fb.points[i]);
        let pj = Some(fb.points[j]);
        let pk = Some(fb.points[k]);
        let negj = Some((fb.points[j].0, sub(0, fb.points[j].1)));
        let negk = Some((fb.points[k].0, sub(0, fb.points[k].1)));
        let sum = ar.add(PRIME31, pi, pj).ok_or("pair sum infinity")?;
        let diff = ar.add(PRIME31, pi, negj).ok_or("pair difference infinity")?;
        let mut group_roots = Vec::with_capacity(4);
        for pair in [sum,diff] {
            for third in [pk,negk] {
                let q = ar.add(PRIME31, Some(pair), third)
                    .ok_or("triple group sum infinity")?;
                group_roots.push(q.0);
            }
        }
        let mut sorted_roots = roots;
        sorted_roots.sort_unstable();
        group_roots.sort_unstable();
        if group_roots.as_slice() != sorted_roots.as_slice() {
            return Err(format!("S4 polynomial/group semantic mismatch {i}/{j}/{k}"));
        }
    }
    if all.len() != 17_980 || scalar_checked != 9_455
        || fnv_le32(&all) != 0xcc15_d719_e3ea_f8c2 {
        return Err("prime31 triple-abscissa exact census drift".into());
    }
    for w in 0..256 {
        if !all.contains(&w) { negative_controls += 1; }
    }
    if negative_controls != 256 {
        return Err("unexpected small-root negative control".into());
    }
    let first: Vec<u64> = all.iter().take(6).copied().collect();
    let last: Vec<u64> = all.iter().rev().take(6).copied().rev().collect();
    println!("{}", json!({
        "standing":"[pcc:unverified]",
        "curve":"ordinary cofactor-one prime-order p=2^31-1 inherited fixture",
        "method":"S4 triple-arm projection via nested S3 quadratics in Fp",
        "algebraic_root_discovery_uses_group_operations":false,
        "not_target_dependent":true,
        "not_a_complete_ECDLP_solver":true,
        "base_points":fb.points.len(),
        "distinct_index_triples":triples.len(),
        "s3_pair_calls":pair_jobs.len(),
        "s3_triple_calls":triple_jobs.len(),
        "s3_total_calls":scalar_checked,
        "algebraic_S4_raw_roots":triples.len()*4,
        "algebraic_S4_distinct_rational_roots":all.len(),
        "collision_classes":0,
        "squarefree_triple_projection_degree":all.len(),
        "batch_field_inversions":2,
        "scalar_reference_field_inversions":scalar_checked,
        "batch_algebraic_stage_ms_diagnostic":algebraic_ms,
        "scalar_reference_stage_ms_diagnostic":scalar_ms,
        "validation_group_additions_post_discovery":
            ar.counts.group_additions - validation_additions_before,
        "root_fnv1a_le32":format!("{:016x}",fnv_le32(&all)),
        "first_six_roots":first,"last_six_roots":last,
        "small_nonmember_controls":negative_controls,
        "caution":"component-only local CI diagnostic; no F4 complete-root or cold-rho ranking"
    }));
    Ok(())
}
