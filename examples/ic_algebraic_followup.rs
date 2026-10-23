//! Native, source-linked structural follow-up. See the frozen pursuit protocol.
//! This measures decomposition stages; it never labels a refutation a DLP solve.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;

use crypto_lib::binary_ecc::curve::{point_add, scalar_mul};
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::inherited_f4::ReducedBasis;
use crypto_lib::cryptanalysis::koblitz_groebner::{
    build_decomposition_system, f4_word_ops_thread, solve_boolean_system, sym_semaev_s4,
    FieldStructure, SolveOptions, SolverEngine, SplitRule, SymElement,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    find_irreducible_sparse, points_with_x, KoblitzCurve,
};
use crypto_lib::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
#[path = "ic_algebraic_followup/exact.rs"]
mod exact;
#[path = "ic_algebraic_followup/hky.rs"]
mod hky;
#[path = "ic_algebraic_followup/wide.rs"]
mod wide;

fn digest(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn element(x: u64, n: u32) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(x), n)
}
fn word(x: &F2mElement) -> u64 {
    x.raw_bits().first().copied().unwrap_or(0)
}
fn point_json(p: &BinaryPoint) -> Value {
    match p {
        BinaryPoint::Infinity => json!({"infinity":true}),
        BinaryPoint::Affine { x, y } => json!({"x":word(x),"y":word(y)}),
    }
}

/// Deterministic binary echelon basis. The span, rather than its ordering, matters.
fn basis(xs: impl IntoIterator<Item = u64>) -> Vec<u64> {
    let mut piv = [0u64; 64];
    for mut x in xs {
        while x != 0 {
            let i = 63 - x.leading_zeros() as usize;
            if piv[i] == 0 {
                piv[i] = x;
                break;
            }
            x ^= piv[i];
        }
    }
    piv.into_iter().filter(|&x| x != 0).collect()
}
fn span(bs: &[u64]) -> Vec<u64> {
    let mut xs = vec![0];
    for &b in bs {
        let len = xs.len();
        for i in 0..len {
            xs.push(xs[i] ^ b);
        }
    }
    xs.sort_unstable();
    xs
}
fn trace(gf: &Gf2, x: u64) -> u64 {
    let (mut y, mut t) = (x, 0);
    for _ in 0..gf.n {
        t ^= y;
        y = gf.sqr(y);
    }
    assert!(t <= 1);
    t
}
fn kernel(rows: &[u64], n: u32) -> Vec<u64> {
    let mut rows = rows.to_vec();
    let mut pivots = Vec::new();
    let mut r = 0;
    for col in 0..n as usize {
        let Some(p) = (r..rows.len()).find(|&i| rows[i] >> col & 1 != 0) else {
            continue;
        };
        rows.swap(r, p);
        for i in 0..rows.len() {
            if i != r && rows[i] >> col & 1 != 0 {
                rows[i] ^= rows[r];
            }
        }
        pivots.push(col);
        r += 1;
    }
    (0..n as usize)
        .filter(|j| !pivots.contains(j))
        .map(|j| {
            let mut v = 1u64 << j;
            for (i, &p) in pivots.iter().enumerate() {
                if rows[i] >> j & 1 != 0 {
                    v |= 1u64 << p;
                }
            }
            v
        })
        .collect()
}
fn make_basis(gf: &Gf2, ell: usize, family: &str, seed: u64) -> Vec<u64> {
    assert!(ell > 0 && ell < gf.n as usize && gf.n < 63);
    let mut rng = StdRng::seed_from_u64(seed);
    if family == "prefix" {
        return (0..ell).map(|j| 1 << j).collect();
    }
    if family == "random" {
        let mut bs = Vec::new();
        while bs.len() < ell {
            bs = basis(bs.into_iter().chain([rng.gen_range(1..1u64 << gf.n)]));
        }
        return bs;
    }
    assert!(
        family == "geometric" || family == "geomtraceu",
        "unknown family"
    );
    loop {
        let g = rng.gen_range(2..1u64 << gf.n);
        let mut powers = vec![1];
        for j in 1..ell {
            powers.push(gf.mul(powers[j - 1], g));
        }
        if basis(powers.iter().copied()).len() != ell {
            continue;
        }
        let c = if family == "geometric" {
            rng.gen_range(1..1u64 << gf.n)
        } else {
            let rows: Vec<u64> = powers
                .iter()
                .map(|&p| (0..gf.n).fold(0, |a, j| a | (trace(gf, gf.mul(p, 1 << j)) << j)))
                .collect();
            let ker = kernel(&rows, gf.n);
            assert!(!ker.is_empty());
            let mut c = 0;
            while c == 0 {
                // XOR, not integer sum: uniform nonzero vector of the kernel.
                for &v in &ker {
                    if rng.gen::<bool>() {
                        c ^= v;
                    }
                }
            }
            c
        };
        let bs: Vec<_> = powers.into_iter().map(|p| gf.mul(c, p)).collect();
        assert_eq!(basis(bs.iter().copied()).len(), ell);
        if family == "geomtraceu" {
            assert!(bs.iter().all(|&b| trace(gf, b) == 0));
        }
        return bs;
    }
}
fn products(gf: &Gf2, bs: &[u64], m: usize) -> Vec<Vec<u64>> {
    let mut out = vec![basis(bs.iter().copied())];
    for k in 1..m {
        out.push(basis(
            out[k - 1]
                .iter()
                .flat_map(|&x| bs.iter().map(move |&y| gf.mul(x, y))),
        ));
    }
    out
}

