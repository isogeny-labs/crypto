//! Independent validator recomputation for research/jv_cover_fbline_20261102.
//!
//! Written by the Validator from the record schema and the study's stated
//! definitions; it does not reuse the producer's `fit/` or the reference
//! study's `negrho_rescore` code (different RNG, percentile rule, loader).
//!
//! usage: fbline_validator_check <repo_root> <out_file>
use serde_json::Value;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const PRIMES: [u64; 3] = [1301, 1451, 1601];
const SEEDS: [u64; 3] = [1, 2, 3];
const NB: usize = 20000;
const N_LINES: u64 = 65536;

// ---------------------------------------------------------------- RNG
/// xoshiro256** seeded through splitmix64 (not the producer's stream).
struct Xo([u64; 4]);
impl Xo {
    fn new(seed: u64) -> Self {
        let mut s = seed;
        let mut st = [0u64; 4];
        for x in st.iter_mut() {
            s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = s;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            *x = z ^ (z >> 31);
        }
        Xo(st)
    }
    fn next(&mut self) -> u64 {
        let r = self.0[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.0[1] << 17;
        self.0[2] ^= self.0[0];
        self.0[3] ^= self.0[1];
        self.0[1] ^= self.0[2];
        self.0[0] ^= self.0[3];
        self.0[2] ^= t;
        self.0[3] = self.0[3].rotate_left(45);
        r
    }
    /// Uniform in `0..n` by rejection (no modulo bias).
    fn below(&mut self, n: usize) -> usize {
        let n64 = n as u64;
        let zone = u64::MAX - (u64::MAX % n64);
        loop {
            let x = self.next();
            if x < zone {
                return (x % n64) as usize;
            }
        }
    }
}

// ---------------------------------------------------------------- stats
fn gmean(x: &[f64]) -> f64 {
    (x.iter().map(|v| v.ln()).sum::<f64>() / x.len() as f64).exp()
}
fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}
/// Type-7 (linear interpolation) quantile of an unsorted sample.
fn quantile(v: &[f64], q: f64) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let h = (s.len() - 1) as f64 * q;
    let lo = h.floor() as usize;
    let hi = (lo + 1).min(s.len() - 1);
    s[lo] + (h - lo as f64) * (s[hi] - s[lo])
}
fn verdict(lo: f64, hi: f64) -> &'static str {
    if hi < 1.0 {
        "below"
    } else if lo > 1.0 {
        "above"
    } else {
        "not resolved"
    }
}

// ---------------------------------------------------------------- IO
fn arr(path: &Path) -> Vec<Value> {
    let s = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let v: Value = serde_json::from_str(&s).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    v.as_array().cloned().unwrap_or_else(|| panic!("{}: not an array", path.display()))
}
fn one(path: &Path) -> Value {
    let a = arr(path);
    assert_eq!(a.len(), 1, "{}: expected one record", path.display());
    a[0].clone()
}
fn gu(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap_or_else(|| panic!("missing u64 {k}"))
}
fn gf(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("missing f64 {k}"))
}
fn gb(v: &Value, k: &str) -> bool {
    v[k].as_bool().unwrap_or_else(|| panic!("missing bool {k}"))
}
/// S in group additions: total F_p mults / (c_add_e * sqrt(l)).
fn s_of(v: &Value) -> f64 {
    gu(v, "total_muls") as f64 / (gf(v, "c_add_e") * (gu(v, "l") as f64).sqrt())
}
fn phase_sum(v: &Value) -> u64 {
    gu(v, "setup_muls")
        + gu(v, "base_muls")
        + gu(v, "stream_muls")
        + gu(v, "oracle_muls")
        + gu(v, "verify_muls")
        + gu(v, "la_muls")
}
fn oracle_sum(v: &Value) -> u64 {
    gu(v, "weil_muls") + gu(v, "f4_muls") + gu(v, "lin_muls") + gu(v, "post_muls")
}
/// Expected count of the search's charged F_p multiplications, derived by the
/// Validator from the code's counters: square table (p-1)/2; plain line
/// 17p (Horner 2*7 + norm 3 per s) + 5 (a = t*t); each of the n-1 random lines
/// 17p + 5 (t*t) + 280 (line_poly Horner: 2*(1+..+7) E2 products * 5)
/// + 21 (inverse of lc) + 40 (8 E2 scalings); moving G and Q: 2 * 187
/// (u: 60 + 21 + 20; v: 30 + 20 (t^2, t^7) + 21 + 15) for degree-3 divisors.
fn predicted_search_muls(p: u64, n: u64) -> u64 {
    (p - 1) / 2 + (17 * p + 5) + (n - 1) * (17 * p + 5 + 280 + 21 + 40) + 2 * 187
}

