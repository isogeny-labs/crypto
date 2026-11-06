//! Guess-then-closure on the chained S₃ descent with *linear* guesses: fix
//! variables to constants (`--fix v=b`) and/or substitute `x_j := x_i + c`
//! (`--sum j=i+c`), then scan the MutantXL closure degree by degree.
//!
//! A raw guess fixes one coordinate of one summand. A sum guess fixes the
//! linear form `x_i + x_j = c`; with `i`, `j` the same coordinate of the two
//! summands of the first link, it fixes a coordinate of `s = x₁ + x₂`, which
//! is invariant under swapping them (the first link sees the summands only
//! through `s` and `p = x₁x₂`). `--sum j=i1+i2+…+c` substitutes a sum of
//! several sources.
//!
//! `--link2 k=c` (m = 3) fixes coordinate `k` of `e₁ + x₃`, the invariant of
//! the second link `S₃(e₁, x₃, x(R))` under swapping its two unknown
//! arguments: `e₁[k] := c + Σ x₃`'s coordinate-`k` terms, that is
//! `x_{3ℓ+k} := c + Σ_{t : v_t has bit k} x_{2ℓ+t}`, read from the draw's basis.
//!
//! `--link j:k=c` (any m, 2 ≤ j ≤ m − 1) fixes coordinate `k` of
//! `e_{j−1} + x_{j+1}`, the swap invariant of link `j`:
//! `x_{mℓ+(j−2)n+k} := c + Σ_{t : v_t has bit k} x_{jℓ+t}`. `k` may be `topI`,
//! the I-th largest coordinate with at least two such `t` on the draw; the
//! resolved coordinates are written to the record's `link` field.
//!
//! ```sh
//! cargo run --release --example dreg_linear_guess -- --to 4 --max-cols 140000 \
//!     --draw 0 --sum 15=7+0 --fix 23=0,24=0 --check \
//!     research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl
//! ```
//!
//! One JSON line per selected draw. The dense scan runs from degree 3 to
//! `--to`, stops at the first refutation, and builds no degree with more than
//! `--max-cols` columns over the variables that occur; a degree not built is
//! unmeasured. `--satisfiable` selects the committed satisfiable draws.
//! `--check` proves the specialisation exact: at 4,096 random points that
//! satisfy every guess, each specialised equation equals the original.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_bench::{closure_refutation_scan, ClosureDegree};
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_groebner_f2::{bit, F2BoolMono, F2BoolPoly, Mask};
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

/// `x_j := c + Σ_i x_i` for each `(j, sources, c)`, then `x_v := b` for each
/// `(v, b)`. A term `m·x_j` becomes `Σ_i m·x_i` (+ `m` when `c = 1`); a term
/// containing a fixed `x_v` vanishes (`b = 0`) or loses it (`b = 1`). Equal
/// terms cancel in pairs ([`F2BoolPoly::from_monos`]).
type Sum = (u32, Vec<u32>, bool);

fn specialise(p: &F2BoolPoly, sums: &[Sum], fixes: &[(u32, bool)]) -> F2BoolPoly {
    let mut masks: Vec<Mask> = p.terms.iter().map(|t| t.mask).collect();
    for (j, sources, c) in sums {
        let (j, c) = (*j, *c);
        let mut next = Vec::with_capacity(masks.len() * (sources.len() + 1));
        for m in masks {
            if m & bit(j) == 0 {
                next.push(m);
            } else {
                let rest = m & !bit(j);
                next.extend(sources.iter().map(|&i| rest | bit(i)));
                if c {
                    next.push(rest);
                }
            }
        }
        masks = next;
    }
    let mut kept = Vec::with_capacity(masks.len());
    'term: for mut m in masks {
        for &(v, b) in fixes {
            if m & bit(v) != 0 {
                if !b {
                    continue 'term;
                }
                m &= !bit(v);
            }
        }
        kept.push(F2BoolMono::from_mask(m));
    }
    F2BoolPoly::from_monos(kept, p.n_vars)
}

/// A random point of the guessed affine subspace: free bits random, each
/// `x_j = c + Σ_i x_i`, each fixed `x_v = b`.
fn guessed_point(state: &mut u64, n_vars: usize, sums: &[Sum], fixes: &[(u32, bool)]) -> Mask {
    let mut point: Mask = 0;
    for k in 0..n_vars as u32 {
        // splitmix64
        *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^= z >> 31;
        if z & 1 == 1 {
            point |= bit(k);
        }
    }
    for &(v, b) in fixes {
        point = (point & !bit(v)) | if b { bit(v) } else { 0 };
    }
    for (j, sources, c) in sums {
        let value = sources
            .iter()
            .fold(*c, |acc, &i| acc ^ (point & bit(i) != 0));
        point = (point & !bit(*j)) | if value { bit(*j) } else { 0 };
    }
    point
}

