//! The summand-swap invariant `p = x₁x₂` of the chained `S₃` descent, against
//! the plain system and a matched non-invariant auxiliary, on committed draws.
//!
//! At `m = 3` the chain is `S₃(x₁, x₂, e₁)`, `S₃(e₁, x₃, x(R))` with
//! `S₃(x₁, x₂, x₃) = (x₁+x₂)²x₃² + x₁x₂x₃ + (x₁x₂)² + b`. The second link is
//! quadratic in the Boolean unknowns (`x(R)` is a constant), so every cubic
//! monomial comes from `x₁x₂e₁`. The first link is invariant under swapping
//! the two summands; the invariants of that swap are `s = x₁ + x₂` (linear)
//! and `p = x₁x₂`. Three arms on the same draw:
//!
//! - `B`: the plain system ([`build_decomposition_system`]).
//! - `Q`: `p` as new unknowns over a basis of `W = span{v_i v_j : i ≤ j}`
//!   (`x₁x₂` always lies in `W`). The first link becomes
//!   `s²e₁² + p e₁ + p² + b`, and `p + x₁x₂ = 0` is added. Every equation is
//!   quadratic.
//! - `R`: the matched control, `r = x₁e₁` as `n` free unknowns. The cubic
//!   term becomes `x₂ r`, and `r + x₁e₁ = 0` is added. Every equation is
//!   quadratic, but `r` is not invariant under the swap.
//!
//! The roots of each arm are the roots of `B`, each extended by the unique
//! value of its auxiliary. `--check` proves the construction exactly: the
//! first link of `B` minus that of `Q` is `def·e₁ + def²` and minus that of
//! `R` is `x₂·def`, as polynomials, and the other link is equal term for term.
//!
//! ```sh
//! cargo run --release --example dreg_invariant -- --arm Q --to 3 --draw 0 \
//!     research/dreg_closure_driver_20261016/draws/m3-22-6.jsonl
//! ```
//!
//! One JSON line per selected draw on stdout. Dense degrees run from 3 to
//! `--to` (stopping at the first refutation) while `C(N, ≤D) ≤ --max-cols`;
//! a degree not built is unmeasured, never a refutation. `--sparse D` runs the
//! closure at degree `D` alone on the sparse engine, capped at
//! `--max-entries`; a capped run is unmeasured. `--satisfiable` selects the
//! committed satisfiable draws (the soundness control: any refutation there is
//! an instrument failure).

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_bench::{
    closure_refutation_scan, sparse_closure_refutes, ClosureDegree,
};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    build_decomposition_system, sym_semaev_s3, FieldStructure, SymElement, MAX_VARS,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_groebner_f2::{bit, F2BoolPoly, Mask};
use num_bigint::BigUint;
use serde_json::Value;

fn monomials_up_to(n_vars: u64, d: u32) -> f64 {
    let (mut total, mut binom) = (0f64, 1f64);
    for i in 0..=d as u64 {
        if i > n_vars {
            break;
        }
        total += binom;
        binom = binom * (n_vars - i) as f64 / (i + 1) as f64;
    }
    total
}

fn degree_json(d: &ClosureDegree) -> String {
    format!(
        r#"{{"degree":{},"xl_refuted":{},"closure_refuted":{},"saturated":{},"xl_rank":{},"closure_rank":{},"columns":{},"closure_products":{},"closure_word_ops":{},"secs":{:.3}}}"#,
        d.degree,
        d.xl_refuted,
        d.closure_refuted,
        d.saturated,
        d.xl_rank,
        d.closure_rank,
        d.columns,
        d.closure_products,
        d.closure_word_ops,
        d.secs
    )
}

/// A basis of `span{v_i v_j : i ≤ j}`, read off the symbolic product
/// `x₁x₂` (`x₁`, `x₂` on disjoint variables): the coefficient vector of
/// monomial `a_i b_j` is `v_i v_j`. Returned as field elements.
fn product_span(x1x2: &SymElement, ell: usize, n: u32) -> Vec<F2mElement> {
    let mut vectors = Vec::new();
    for i in 0..ell {
        for j in i..ell {
            let mono = bit(i as u32) | bit((ell + j) as u32);
            let mut v = 0u64;
            for (k, c) in x1x2.coords.iter().enumerate() {
                if c.terms.iter().any(|t| t.mask == mono) {
                    v |= 1 << k;
                }
            }
            vectors.push(v);
        }
    }
    // Echelon form over F_2: keep each vector that is independent of those kept.
    let mut kept: Vec<u64> = Vec::new();
    for mut v in vectors {
        for &k in &kept {
            let top = 63 - k.leading_zeros();
            if v >> top & 1 == 1 {
                v ^= k;
            }
        }
        if v != 0 {
            kept.push(v);
            kept.sort_unstable_by(|a, b| b.cmp(a));
        }
    }
    kept.into_iter()
        .map(|v| F2mElement::from_biguint(&BigUint::from(v), n))
        .collect()
}