struct Ref {
    p: u64,
    seed: u64,
    u1: f64,
    u2: f64,
    nat: f64, // native ops / sqrt(l)
    fpa: f64, // F_p additions / (c_add_e sqrt(l)), in S units at 1 mult-equivalent per addition
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let root = PathBuf::from(&a[1]);
    let out = PathBuf::from(&a[2]);
    let da = root.join("research/jv_cover_crossover_20261101/results");
    let db = root.join("research/jv_cover_fbline_20261102/results");
    let dr = root.join("research/jv_cover_negrho_rescore_20261101/results");
    let mut t = String::new();
    let mut fails: Vec<String> = vec![];
    macro_rules! say { ($($x:tt)*) => {{ let s = format!($($x)*); println!("{s}"); t += &s; t += "\n"; }} }
    macro_rules! chk { ($ok:expr, $($x:tt)*) => {{ let ok: bool = $ok; let s = format!($($x)*); if !ok { fails.push(s.clone()); } say!("  [{}] {}", if ok { "ok" } else { "FAIL" }, s); }} }

    // ============================================================ 1. receipts
    say!("== 1. Receipts: arm A (crossover) vs arm B (line search), field-level checks");
    let mut rows: Vec<(u64, u64, Value, Value)> = vec![];
    let mut zero_skips: i64 = 0;
    let mut exp_skips: f64 = 0.0;
    for &p in &PRIMES {
        for &s in &SEEDS {
            let ra = one(&da.join(format!("dlp_p{p}_s{s}.json")));
            let rb = one(&db.join(format!("line_p{p}_s{s}.json")));
            say!("p={p} seed={s}");
            chk!(gu(&ra, "p") == p && gu(&ra, "seed") == s && gu(&rb, "p") == p && gu(&rb, "seed") == s, "p/seed fields match file names");
            chk!(ra["fb_line"].is_null() && !rb["fb_line"].is_null(), "A has fb_line null, B has fb_line");
            chk!(gb(&ra, "solved") && gb(&ra, "correct"), "A solved and correct");
            chk!(gb(&rb, "solved") && gb(&rb, "correct"), "B solved and correct");
            chk!(gu(&rb, "mismatches") == 0 && gu(&rb, "timed_out") == 0, "B mismatches 0, timed_out 0");
            say!("  B incomplete={} A incomplete={} (tests not zero-dim/truncated: can only lose relations)", gu(&rb, "incomplete"), gu(&ra, "incomplete"));
            chk!(gu(&ra, "l") == gu(&rb, "l"), "l equal: {}", gu(&rb, "l"));
            chk!(gf(&ra, "c_add_e") == gf(&rb, "c_add_e"), "c_add_e equal: {}", gf(&rb, "c_add_e"));
            say!("  c_add_j A {} B {}", gf(&ra, "c_add_j"), gf(&rb, "c_add_j"));
            chk!(ra["stop_mode"] == "full" && rb["stop_mode"] == "full", "stop_mode full on both");
            let fl = &rb["fb_line"];
            chk!(gu(fl, "candidates") == N_LINES, "fb_line.candidates = {}", gu(fl, "candidates"));
            chk!(gu(fl, "b_plain") == gu(&ra, "base"), "fb_line.b_plain {} = A base {}", gu(fl, "b_plain"), gu(&ra, "base"));
            chk!(gu(fl, "b_best") == gu(&rb, "base"), "fb_line.b_best {} = B base {}", gu(fl, "b_best"), gu(&rb, "base"));
            chk!(gu(fl, "best_index") > 0 && gu(fl, "b_best") > gu(fl, "b_plain"), "best line is a moved line (index {})", gu(fl, "best_index"));
            for (nm, r) in [("A", &ra), ("B", &rb)] {
                chk!(phase_sum(r) == gu(r, "total_muls"), "{nm} total_muls {} = setup+base+stream+oracle+verify+la {}", gu(r, "total_muls"), phase_sum(r));
                chk!(oracle_sum(r) == gu(r, "oracle_muls"), "{nm} oracle = weil+f4+lin+post");
                chk!(gu(r, "la_muls") == 16 * gu(r, "la_ops"), "{nm} la_muls = 16*la_ops");
                let sc = s_of(r);
                chk!(((sc - gf(r, "s")) / gf(r, "s")).abs() < 1e-12, "{nm} S recomputed {sc:.6} = recorded {:.6}", gf(r, "s"));
                let rate = gu(r, "relations") as f64 / gu(r, "residuals") as f64;
                chk!(((rate - gf(r, "decomposition_rate")) / rate).abs() < 1e-12, "{nm} decomposition_rate = relations/residuals");
            }
            let fm = gu(fl, "muls");
            let sb = gu(&rb, "setup_muls");
            chk!(sb >= fm, "B setup_muls {} contains fb_line.muls {}", sb, fm);
            say!("  B setup minus search = {} vs A setup {} (diff {}: stream start/step on the moved model, different rng draws)", sb - fm, gu(&ra, "setup_muls"), (sb - fm) as i64 - gu(&ra, "setup_muls") as i64);
            let pred = predicted_search_muls(p, N_LINES);
            let dlt = fm as i64 - pred as i64;
            // pmul skips a zero coefficient of its left factor: each skip removes
            // exactly 2 E2 products (10 F_p mults) that were not executed either.
            chk!(dlt <= 0 && dlt % 10 == 0, "fb_line.muls {} vs validator-derived count {} (diff {dlt}: {} pmul zero-coefficient skip(s) of 10 mults; never above the executed work)", fm, pred, -dlt / 10);
            zero_skips += -dlt / 10;
            exp_skips += (N_LINES - 1) as f64 * 21.0 / (p as f64 * p as f64);
            say!("  search share of B total: {:.4}%", 100.0 * fm as f64 / gu(&rb, "total_muls") as f64);
            let bb = gu(&rb, "base") as f64;
            let model = (2.0 * bb / p as f64).powi(6) / 720.0;
            let c4 = gf(&rb, "decomposition_rate") / model;
            chk!((0.8..=1.25).contains(&c4), "C4 rate_B/((2B/p)^6/720) = {c4:.4}");
            rows.push((p, s, ra, rb));
        }
    }