/// FPPR-style symmetric relaxation on the actual product spaces V^(k).
fn symmetric_system(
    gf: &Gf2,
    bs: &[u64],
    xr: u64,
    st: &FieldStructure,
) -> Option<(Vec<F2BoolPoly>, Vec<Vec<u64>>)> {
    let ps = products(gf, bs, 3);
    let nv = ps.iter().map(Vec::len).sum::<usize>();
    if nv > 64 {
        return None;
    }
    let mut offset = 0;
    let es: Vec<_> = ps
        .iter()
        .map(|p| {
            let xs: Vec<_> = p.iter().map(|&x| element(x, gf.n)).collect();
            let e = SymElement::from_subspace_vars(&xs, offset, gf.n, nv);
            offset += p.len();
            e
        })
        .collect();
    let (e1, e2, e3) = (&es[0], &es[1], &es[2]);
    let (e12, e22, e32) = (e1.square(st), e2.square(st), e3.square(st));
    let r1 = SymElement::constant(&element(xr, gf.n), gf.n, nv);
    let r2 = r1.square(st);
    let r3 = r2.mul(&r1, st);
    let r4 = r2.square(st);
    let out = r4
        .add(&e12.square(st))
        .add(&e32.square(st))
        .add(&e22.square(st).mul(&r4, st))
        .add(&e32.mul(e3, st).mul(&r1, st))
        .add(&e3.mul(&e22, st).mul(&r3, st))
        .add(&e3.mul(&e12, st).mul(&r1, st))
        .add(&e3.mul(&r3, st))
        .add(&e12.mul(&e32, st).mul(&r2, st))
        .add(&e32.mul(&r4, st))
        .add(&e32)
        .add(&e22.mul(&r2, st));
    assert!(out
        .coords
        .iter()
        .flat_map(|p| &p.terms)
        .all(|t| t.mask.count_ones() <= 2));
    Some((out.coords, ps))
}
fn symmetric_values(gf: &Gf2, xs: [u64; 3]) -> [u64; 3] {
    [
        xs[0] ^ xs[1] ^ xs[2],
        gf.mul(xs[0], xs[1]) ^ gf.mul(xs[0], xs[2]) ^ gf.mul(xs[1], xs[2]),
        gf.mul(gf.mul(xs[0], xs[1]), xs[2]),
    ]
}
fn decode(ps: &[Vec<u64>], assignment: u64) -> [u64; 3] {
    let mut offset = 0;
    std::array::from_fn(|i| {
        let mut x = 0;
        for (j, &b) in ps[i].iter().enumerate() {
            if assignment >> (offset + j) & 1 != 0 {
                x ^= b;
            }
        }
        offset += ps[i].len();
        x
    })
}
fn encode(ps: &[Vec<u64>], es: [u64; 3]) -> u64 {
    let mut offset = 0;
    let mut assignment = 0;
    for i in 0..3 {
        let j = span(&ps[i]).into_iter().position(|x| x == es[i]);
        assert!(j.is_some()); // membership only; recover coefficients separately.
        let mut found = None;
        for bits in 0..1u64 << ps[i].len() {
            let x = ps[i]
                .iter()
                .enumerate()
                .filter(|(j, _)| bits >> j & 1 != 0)
                .fold(0, |a, (_, b)| a ^ b);
            if x == es[i] {
                found = Some(bits);
                break;
            }
        }
        assignment |= found.expect("product-space membership") << offset;
        offset += ps[i].len();
    }
    assignment
}
/// Exact synthetic division, including repeated roots. No distinct-root shortcut.
fn split(gf: &Gf2, domain: &[u64], es: [u64; 3], tests: &mut u64) -> Option<[u64; 3]> {
    let mut coeff = vec![1, es[0], es[1], es[2]];
    let mut roots = [0; 3];
    for dst in &mut roots {
        let root = domain.iter().copied().find(|&x| {
            *tests += 1;
            coeff.iter().fold(0, |a, &c| gf.mul(a, x) ^ c) == 0
        })?;
        *dst = root;
        let mut q = vec![coeff[0]];
        for i in 1..coeff.len() - 1 {
            q.push(coeff[i] ^ gf.mul(root, q[i - 1]));
        }
        assert_eq!(coeff.last().unwrap() ^ gf.mul(root, *q.last().unwrap()), 0);
        coeff = q;
    }
    roots.sort_unstable();
    assert_eq!(symmetric_values(gf, roots), es);
    Some(roots)
}
fn actual_decomposition(
    kc: &KoblitzCurve,
    target: &BinaryPoint,
    xs: [u64; 3],
    checks: &mut u64,
) -> bool {
    let points: Vec<_> = xs
        .into_iter()
        .map(|x| points_with_x(&kc.curve, &element(x, kc.n)))
        .collect();
    for p in &points[0] {
        for q in &points[1] {
            for r in &points[2] {
                *checks += 1;
                if point_add(&kc.curve, &point_add(&kc.curve, p, q), r) == *target {
                    return true;
                }
            }
        }
    }
    false
}
fn columns(n: usize, degree: u32) -> u128 {
    let (mut total, mut binom) = (1u128, 1u128);
    for k in 1..=degree.min(n as u32) as usize {
        binom = binom * (n - k + 1) as u128 / k as u128;
        total += binom;
    }
    total
}
fn occurring(eqs: &[F2BoolPoly]) -> usize {
    eqs.iter()
        .flat_map(|p| &p.terms)
        .fold(0u64, |a, t| a | t.mask)
        .count_ones() as usize
}
fn specialise(eqs: &[F2BoolPoly], vars: &[usize], assignment: u64) -> Vec<F2BoolPoly> {
    eqs.iter()
        .map(|p| {
            let mut terms = Vec::new();
            'term: for t in &p.terms {
                let mut mask = t.mask;
                for (j, &v) in vars.iter().enumerate() {
                    if mask >> v & 1 != 0 {
                        if assignment >> j & 1 == 0 {
                            continue 'term;
                        }
                        mask &= !(1 << v);
                    }
                }
                terms.push(F2BoolMono { mask });
            }
            F2BoolPoly::from_monos(terms, p.n_vars)
        })
        .filter(|p| !p.terms.is_empty())
        .collect()
}
fn emit(v: &Value) {
    println!("{v}");
    std::io::stdout().flush().expect("flush record");
}

