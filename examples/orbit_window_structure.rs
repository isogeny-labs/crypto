//! Structure of the Frobenius-orbit merges in a prefix-subspace factor base
//! (`research/orbit_aware_large_prime_20261011/RESULTS_ADDENDUM_20261026.md`,
//! finding D of the independent review).
//!
//! ```text
//! orbit_window_structure report --out DIR [--receipts DIR] [--random 8] [--seed 1]
//! orbit_window_structure scan --small 5 --env 10 [--n-min 9] [--n-max 31]
//! ```
//!
//! `report` answers, for the five measured curves (n = 17, 19, 23, 29, 31):
//!
//! 1. How many unordered pairs `{x0, x1}` of abscissae in the large part of
//!    the window (`2^s <= x < 2^env`) are exact Frobenius images of each other
//!    (`x1 = x0^(2^m)`, `1 <= m < n`), for the prefix window and for `K`
//!    seeded random subspaces of the same dimension?
//! 2. Recomputing the large-point table from its specification (an independent
//!    oracle, `tests/common/orbit_support.rs`): how many of the merges are
//!    raw-abscissa Frobenius images and how many exist only after the cofactor
//!    projection?  The recomputed counts are compared with the merged
//!    receipts.
//! 3. The same on the seeded random (dense) modulus of the realisation
//!    series, i.e. the same abstract curve in a different polynomial basis.
//!
//! No Python is involved.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use crypto_lib::cryptanalysis::ic_boundary::{koblitz_instance, BinaryInstance};
use serde_json::{json, Value};

#[path = "../tests/common/orbit_support.rs"]
mod orbit_support;
use orbit_support::{
    brute_force_counts, classify_merges, conjugacy_components, instance_with_modulus, oracle_base,
    random_modulus, OracleBase, SplitMix,
};

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn num(args: &[String], name: &str, default: u64) -> Result<u64, String> {
    arg(args, name).map_or(Ok(default), |v| {
        v.parse::<u64>().map_err(|e| format!("{name}: {e}"))
    })
}

/// The five measured curves: `(n, small dimension)`, envelope 10.
const CURVES: [(u32, u32); 5] = [(17, 5), (19, 6), (23, 7), (29, 7), (31, 7)];
const ENV: u32 = 10;
/// Primary cap per n, to name the merged receipt of seed 1.
const CAPS: [(u32, u64); 5] = [(17, 200), (19, 400), (23, 450), (29, 300), (31, 450)];

/// The curve the bench measures at `n`: `K_1` or `K_0`, whichever has the
/// larger prime subgroup.
fn bench_instance(n: u32) -> Result<BinaryInstance, String> {
    [koblitz_instance(1, n), koblitz_instance(0, n)]
        .into_iter()
        .flatten()
        .max_by_key(|i| i.r)
        .ok_or_else(|| format!("no Koblitz curve at n={n}"))
}

// ----------------------------------------------------------------- windows

/// An `F_2`-subspace of the field given by an ordered basis; element number
/// `mask` is the XOR of the basis vectors selected by its bits.  The "large"
/// part is `mask >= 2^small`, i.e. the elements that use a basis vector of
/// index `>= small`.
struct Window {
    /// element -> mask, for masks `>= 2^small`.
    large: HashMap<u64, u32>,
}

impl Window {
    fn new(basis: &[u64], small: u32) -> Self {
        let dim = basis.len();
        let mut elements = vec![0u64; 1 << dim];
        for mask in 1usize..1 << dim {
            let low = mask.trailing_zeros() as usize;
            elements[mask] = elements[mask & (mask - 1)] ^ basis[low];
        }
        let large = (1usize << small..1 << dim)
            .map(|mask| (elements[mask], mask as u32))
            .collect();
        Self { large }
    }

    /// Unordered pairs `{x0, x1}` of large elements with `x1 = x0^(2^m)`,
    /// `1 <= m < n`.
    fn frobenius_pairs(&self, sqr: &dyn Fn(u64) -> u64, n: u32) -> usize {
        let mut pairs = std::collections::BTreeSet::new();
        for &x0 in self.large.keys() {
            let mut x = x0;
            for _ in 1..n {
                x = sqr(x);
                if x != x0 && self.large.contains_key(&x) {
                    pairs.insert((x0.min(x), x0.max(x)));
                }
            }
        }
        pairs.len()
    }
}