    say!("pmul zero-coefficient skips over the 9 runs: observed {zero_skips}, expected about {exp_skips:.2} (21 non-leading Horner coefficients per moved line, each zero w.p. 1/p^2)");

    // ============================================================ controls C1, C2
    say!("\n== Controls C1, C2 (every field compared)");
    let a13 = one(&da.join("dlp_p1301_s3.json"));
    let c1 = one(&db.join("c1_plain_p1301_s3.json"));
    let c2 = one(&db.join("c2_line1_p1301_s3.json"));
    let timing = ["wall_ms", "f4_ms", "lin_ms"];
    let diff_keys = |x: &Value, y: &Value| -> Vec<String> {
        let (xo, yo) = (x.as_object().unwrap(), y.as_object().unwrap());
        let mut d: Vec<String> = xo.keys().chain(yo.keys()).filter(|k| xo.get(*k) != yo.get(*k)).cloned().collect();
        d.sort();
        d.dedup();
        d
    };
    let d1 = diff_keys(&a13, &c1);
    let d1n: Vec<&String> = d1.iter().filter(|k| !timing.contains(&k.as_str())).collect();
    say!("C1 vs crossover p1301 s3: differing keys {:?}", d1);
    say!("  crossover record has fb_line key: {} (C1: {}); f4_muls_per_test_sum A {} C1 {} (multi-thread diagnostic, not in S; f4_muls equal: {})",
        a13.get("fb_line").is_some(), c1["fb_line"], a13["f4_muls_per_test_sum"], c1["f4_muls_per_test_sum"], a13["f4_muls"] == c1["f4_muls"]);
    chk!(d1n.iter().all(|k| k.as_str() == "f4_muls_per_test_sum" || (k.as_str() == "fb_line" && a13.get("fb_line").is_none() && c1["fb_line"].is_null())),
        "C1: every deterministic field equal (diffs only timing, the racy f4_muls_per_test_sum, and the new fb_line=null key absent from the older schema)");
    for k in ["residuals", "relation_digest", "total_muls", "decompositions"] {
        chk!(a13[k] == c1[k], "C1 {k} equal: {}", c1[k]);
    }
    let d2 = diff_keys(&c1, &c2);
    say!("C2 vs C1: differing keys {:?}", d2);
    let lm = gu(&c2["fb_line"], "muls");
    chk!(c1["residuals"] == c2["residuals"] && c1["relation_digest"] == c2["relation_digest"] && c1["decompositions"] == c2["decompositions"], "C2 residuals/digest/decompositions equal to C1");
    chk!(gu(&c2, "total_muls") - gu(&c1, "total_muls") == lm && gu(&c2, "setup_muls") - gu(&c1, "setup_muls") == lm, "C2 total and setup exceed C1 by exactly fb_line.muls {lm}");
    let pred1 = (1301 - 1) / 2 + 17 * 1301 + 5;
    chk!(lm == pred1, "C2 fb_line.muls {lm} = derived (p-1)/2 + 17p + 5 = {pred1}");
    let allowed2 = ["fb_line", "setup_muls", "total_muls", "s", "s_over_rho", "relation_over_rho", "f4_muls_per_test_sum", "wall_ms", "f4_ms", "lin_ms"];
    let extra2: Vec<&String> = d2.iter().filter(|k| !allowed2.contains(&k.as_str())).collect();
    chk!(extra2.is_empty(), "C2: no other field differs ({:?})", extra2);