fn equal(a: &SymElement, b: &SymElement) -> bool {
    a.coords
        .iter()
        .zip(&b.coords)
        .all(|(x, y)| x.add(y).is_zero())
}

struct Arm {
    equations: Vec<F2BoolPoly>,
    n_vars: usize,
    aux: usize,
    /// The exact identity check passed (`--check`), or was not asked for.
    checked: Option<bool>,
}

fn build_arm(
    arm: &str,
    basis: &[F2mElement],
    x_r: &F2mElement,
    b: &F2mElement,
    st: &FieldStructure,
    check: bool,
) -> Arm {
    let (n, ell) = (st.n, basis.len());
    let base_vars = 3 * ell + n as usize;
    if arm == "B" {
        let sys = build_decomposition_system(basis, x_r, b, 3, st).expect("system");
        return Arm {
            equations: sys.equations,
            n_vars: sys.n_vars,
            aux: 0,
            checked: None,
        };
    }
    // The auxiliary's width decides the layout, so build the product once on a
    // scratch layout to read W (its variables are only the summand blocks).
    let w_basis = {
        let x1 = SymElement::from_subspace_vars(basis, 0, n, base_vars);
        let x2 = SymElement::from_subspace_vars(basis, ell, n, base_vars);
        product_span(&x1.mul(&x2, st), ell, n)
    };
    let aux = match arm {
        "Q" => w_basis.len(),
        "R" => n as usize,
        _ => panic!("--arm B|Q|R"),
    };
    let n_vars = base_vars + aux;
    assert!(n_vars <= MAX_VARS, "{n_vars} unknowns exceed {MAX_VARS}");
    let x1 = SymElement::from_subspace_vars(basis, 0, n, n_vars);
    let x2 = SymElement::from_subspace_vars(basis, ell, n, n_vars);
    let x3 = SymElement::from_subspace_vars(basis, 2 * ell, n, n_vars);
    let e1 = SymElement::from_free_vars(3 * ell, n, n_vars);
    let target = SymElement::constant(x_r, n, n_vars);
    let bconst = SymElement::constant(b, n, n_vars);
    let x1x2 = x1.mul(&x2, st);
    let s2e2 = x1.add(&x2).square(st).mul(&e1.square(st), st);
    let (link1, def) = if arm == "Q" {
        let p = SymElement::from_subspace_vars(&w_basis, base_vars, n, n_vars);
        let link = s2e2.add(&p.mul(&e1, st)).add(&p.square(st)).add(&bconst);
        (link, p.add(&x1x2))
    } else {
        let r = SymElement::from_free_vars(base_vars, n, n_vars);
        let link = s2e2.add(&x2.mul(&r, st)).add(&x1x2.square(st)).add(&bconst);
        (link, r.add(&x1.mul(&e1, st)))
    };
    let link2 = sym_semaev_s3(&e1, &x3, &target, b, st);
    let checked = check.then(|| {
        let plain = SymElement {
            coords: sym_semaev_s3(&x1, &x2, &e1, b, st),
        };
        let diff = plain.add(&link1);
        let expected = if arm == "Q" {
            def.mul(&e1, st).add(&def.square(st))
        } else {
            x2.mul(&def, st)
        };
        let reference = build_decomposition_system(basis, x_r, b, 3, st).expect("system");
        // The plain system on the wider layout: the same masks, so compare masks.
        let second_equal = reference.equations[n as usize..]
            .iter()
            .zip(&link2)
            .all(|(a, c)| a.terms == c.terms);
        equal(&diff, &expected) && second_equal
    });
    let mut equations = link1.coords;
    equations.extend(link2);
    equations.extend(def.coords);
    equations.retain(|p| !p.is_zero());
    Arm {
        equations,
        n_vars,
        aux,
        checked,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let arm = flag("--arm").expect("--arm B|Q|R");
    let max_cols: f64 = flag("--max-cols").map_or(130_000.0, |v| v.parse().expect("--max-cols"));
    let to: u32 = flag("--to").map_or(4, |v| v.parse().expect("--to D"));
    let only_draw: Option<u64> = flag("--draw").map(|v| v.parse().expect("--draw K"));
    let sparse: Option<u32> = flag("--sparse").map(|v| v.parse().expect("--sparse D"));
    let max_entries: usize =
        flag("--max-entries").map_or(1_500_000_000, |v| v.parse().expect("--max-entries"));
    let satisfiable = args.iter().any(|a| a == "--satisfiable");
    let check = args.iter().any(|a| a == "--check");
    let valued = [
        "--arm",
        "--max-cols",
        "--to",
        "--draw",
        "--sparse",
        "--max-entries",
    ];
    let files: Vec<&String> = args
        .iter()
        .enumerate()
        .filter(|(i, a)| {
            !valued.contains(&a.as_str())
                && !a.starts_with("--")
                && !(*i > 0 && valued.contains(&args[i - 1].as_str()))
        })
        .map(|(_, a)| a)
        .collect();
    for file in files {
        let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{file}: {e}"));
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).expect("json");
            let sat = v["outcome"]["kind"] == "satisfiable";
            if v.get("control").is_some() || sat != satisfiable {
                continue;
            }
            if only_draw.is_some_and(|k| v["draw"].as_u64() != Some(k)) {
                continue;
            }
            let m = v.get("m").and_then(Value::as_u64).unwrap_or(3);
            assert_eq!(m, 3, "{file}: the invariant arms are built for m = 3");
            let n = v["n"].as_u64().expect("n") as u32;
            let ell = v["ell"].as_u64().expect("ell") as usize;
            let irr = find_irreducible_sparse(n).expect("field");
            let st = FieldStructure::new(n, &irr);
            let b = F2mElement::one(n);
            let basis: Vec<F2mElement> = v["v_basis"]
                .as_array()
                .expect("v_basis")
                .iter()
                .map(|w| F2mElement::from_biguint(&BigUint::from(w.as_u64().unwrap()), n))
                .collect();
            let x_r = F2mElement::from_biguint(&BigUint::from(v["x_r"].as_u64().unwrap()), n);
            assert_eq!(basis.len(), ell);
            let a = build_arm(&arm, &basis, &x_r, &b, &st, check);
            if arm == "B" {
                assert_eq!(
                    a.n_vars as u64,
                    v["n_vars"].as_u64().unwrap(),
                    "rebuilt shape"
                );
            }
            let occurring = a
                .equations
                .iter()
                .flat_map(|p| p.terms.iter())
                .fold(0 as Mask, |acc, t| acc | t.mask)
                .count_ones() as u64;
            let max_deg = a
                .equations
                .iter()
                .flat_map(|p| p.terms.iter())
                .map(|t| t.mask.count_ones())
                .max()
                .unwrap_or(0);
            let head = format!(
                r#""source":"{file}","cell":"{}","arm":"{arm}","m":3,"n":{n},"ell":{ell},"draw":{},"committed_outcome":{},"n_vars":{},"aux":{},"occurring":{occurring},"equations":{},"max_degree":{max_deg},"identity_check":{}"#,
                v["cell"].as_str().unwrap_or(""),
                v["draw"],
                v["outcome"],
                a.n_vars,
                a.aux,
                a.equations.len(),
                a.checked.map_or("null".to_string(), |c| c.to_string()),
            );
            if let Some(d) = sparse {
                let started = std::time::Instant::now();
                let got = sparse_closure_refutes(&a.equations, a.n_vars, d, max_entries);
                let secs = started.elapsed().as_secs_f64();
                let body = match &got {
                    Some(c) => format!(
                        r#""capped":false,"closure_refuted":{},"saturated":{},"rank":{},"falls":{},"products":{},"max_weight":{}"#,
                        c.refuted, c.saturated, c.rank, c.falls, c.products, c.max_weight
                    ),
                    None => r#""capped":true"#.to_string(),
                };
                println!(
                    r#"{{{head},"engine":"sparse","degree":{d},"max_entries":{max_entries},{body},"secs":{secs:.3}}}"#
                );
                eprintln!(
                    "{arm} ({n},{ell}) draw {} sparse {d}: {body} ({secs:.1}s)",
                    v["draw"]
                );
                continue;
            }
            let mut scan = Vec::new();
            for d in (3..=to).take_while(|&d| monomials_up_to(occurring, d) <= max_cols) {
                let Some(step) = closure_refutation_scan(&a.equations, a.n_vars, d, d)
                    .into_iter()
                    .next()
                else {
                    break;
                };
                let done = step.closure_refuted;
                scan.push(step);
                if done {
                    break;
                }
            }
            let closure = scan.iter().find(|d| d.closure_refuted).map(|d| d.degree);
            let measured_to = scan.last().map(|d| d.degree).unwrap_or(2);
            println!(
                r#"{{{head},"engine":"dense","closure_degree":{},"measured_to":{measured_to},"max_cols":{max_cols},"scan":[{}]}}"#,
                closure.map_or("null".to_string(), |d| d.to_string()),
                scan.iter().map(degree_json).collect::<Vec<_>>().join(",")
            );
            eprintln!(
                "{arm} ({n},{ell}) draw {}: N={} closure {:?} (measured to {measured_to})",
                v["draw"], a.n_vars, closure
            );
        }
    }
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        if let Some(l) = status.lines().find(|l| l.starts_with("VmHWM:")) {
            eprintln!("peak {l}");
        }
    }
}
