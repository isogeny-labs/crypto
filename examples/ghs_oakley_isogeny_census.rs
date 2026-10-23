//! Census of GHS magic numbers over the l-isogeny neighbourhood (l in {2,3})
//! of the IPsec Oakley EC groups 3 (GF(2^155)) and 4 (GF(2^185)).
//!
//! Structural only: the magic number is the exact GHS invariant of the
//! (a, b) representative reached; no DLP cost or transport is established.
//! The walk is j-invariant level (see `binary_isogeny`); both twists are
//! visited, and curves whose cover does not descend (MMT condition 8) are
//! counted separately.
use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::binary_isogeny::{curve_from_j, j_invariant, l_isogenous_neighbours};
use crypto_lib::cryptanalysis::ec_trapdoor::{magic_number_full, mmt_condition_8, FieldTower};
use crypto_lib::cryptanalysis::ghs_descent::ECurve;
use std::collections::{BTreeMap, HashSet, VecDeque};

fn census(name: &str, n_deg: u32, mid: u32, a: &str, b: &str, towers: &[u32], max_visited: usize) {
    let irr = IrreduciblePoly { degree: n_deg, low_terms: vec![0, mid] };
    let start = ECurve::new(
        n_deg,
        irr.clone(),
        F2mElement::from_hex(a, n_deg),
        F2mElement::from_hex(b, n_deg),
    );
    for &n in towers {
        let l = n_deg / n;
        let tower = FieldTower::new(n_deg, n, l, irr.clone());
        let mut hist: BTreeMap<u32, (usize, usize)> = BTreeMap::new();
        let mut seen: HashSet<Vec<u64>> = HashSet::new();
        let mut q = VecDeque::new();
        // Phi_2 = (X^2-Y)(X-Y^2) mod 2: 2-isogenies are Frobenius/Verschiebung,
        // so orbits are identified and only odd l moves between orbits.
        let canon = |e: &ECurve| -> Vec<u64> {
            let j = j_invariant(e);
            let mut best = j.raw_bits().to_vec();
            let mut c = j;
            for _ in 1..n_deg {
                c = c.square_k_times(1, &irr);
                if c.raw_bits().to_vec() < best { best = c.raw_bits().to_vec(); }
            }
            best
        };
        seen.insert(canon(&start));
        q.push_back((start.clone(), 0u32));
        let mut best: Option<(u32, u32)> = None;
        let mut visited = 0usize;
        let mut reps = 0usize;
        let mut depth_max = 0;
        while let Some((cur, d)) = q.pop_front() {
            if visited >= max_visited { break; }
            visited += 1;
            depth_max = depth_max.max(d);
            // Both twist representatives (a = 0 and a = 1) of the orbit's j.
            let jc = j_invariant(&cur);
            for a_choice in [0u8, 1u8] {
                let Some(rep) = curve_from_j(&jc, n_deg, &irr, a_choice) else { continue };
                let m = magic_number_full(&tower, &rep.a, &rep.b);
                let ok = mmt_condition_8(&tower, &rep.a, &rep.b);
                let e = hist.entry(m).or_insert((0, 0));
                if ok { e.0 += 1 } else { e.1 += 1 }
                reps += 1;
                if ok && best.map_or(true, |(bm, _)| m < bm) { best = Some((m, d)); }
            }
            for &ell in &[3u32, 5, 7, 11, 13] {
                for nb in l_isogenous_neighbours(&cur, ell) {
                    if seen.insert(canon(&nb)) {
                        q.push_back((nb, d + 1));
                    }
                }
            }
        }
        println!("{name} N={n_deg} tower n={n} l={l}: orbits_visited={visited} representatives={reps} (both twists) max_depth={depth_max} best(descending)={best:?}");
        for (m, (ok, bad)) in &hist { println!("  magic={m:>4}  descends={ok:>5}  no-descent={bad:>5}"); }
    }
}

/// Exponent-level log2 cost model for the extended GHS attack on
/// `E/F_(q^n)`, q = 2^l, target magic m. NOT a measured result.
/// - P(hit) per isogeny-class orbit ~ q^(m-n) (GHS fraction heuristic).
/// - walk: (n-m)*l + WALK_STEP_LOG2, where one l=3 orbit step measured
///   ~0.7 ms here, taken as 2^13 rho-step-equivalents (assumes 100 ns per
///   rho step: an UNMEASURED assumption, see the report).
/// - HCDLP, genus g = 2^(m-1): Gaudry g!*q^2 (constant 1, concrete) and
///   Nagao q^(2-2/g) (exponent only, hidden constant unknown: not a bound).
const WALK_STEP_LOG2: f64 = 13.0;

fn cost_model(name: &str, big_n: u32, n: u32, rho_log2: f64) {
    let l = (big_n / n) as f64;
    let lg = |g: f64| (1..=(g as u64)).map(|k| (k as f64).log2()).sum::<f64>();
    println!("cost model {name}: N={big_n} n={n} q=2^{l} rho~2^{rho_log2}");
    for m in 2..=n {
        let g = (1u64 << (m - 1)) as f64;
        let walk = (n - m) as f64 * l + WALK_STEP_LOG2;
        let gaudry = lg(g) + 2.0 * l;
        let nagao = (2.0 - 2.0 / g) * l;
        let total_g = walk.max(gaudry);
        println!("  m={m} g={g:>2} walk=2^{walk:>6.1} gaudry=2^{gaudry:>6.1} nagao(exp only)=2^{nagao:>5.1} total(gaudry)=2^{total_g:>6.1} {}",
            if total_g < rho_log2 { "below rho (constant-1 model)" } else { "above rho" });
    }
}

fn main() {
    cost_model("oakley3", 155, 5, 76.5);
    cost_model("oakley4", 185, 5, 91.5);
    let cap: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(300);
    census("oakley3", 155, 62, "0", "7338f", &[5, 31, 155], cap);
    census("oakley4", 185, 69, "0", "1ee9", &[5, 37, 185], cap);
}