    // ============================================================ 3. paired recomputation
    say!("\n== 3. Paired recomputation");
    say!("| p | seed | B_plain | B_best | S_A | S_B | S_B/S_A | (B_plain/B_best)^5 |");
    let mut r = vec![];
    let mut pr = vec![];
    let (mut sa_all, mut sb_all) = (vec![], vec![]);
    for (p, s, ra, rb) in &rows {
        let (sa, sb) = (s_of(ra), s_of(rb));
        let (bp, bb) = (gu(ra, "base") as f64, gu(rb, "base") as f64);
        let m = (bp / bb).powi(5);
        say!("| {p} | {s} | {bp} | {bb} | {sa:.4} | {sb:.4} | {:.4} | {m:.4} |", sb / sa);
        r.push(sb / sa);
        pr.push(m);
        sa_all.push(sa);
        sb_all.push(sb);
    }
    let g = gmean(&r);
    let mut rng = Xo::new(0x7A11_DA7E_F8B1_0002);
    let bs: Vec<f64> = (0..NB)
        .map(|_| {
            let v: Vec<f64> = (0..r.len()).map(|_| r[rng.below(r.len())]).collect();
            gmean(&v)
        })
        .collect();
    let (lo, hi) = (quantile(&bs, 0.025), quantile(&bs, 0.975));
    say!("geo-mean S_B/S_A = {g:.4} [{lo:.4}, {hi:.4}] (validator bootstrap, {NB}, xoshiro, type-7 quantiles); speed-up x{:.3} [{:.3}, {:.3}]", 1.0 / g, 1.0 / hi, 1.0 / lo);
    say!("model geo-mean (B_plain/B_best)^5 = {:.4}", gmean(&pr));
    let all_lower = r.iter().all(|x| *x < 1.0);
    chk!(hi < 1.0 && all_lower && r.len() == 9, "H1 decision rule (upper end < 1 and every S_B < S_A) holds");
    let spread = |v: &[f64]| v.iter().cloned().fold(f64::MIN, f64::max) / v.iter().cloned().fold(f64::MAX, f64::min);
    say!("spread max/min: S_A {:.3}x, S_B {:.3}x", spread(&sa_all), spread(&sb_all));
    // log-scale correlation of measured vs model ratio
    let lr: Vec<f64> = r.iter().map(|x| x.ln()).collect();
    let lm_: Vec<f64> = pr.iter().map(|x| x.ln()).collect();
    let (mx, my) = (mean(&lm_), mean(&lr));
    let cov: f64 = lm_.iter().zip(&lr).map(|(x, y)| (x - mx) * (y - my)).sum();
    let vx: f64 = lm_.iter().map(|x| (x - mx).powi(2)).sum();
    let vy: f64 = lr.iter().map(|y| (y - my).powi(2)).sum();
    say!("log(measured) vs log(model): Pearson r = {:.4}, slope = {:.4}", cov / (vx * vy).sqrt(), cov / vx);