fn profile(args: &BTreeMap<String, String>) {
    let get = |k: &str| args.get(k).unwrap_or_else(|| panic!("missing {k}"));
    let n: u32 = get("--n").parse().unwrap();
    let ell: usize = get("--ell").parse().unwrap();
    let rep: u64 = get("--rep").parse().unwrap();
    let seed: u64 = get("--seed").parse().unwrap();
    let family = get("--family");
    let targets: u64 = get("--targets").parse().unwrap();
    let max_cols: u128 = get("--max-cols").parse().unwrap();
    let degree: u32 = get("--degree").parse().unwrap();
    let kc = KoblitzCurve::new(0, n).expect("supported Koblitz subgroup");
    let gf = Gf2::new(&kc.curve.irreducible);
    let bs = make_basis(
        &gf,
        ell,
        family,
        seed ^ ((n as u64) << 32) ^ ((ell as u64) << 16) ^ rep,
    );
    let base: Vec<_> = bs.iter().map(|&x| element(x, n)).collect();
    let domain = span(&bs);
    let st = FieldStructure::new(n, &kc.curve.irreducible);
    let mut rng = StdRng::seed_from_u64(seed ^ n as u64);
    for t in 0..targets {
        let scalar = rng.gen_range(1..kc.subgroup_order.to_u64().expect("word order"));
        let target = scalar_mul(&kc.curve, &kc.curve.generator, &BigUint::from(scalar));
        let xr = match &target {
            BinaryPoint::Affine { x, .. } => word(x),
            _ => panic!("nonzero subgroup target"),
        };
        let ps = products(&gf, &bs, 3);
        for formulation in ["chain", "eliminated-s4", "symmetric-e"] {
            let (eqs, nv) = match formulation {
                "chain" => {
                    let s = build_decomposition_system(
                        &base,
                        &element(xr, n),
                        &F2mElement::one(n),
                        3,
                        &st,
                    )
                    .unwrap();
                    (s.equations, s.n_vars)
                }
                "eliminated-s4" => {
                    let nv = 3 * ell;
                    let xs: Vec<_> = (0..3)
                        .map(|i| SymElement::from_subspace_vars(&base, i * ell, n, nv))
                        .collect();
                    (
                        sym_semaev_s4(&xs[0], &xs[1], &xs[2], &element(xr, n), &st),
                        nv,
                    )
                }
                _ => {
                    let (s, p) = symmetric_system(&gf, &bs, xr, &st).expect("symmetric layout");
                    (s, p.iter().map(Vec::len).sum())
                }
            };
            let mut row = json!({"schema":"ic-algebraic-profile/v1","field_degree":n,"field_modulus":gf.irr,
                "curve_a":0,"curve_b":1,"group_order":kc.group_order.to_string(),
                "subgroup_order":kc.subgroup_order.to_string(),"cofactor":kc.cofactor.to_string(),
                "generator":point_json(&kc.curve.generator),"target":point_json(&target),"fixture_scalar":scalar,
                "family":family,"base_rep":rep,"basis":bs,"ell":ell,"seed":seed,"target_index":t,
                "product_dimensions":ps.iter().map(Vec::len).collect::<Vec<_>>(),
                "formulation":formulation,"variables":nv,"equations":eqs.len(),
                "equations_sha256":digest(&serde_json::to_vec(&eqs).unwrap()),"degree":degree,
                "candidate_id":null,"complete_dlp_cost":null,"rho_ratio":null,
                "isolation":"L0","status":"unmeasured"});
            let input_degree = eqs
                .iter()
                .flat_map(|p| &p.terms)
                .map(|m| m.mask.count_ones())
                .max()
                .unwrap_or(0);
            row["input_max_degree"] = json!(input_degree);
            row["extraction_effective_degree"] = json!(degree.max(input_degree).max(2));
            let count = columns(occurring(&eqs), degree);
            row["estimated_columns"] = json!(count.to_string());
            if count > max_cols {
                row["status"] = json!("column_cap");
                emit(&row);
                continue;
            }
            let start = std::time::Instant::now();
            let Some((mut closed, mut cost)) =
                ReducedBasis::from_system_closed_full(&eqs, nv, degree, 1_000_000)
            else {
                row["status"] = json!("engine_cap");
                emit(&row);
                continue;
            };
            let refuted = closed.is_refuted();
            row["closure_refuted"] = json!(refuted);
            row["closure_saturated"] = json!(refuted || closed.pending_falls() == 0);
            row["columns"] = json!(closed.columns());
            row["rank"] = json!(closed.rank());
            row["closure_products"] = json!(cost.closure_products);
            row["closure_word_ops"] = json!(cost.word_ops());
            row["status"] = json!(if refuted {
                "refuted"
            } else if closed.pending_falls() == 0 {
                "saturated_not_refuted"
            } else {
                "not_saturated"
            });
            let mut accepted = BTreeSet::new();
            let (mut split_tests, mut point_checks, mut rejected) = (0, 0, 0);
            if !refuted && nv <= 21 {
                let mut augmented = eqs.clone();
                augmented.extend(closed.decisive_rows(&mut cost));
                let opts = SolveOptions {
                    engine: SolverEngine::InheritedF4 { max_degree: degree },
                    max_solutions: 4096,
                    node_budget: 20_000,
                    split_rule: SplitRule::HighestFree,
                };
                let before = f4_word_ops_thread();
                let (roots, stats) = solve_boolean_system(&augmented, nv, &opts);
                row["extraction_word_ops"] = json!(f4_word_ops_thread().wrapping_sub(before));
                row["solver_stats"] = serde_json::to_value(&stats).unwrap();
                row["algebraic_roots"] = json!(roots.len());
                row["extraction_complete"] = json!(!stats.exhausted && roots.len() < 4096);
                for root in roots {
                    assert!(eqs.iter().all(|p| p.eval(root) == 0));
                    let xs = if formulation == "symmetric-e" {
                        split(&gf, &domain, decode(&ps, root), &mut split_tests)
                    } else {
                        let mut xs = std::array::from_fn(|i| {
                            bs.iter()
                                .enumerate()
                                .filter(|(j, _)| root >> (i * ell + j) & 1 != 0)
                                .fold(0, |a, (_, b)| a ^ b)
                        });
                        xs.sort_unstable();
                        Some(xs)
                    };
                    match xs {
                        Some(xs) if actual_decomposition(&kc, &target, xs, &mut point_checks) => {
                            accepted.insert(xs);
                        }
                        _ => {
                            rejected += 1;
                        }
                    }
                }
            } else {
                row["extraction_complete"] = json!(refuted);
                row["algebraic_roots"] = if refuted { json!(0) } else { Value::Null };
            }
            row["closure_and_readback_word_ops"] = json!(cost.word_ops());
            row["accepted_x_multisets"] = json!(accepted);
            row["rejected_roots"] = json!(rejected);
            row["split_tests"] = json!(split_tests);
            row["point_checks"] = json!(point_checks);
            row["diagnostic_seconds"] = json!(start.elapsed().as_secs_f64());
            emit(&row);
        }
    }
}
fn guess(args: &BTreeMap<String, String>) {
    let get = |k: &str| args.get(k).unwrap_or_else(|| panic!("missing {k}"));
    let source = std::fs::read(get("--input")).expect("input");
    let draw: u64 = get("--draw").parse().unwrap();
    let degree: u32 = get("--degree").parse().unwrap();
    let cap: u128 = get("--max-cols").parse().unwrap();
    let first: u64 = get("--first-branch").parse().unwrap();
    let end: u64 = get("--end-branch").parse().unwrap();
    assert!(first < end);
    let input: Value = std::str::from_utf8(&source)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .find(|v| v["draw"] == draw)
        .expect("draw");
    assert_eq!(
        input["solutions"].as_u64(),
        Some(0),
        "UNSAT input required; SAT controls use selftest"
    );
    let n = input["n"].as_u64().unwrap() as u32;
    let m = input["m"].as_u64().unwrap_or(3) as usize;
    let irr = find_irreducible_sparse(n).unwrap();
    let st = FieldStructure::new(n, &irr);
    let bs: Vec<_> = input["v_basis"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| element(v.as_u64().unwrap(), n))
        .collect();
    let sys = build_decomposition_system(
        &bs,
        &element(input["x_r"].as_u64().unwrap(), n),
        &F2mElement::one(n),
        m,
        &st,
    );
    let Some(sys) = sys else {
        emit(
            &json!({"schema":"ic-algebraic-guess/v1","input_sha256":digest(&source),
            "draw":draw,"m":m,"n":n,"ell":bs.len(),"variables":input["n_vars"],
            "guess_spec":get("--vars"),"first_branch":first,"end_branch":end,
            "complete_dlp_cost":null,"rho_ratio":null,"status":"unsupported_over_64_variables"}),
        );
        return;
    };
    let vars: Vec<usize> = if let Some((ordering, k)) = get("--vars").split_once(':') {
        let k: usize = k.parse().unwrap();
        let mut frequency = vec![0usize; sys.n_vars];
        for p in &sys.equations {
            for term in &p.terms {
                for (v, f) in frequency.iter_mut().enumerate() {
                    if term.mask >> v & 1 != 0 {
                        *f += 1;
                    }
                }
            }
        }
        let mut candidates: Vec<_> = (0..sys.n_vars).filter(|&v| frequency[v] > 0).collect();
        match ordering {
            "lowest" => {}
            "frequent" => candidates.sort_by_key(|&v| (std::cmp::Reverse(frequency[v]), v)),
            _ => panic!("ordering must be lowest or frequent"),
        }
        assert!(k <= candidates.len());
        candidates.truncate(k);
        candidates
    } else if get("--vars") == "none" {
        vec![]
    } else {
        get("--vars")
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect()
    };
    assert!(vars.len() <= 16 && vars.iter().copied().collect::<BTreeSet<_>>().len() == vars.len());
    assert!(end <= 1u64 << vars.len());
    assert!(vars.iter().all(|&v| v < sys.n_vars));
    assert_eq!(sys.n_vars as u64, input["n_vars"].as_u64().unwrap());
    for branch in first..end {
        let eqs = specialise(&sys.equations, &vars, branch);
        let cols = columns(occurring(&eqs), degree);
        let engine = args.get("--engine").map(String::as_str).unwrap_or("dense");
        assert!(engine == "dense" || engine == "sparse");
        let mut row = json!({"schema":"ic-algebraic-guess/v1","input_sha256":digest(&source),"engine":engine,
            "draw":draw,"m":m,"n":n,"ell":bs.len(),"variables":sys.n_vars,"guessed_variables":vars,
            "branch":branch,"degree":degree,"estimated_columns":cols.to_string(),"column_cap":cap.to_string(),
            "complete_dlp_cost":null,"rho_ratio":null,"status":"column_cap"});
        if engine == "sparse" {
            let limit: usize = get("--max-entries").parse().unwrap();
            let start = std::time::Instant::now();
            let result = crypto_lib::cryptanalysis::koblitz_bench::sparse_closure_refutes(
                &eqs, sys.n_vars, degree, limit,
            );
            row["max_entries"] = json!(limit);
            if let Some(r) = result {
                row["status"] = json!(if r.refuted {
                    "refuted"
                } else if r.saturated {
                    "saturated_not_refuted"
                } else {
                    "not_saturated"
                });
                row["sparse_rank"] = json!(r.rank);
                row["sparse_products"] = json!(r.products);
                row["sparse_max_weight"] = json!(r.max_weight);
            } else {
                row["status"] = json!("sparse_entry_cap");
            }
            row["diagnostic_seconds"] = json!(start.elapsed().as_secs_f64());
        } else if cols <= cap {
            let start = std::time::Instant::now();
            if let Some((closed, cost)) =
                ReducedBasis::from_system_closed_full(&eqs, sys.n_vars, degree, 1_000_000)
            {
                row["status"] = json!(if closed.is_refuted() {
                    "refuted"
                } else if closed.pending_falls() == 0 {
                    "saturated_not_refuted"
                } else {
                    "not_saturated"
                });
                row["columns"] = json!(closed.columns());
                row["rank"] = json!(closed.rank());
                row["closure_word_ops"] = json!(cost.word_ops());
                row["closure_products"] = json!(cost.closure_products);
            } else {
                row["status"] = json!("engine_cap");
            }
            row["diagnostic_seconds"] = json!(start.elapsed().as_secs_f64());
        }
        emit(&row);
    }
}
fn selftest() {
    let (mut assignments, mut symmetric_roots, mut lifted_cases) = (0u64, 0u64, 0u64);
    for n in [5, 7] {
        let irr = find_irreducible_sparse(n).unwrap();
        let gf = Gf2::new(&irr);
        let st = FieldStructure::new(n, &irr);
        let kc = KoblitzCurve::new(0, n).unwrap();
        for family in ["prefix", "geometric", "geomtraceu", "random"] {
            for ell in [1, 2] {
                let bs = make_basis(&gf, ell, family, 20261023);
                let domain = span(&bs);
                let base: Vec<_> = bs.iter().map(|&x| element(x, n)).collect();
                for xr in [0, 1, 3, 7] {
                    let (eqs, ps) = symmetric_system(&gf, &bs, xr, &st).unwrap();
                    let nv: usize = ps.iter().map(Vec::len).sum();
                    let direct: Vec<_> = (0..3)
                        .map(|i| SymElement::from_subspace_vars(&base, i * ell, n, 3 * ell))
                        .collect();
                    let direct =
                        sym_semaev_s4(&direct[0], &direct[1], &direct[2], &element(xr, n), &st);
                    let mut direct_sets = BTreeSet::new();
                    for assignment in 0..1u64 << (3 * ell) {
                        assignments += 1;
                        let xs = std::array::from_fn(|i| {
                            bs.iter()
                                .enumerate()
                                .filter(|(j, _)| assignment >> (ell * i + j) & 1 != 0)
                                .fold(0, |a, (_, b)| a ^ b)
                        });
                        let es = symmetric_values(&gf, xs);
                        let ea = encode(&ps, es);
                        assert_eq!(
                            eqs.iter().all(|p| p.eval(ea) == 0),
                            direct.iter().all(|p| p.eval(assignment) == 0)
                        );
                        let mut tests = 0;
                        let got = split(&gf, &domain, es, &mut tests).unwrap();
                        let mut expected = xs;
                        expected.sort_unstable();
                        assert_eq!(got, expected);
                        if direct.iter().all(|p| p.eval(assignment) == 0) {
                            direct_sets.insert(expected);
                        }
                        // Independent group law checks the signed lift, including
                        // torsion and repeated x values. S4 is only a relaxation.
                        for target in points_with_x(&kc.curve, &element(xr, n)) {
                            let mut checks = 0;
                            if actual_decomposition(&kc, &target, xs, &mut checks) {
                                assert!(direct.iter().all(|p| p.eval(assignment) == 0));
                            }
                            lifted_cases += 1;
                        }
                    }
                    let mut split_sets = BTreeSet::new();
                    let mut has_root = false;
                    for a in 0..1u64 << nv {
                        if eqs.iter().all(|p| p.eval(a) == 0) {
                            symmetric_roots += 1;
                            has_root = true;
                            let mut tests = 0;
                            if let Some(xs) = split(&gf, &domain, decode(&ps, a), &mut tests) {
                                split_sets.insert(xs);
                            }
                        }
                    }
                    assert_eq!(
                        split_sets, direct_sets,
                        "complete split/direct root correspondence"
                    );
                    // The closed ideal must never refute any known satisfying input.
                    let (closed, _) =
                        ReducedBasis::from_system_closed_full(&eqs, nv, 3, 1_000_000).unwrap();
                    assert!(!has_root || !closed.is_refuted());
                }
            }
        }
    }
    let p = F2BoolPoly::from_monos(vec![F2BoolMono { mask: 3 }, F2BoolMono { mask: 1 }], 2);
    assert!(specialise(&[p], &[1], 1).is_empty());
    assert_eq!(columns(43, 4), 136698);
    emit(
        &json!({"schema":"ic-algebraic-selftest/v1","status":"passed","fields":[5,7],"families":4,
        "ell":[1,2],"direct_assignments_checked":assignments,"symmetric_roots_checked":symmetric_roots,
        "signed_lift_cases_checked":lifted_cases,"complete_small_root_correspondence":true,
        "symmetric_identity":true,"repeated_root_split":true,"sat_soundness":true}),
    );
}
fn fixtures(args: &BTreeMap<String, String>) {
    let get = |k: &str| args.get(k).unwrap_or_else(|| panic!("missing {k}"));
    let n: u32 = get("--n").parse().unwrap();
    let ell: usize = get("--ell").parse().unwrap();
    let m: usize = get("--m").parse().unwrap();
    let seed: u64 = get("--seed").parse().unwrap();
    let draws: u64 = get("--draws").parse().unwrap();
    let family = get("--family");
    let irr = find_irreducible_sparse(n).unwrap();
    let gf = Gf2::new(&irr);
    let mut rng =
        StdRng::seed_from_u64(seed ^ ((m as u64) << 48) ^ ((n as u64) << 24) ^ ell as u64);
    for draw in 0..draws {
        let bs = make_basis(&gf, ell, family, rng.gen());
        let xr = rng.gen_range(0..1u64 << n);
        let start = std::time::Instant::now();
        let solutions = exact::count(&gf, &bs, xr, m);
        emit(
            &json!({"schema":"ic-algebraic-fixture/v1","m":m,"n":n,"ell":ell,"family":family,
            "seed":seed,"draw":draw,"field_modulus":gf.irr,"n_vars":m*ell+(m-2)*n as usize,
            "v_basis":bs,"x_r":xr,"solutions":solutions,"outcome":{"kind":"unmeasured"},
            "oracle":"quadratic-chain-mitm-v1","oracle_diagnostic_seconds":start.elapsed().as_secs_f64()}),
        );
    }
}

