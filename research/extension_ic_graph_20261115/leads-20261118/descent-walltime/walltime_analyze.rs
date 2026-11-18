//! descent-walltime analysis (std-only Rust; no crates).
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/descent-walltime.
//! Every output is a producer observation or a model, [pcc:unverified].
//!
//!   walltime_analyze ab DIR SEED HOOKS ROUNDS
//!       A/B wall-time summary of run_ab.sh output (indicative, non-isolated).
//!   walltime_analyze counted DIR SEED N
//!       Complete counted descent work per arm from run_counted.sh output,
//!       with and without the learning-RREF bound, and the field_mul
//!       reconciliation that shows which named terms are inside the hooks.
//!   walltime_analyze run50 RUN50_DIR PROD_COUNTED_JSON TRACED_COUNTED_JSON
//!       MODEL ONLY: Run50 arm totals with descent normalised to the
//!       expected residual count (pooled success rate), not observed counts.
//!
//! JSON parser copied unchanged from descent-finder/descent_breakdown.rs.
#![allow(dead_code)]
#[allow(unused_imports)]
use std::collections::BTreeMap;
use std::fs;

#[derive(Clone, Debug)]
enum V {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<V>),
    Obj(Vec<(String, V)>),
}

struct P<'a> {
    s: &'a [u8],
    i: usize,
}

impl<'a> P<'a> {
    fn ws(&mut self) {
        while self.i < self.s.len() && (self.s[self.i] as char).is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn lit(&mut self, t: &str) -> Result<(), String> {
        if self.s[self.i..].starts_with(t.as_bytes()) {
            self.i += t.len();
            Ok(())
        } else {
            Err(format!("expected {t} at {}", self.i))
        }
    }
    fn string(&mut self) -> Result<String, String> {
        self.lit("\"")?;
        let mut out = Vec::new();
        while self.i < self.s.len() {
            let c = self.s[self.i];
            self.i += 1;
            match c {
                b'"' => return String::from_utf8(out).map_err(|e| e.to_string()),
                b'\\' => {
                    let e = self.s[self.i];
                    self.i += 1;
                    match e {
                        b'"' | b'\\' | b'/' => out.push(e),
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'u' => {
                            let h = std::str::from_utf8(&self.s[self.i..self.i + 4])
                                .map_err(|e| e.to_string())?;
                            let cp = u32::from_str_radix(h, 16).map_err(|e| e.to_string())?;
                            self.i += 4;
                            let ch = char::from_u32(cp).unwrap_or('\u{fffd}');
                            let mut b = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut b).as_bytes());
                        }
                        _ => return Err("bad escape".into()),
                    }
                }
                _ => out.push(c),
            }
        }
        Err("unterminated string".into())
    }
    fn value(&mut self) -> Result<V, String> {
        self.ws();
        let c = *self.s.get(self.i).ok_or("eof")?;
        let v = match c {
            b'{' => {
                self.i += 1;
                let mut m = Vec::new();
                self.ws();
                if self.s[self.i] == b'}' {
                    self.i += 1;
                    return Ok(V::Obj(m));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.ws();
                    self.lit(":")?;
                    let v = self.value()?;
                    m.push((k, v));
                    self.ws();
                    if self.s[self.i] == b',' {
                        self.i += 1;
                    } else {
                        self.lit("}")?;
                        break;
                    }
                }
                V::Obj(m)
            }
            b'[' => {
                self.i += 1;
                let mut a = Vec::new();
                self.ws();
                if self.s[self.i] == b']' {
                    self.i += 1;
                    return Ok(V::Arr(a));
                }
                loop {
                    a.push(self.value()?);
                    self.ws();
                    if self.s[self.i] == b',' {
                        self.i += 1;
                    } else {
                        self.lit("]")?;
                        break;
                    }
                }
                V::Arr(a)
            }
            b'"' => V::Str(self.string()?),
            b't' => {
                self.lit("true")?;
                V::Bool(true)
            }
            b'f' => {
                self.lit("false")?;
                V::Bool(false)
            }
            b'n' => {
                self.lit("null")?;
                V::Null
            }
            _ => {
                let st = self.i;
                while self.i < self.s.len() && b"+-0123456789.eE".contains(&self.s[self.i]) {
                    self.i += 1;
                }
                V::Num(String::from_utf8(self.s[st..self.i].to_vec()).unwrap())
            }
        };
        Ok(v)
    }
}

