//! Treewidth of the field-level (Weil-descended) binary Semaev S3 point
//! decomposition system, next to its measured refutation degree.
//!
//! The system is the one `cryptanalysis::pc_degree_harness` refutes: the n
//! coordinate equations of `S3(X1, X2, x(R)) = 0` over `F_{2^n}` with `X1, X2`
//! in the subspace `V = span{z^0..z^{n'-1}}`, `2n'` Boolean variables, each
//! equation quadratic. It is measured in two primal-graph encodings:
//!
//! * `bb`: one factor per equation over its support (D.1's black-box reading);
//! * `split`: every quadratic monomial an AND gadget `(x_i, x_j, a_ij)` shared
//!   across equations, every equation an XOR chain of arity-3 gadgets (the L09
//!   subdivision rule, cap 3, monomial sharing).
//!
//! Bounds come from `hashed_ecdl_tensor::treewidth::analyse` (safe reductions,
//! MMD+ lower bounds, min-degree / min-fill orders, widths recomputed exactly
//! from the executed order). The refutation degree `D*` and the Macaulay
//! sizes come from `measure_refutation_degree` on the same instance. One JSON
//! row per `n`; the layout and reading live in the study directory.

use crypto::cryptanalysis::ffd_harness::{
    choose_irreducible, random_nonzero_f2m, weil_descend_s3, F2BoolPoly,
};
use crypto::cryptanalysis::pc_degree_harness::{
    is_decomposable, measure_refutation_degree, restrict_to_subspace,
};
use hashed_ecdl_tensor::treewidth::{analyse, Analysis, PrimalGraph};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::time::Instant;

/// Field degrees whose irreducible polynomial `choose_irreducible` is known
/// to return: the harness table for 2..=16, and `x^n + x + 1` (its default)
/// for the degrees where that trinomial is irreducible (Ben-Or test).
const SUPPORTED_N: &[u32] = &[
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 22, 28, 30,
];

const USAGE: &str = "semaev-descent-treewidth --n N[,N...] [--d-max 4] [--seed 20261112] \
[--restarts 3] [--skip-refutation] [--decomposable-search-max-bits 24] [--out rows.json]\n\
supported n: 2..=16, 22, 28, 30 (n' = n/2 as in the refutation harness)";

struct Options {
    n: Vec<u32>,
    d_max: u32,
    seed: u64,
    restarts: usize,
    refutation: bool,
    search_max_bits: u32,
    out: Option<String>,
}

fn parse(args: &[String]) -> Result<Options, String> {
    let mut options = Options {
        n: Vec::new(),
        d_max: 4,
        seed: 20_261_112,
        restarts: 3,
        refutation: true,
        search_max_bits: 24,
        out: None,
    };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--skip-refutation" => {
                options.refutation = false;
                i += 1;
                continue;
            }
            "--help" => return Err(USAGE.to_string()),
            _ => {}
        }
        let value = args
            .get(i + 1)
            .ok_or_else(|| format!("{} needs a value\n{USAGE}", args[i]))?;
        match args[i].as_str() {
            "--n" => {
                options.n = value
                    .split(',')
                    .map(|v| v.parse::<u32>().map_err(|_| format!("bad n {v}")))
                    .collect::<Result<_, _>>()?;
            }
            "--d-max" => options.d_max = value.parse().map_err(|_| "bad d-max".to_string())?,
            "--seed" => options.seed = value.parse().map_err(|_| "bad seed".to_string())?,
            "--restarts" => {
                options.restarts = value.parse().map_err(|_| "bad restarts".to_string())?
            }
            "--decomposable-search-max-bits" => {
                options.search_max_bits =
                    value.parse().map_err(|_| "bad search bits".to_string())?
            }
            "--out" => options.out = Some(value.clone()),
            other => return Err(format!("unknown option {other}\n{USAGE}")),
        }
        i += 2;
    }
    if options.n.is_empty() {
        return Err(USAGE.to_string());
    }
    for &n in &options.n {
        if !SUPPORTED_N.contains(&n) {
            return Err(format!(
                "n = {n} has no certified irreducible polynomial here\n{USAGE}"
            ));
        }
    }
    Ok(options)
}

/// Linear and quadratic support of one descended equation.
struct Equation {
    constant: bool,
    linear: Vec<u32>,
    quadratic: Vec<(u32, u32)>,
}