fn verify_fixtures(args: &BTreeMap<String, String>) {
    let path = args.get("--input").expect("--input");
    let source = std::fs::read(path).unwrap();
    for line in std::str::from_utf8(&source).unwrap().lines() {
        let input: Value = serde_json::from_str(line).unwrap();
        let n = input["n"].as_u64().unwrap() as u32;
        let m = input["m"].as_u64().unwrap_or(3) as usize;
        let bs: Vec<_> = input["v_basis"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect();
        let xr = input["x_r"].as_u64().unwrap();
        let irr = find_irreducible_sparse(n).unwrap();
        let gf = Gf2::new(&irr);
        let count = exact::count(&gf, &bs, xr, m);
        assert_eq!(
            count,
            input["solutions"].as_u64().unwrap(),
            "fixture exact count mismatch"
        );
        let witness = exact::witness(&gf, &bs, xr, m);
        assert_eq!(witness.is_some(), count > 0);
        let mut row = json!({"schema":"ic-algebraic-fixture-replay/v1","input_sha256":digest(&source),
            "draw":input["draw"],"m":m,"n":n,"ell":bs.len(),"solutions":count,"count_matches":true,
            "witness_checked":false,"status":"verified"});
        if let Some((xs, us)) = witness {
            let base: Vec<_> = bs.iter().map(|&x| element(x, n)).collect();
            let st = FieldStructure::new(n, &irr);
            let sys = build_decomposition_system(&base, &element(xr, n), &element(1, n), m, &st);
            row["witness_x"] = json!(xs);
            row["witness_intermediates"] = json!(us);
            let mut assignment = 0u128;
            for (i, &x) in xs.iter().enumerate() {
                let bits = (0..1u64 << bs.len())
                    .find(|&bits| {
                        bs.iter()
                            .enumerate()
                            .filter(|(j, _)| bits >> j & 1 != 0)
                            .fold(0, |a, (_, b)| a ^ b)
                            == x
                    })
                    .unwrap();
                assignment |= (bits as u128) << (i * bs.len());
            }
            for (i, &u) in us.iter().enumerate() {
                assignment |= (u as u128) << (m * bs.len() + i * n as usize);
            }
            let (wide_eqs, _) = wide::chain(&gf, &bs, xr, m);
            assert!(wide_eqs
                .iter()
                .all(|p| p.iter().filter(|&&m| assignment & m == m).count() % 2 == 0));
            row["witness_assignment_hex"] = json!(format!("{assignment:x}"));
            row["witness_checked"] = json!(true);
            if let Some(sys) = sys {
                assert!(sys
                    .equations
                    .iter()
                    .all(|p| p.eval(assignment.try_into().unwrap()) == 0));
                // A SAT control at the affordable degree must not refute.
                if columns(sys.n_vars, 3) <= 30000 {
                    let (closed, _) = ReducedBasis::from_system_closed_full(
                        &sys.equations,
                        sys.n_vars,
                        3,
                        1_000_000,
                    )
                    .unwrap();
                    assert!(!closed.is_refuted());
                    row["sat_degree3_not_refuted"] = json!(true);
                } else {
                    row["sat_degree3_status"] = json!("column_cap_30000");
                }
            } else {
                row["boolean_layout_status"] = json!("unsupported_over_64_variables");
            }
        }
        emit(&row);
    }
}

fn hky_run(args: &BTreeMap<String, String>) {
    let get = |k: &str| args.get(k).unwrap_or_else(|| panic!("missing {k}"));
    let source = std::fs::read(get("--input")).unwrap();
    let draw: u64 = get("--draw").parse().unwrap();
    let input: Value = std::str::from_utf8(&source)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .find(|v| v["draw"] == draw)
        .unwrap();
    assert_eq!(input["solutions"], 0);
    let n = input["n"].as_u64().unwrap() as u32;
    let m = input["m"].as_u64().unwrap_or(3) as usize;
    let bs: Vec<_> = input["v_basis"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap())
        .collect();
    let irr = find_irreducible_sparse(n).unwrap();
    let (eqs, nv) = wide::chain(&Gf2::new(&irr), &bs, input["x_r"].as_u64().unwrap(), m);
    let mut row = json!({"schema":"ic-algebraic-hky/v1","input_sha256":digest(&source),"draw":draw,"m":m,"n":n,
        "ell":bs.len(),"complete_dlp_cost":null,"rho_ratio":null});
    row["variables"] = json!(nv);
    let engine = args.get("--engine").map(String::as_str).unwrap_or("dense");
    assert!(engine == "dense" || engine == "sparse");
    row["engine"] = json!(engine);
    let cmax: u32 = get("--degree").parse().unwrap();
    let cap: u128 = get("--max-cols").parse().unwrap();
    for degree in 2..=cmax {
        let cols = columns(nv, degree);
        row["degree"] = json!(degree);
        row["columns"] = json!(cols.to_string());
        if cols > cap && engine == "dense" {
            row["status"] = json!("column_cap");
            emit(&row);
            return;
        }
        row["status"] = json!("started");
        emit(&row);
        let start = std::time::Instant::now();
        if engine == "sparse" {
            let limit: usize = get("--max-entries").parse().unwrap();
            let result = wide::closure(&eqs, nv, degree, limit);
            row["status"] = json!(result.status);
            row["sparse"] = serde_json::to_value(&result).unwrap();
            row["diagnostic_seconds"] = json!(start.elapsed().as_secs_f64());
            emit(&row);
            if result.status != "saturated_not_refuted" {
                return;
            }
            row.as_object_mut().unwrap().remove("sparse");
            row.as_object_mut().unwrap().remove("diagnostic_seconds");
            continue;
        }
        let (plain, closed, rounds) = hky::closure_par(&eqs, nv as u32, degree, cols as usize);
        row["status"] = json!(if closed == cols as usize {
            "refuted"
        } else {
            "saturated_not_refuted"
        });
        row["plain_dimension"] = json!(plain);
        row["closure_dimension"] = json!(closed);
        row["rounds"] = json!(rounds);
        row["threads"] = json!(rayon::current_num_threads());
        row["diagnostic_seconds"] = json!(start.elapsed().as_secs_f64());
        emit(&row);
        if closed == cols as usize {
            return;
        }
        for key in [
            "plain_dimension",
            "closure_dimension",
            "rounds",
            "threads",
            "diagnostic_seconds",
        ] {
            row.as_object_mut().unwrap().remove(key);
        }
    }
}

fn audit(args: &BTreeMap<String, String>) {
    let directory = std::path::Path::new(args.get("--run").expect("--run"));
    let mut cells: Vec<_> = std::fs::read_dir(directory)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();
    cells.sort();
    let mut total = 0u64;
    let mut complete = 0u64;
    let mut failures = 0u64;
    for cell in cells {
        let path = cell.join("rows.jsonl");
        if !path.exists() {
            continue;
        }
        let mut oracle_cache = BTreeMap::new();
        for line in std::fs::read_to_string(&path).unwrap().lines() {
            let row: Value = serde_json::from_str(line).unwrap();
            if row["schema"] != "ic-algebraic-profile/v1" {
                continue;
            }
            let n = row["field_degree"].as_u64().unwrap() as u32;
            let target_id = row["target_index"].as_u64().unwrap();
            let kc = KoblitzCurve::new(0, n).unwrap();
            let target = BinaryPoint::Affine {
                x: element(row["target"]["x"].as_u64().unwrap(), n),
                y: element(row["target"]["y"].as_u64().unwrap(), n),
            };
            assert_eq!(point_json(&kc.curve.generator), row["generator"]);
            let bs: Vec<_> = row["basis"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap())
                .collect();
            let expected = oracle_cache.entry(target_id).or_insert_with(|| {
                let domain = span(&bs);
                let lifts: Vec<_> = domain
                    .iter()
                    .map(|&x| points_with_x(&kc.curve, &element(x, n)))
                    .collect();
                let mut found = BTreeSet::new();
                // Independent loop shape: enumerate unordered x multisets,
                // cache lifts, and sum the last two points first.
                for i in 0..domain.len() {
                    for j in i..domain.len() {
                        for k in j..domain.len() {
                            let mut yes = false;
                            for q in &lifts[j] {
                                for r in &lifts[k] {
                                    let qr = point_add(&kc.curve, q, r);
                                    for p in &lifts[i] {
                                        if point_add(&kc.curve, p, &qr) == target {
                                            yes = true;
                                        }
                                    }
                                }
                            }
                            if yes {
                                found.insert([domain[i], domain[j], domain[k]]);
                            }
                        }
                    }
                }
                found
            });
            let measured: BTreeSet<[u64; 3]> = row
                .get("accepted_x_multisets")
                .map(|x| serde_json::from_value(x.clone()).unwrap())
                .unwrap_or_default();
            let extracted = row["extraction_complete"] == true;
            let subset = measured.is_subset(expected);
            let matches = extracted && measured == *expected;
            let mandatory = row["formulation"] != "chain";
            let valid = subset && (!extracted || matches || !mandatory);
            total += 1;
            complete += u64::from(extracted);
            failures += u64::from(!valid);
            emit(
                &json!({"schema":"ic-algebraic-query-audit/v1","cell":cell.file_name().unwrap().to_str().unwrap(),
                "n":n,"ell":row["ell"],"family":row["family"],"rep":row["base_rep"],"target_index":target_id,
                "formulation":row["formulation"],"measured_status":row["status"],"exact_truth":expected,
                "measured_accepted":measured,"extraction_complete":extracted,"subset_valid":subset,
                "complete_matches":if extracted{json!(matches)}else{Value::Null},
                "status":if valid{"verified"}else{"mismatch"},
                "chain_degenerate_mismatch":extracted && !matches && !mandatory}),
            );
        }
    }
    emit(
        &json!({"schema":"ic-algebraic-query-audit-summary/v1","rows":total,
        "complete_extractions":complete,"failures":failures,"status":if failures==0{"passed"}else{"failed"}}),
    );
    assert_eq!(failures, 0, "independent exact-query audit failed");
}

fn main() {
    let mut it = std::env::args().skip(1);
    let mode = it.next().expect("mode: selftest/profile/guess");
    let mut args = BTreeMap::new();
    while let Some(k) = it.next() {
        assert!(k.starts_with("--"));
        let v = it.next().expect("flag value");
        assert!(args.insert(k, v).is_none());
    }
    match mode.as_str() {
        "selftest" => selftest(),
        "profile" => profile(&args),
        "guess" => guess(&args),
        "fixtures" => fixtures(&args),
        "verify-fixtures" => verify_fixtures(&args),
        "audit" => audit(&args),
        "hky" => hky_run(&args),
        _ => panic!("unknown mode"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_correspondence_and_soundness() {
        selftest();
    }
    #[test]
    fn trace_kernel_is_exact() {
        let gf = Gf2::new(&find_irreducible_sparse(7).unwrap());
        let bs = make_basis(&gf, 3, "geomtraceu", 77);
        assert!(span(&bs).into_iter().all(|x| trace(&gf, x) == 0));
    }
    #[test]
    fn partial_split_rejected() {
        let gf = Gf2::new(&find_irreducible_sparse(5).unwrap());
        let mut tests = 0;
        assert_eq!(split(&gf, &[0], [1, 1, 1], &mut tests), None);
    }
    #[test]
    fn parallel_hky_agrees_with_dense_fixed_point() {
        let n = 5;
        let irr = find_irreducible_sparse(n).unwrap();
        let st = FieldStructure::new(n, &irr);
        let bs = vec![element(1, n), element(2, n)];
        for m in [3, 4] {
            for xr in 0..8 {
                let sys = build_decomposition_system(&bs, &element(xr, n), &element(1, n), m, &st)
                    .unwrap();
                let eqs: Vec<Vec<u128>> = sys
                    .equations
                    .iter()
                    .map(|p| p.terms.iter().map(|m| m.mask as u128).collect())
                    .collect();
                for d in [2, 3] {
                    let total = columns(sys.n_vars, d) as usize;
                    let (_, dim, _) = hky::closure_par(&eqs, sys.n_vars as u32, d, total);
                    let (closed, _) = ReducedBasis::from_system_closed_full(
                        &sys.equations,
                        sys.n_vars,
                        d,
                        1_000_000,
                    )
                    .unwrap();
                    assert_eq!(dim == total, closed.is_refuted());
                    if !closed.is_refuted() {
                        assert_eq!(dim, closed.rank());
                    }
                }
            }
        }
    }
}