/// `dim` random linearly independent field elements (`n`-bit masks).
fn random_basis(n: u32, dim: u32, rng: &mut SplitMix) -> Vec<u64> {
    let mut basis: Vec<u64> = Vec::new();
    let mut reduced: Vec<u64> = Vec::new();
    while basis.len() < dim as usize {
        let v = rng.next() & ((1u64 << n) - 1);
        let mut w = v;
        for &b in &reduced {
            w = w.min(w ^ b);
        }
        if w != 0 {
            reduced.push(w);
            reduced.sort_unstable_by(|a, b| b.cmp(a));
            basis.push(v);
        }
    }
    basis
}

struct PairCounts {
    large_elements: usize,
    prefix: usize,
    random: Vec<usize>,
    /// `|W|^2 (n-1) / 2^(n+1)`: expected unordered pairs for a random window.
    expected: f64,
}

fn pair_counts(inst: &BinaryInstance, small: u32, random: u64, seed: u64) -> PairCounts {
    let n = inst.n;
    let sqr = |x: u64| inst.fast.field.sqr(x);
    let prefix_basis: Vec<u64> = (0..ENV).map(|i| 1u64 << i).collect();
    let prefix = Window::new(&prefix_basis, small);
    let mut counts = Vec::new();
    for k in 0..random {
        let mut rng = SplitMix(seed ^ (u64::from(n) << 32) ^ (k + 1).wrapping_mul(0x9e37_79b9));
        let window = Window::new(&random_basis(n, ENV, &mut rng), small);
        counts.push(window.frobenius_pairs(&sqr, n));
    }
    let w = prefix.large.len() as f64;
    PairCounts {
        large_elements: prefix.large.len(),
        prefix: prefix.frobenius_pairs(&sqr, n),
        random: counts,
        expected: w * w * f64::from(n - 1) / 2f64.powi(n as i32 + 1),
    }
}

// ------------------------------------------------------------------ report

fn merge_table(inst: &BinaryInstance, small: u32) -> (OracleBase, Value, Vec<u64>) {
    let base = oracle_base(inst, small, ENV);
    let counts = brute_force_counts(inst, &base);
    let split = classify_merges(inst, &base);
    // Largest orbit, by raw abscissae of its members.
    let points: Vec<_> = base.large.iter().map(|p| p.point).collect();
    let labels = conjugacy_components(inst, &points);
    let mut orbits: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (i, &label) in labels.iter().enumerate() {
        orbits.entry(label).or_default().push(i);
    }
    let largest = orbits
        .values()
        .max_by_key(|m| m.len())
        .map(|members| {
            let mut xs: Vec<u64> = members
                .iter()
                .flat_map(|&i| base.large[i].raw_abscissae.clone())
                .collect();
            xs.sort_unstable();
            xs
        })
        .unwrap_or_default();
    let l = counts.large_points as f64;
    let random_model =
        l * (l - 1.0) / 2.0 * (2.0 * f64::from(inst.n) - 2.0) / (inst.r as f64 - 1.0);
    let table = json!({
        "large_points": counts.large_points,
        "orbits": counts.orbits,
        "merged": counts.merged,
        "absorbable": counts.absorbed,
        "merged_after_absorb": counts.merged_after_absorb,
        "multi_member_orbits": split.multi_member_orbits,
        "raw_frobenius_merges": split.raw_frobenius,
        "projected_only_merges": split.projected_only,
        "largest_orbit_members": split.largest_orbit,
        "random_model_expected_pairs": random_model,
    });
    (base, table, largest)
}

fn receipt_orbit(dir: &Path, n: u32, cap: u64) -> Option<(u64, u64, u64)> {
    let path = dir.join(format!("cell_primary_n{n}_dlp_orbit_t{cap}_s1_r1.json"));
    let value: Value = serde_json::from_str(&fs::read_to_string(path).ok()?).ok()?;
    let orbit = value.pointer("/report/factor_base/orbit")?;
    Some((
        orbit.get("large_points")?.as_u64()?,
        orbit.get("large_orbits")?.as_u64()?,
        orbit.get("merged_points")?.as_u64()?,
    ))
}

fn poly_string(mask: u64) -> String {
    let mut terms: Vec<String> = (0..64)
        .rev()
        .filter(|i| (mask >> i) & 1 == 1)
        .map(|i| match i {
            0 => "1".to_string(),
            1 => "x".to_string(),
            _ => format!("x^{i}"),
        })
        .collect();
    if terms.is_empty() {
        terms.push("0".into());
    }
    terms.join("+")
}