    // ============================================================ reference runs
    say!("\n== Reference: negation-map rhostrong runs of #3278");
    let mut refs: Vec<Ref> = vec![];
    let mut files: Vec<PathBuf> = vec![];
    for sub in ["inputs/old_rhostrong", "new"] {
        let mut f: Vec<PathBuf> = std::fs::read_dir(dr.join(sub)).unwrap().map(|e| e.unwrap().path())
            .filter(|p| p.extension().map_or(false, |e| e == "json")).collect();
        f.sort();
        files.extend(f);
    }
    let mut excluded = 0;
    let mut identity_fail = 0;
    let mut max_rel_harness: f64 = 0.0;
    let mut max_rel_tuned: f64 = 0.0;
    for f in &files {
        for x in arr(f) {
            let p = gu(&x, "p");
            if !PRIMES.contains(&p) {
                continue;
            }
            let ok = gb(&x, "correct") && !gb(&x, "capped") && gb(&x, "negation") && gu(&x, "batch") == 16 && gu(&x, "r") == 256 && gu(&x, "dp_bits") == 16;
            if !ok {
                excluded += 1;
                continue;
            }
            let seed = gu(&x, "seed");
            // identity with the cover instance
            let cov = rows.iter().find(|w| w.0 == p && w.1 == seed).expect("instance");
            if gu(&x, "l") != gu(&cov.3, "l") || gf(&x, "c_add_e") != gf(&cov.3, "c_add_e") {
                identity_fail += 1;
            }
            let sl = (gu(&x, "l") as f64).sqrt();
            let ce = gf(&x, "c_add_e");
            let h = gf(&x, "muls_harness") / (ce * sl);
            let tu = gf(&x, "muls_tuned") / (ce * sl);
            max_rel_harness = max_rel_harness.max(((h - gf(&x, "s_harness")) / h).abs());
            max_rel_tuned = max_rel_tuned.max(((tu - gf(&x, "s_tuned")) / tu).abs());
            let o = &x["ops"];
            let nat = ["canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp"].iter().map(|k| gu(o, k) as f64).sum::<f64>() / sl;
            let fpa = gu(o, "fp_adds") as f64 / (ce * sl);
            refs.push(Ref { p, seed, u1: gf(&x, "s_harness"), u2: gf(&x, "s_tuned"), nat, fpa });
        }
    }
    chk!(refs.len() == 126 && excluded == 0, "126 reference runs at p in {{1301,1451,1601}} admitted, {excluded} excluded");
    for &p in &PRIMES {
        for &s in &SEEDS {
            let n = refs.iter().filter(|x| x.p == p && x.seed == s).count();
            chk!(n == 14, "p={p} seed={s}: {n} reference runs");
        }
    }
    chk!(identity_fail == 0, "l and c_add_e of every reference run equal the cover instance's");
    chk!(max_rel_harness < 1e-12 && max_rel_tuned < 1e-12, "s_harness/s_tuned recompute from muls_*/(c_add_e sqrt l): max rel err {max_rel_harness:.2e} / {max_rel_tuned:.2e}");
    let u1v: Vec<f64> = refs.iter().map(|x| x.u1).collect();
    let u2v: Vec<f64> = refs.iter().map(|x| x.u2).collect();
    let natv: Vec<f64> = refs.iter().map(|x| x.nat).collect();
    let (ru1, ru2, rnat) = (mean(&u1v), mean(&u2v), mean(&natv));
    say!("pooled (b): n {} mean s_harness {ru1:.4} mean s_tuned {ru2:.4} native/sqrt(l) {rnat:.2}", refs.len());
    chk!((ru1 * 1e4).round() == 9322.0 && (ru2 * 1e4).round() == 3879.0, "pooled reference means agree with 0.9322 / 0.3879 to 4 decimals");
    for &p in &PRIMES {
        let a1: Vec<f64> = refs.iter().filter(|x| x.p == p).map(|x| x.u1).collect();
        let a2: Vec<f64> = refs.iter().filter(|x| x.p == p).map(|x| x.u2).collect();
        say!("  per-prime (a) p={p}: n {} s_harness {:.4} s_tuned {:.4}", a1.len(), mean(&a1), mean(&a2));
    }