fn parse(path: &str) -> V {
    let b = fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut p = P { s: &b, i: 0 };
    let v = p.value().unwrap_or_else(|e| panic!("{path}: {e}"));
    p.ws();
    assert_eq!(p.i, b.len(), "{path}: trailing bytes");
    v
}

impl V {
    fn get(&self, k: &str) -> &V {
        match self {
            V::Obj(m) => m.iter().find(|(a, _)| a == k).map(|(_, v)| v).unwrap_or(&V::Null),
            _ => &V::Null,
        }
    }
    fn arr(&self) -> &[V] {
        match self {
            V::Arr(a) => a,
            _ => &[],
        }
    }
    /// Integers may be recorded as JSON numbers or as decimal strings.
    fn u(&self) -> Option<u128> {
        match self {
            V::Num(s) | V::Str(s) => s.parse().ok(),
            _ => None,
        }
    }
    fn s(&self) -> &str {
        match self {
            V::Str(s) => s,
            _ => "",
        }
    }
}

impl V {
    fn f(&self) -> Option<f64> {
        match self {
            V::Num(s) | V::Str(s) => s.parse().ok(),
            _ => None,
        }
    }
    fn path(&self, keys: &[&str]) -> &V {
        keys.iter().fold(self, |v, k| v.get(k))
    }
}

// bitop/v1 prime-field prices (formulas as descent_breakdown.rs / src/cryptanalysis/bitops.rs)
fn int_add(w: u128) -> u128 {
    5 * w
}
fn int_mul(w: u128) -> u128 {
    w * w + 5 * w * w.saturating_sub(1)
}
fn fp_add(b: u128) -> u128 {
    2 * int_add(b) + 3 * b
}
fn fp_mul(b: u128) -> u128 {
    2 * int_mul(b) + fp_add(b)
}
fn bits(x: u128) -> u128 {
    128 - x.leading_zeros() as u128
}