fn decode(poly: &F2BoolPoly, num_vars: u32) -> Equation {
    let v = num_vars as usize;
    let constant = poly.coeffs[0];
    let linear: Vec<u32> = (0..num_vars)
        .filter(|&i| poly.coeffs[1 + i as usize])
        .collect();
    let mut quadratic = Vec::new();
    let mut index = 1 + v;
    for i in 0..num_vars {
        for j in i + 1..num_vars {
            if poly.coeffs[index] {
                quadratic.push((i, j));
            }
            index += 1;
        }
    }
    debug_assert_eq!(index, poly.coeffs.len());
    Equation {
        constant,
        linear,
        quadratic,
    }
}

/// Black-box encoding: one factor per equation over its variable support.
fn bb_scopes(equations: &[Equation]) -> Vec<Vec<u32>> {
    equations
        .iter()
        .map(|eq| {
            let mut scope: Vec<u32> = eq.linear.clone();
            for &(i, j) in &eq.quadratic {
                scope.push(i);
                scope.push(j);
            }
            scope.sort_unstable();
            scope.dedup();
            scope
        })
        .collect()
}

/// Split encoding: shared AND gadgets for monomials, XOR chains of arity 3.
/// Returns (variables, scopes, number of AND gadgets, number of chain variables).
fn split_scopes(equations: &[Equation], num_vars: u32) -> (usize, Vec<Vec<u32>>, usize, usize) {
    let mut next = num_vars;
    let mut and_index: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    let mut scopes = Vec::new();
    for eq in equations {
        for &(i, j) in &eq.quadratic {
            if !and_index.contains_key(&(i, j)) {
                and_index.insert((i, j), next);
                scopes.push(vec![i, j, next]);
                next += 1;
            }
        }
    }
    let and_count = and_index.len();
    let mut chain_count = 0;
    for eq in equations {
        let mut terms: Vec<u32> = eq.linear.clone();
        terms.extend(eq.quadratic.iter().map(|m| and_index[m]));
        let _ = eq.constant; // folded into the last gadget's tensor
        match terms.len() {
            0 => {}
            1 | 2 | 3 => scopes.push(terms),
            _ => {
                // c1 = t0 ^ t1; c_k = c_{k-1} ^ t_k; last: c ^ t_last = constant.
                let mut carry = next;
                next += 1;
                chain_count += 1;
                scopes.push(vec![terms[0], terms[1], carry]);
                for &t in &terms[2..terms.len() - 1] {
                    let fresh = next;
                    next += 1;
                    chain_count += 1;
                    scopes.push(vec![carry, t, fresh]);
                    carry = fresh;
                }
                scopes.push(vec![carry, terms[terms.len() - 1]]);
            }
        }
    }
    (next as usize, scopes, and_count, chain_count)
}

fn summarise(analysis: &Analysis) -> Value {
    let upper = [
        Some(analysis.min_degree.width),
        analysis.min_fill.as_ref().map(|r| r.width),
        analysis.min_fill_lazy.as_ref().map(|r| r.width),
    ]
    .into_iter()
    .flatten()
    .min();
    json!({
        "lower_bound": analysis.lower_bound,
        "upper_bound": upper,
        "min_degree": analysis.min_degree.width,
        "min_fill": analysis.min_fill.as_ref().map(|r| r.width),
        "min_fill_lazy": analysis.min_fill_lazy.as_ref().map(|r| r.width),
        "core_variables": analysis.core_variables,
        "exact": analysis.exact,
        "log2_cost_best": [
            Some(&analysis.min_degree),
            analysis.min_fill.as_ref(),
            analysis.min_fill_lazy.as_ref(),
        ]
        .into_iter()
        .flatten()
        .filter(|r| Some(r.width) == upper)
        .map(|r| r.log2_cost)
        .fold(f64::INFINITY, f64::min),
    })
}