    // ============================================================ re-score
    say!("\n== Re-score per prime (estimator (b) pooled, geo-mean over 3 seeds / reference mean; joint bootstrap seeds x refs, {NB})");
    let mut rng2 = Xo::new(0x7A11_DA7E_F8B1_0003);
    let pts = |arm: usize, w: f64, extra: bool, rng: &mut Xo| -> Vec<(u64, f64, f64, f64, f64, f64)> {
        // returns per prime (p, S_cover, U1, U1lo, U1hi, ...) -> we pack U1 and U2 in two calls
        let mut o = vec![];
        for &p in &PRIMES {
            let sv: Vec<f64> = rows.iter().filter(|x| x.0 == p).map(|x| {
                let rec = if arm == 0 { &x.2 } else { &x.3 };
                let sl = (gu(rec, "l") as f64).sqrt();
                let mut nat = gu(rec, "f4_native_ops") as f64;
                if extra && arm == 1 {
                    nat += (N_LINES * p) as f64; // the search's square-table lookups, unemitted
                }
                s_of(rec) + w * nat / sl
            }).collect();
            let r1: Vec<f64> = refs.iter().map(|x| x.u1 + w * x.nat).collect();
            let r2: Vec<f64> = refs.iter().map(|x| x.u2 + w * x.nat).collect();
            let gs = gmean(&sv);
            let (p1, p2) = (gs / mean(&r1), gs / mean(&r2));
            let mut b1 = Vec::with_capacity(NB);
            let mut b2 = Vec::with_capacity(NB);
            for _ in 0..NB {
                let sres: Vec<f64> = (0..3).map(|_| sv[rng.below(3)]).collect();
                let idx: Vec<usize> = (0..r1.len()).map(|_| rng.below(r1.len())).collect();
                let m1 = idx.iter().map(|&i| r1[i]).sum::<f64>() / idx.len() as f64;
                let m2 = idx.iter().map(|&i| r2[i]).sum::<f64>() / idx.len() as f64;
                let gsr = gmean(&sres);
                b1.push(gsr / m1);
                b2.push(gsr / m2);
            }
            o.push((p, gs, p1, quantile(&b1, 0.025), quantile(&b1, 0.975), p2));
            o.push((p, gs, p2, quantile(&b2, 0.025), quantile(&b2, 0.975), 0.0));
        }
        o
    };
    let claimed_u1 = [0.791, 0.659, 0.548];
    let claimed_u2 = [1.900, 1.585, 1.318];
    let before_u1 = [1.405, 1.160, 0.870];
    let before_u2 = [3.376, 2.787, 2.090];
    let mut u1_pts = [0.0; 3];
    let mut u2_pts = [0.0; 3];
    for (arm, name) in [(1usize, "arm B (line search)"), (0usize, "arm A (plain; consistency with #3278)")] {
        say!("{name}:");
        let o = pts(arm, 0.0, false, &mut rng2);
        for (i, &p) in PRIMES.iter().enumerate() {
            let (u1, u2) = (&o[2 * i], &o[2 * i + 1]);
            say!("  p={p}: geo-mean S {:.4} | U1 S/rho {:.4} [{:.3}, {:.3}] {} | U2 S/rho {:.4} [{:.3}, {:.3}] {}",
                u1.1, u1.2, u1.3, u1.4, verdict(u1.3, u1.4), u2.2, u2.3, u2.4, verdict(u2.3, u2.4));
            let (c1x, c2x) = if arm == 1 { (claimed_u1[i], claimed_u2[i]) } else { (before_u1[i], before_u2[i]) };
            chk!((u1.2 * 1e3).round() / 1e3 == c1x && (u2.2 * 1e3).round() / 1e3 == c2x, "p={p} {name}: U1 {:.3} / U2 {:.3} equal claimed {c1x} / {c2x} to 3 decimals", u1.2, u2.2);
            if arm == 1 {
                u1_pts[i] = u1.2;
                u2_pts[i] = u2.2;
            }
        }
    }