/// Deterministic xorshift64* for the bootstrap (no crates).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n % 2 == 1 {
        s[n / 2]
    } else {
        0.5 * (s[n / 2 - 1] + s[n / 2])
    }
}
fn geomean(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
/// Percentile bootstrap of the geometric mean over whole paired rounds.
fn boot(v: &[f64], draws: usize, seed: u64) -> (f64, f64) {
    let mut r = Rng(seed | 1);
    let mut g: Vec<f64> = (0..draws)
        .map(|_| geomean(&(0..v.len()).map(|_| v[r.below(v.len())]).collect::<Vec<_>>()))
        .collect();
    g.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (g[(0.025 * draws as f64) as usize], g[(0.975 * draws as f64) as usize - 1])
}
fn arr(v: &[f64]) -> String {
    format!("[{}]", v.iter().map(|x| format!("{x:.4}")).collect::<Vec<_>>().join(","))
}

fn ab(dir: &str, seed: &str, hooks: &str, rounds: usize) {
    let metrics = ["descent_wall", "f4", "linear_stage", "nagao_total", "process"];
    let mut per: Vec<(Vec<f64>, Vec<f64>)> = vec![(vec![], vec![]); metrics.len()];
    let mut digests = std::collections::BTreeSet::new();
    let mut f4_ops = (0f64, 0f64);
    for r in 1..=rounds {
        for (arm, slot) in [("production", 0), ("traced", 1)] {
            let v = parse(&format!("{dir}/seed{seed}-hooks{hooks}-r{r}-{arm}.json"));
            digests.insert(v.get("output_digest").s().to_string());
            let ops = v.path(&["counts", "f4_ops"]).f().expect("f4_ops");
            if slot == 0 { f4_ops.0 = ops } else { f4_ops.1 = ops }
            for (k, m) in metrics.iter().enumerate() {
                let x = v.path(&["ms", m]).f().expect("ms");
                if slot == 0 { per[k].0.push(x) } else { per[k].1.push(x) }
            }
        }
    }
    let mut out = Vec::new();
    for (k, m) in metrics.iter().enumerate() {
        let (a, b) = &per[k];
        let ratio: Vec<f64> = a.iter().zip(b).map(|(x, y)| x / y).collect();
        let aa_a: Vec<f64> = a.windows(2).map(|w| w[0] / w[1]).collect();
        let aa_b: Vec<f64> = b.windows(2).map(|w| w[0] / w[1]).collect();
        let (lo, hi) = boot(&ratio, 10000, 0xD5CE_0000 ^ k as u64);
        out.push(format!(
            "\"{m}\":{{\"production_ms\":{},\"traced_ms\":{},\"production_over_traced_per_round\":{},\"geomean\":{:.4},\"median\":{:.4},\"min\":{:.4},\"max\":{:.4},\"bootstrap95_geomean_over_rounds\":[{lo:.4},{hi:.4}],\"aa_production_consecutive\":{},\"aa_traced_consecutive\":{},\"aa_max_abs_log\":{:.4}}}",
            arr(a), arr(b), arr(&ratio), geomean(&ratio), median(&ratio),
            ratio.iter().cloned().fold(f64::INFINITY, f64::min), ratio.iter().cloned().fold(0.0, f64::max),
            arr(&aa_a), arr(&aa_b),
            aa_a.iter().chain(&aa_b).map(|x| x.ln().abs()).fold(0.0, f64::max)
        ));
    }
    let f4a = median(&per[1].0) * 1e6 / f4_ops.0;
    let f4b = median(&per[1].1) * 1e6 / f4_ops.1;
    println!(
        "{{\"schema\":\"descent-walltime.ab-summary/v1\",\"pcc\":\"unverified\",\"label\":\"indicative non-isolated local timing; 1 thread; shared 4-vCPU host; order A,B per round\",\"seed\":{seed},\"hooks\":{hooks},\"rounds\":{rounds},\"identical_outputs_all_runs\":{},\"output_digests\":{:?},{},\"f4_ns_per_counted_f4_op_median\":{{\"production\":{f4a:.3},\"traced\":{f4b:.3},\"traced_over_production\":{:.4}}},\"counted_f4_ops\":{{\"production\":{},\"traced\":{},\"production_over_traced\":{:.4}}}}}",
        digests.len() == 1, digests.iter().collect::<Vec<_>>(), out.join(","), f4b / f4a, f4_ops.0, f4_ops.1, f4_ops.0 / f4_ops.1
    );
}

fn counted(dir: &str, seed: &str, n: &str) {
    let load = |arm: &str| parse(&format!("{dir}/seed{seed}-n{n}-{arm}.json"));
    let (a, b) = (load("production"), load("traced"));
    let pb = bits(1009);
    let (fm, fa) = (fp_mul(pb), fp_add(pb));
    let u = |v: &V, k: &[&str]| v.path(k).u().unwrap_or_else(|| panic!("{k:?}"));
    let total = |v: &V| u(v, &["counts", "hooked_descent_bitops"]);
    let learn = u(&b, &["trace", "learning_rref_upper_bound_uncounted_by_f4fp"]) * (fm + fa);
    let (ta, tb) = (total(&a), total(&b));
    let same_lin = a.path(&["counts", "lin_ops"]).path(&["add"]).u() == b.path(&["counts", "lin_ops"]).path(&["add"]).u()
        && a.path(&["counts", "lin_ops"]).path(&["mac"]).u() == b.path(&["counts", "lin_ops"]).path(&["mac"]).u()
        && a.path(&["counts", "lin_ops"]).path(&["mul"]).u() == b.path(&["counts", "lin_ops"]).path(&["mul"]).u()
        && a.path(&["counts", "lin_ops"]).path(&["word"]).u() == b.path(&["counts", "lin_ops"]).path(&["word"]).u();
    // field_mul reconciliation: hooked field_mul minus the named legacy counters.
    let recon = |v: &V| {
        let hooked = u(v, &["counts", "hooked_descent_primitives", "field_mul"]);
        let lin = u(v, &["counts", "lin_ops", "mul"]) + u(v, &["counts", "lin_ops", "mac"]);
        let named = u(v, &["counts", "f4_ops"]) + u(v, &["counts", "weil_muls"]) + u(v, &["counts", "post_muls"]) + lin;
        format!(
            "{{\"hooked_field_mul\":{hooked},\"f4_ops\":{},\"weil_muls\":{},\"post_muls\":{},\"linear_stage_mul_plus_mac\":{lin},\"hooked_minus_named\":{}}}",
            u(v, &["counts", "f4_ops"]), u(v, &["counts", "weil_muls"]), u(v, &["counts", "post_muls"]),
            hooked as i128 - named as i128
        )
    };
    println!(
        "{{\"schema\":\"descent-walltime.counted-compare/v1\",\"pcc\":\"unverified\",\"seed\":{seed},\"residuals\":{n},\"prices\":{{\"fp_mul\":{fm},\"fp_add\":{fa}}},\"identical_linear_stage_ops_both_arms\":{same_lin},\"identical_outputs\":{},\"complete_counted_descent_bitops\":{{\"production\":{ta},\"traced\":{tb},\"production_over_traced\":{:.4}}},\"plus_learning_rref_bound\":{{\"bound_bitops\":{learn},\"traced\":{},\"production_over_traced\":{:.4}}},\"field_mul_reconciliation\":{{\"production\":{},\"traced\":{}}},\"still_unpriced\":[\"f4_replay trace bookkeeping (row/multiplier lookup, leading-monomial checks) beyond f4_fp's own hooks\",\"learning-run tracked RREF (bounded, not counted)\",\"allocation/containers\"]}}",
        a.get("output_digest").s() == b.get("output_digest").s(),
        ta as f64 / tb as f64, tb + learn, ta as f64 / (tb + learn) as f64, recon(&a), recon(&b)
    );
}

/// Poisson CDF P(X <= k; lambda).
fn pois_cdf(k: u64, lam: f64) -> f64 {
    let mut term = (-lam).exp();
    let mut s = term;
    for i in 1..=k {
        term *= lam / i as f64;
        s += term;
    }
    s
}
fn bisect(f: impl Fn(f64) -> f64, mut lo: f64, mut hi: f64) -> f64 {
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if f(lo).signum() == f(mid).signum() { lo = mid } else { hi = mid }
    }
    0.5 * (lo + hi)
}