fn report(args: &[String]) -> Result<(), String> {
    let out = PathBuf::from(arg(args, "--out").ok_or("missing --out")?);
    let receipts = arg(args, "--receipts").map(PathBuf::from);
    let random = num(args, "--random", 8)?;
    let seed = num(args, "--seed", 1)?;
    let mut md = String::new();
    let mut all = Vec::new();
    md.push_str("### Table S1. Abscissa pairs that are exact Frobenius images, prefix window vs random 10-dimensional subspaces\n\n");
    md.push_str(&format!(
        "Unordered pairs `{{x0, x1}}` in the large part of the window (`2^s <= x < 2^{ENV}`, nested random subspaces with the same dimensions) with `x1 = x0^(2^m)`, `1 <= m < n`; {random} seeded random subspaces per n. `E` is the random-window expectation `|W|^2 (n-1) / 2^(n+1)`.\n\n"
    ));
    md.push_str("| n | modulus | s | \\|W\\| | prefix window | random windows | E |\n|---|---|---|---|---|---|---|\n");
    let mut rows = Vec::new();
    for &(n, small) in &CURVES {
        let inst = bench_instance(n)?;
        let pc = pair_counts(&inst, small, random, seed);
        let mut sorted = pc.random.clone();
        sorted.sort_unstable();
        md.push_str(&format!(
            "| {n} | `{}` | {small} | {} | {} | {} (min {}, max {}) | {:.2} |\n",
            poly_string(modulus_of(&inst)),
            pc.large_elements,
            pc.prefix,
            sorted
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(", "),
            sorted.first().copied().unwrap_or(0),
            sorted.last().copied().unwrap_or(0),
            pc.expected
        ));
        rows.push(json!({
            "n": n, "small": small, "modulus": modulus_of(&inst),
            "large_elements": pc.large_elements, "prefix_pairs": pc.prefix,
            "random_pairs": pc.random, "expected_random": pc.expected,
        }));
    }
    all.push(("window_pairs", Value::Array(rows)));

    md.push_str("\n### Table S2. Mechanism spot checks (the bench's sparse modulus)\n\n");
    md.push_str("| n | modulus | alpha^(2^5) | alpha^n | note |\n|---|---|---|---|---|\n");
    let mut mech = Vec::new();
    for &(n, _) in &CURVES {
        let inst = bench_instance(n)?;
        let f = &inst.fast.field;
        let a32 = f.sqr_k(2, 5);
        let mut alpha_n = 1u64;
        for _ in 0..n {
            alpha_n = f.mul(alpha_n, 2);
        }
        let note = if inst.n == 31 || inst.n == 29 {
            "n+1 = 32 = 2^5 would make the low-degree window nearly closed under pi^5"
        } else {
            ""
        };
        md.push_str(&format!(
            "| {n} | `{}` | `{}` | `{}` | {note} |\n",
            poly_string(modulus_of(&inst)),
            poly_string(a32),
            poly_string(alpha_n)
        ));
        mech.push(json!({"n": n, "alpha_2pow5": a32, "alpha_n": alpha_n}));
    }
    {
        let inst = bench_instance(31)?;
        let f = &inst.fast.field;
        let a = f.sqr_k(256, 26);
        let b = f.sqr_k(300, 12);
        md.push_str(&format!(
            "\nSpot checks at n = 31: `256^(2^26) = {a}` (reviewer: 258), `300^(2^12) = {b}` (reviewer: 558).\n"
        ));
        mech.push(json!({"n": 31, "256^(2^26)": a, "300^(2^12)": b}));
    }
    all.push(("mechanism", Value::Array(mech)));

    md.push_str("\n### Table S3. Merges of the measured large-point tables, split by cause (bench modulus)\n\n");
    md.push_str("`raw` = members that are exact Frobenius images of each other on the raw abscissae (`x1 = x0^(2^m)`); `projected only` = conjugate only after the cofactor projection. The recomputed `L`, orbits and merged points are compared with the seed-1 receipt of the merged run.\n\n");
    md.push_str("| n | L | orbits | merged | raw | projected only | multi-member orbits | largest orbit | E[pairs] random model | receipt (L, orbits, merged) | match |\n|---|---|---|---|---|---|---|---|---|---|---|\n");
    let mut merges = Vec::new();
    let mut largest_orbit_text = String::new();
    for &(n, small) in &CURVES {
        let inst = bench_instance(n)?;
        let (_, table, largest) = merge_table(&inst, small);
        let cap = CAPS.iter().find(|c| c.0 == n).map_or(0, |c| c.1);
        let receipt = receipts.as_deref().and_then(|d| receipt_orbit(d, n, cap));
        let recomputed = (
            table["large_points"].as_u64().unwrap_or(0),
            table["orbits"].as_u64().unwrap_or(0),
            table["merged"].as_u64().unwrap_or(0),
        );
        let verdict = match receipt {
            Some(r) if r == recomputed => "equal".to_string(),
            Some(r) => format!("DIFFERENT: receipt {r:?}"),
            None => "no receipt given".to_string(),
        };
        md.push_str(&format!(
            "| {n} | {} | {} | {} | {} | {} | {} | {} | {:.1} | {} | {verdict} |\n",
            table["large_points"],
            table["orbits"],
            table["merged"],
            table["raw_frobenius_merges"],
            table["projected_only_merges"],
            table["multi_member_orbits"],
            table["largest_orbit_members"],
            table["random_model_expected_pairs"].as_f64().unwrap_or(0.0),
            receipt.map_or("-".to_string(), |r| format!("{}, {}, {}", r.0, r.1, r.2)),
        ));
        if n == 31 {
            largest_orbit_text =
                format!("\nLargest orbit at n = 31 (raw abscissae of its members): {largest:?}\n");
        }
        let mut entry = table.clone();
        entry["n"] = json!(n);
        entry["receipt"] = json!(receipt);
        entry["match"] = json!(verdict);
        merges.push(entry);
    }
    md.push_str(&largest_orbit_text);
    all.push(("merges_bench_modulus", Value::Array(merges)));

    md.push_str("\n### Table S4. The same abstract curve over a seeded random (dense) modulus\n\n");
    md.push_str("Same group, `r` and cofactor; the prefix window `x < 2^10` is a different `F_2`-subspace of the field. `prefix pairs` as in S1.\n\n");
    md.push_str("| n | random modulus | terms | prefix pairs | L | orbits | merged | raw | projected only | E[pairs] random model |\n|---|---|---|---|---|---|---|---|---|---|\n");
    let mut dense = Vec::new();
    for &(n, small) in &CURVES {
        let sparse = bench_instance(n)?;
        let modulus = random_modulus(n, 1);
        let inst = instance_with_modulus(sparse.a, n, modulus, sparse.group_order, sparse.r)?;
        let pc = pair_counts(&inst, small, 0, seed);
        let (_, table, _) = merge_table(&inst, small);
        md.push_str(&format!(
            "| {n} | `{}` | {} | {} | {} | {} | {} | {} | {} | {:.1} |\n",
            poly_string(modulus),
            modulus.count_ones(),
            pc.prefix,
            table["large_points"],
            table["orbits"],
            table["merged"],
            table["raw_frobenius_merges"],
            table["projected_only_merges"],
            table["random_model_expected_pairs"].as_f64().unwrap_or(0.0),
        ));
        let mut entry = table.clone();
        entry["n"] = json!(n);
        entry["modulus"] = json!(modulus);
        entry["prefix_pairs"] = json!(pc.prefix);
        dense.push(entry);
    }
    all.push(("merges_random_modulus", Value::Array(dense)));

    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    fs::write(out.join("structure.md"), &md).map_err(|e| e.to_string())?;
    let doc: serde_json::Map<String, Value> =
        all.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    fs::write(
        out.join("structure.json"),
        serde_json::to_string_pretty(&Value::Object(doc)).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    print!("{md}");
    eprintln!("wrote structure.md and structure.json in {}", out.display());
    Ok(())
}

/// The instance's modulus as a bit mask.
fn modulus_of(inst: &BinaryInstance) -> u64 {
    inst.irreducible
        .low_terms
        .iter()
        .fold(1u64 << inst.n, |m, &t| m | (1u64 << t))
}

/// Which small Koblitz curves have merged and absorbable large points at a
/// given window: used to pick a fast configuration for the unit tests.
fn scan(args: &[String]) -> Result<(), String> {
    let small = num(args, "--small", 5)? as u32;
    let env = num(args, "--env", 10)? as u32;
    let lo = num(args, "--n-min", 9)? as u32;
    let hi = num(args, "--n-max", 31)? as u32;
    println!("n a r cofactor L merged absorbable small_points");
    for n in lo..=hi {
        for a in [0u8, 1] {
            let Some(inst) = koblitz_instance(a, n) else {
                continue;
            };
            if env > n || small > env {
                continue;
            }
            // Same specification as the library, at this window.
            let base = oracle_base(&inst, small, env);
            let counts = brute_force_counts(&inst, &base);
            println!(
                "{n} {a} {} {} {} {} {} {}",
                inst.r,
                inst.cofactor,
                counts.large_points,
                counts.merged,
                counts.absorbed,
                base.small.len()
            );
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("report") => report(&args),
        Some("scan") => scan(&args),
        _ => Err("usage: orbit_window_structure report|scan ...".into()),
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(2);
    }
}