    // ============================================================ 5a band
    say!("\n== Section 5a band (w GAE per native op; cover native = f4_native_ops, lower bound)");
    for extra in [false, true] {
        say!("{}", if extra { "variant: + the search's 65536*p square-table lookups as cover native ops (validator addition)" } else { "as specified (f4_native_ops only)" });
        for w in [0.0, 0.1, 1.0] {
            let o = pts(1, w, extra, &mut rng2);
            let mut line = format!("  w={w}:");
            for (i, &p) in PRIMES.iter().enumerate() {
                let (u1, u2) = (&o[2 * i], &o[2 * i + 1]);
                write!(line, " p={p} U1 {:.3} [{:.3},{:.3}] {} / U2 {:.3} [{:.3},{:.3}] {};", u1.2, u1.3, u1.4, verdict(u1.3, u1.4), u2.2, u2.3, u2.4, verdict(u2.3, u2.4)).unwrap();
            }
            say!("{line}");
        }
    }

    // ============================================================ crossover model
    say!("\n== Crossover p* (model, slope -2.10 not refit; G = geo-mean of S/rho_i * p_i^2.10)");
    for (name, v) in [("U1", u1_pts), ("U2", u2_pts)] {
        let lg = PRIMES.iter().zip(v.iter()).map(|(p, x)| x.ln() + 2.10 * (*p as f64).ln()).sum::<f64>() / 3.0;
        say!("  {name}: p* = {:.0}", (lg / 2.10).exp());
    }

    // ============================================================ F_p additions sensitivity (assumption)
    say!("\n== Sensitivity of the U1 'below' verdicts to the cover's unemitted F_p additions (ASSUMPTION: k additions per counted F_p mult on the cover; rho's fp_adds are emitted)");
    let fpa_ref = mean(&refs.iter().map(|x| x.fpa).collect::<Vec<f64>>());
    say!("  reference fp_adds in S units (per c_add_e*sqrt l): {fpa_ref:.4}");
    for (i, &p) in PRIMES.iter().enumerate() {
        let sp = u1_pts[i] * ru1; // geo-mean S_B at p
        for k in [0.5f64, 1.0] {
            let a_star = (ru1 - sp) / (sp * k - fpa_ref);
            say!("  p={p}: S_B {sp:.4}; with k={k}: U1 point estimate reaches 1 when an F_p addition costs alpha* = {a_star:.3} F_p mults");
        }
    }