fn log2_binomial(n: u64, k: u64) -> f64 {
    let k = k.min(n - k);
    (0..k)
        .map(|i| ((n - i) as f64).log2() - ((i + 1) as f64).log2())
        .sum()
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let options = parse(&args)?;
    let mut rows = Vec::new();
    for &n in &options.n {
        let n_sub = (n / 2).max(1);
        let num_vars = 2 * n_sub;
        let mut rng = StdRng::seed_from_u64(options.seed ^ (u64::from(n) << 32));
        let irr = choose_irreducible(n);
        let b = random_nonzero_f2m(&mut rng, n);
        let mut x3 = random_nonzero_f2m(&mut rng, n);
        let searched = num_vars <= options.search_max_bits;
        let mut decomposable = None;
        if searched {
            for _ in 0..256 {
                if !is_decomposable(n, n_sub, &irr, &b, &x3) {
                    break;
                }
                x3 = random_nonzero_f2m(&mut rng, n);
            }
            decomposable = Some(is_decomposable(n, n_sub, &irr, &b, &x3));
        }
        let started = Instant::now();
        let eqs = restrict_to_subspace(&weil_descend_s3(n, &irr, &b, &x3), n, n_sub);
        let equations: Vec<Equation> = eqs.iter().map(|p| decode(p, num_vars)).collect();
        let build_seconds = started.elapsed().as_secs_f64();
        let quadratic_monomials: usize = {
            let mut all: Vec<(u32, u32)> = equations
                .iter()
                .flat_map(|e| e.quadratic.iter().copied())
                .collect();
            all.sort_unstable();
            all.dedup();
            all.len()
        };
        let bb = bb_scopes(&equations);
        let bb_graph = PrimalGraph::from_scopes(num_vars as usize, &bb);
        let started = Instant::now();
        let bb_analysis = analyse(&bb_graph, options.restarts, true, true);
        let bb_seconds = started.elapsed().as_secs_f64();
        let (split_vars, split, and_count, chain_count) = split_scopes(&equations, num_vars);
        let split_graph = PrimalGraph::from_scopes(split_vars, &split);
        let started = Instant::now();
        let split_analysis = analyse(&split_graph, options.restarts, true, true);
        let split_seconds = started.elapsed().as_secs_f64();
        let refutation = if options.refutation {
            let started = Instant::now();
            let row = measure_refutation_degree(n, n_sub, &irr, &b, &x3, options.d_max);
            let macaulay: Vec<Value> = row
                .per_degree
                .iter()
                .map(|m| {
                    json!({
                        "degree": m.degree, "rows": m.rows_constructed, "cols": m.cols,
                        "rank": m.rank, "refuted": m.refuted,
                    })
                })
                .collect();
            json!({
                "d_max": options.d_max,
                "first_fall": row.first_fall,
                "refutation_degree": row.refutation_degree,
                "decomposable": row.decomposable,
                "macaulay": macaulay,
                "log2_columns_at_refutation": row.refutation_degree.map(|d| {
                    (0..=u64::from(d)).map(|k| 2f64.powf(log2_binomial(u64::from(num_vars) + 0, k))).sum::<f64>().log2()
                }),
                "seconds": started.elapsed().as_secs_f64(),
            })
        } else {
            Value::Null
        };
        let row = json!({
            "n": n,
            "n_sub": n_sub,
            "num_vars": num_vars,
            "num_eqs": equations.len(),
            "quadratic_monomials": quadratic_monomials,
            "decomposable_target": decomposable,
            "target_search": searched,
            "bb": {
                "factors": bb.len(),
                "ports": bb.iter().map(Vec::len).sum::<usize>(),
                "max_arity": bb.iter().map(Vec::len).max().unwrap_or(0),
                "primal_edges": bb_graph.edge_count(),
                "width": summarise(&bb_analysis),
                "seconds": bb_seconds,
            },
            "split": {
                "variables": split_vars,
                "and_gadgets": and_count,
                "chain_variables": chain_count,
                "factors": split.len(),
                "ports": split.iter().map(Vec::len).sum::<usize>(),
                "primal_edges": split_graph.edge_count(),
                "width": summarise(&split_analysis),
                "seconds": split_seconds,
            },
            "enumeration_exponent_one_slot": n_sub,
            "enumeration_exponent_both_slots": num_vars,
            "refutation": refutation,
            "build_seconds": build_seconds,
            "restarts": options.restarts,
            "seed": options.seed,
        });
        println!("{row}");
        rows.push(row);
    }
    if let Some(out) = &options.out {
        std::fs::write(out, serde_json::to_string_pretty(&rows).unwrap() + "\n")
            .map_err(|e| format!("write {out}: {e}"))?;
    }
    Ok(())
}

fn main() {
    if let Err(message) = run() {
        eprintln!("semaev-descent-treewidth: {message}");
        std::process::exit(1);
    }
}