fn run50(dir: &str, prod: &str, traced: &str) {
    let study = parse(&format!("{dir}/run-study.json"));
    // Independent residual streams (successes, residuals): Run50 original IC,
    // Run50 shared graph stream (counted once), nagao-e2e seed 401.
    let streams = [("run50-original-ic", 2u64, 1728u64), ("run50-graph-shared", 2, 3415), ("nagao-e2e-seed401", 2, 1506)];
    let (k, n): (u64, u64) = streams.iter().fold((0, 0), |a, s| (a.0 + s.1, a.1 + s.2));
    let p_hat = k as f64 / n as f64;
    // Exact Garwood 95% interval on the pooled Poisson count k over exposure n.
    let lam_lo = bisect(|l| (1.0 - pois_cdf(k - 1, l)) - 0.025, 1e-9, 100.0);
    let lam_hi = bisect(|l| pois_cdf(k, l) - 0.025, 1e-9, 100.0);
    let need = 2.0; // every Run50 arm stopped at 2 accepted descents
    let e_n = |p: f64| need / p;
    let scenarios = [("mle", e_n(p_hat)), ("garwood95_low_rate", e_n(lam_hi / n as f64)), ("garwood95_high_rate", e_n(lam_lo / n as f64))];
    let pc = parse(prod);
    let tc = parse(traced);
    let res = 3415f64;
    let per_res_complete = pc.path(&["counts", "hooked_descent_bitops"]).f().unwrap() / res;
    let pb = bits(1009);
    let learn = tc.path(&["trace", "learning_rref_upper_bound_uncounted_by_f4fp"]).f().unwrap() * (fp_mul(pb) + fp_add(pb)) as f64;
    let per_res_traced = (tc.path(&["counts", "hooked_descent_bitops"]).f().unwrap() + learn) / res;
    let mut rows = Vec::new();
    for o in study.get("outcomes").arr() {
        let mode = o.get("mode").s();
        if mode.starts_with("rho") || mode.is_empty() {
            continue;
        }
        let total = o.get("observed_incomplete_bitops").f().unwrap();
        let resid = o.get("descent_residuals").f().unwrap();
        let descent = o.get("phases").arr().iter().find(|p| p.get("phase").s() == "descent").unwrap().get("observed_incomplete_bitops").f().unwrap();
        let per_res = descent / resid;
        let non_descent = total - descent;
        let sc: Vec<String> = scenarios
            .iter()
            .map(|(name, en)| {
                format!(
                    "\"{name}\":{{\"expected_residuals\":{en:.1},\"recorded_scope\":{:.0},\"complete_linear_stage\":{:.0},\"complete_with_traced_descent\":{:.0}}}",
                    non_descent + en * per_res,
                    non_descent + en * per_res_complete,
                    non_descent + en * per_res_traced
                )
            })
            .collect();
        rows.push((non_descent, format!(
            "{{\"mode\":\"{mode}\",\"observed_total\":{total:.0},\"observed_descent\":{descent:.0},\"observed_residuals\":{resid:.0},\"descent_per_residual\":{per_res:.1},\"non_descent\":{non_descent:.0},\"normalised\":{{{}}}}}",
            sc.join(",")
        )));
    }
    let observed_rank: Vec<String> = {
        let mut v: Vec<(f64, String)> = study.get("outcomes").arr().iter().filter(|o| !o.get("mode").s().starts_with("rho"))
            .map(|o| (o.get("observed_incomplete_bitops").f().unwrap(), o.get("mode").s().to_string())).collect();
        v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        v.into_iter().map(|x| format!("\"{}\"", x.1)).collect()
    };
    let mut by_nd = rows.clone();
    by_nd.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let model_rank: Vec<String> = by_nd.iter().map(|r| {
        let s = &r.1; let i = s.find("\"mode\":\"").unwrap() + 8; let j = s[i..].find('"').unwrap();
        format!("\"{}\"", &s[i..i + j])
    }).collect();
    println!(
        "{{\"schema\":\"descent-walltime.run50-normalised-model/v1\",\"pcc\":\"unverified\",\"label\":\"MODEL ONLY: descent normalised to the expected residual count; not a measurement and not a re-accounting of Run50\",\"streams\":[{}],\"pooled_successes\":{k},\"pooled_residuals\":{n},\"p_hat\":{p_hat:.6e},\"garwood95_rate\":[{:.6e},{:.6e}],\"accepted_needed\":2,\"assumptions\":[\"per-residual success probability is arm-independent (same Nagao descent code, same 509-element base size; graph arms rank 508)\",\"accepted descents needed = 2 in every arm (all Run50 arms stopped at 2)\",\"descent cost per residual is the arm's recorded mean\",\"complete_linear_stage uses this lane's counted production descent per residual on the reconstructed Run50 graph stream\",\"complete_with_traced_descent uses the traced arm plus the learning-RREF bound, as if C1 replaced production descent in every arm\"],\"observed_rank_low_to_high\":[{}],\"model_rank_low_to_high\":[{}],\"rank_note\":\"with an arm-independent expected residual count the descent term is common, so the model ranking equals the non-descent ranking for every expected count\",\"arms\":[{}]}}",
        streams.iter().map(|s| format!("{{\"stream\":\"{}\",\"successes\":{},\"residuals\":{}}}", s.0, s.1, s.2)).collect::<Vec<_>>().join(","),
        lam_lo / n as f64, lam_hi / n as f64,
        observed_rank.join(","), model_rank.join(","),
        rows.iter().map(|r| r.1.clone()).collect::<Vec<_>>().join(",")
    );
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("ab") => ab(&a[2], &a[3], &a[4], a[5].parse().expect("rounds")),
        Some("counted") => counted(&a[2], &a[3], &a[4]),
        Some("run50") => run50(&a[2], &a[3], &a[4]),
        _ => panic!("usage: walltime_analyze ab|counted|run50 ..."),
    }
}