    // ============================================================ gain vs p (model check of the extrapolation)
    say!("\n== Line-search gain vs p (model): B_best = p/2 + k*sqrt(p)/2, gain g(p) = (1 + k/sqrt p)^-5 against a mean plain line");
    let mut ks = vec![];
    let mut kp = vec![];
    for (p, _s, ra, rb) in &rows {
        let half = *p as f64 / 2.0;
        let sd = (*p as f64).sqrt() / 2.0;
        ks.push((gu(rb, "base") as f64 - half) / sd);
        kp.push((gu(ra, "base") as f64 - half) / sd);
    }
    let kbar = mean(&ks);
    let kpm = mean(&kp);
    let kps = (kp.iter().map(|x| (x - kpm).powi(2)).sum::<f64>() / (kp.len() - 1) as f64).sqrt();
    say!("  B_best z-scores: {:?}; mean k = {kbar:.3}", ks.iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>());
    say!("  B_plain z-scores: mean {kpm:.3}, sd {kps:.3} (sd 1 expected if sigma_B = sqrt(p)/2)");
    let g = |p: f64| (1.0 + kbar / p.sqrt()).powi(-5);
    for p in [1301.0f64, 1451.0, 1601.0, 1799.0, 1850.0, 2039.0, 4096.0, 65536.0] {
        say!("  p={p}: g(p) = {:.4}, local d ln g / d ln p = {:+.3}", g(p), 2.5 * kbar / (p.sqrt() + kbar));
    }
    for (name, v) in [("U1", u1_pts), ("U2", u2_pts)] {
        let lgb = PRIMES.iter().zip(v.iter()).map(|(p, x)| x.ln() + 2.10 * (*p as f64).ln() - g(*p as f64).ln()).sum::<f64>() / 3.0;
        let f = |p: f64| lgb - 2.10 * p.ln() + g(p).ln();
        let (mut lo, mut hi) = (500.0f64, 10000.0f64);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if f(mid) > 0.0 { lo = mid } else { hi = mid }
        }
        say!("  {name}: p* with the gain's p-dependence added to slope -2.10: {:.0}", 0.5 * (lo + hi));
    }

    // ============================================================ 5. replay
    if a.len() > 3 {
        say!("\n== 5. Replay comparison: {} vs committed results/line_p1301_s3.json", a[3]);
        let rp = one(Path::new(&a[3]));
        let orig = one(&db.join("line_p1301_s3.json"));
        let (xo, yo) = (orig.as_object().unwrap(), rp.as_object().unwrap());
        let mut keys: Vec<&String> = xo.keys().chain(yo.keys()).collect();
        keys.sort();
        keys.dedup();
        let mut differ = vec![];
        for k in &keys {
            if xo.get(*k) != yo.get(*k) {
                differ.push(k.to_string());
            }
        }
        say!("  fields compared: {}; differing: {:?}", keys.len(), differ);
        for k in ["total_muls", "residuals", "relation_digest", "decompositions", "relations", "base", "setup_muls", "f4_muls", "f4_native_ops", "la_ops", "solved", "correct"] {
            say!("  {k}: committed {} | replay {}", orig[k], rp[k]);
        }
        say!("  fb_line: committed {} | replay {}", orig["fb_line"], rp["fb_line"]);
        let det: Vec<&String> = differ.iter().filter(|k| !["wall_ms", "f4_ms", "lin_ms", "f4_muls_per_test_sum"].contains(&k.as_str())).collect();
        chk!(det.is_empty(), "replay equal in every deterministic field (allowed to differ: wall_ms, f4_ms, lin_ms, f4_muls_per_test_sum); deterministic diffs {:?}", det);
    }

    say!("\nfailures: {}", fails.len());
    for f in &fails {
        say!("  FAIL: {f}");
    }
    std::fs::write(&out, &t).unwrap();
}