fn parse_list<T>(text: Option<String>, item: impl Fn(&str) -> T) -> Vec<T> {
    text.map_or(Vec::new(), |t| {
        t.split(',').filter(|s| !s.is_empty()).map(item).collect()
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let max_cols: f64 = flag("--max-cols").map_or(140_000.0, |v| v.parse().expect("--max-cols"));
    let to: u32 = flag("--to").map_or(4, |v| v.parse().expect("--to D"));
    let only_draw: Option<u64> = flag("--draw").map(|v| v.parse().expect("--draw K"));
    let satisfiable = args.iter().any(|a| a == "--satisfiable");
    let check = args.iter().any(|a| a == "--check");
    // `--fix v=b,…`: x_v := b.
    let fixes: Vec<(u32, bool)> = parse_list(flag("--fix"), |kv| {
        let (k, b) = kv.split_once('=').expect("--fix v=b");
        (k.parse().expect("variable"), b == "1")
    });
    // `--sum j=i1+i2+…+c,…`: x_j := x_i1 + x_i2 + … + c.
    let sums: Vec<Sum> = parse_list(flag("--sum"), |kv| {
        let (j, rhs) = kv.split_once('=').expect("--sum j=i+c");
        let mut parts: Vec<&str> = rhs.split('+').collect();
        let c = parts.pop().expect("--sum j=i+c") == "1";
        assert!(!parts.is_empty(), "--sum j=i+c needs a source");
        (
            j.parse().expect("variable"),
            parts.iter().map(|i| i.parse().expect("variable")).collect(),
            c,
        )
    });
    // `--link2 k=c,…`: coordinate k of e₁ + x₃ (resolved per draw).
    let link2: Vec<(usize, bool)> = parse_list(flag("--link2"), |kv| {
        let (k, c) = kv.split_once('=').expect("--link2 k=c");
        (k.parse().expect("coordinate"), c == "1")
    });
    // `--link j:k=c,…` (any m ≥ 3, 2 ≤ j ≤ m − 1): coordinate k of
    // e_{j−1} + x_{j+1}, the swap invariant of link j. `k` is a number, or
    // `topI`: the I-th largest coordinate whose basis set T_k has at least two
    // members on this draw (resolved per draw; I counts from 1).
    let links: Vec<(usize, String, bool)> = parse_list(flag("--link"), |kv| {
        let (jk, c) = kv.split_once('=').expect("--link j:k=c");
        let (j, k) = jk.split_once(':').expect("--link j:k=c");
        (j.parse().expect("link"), k.to_string(), c == "1")
    });
    let valued = [
        "--max-cols",
        "--to",
        "--draw",
        "--fix",
        "--sum",
        "--link2",
        "--link",
    ];
    let files: Vec<&String> = args
        .iter()
        .enumerate()
        .filter(|(i, a)| {
            !a.starts_with("--") && !(*i > 0 && valued.contains(&args[i - 1].as_str()))
        })
        .map(|(_, a)| a)
        .collect();
    // A single source is written `[j, i, c]` (the form earlier records use),
    // several `[j, [i…], c]`.
    let guesses_json = |sums: &[Sum], resolved: &[(usize, usize, bool)]| {
        let link = if resolved.is_empty() {
            String::new()
        } else {
            format!(
                r#","link":[{}]"#,
                resolved
                    .iter()
                    .map(|(j, k, c)| format!("[{j},{k},{}]", *c as u8))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        };
        format!(
            r#""fixed":[{}],"sums":[{}],"link2":[{}]{link}"#,
            fixes
                .iter()
                .map(|(v, b)| format!("[{v},{}]", *b as u8))
                .collect::<Vec<_>>()
                .join(","),
            sums.iter()
                .map(|(j, i, c)| match i.as_slice() {
                    [one] => format!("[{j},{one},{}]", *c as u8),
                    many => format!("[{j},{many:?},{}]", *c as u8),
                })
                .collect::<Vec<_>>()
                .join(","),
            link2
                .iter()
                .map(|(k, c)| format!("[{k},{}]", *c as u8))
                .collect::<Vec<_>>()
                .join(",")
        )
    };
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
            let m = v.get("m").and_then(Value::as_u64).unwrap_or(3) as usize;
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
            let sys = build_decomposition_system(&basis, &x_r, &b, m, &st).expect("system");
            let mut sums = sums.clone();
            for &(k, c) in &link2 {
                assert!(m == 3 && k < n as usize, "--link2 needs m = 3 and k < n");
                let sources: Vec<u32> = (0..ell)
                    .filter(|&t| basis[t].raw_bits().first().copied().unwrap_or(0) >> k & 1 == 1)
                    .map(|t| (2 * ell + t) as u32)
                    .collect();
                assert!(
                    !sources.is_empty(),
                    "--link2 {k}: no basis vector has bit {k}"
                );
                sums.push(((3 * ell + k) as u32, sources, c));
            }
            // T_k of the draw's basis: the summand coordinates t whose basis
            // vector has bit k.
            let t_of = |k: usize| -> Vec<usize> {
                (0..ell)
                    .filter(|&t| basis[t].raw_bits().first().copied().unwrap_or(0) >> k & 1 == 1)
                    .collect()
            };
            let eligible: Vec<usize> = (0..n as usize)
                .rev()
                .filter(|&k| t_of(k).len() >= 2)
                .collect();
            let mut resolved = Vec::new();
            for (j, spec, c) in &links {
                let j = *j;
                assert!(
                    (2..m).contains(&j),
                    "--link {j}: needs 2 <= j <= m - 1 (m = {m})"
                );
                let k = match spec.strip_prefix("top") {
                    Some(i) => {
                        let i: usize = i.parse().expect("--link j:topI");
                        assert!(i >= 1, "--link j:topI counts from 1");
                        *eligible.get(i - 1).unwrap_or_else(|| {
                            panic!(
                                "--link {j}:{spec}: only {} coordinates have |T_k| >= 2",
                                eligible.len()
                            )
                        })
                    }
                    None => spec.parse().expect("--link j:k"),
                };
                assert!(k < n as usize, "--link {j}:{k}: needs k < n");
                let sources: Vec<u32> = t_of(k).iter().map(|&t| (j * ell + t) as u32).collect();
                assert!(
                    !sources.is_empty(),
                    "--link {j}:{k}: no basis vector has bit {k}"
                );
                sums.push(((m * ell + (j - 2) * n as usize + k) as u32, sources, *c));
                resolved.push((j, k, *c));
            }
            // The substituted variables must be distinct and never fixed or used
            // as a source, or the order of substitution would matter.
            let targets: Vec<u32> = sums.iter().map(|s| s.0).collect();
            for (j, sources, _) in &sums {
                assert!(
                    targets.iter().filter(|&&t| t == *j).count() == 1
                        && sources.iter().all(|i| !targets.contains(i))
                        && !fixes.iter().any(|f| f.0 == *j || sources.contains(&f.0)),
                    "--sum/--link2 target {j} overlaps another guess"
                );
            }
            assert_eq!(
                sys.n_vars as u64,
                v["n_vars"].as_u64().unwrap(),
                "rebuilt shape"
            );
            let equations: Vec<F2BoolPoly> = sys
                .equations
                .iter()
                .map(|p| specialise(p, &sums, &fixes))
                .collect();
            let identity = check.then(|| {
                let mut state = 0x5eed_u64 ^ v["draw"].as_u64().unwrap_or(0);
                (0..4096).all(|_| {
                    let z = guessed_point(&mut state, sys.n_vars, &sums, &fixes);
                    sys.equations
                        .iter()
                        .zip(&equations)
                        .all(|(o, s)| o.eval(z) == s.eval(z))
                })
            });
            let equations: Vec<F2BoolPoly> = equations
                .into_iter()
                .filter(|p| !p.terms.is_empty())
                .collect();
            let occurring = equations
                .iter()
                .flat_map(|p| p.terms.iter())
                .fold(0 as Mask, |acc, t| acc | t.mask)
                .count_ones() as u64;
            let mut scan = Vec::new();
            for d in (3..=to).take_while(|&d| monomials_up_to(occurring, d) <= max_cols) {
                let Some(step) = closure_refutation_scan(&equations, sys.n_vars, d, d)
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
                r#"{{"source":"{file}","cell":"{}","m":{m},"n":{n},"ell":{ell},"draw":{},"n_vars":{},"occurring":{occurring},"committed_outcome":{},{},"identity_check":{},"closure_degree":{},"measured_to":{measured_to},"max_cols":{max_cols},"scan":[{}]}}"#,
                v["cell"].as_str().unwrap_or(""),
                v["draw"],
                sys.n_vars,
                v["outcome"],
                guesses_json(&sums, &resolved),
                identity.map_or("null".to_string(), |c| c.to_string()),
                closure.map_or("null".to_string(), |d| d.to_string()),
                scan.iter().map(degree_json).collect::<Vec<_>>().join(",")
            );
            eprintln!(
                "({n},{ell}) draw {}: occurring {occurring}, closure {:?} (measured to {measured_to})",
                v["draw"], closure
            );
        }
    }
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        if let Some(l) = status.lines().find(|l| l.starts_with("VmHWM:")) {
            eprintln!("peak {l}");
        }
    }
}
