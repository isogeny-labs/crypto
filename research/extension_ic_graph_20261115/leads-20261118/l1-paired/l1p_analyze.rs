//! L1 paired protocol: frozen analysis program (std only; no crates).
//! Lane research/extension_ic_graph_20261115/leads-20261118/l1-paired. [pcc:unverified]
//!
//!   l1p_analyze analyze TARGETS_TSV AA_TSV BOOT_SEED CONTROLS_KV OUT_JSON
//!   l1p_analyze power [SIMS] [BOOT]          (design-time power table; no data read)
//!
//! TARGETS_TSV / AA_TSV: the concatenated `target.tsv` files (one header, then one
//! line per target; `tag` = primary or aa). CONTROLS_KV: `key=value` lines written
//! by run.sh (pc1, pc2, dc, unseen, budget_truncated, t_planned, t_done).
//! The decision rule is PROTOCOL.md section 4, implemented in `decide`.
use std::collections::HashMap;
use std::fmt::Write as _;

// ---------- deterministic RNG (SplitMix64) ----------
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        // rejection sampling, exact uniform
        let zone = u64::MAX - u64::MAX % n;
        loop {
            let x = self.next();
            if x < zone {
                return x % n;
            }
        }
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn poisson(&mut self, mu: f64) -> u64 {
        if mu > 30.0 {
            // normal approximation with continuity, adequate for design tables only
            let (u1, u2) = (self.unit().max(1e-300), self.unit());
            let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            return (mu + mu.sqrt() * z + 0.5).floor().max(0.0) as u64;
        }
        let lim = (-mu).exp();
        let (mut k, mut prod) = (0u64, self.unit());
        while prod > lim {
            k += 1;
            prod *= self.unit();
        }
        k
    }
    fn normal(&mut self) -> f64 {
        let (u1, u2) = (self.unit().max(1e-300), self.unit());
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

// ---------- exact intervals ----------
fn ln_choose(n: u64, k: u64) -> f64 {
    (1..=k).map(|i| ((n - k + i) as f64).ln() - (i as f64).ln()).sum()
}
fn binom_cdf(k: i64, n: u64, q: f64) -> f64 {
    if k < 0 {
        return 0.0;
    }
    (0..=k as u64).map(|i| (ln_choose(n, i) + i as f64 * q.ln() + (n - i) as f64 * (1.0 - q).ln()).exp()).sum::<f64>().min(1.0)
}
/// Clopper-Pearson two-sided interval at level 1 - alpha.
fn clopper_pearson(k: u64, n: u64, alpha: f64) -> (f64, f64) {
    if n == 0 {
        return (0.0, 1.0);
    }
    let solve = |target: f64, kk: i64| -> f64 {
        let (mut lo, mut hi) = (1e-15f64, 1.0 - 1e-15);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if binom_cdf(kk, n, mid) > target {
                lo = mid
            } else {
                hi = mid
            }
        }
        0.5 * (lo + hi)
    };
    let lower = if k == 0 { 0.0 } else { solve(1.0 - alpha / 2.0, k as i64 - 1) };
    let upper = if k == n { 1.0 } else { solve(alpha / 2.0, k as i64) };
    (lower, upper)
}
/// Rate ratio M/F from the conditional binomial M | M+F.
fn ratio_cp(m: u64, f: u64, alpha: f64) -> (f64, f64) {
    let (lo, hi) = clopper_pearson(m, m + f, alpha);
    (lo / (1.0 - lo), if hi >= 1.0 { f64::INFINITY } else { hi / (1.0 - hi) })
}
fn pct(sorted: &[f64], q: f64) -> f64 {
    let i = ((q * sorted.len() as f64).floor() as usize).min(sorted.len() - 1);
    sorted[i]
}
/// Cluster (target) percentile bootstrap of sum(num)/sum(den); B resamples.
fn boot_ratio(num: &[f64], den: &[f64], b: usize, rng: &mut Rng) -> (f64, f64) {
    let t = num.len();
    let mut v = Vec::with_capacity(b);
    for _ in 0..b {
        let (mut sn, mut sd) = (0.0, 0.0);
        for _ in 0..t {
            let j = rng.below(t as u64) as usize;
            sn += num[j];
            sd += den[j];
        }
        v.push(if sd > 0.0 { sn / sd } else { f64::INFINITY });
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (pct(&v, 0.025), pct(&v, 0.975))
}

// ---------- frozen thresholds (PROTOCOL.md section 4) ----------
const KILL_LINE: f64 = 1.5;
const STRONG_LINE: f64 = 2.0;
const MIN_SUCCESSES: u64 = 100;
const BOOT_B: usize = 20_000;
const AA_ALPHA: f64 = 0.01;
const MAX_UNREPLAYABLE_SHARE: f64 = 0.01;

#[derive(Debug, Clone)]
struct Interval {
    point: f64,
    cp: (f64, f64),
    boot: (f64, f64),
    env: (f64, f64),
}
fn primary_interval(m: &[f64], f: &[f64], rng: &mut Rng, b: usize) -> Interval {
    let (sm, sf) = (m.iter().sum::<f64>(), f.iter().sum::<f64>());
    let cp = ratio_cp(sm as u64, sf as u64, 0.05);
    let boot = boot_ratio(m, f, b, rng);
    let env = (cp.0.min(boot.0), cp.1.max(boot.1));
    Interval { point: if sf > 0.0 { sm / sf } else { f64::INFINITY }, cp, boot, env }
}
/// Returns (verdict, tier).
fn decide(env: (f64, f64)) -> (&'static str, &'static str) {
    if env.0 >= KILL_LINE {
        ("PROMOTE", if env.0 >= STRONG_LINE { "clears-2.0-line" } else { "clears-1.5-line" })
    } else if env.1 < KILL_LINE {
        ("KILL", "below-1.5-line")
    } else {
        ("INCONCLUSIVE", "interval-straddles-1.5")
    }
}

// ---------- TSV ----------
struct Table {
    rows: Vec<HashMap<String, String>>,
}
fn read_tsv(path: &str) -> Table {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut lines = text.lines();
    let header: Vec<String> = lines.next().expect("header").split('\t').map(String::from).collect();
    let mut rows = Vec::new();
    for line in lines {
        if line.is_empty() || line == header.join("\t") {
            continue; // concatenated per-target files repeat the header
        }
        let vals: Vec<&str> = line.split('\t').collect();
        assert_eq!(vals.len(), header.len(), "ragged TSV line in {path}");
        rows.push(header.iter().cloned().zip(vals.iter().map(|s| s.to_string())).collect());
    }
    Table { rows }
}
fn num(r: &HashMap<String, String>, k: &str) -> f64 {
    r.get(k).unwrap_or_else(|| panic!("missing column {k}")).parse::<f64>().unwrap_or_else(|_| panic!("column {k}"))
}
fn sum(rows: &[&HashMap<String, String>], k: &str) -> f64 {
    rows.iter().map(|r| num(r, k)).sum()
}
fn j(x: f64) -> String {
    if x.is_finite() {
        format!("{x:.6}")
    } else {
        "\"inf\"".into()
    }
}

fn analyze(args: &[String]) {
    let (tpath, apath, seed, kvpath, out) = (&args[2], &args[3], args[4].parse::<u64>().expect("seed"), &args[5], &args[6]);
    let kv: HashMap<String, String> = std::fs::read_to_string(kvpath)
        .expect("controls")
        .lines()
        .filter_map(|l| l.split_once('=').map(|(a, b)| (a.trim().to_string(), b.trim().to_string())))
        .collect();
    let t = read_tsv(tpath);
    let prim: Vec<&HashMap<String, String>> = t.rows.iter().filter(|r| r["tag"] == "primary").collect();
    let aa_t = read_tsv(apath);
    let aa: Vec<&HashMap<String, String>> = aa_t.rows.iter().filter(|r| r["tag"] == "aa").collect();
    let mut rng = Rng(seed);
    let mut notes: Vec<String> = Vec::new();
    // ---- validity gates ----
    let zero_cols = ["group_bad", "anchor_bad", "anchor_replay_bad", "replay_fail_F", "affine_inconsistent"];
    let mut gate_fail: Vec<String> = Vec::new();
    for c in zero_cols {
        let s = sum(&prim, c) + sum(&aa, c);
        if s != 0.0 {
            gate_fail.push(format!("{c}={s}"));
        }
    }
    for c in ["pipeline_rows_match", "pipeline_consistency"] {
        let bad = prim.iter().filter(|r| num(r, c) != 1.0).count();
        if bad != 0 {
            gate_fail.push(format!("{c}: {bad} targets"));
        }
    }
    let flip_total = sum(&prim, "flip_total") + sum(&aa, "flip_total");
    let flip_ref = sum(&prim, "flip_refused") + sum(&aa, "flip_refused");
    if flip_ref != flip_total {
        gate_fail.push(format!("negative control: {flip_ref} of {flip_total} flipped rows refused"));
    }
    let rows_all = sum(&prim, "F_rows") + sum(&prim, "M_rows");
    let unrep = sum(&prim, "unreplayable");
    if rows_all > 0.0 && unrep / rows_all > MAX_UNREPLAYABLE_SHARE {
        gate_fail.push(format!("unreplayable share {:.4}", unrep / rows_all));
    }
    for c in ["pc1", "pc2", "dc", "unseen"] {
        if kv.get(c).map(String::as_str) != Some("pass") {
            gate_fail.push(format!("control {c} = {:?}", kv.get(c)));
        }
    }
    let correctness_fail = sum(&prim, "replay_fail_M") + sum(&aa, "replay_fail_M");
    // ---- primary ----
    let fv: Vec<f64> = prim.iter().map(|r| num(r, "F")).collect();
    let mv: Vec<f64> = prim.iter().map(|r| num(r, "M")).collect();
    let (sf, sm) = (fv.iter().sum::<f64>(), mv.iter().sum::<f64>());
    let iv = primary_interval(&mv, &fv, &mut rng, BOOT_B);
    // ---- A/A ----
    let by_sha: HashMap<&str, &&HashMap<String, String>> = prim.iter().map(|r| (r["public_sha256"].as_str(), r)).collect();
    let (mut fa, mut ma, mut fp, mut mp) = (0.0, 0.0, 0.0, 0.0);
    for r in &aa {
        let p = by_sha.get(r["public_sha256"].as_str()).expect("A/A target missing from primary");
        fa += num(r, "F");
        ma += num(r, "M");
        fp += num(p, "F");
        mp += num(p, "M");
    }
    let aa_f = ratio_cp(fa as u64, fp as u64, AA_ALPHA);
    let aa_m = ratio_cp(ma as u64, mp as u64, AA_ALPHA);
    let lrr = if fa > 0.0 && ma > 0.0 && fp > 0.0 && mp > 0.0 { ((ma / fa) / (mp / fp)).ln() } else { f64::NAN };
    let se = (1.0 / ma + 1.0 / fa + 1.0 / mp + 1.0 / fp).sqrt();
    let aa_rr = ((lrr - 2.5758 * se).exp(), (lrr + 2.5758 * se).exp());
    let aa_pass = aa.len() > 0
        && aa_f.0 <= 1.0 && 1.0 <= aa_f.1
        && aa_m.0 <= 1.0 && 1.0 <= aa_m.1
        && aa_rr.0 <= 1.0 && 1.0 <= aa_rr.1;
    // ---- decision ----
    let budget_trunc = kv.get("budget_truncated").map(String::as_str) == Some("1");
    let (verdict, tier) = if correctness_fail > 0.0 {
        ("KILL", "correctness: a group- and anchor-verified L1 row failed the affine/elliptic replay")
    } else if !gate_fail.is_empty() {
        ("INVALID", "a validity gate failed; no verdict")
    } else if !aa_pass {
        ("INVALID", "A/A control failed; no verdict")
    } else if (sf as u64) < MIN_SUCCESSES || ((sf + sm) as u64) < MIN_SUCCESSES {
        ("INCONCLUSIVE", "underpowered: fewer than 100 pooled successes in an arm")
    } else {
        decide(iv.env)
    };
    if budget_trunc {
        notes.push("budget stop: fewer targets than planned were completed (PROTOCOL.md section 6)".into());
    }
    // ---- secondary (descriptive; decide nothing) ----
    let sa: Vec<f64> = prim.iter().map(|r| num(r, "S_A")).collect();
    let sb: Vec<f64> = prim.iter().map(|r| num(r, "S_B")).collect();
    let g_point = sb.iter().sum::<f64>() / sa.iter().sum::<f64>();
    let g_boot = boot_ratio(&sb, &sa, BOOT_B, &mut rng);
    let cf = sum(&prim, "classes_formed");
    let ci = sum(&prim, "classes_in_dexact");
    let fbar = ci / cf;
    let rho_model = 15.0 * fbar + 45.0 * fbar * fbar + 15.0 * fbar.powi(3);
    let mut exp_m = [0.0f64; 3];
    for r in &prim {
        let ft = if num(r, "classes_formed") > 0.0 { num(r, "classes_in_dexact") / num(r, "classes_formed") } else { 0.0 };
        exp_m[0] += num(r, "lin_valid_1^4,2") * ft;
        exp_m[1] += num(r, "lin_valid_1^2,2^2") * ft * ft;
        exp_m[2] += num(r, "lin_valid_2^3") * ft.powi(3);
    }
    let sols = sum(&prim, "solutions");
    let fsol = clopper_pearson(sf as u64, sols as u64, 0.05);
    let rmean = |k: &str| -> f64 {
        prim.iter().map(|r| { let x = num(r, k); if x < 0.0 { num(r, "n_residuals") } else { x + 1.0 } }).sum::<f64>() / prim.len() as f64
    };
    let mut o = String::new();
    let _ = write!(
        o,
        concat!(
            "{{\"schema\":\"l1-paired.analysis/v1\",\"pcc\":\"unverified\",\"targets\":{},\"aa_targets\":{},",
            "\"F\":{},\"M\":{},\"rho_hat\":{},\"rho_cp95\":[{},{}],\"rho_boot95\":[{},{}],\"rho_envelope95\":[{},{}],",
            "\"arm_B_over_arm_A_success_rate\":{},\"verdict\":\"{}\",\"tier\":\"{}\",\"kill_line\":{},\"strong_line\":{},",
            "\"gates_failed\":[{}],\"replay_fail_M\":{},\"negative_control\":{{\"flipped\":{},\"refused\":{}}},\"unreplayable\":{},",
            "\"aa\":{{\"F_aa\":{},\"F_primary\":{},\"M_aa\":{},\"M_primary\":{},\"F_ratio_cp99\":[{},{}],\"M_ratio_cp99\":[{},{}],",
            "\"rho_ratio_wald99\":[{},{}],\"pass\":{}}},",
            "\"secondary\":{{\"S_A\":{},\"S_B\":{},\"residual_level_gain\":{},\"residual_level_gain_boot95\":[{},{}],",
            "\"class_match_rate_f\":{},\"rho_random_sextic_model\":{},\"expected_M_by_pattern_given_f\":[{},{},{}],",
            "\"observed_M_by_pattern\":[{},{},{}],\"full_split_per_solution\":{},\"full_split_per_solution_cp95\":[{},{}],",
            "\"model_full_split_per_solution\":{},\"mean_residuals_to_first_success_A_censored\":{},\"mean_residuals_to_first_success_B_censored\":{},",
            "\"mean_residuals_to_second_success_A_censored\":{},\"mean_residuals_to_second_success_B_censored\":{}}},",
            "\"notes\":[{}]}}"
        ),
        prim.len(), aa.len(), sf, sm, j(iv.point), j(iv.cp.0), j(iv.cp.1), j(iv.boot.0), j(iv.boot.1), j(iv.env.0), j(iv.env.1),
        j(1.0 + iv.point), verdict, tier, KILL_LINE, STRONG_LINE,
        gate_fail.iter().map(|g| format!("\"{g}\"")).collect::<Vec<_>>().join(","), correctness_fail, flip_total, flip_ref, unrep,
        fa, fp, ma, mp, j(aa_f.0), j(aa_f.1), j(aa_m.0), j(aa_m.1), j(aa_rr.0), j(aa_rr.1), aa_pass,
        sum(&prim, "S_A"), sum(&prim, "S_B"), j(g_point), j(g_boot.0), j(g_boot.1),
        j(fbar), j(rho_model), j(exp_m[0]), j(exp_m[1]), j(exp_m[2]),
        sum(&prim, "M_1^4,2"), sum(&prim, "M_1^2,2^2"), sum(&prim, "M_2^3"), j(sf / sols), j(fsol.0), j(fsol.1),
        j(1.0 / 720.0), j(rmean("first_A")), j(rmean("first_B")), j(rmean("second_A")), j(rmean("second_B")),
        notes.iter().map(|g| format!("\"{g}\"")).collect::<Vec<_>>().join(",")
    );
    std::fs::write(out, format!("{o}\n")).expect("write analysis");
    println!("{o}");
}

/// Design-time power table: Poisson counts per target with between-target
/// heterogeneity of the match rate (lognormal, sd 0.15 on the log scale).
fn power(args: &[String]) {
    let sims: usize = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(1000);
    let b: usize = args.get(3).map(|s| s.parse().unwrap()).unwrap_or(2000);
    let lam_f = 31.0 / 30000.0; // full splits per residual, Run50 stream (steps 2-3 extension)
    println!("design,T,N,lambda_F_per_residual,rho_true,E[F],P(PROMOTE),P(KILL),P(INCONCLUSIVE),P(F<100)");
    for &(tt, n, lf) in &[(96usize, 4096u64, lam_f), (96, 4096, 1.0 / 1400.0), (32, 4096, lam_f)] {
        for &rho in &[0.75f64, 1.0, 1.2, 1.51, 1.84, 2.2, 2.6] {
            let mut rng = Rng(0x5EED_0000 ^ (tt as u64) << 20 ^ (rho * 1000.0) as u64 ^ (1.0 / lf) as u64);
            let (mut pr, mut kl, mut inc, mut under) = (0, 0, 0, 0);
            for _ in 0..sims {
                let mut f = Vec::with_capacity(tt);
                let mut m = Vec::with_capacity(tt);
                for _ in 0..tt {
                    let h = (0.15 * rng.normal() - 0.5 * 0.15 * 0.15).exp();
                    f.push(rng.poisson(n as f64 * lf) as f64);
                    m.push(rng.poisson(n as f64 * lf * rho * h) as f64);
                }
                if (f.iter().sum::<f64>() as u64) < MIN_SUCCESSES {
                    under += 1;
                    inc += 1;
                    continue;
                }
                let iv = primary_interval(&m, &f, &mut rng, b);
                match decide(iv.env).0 {
                    "PROMOTE" => pr += 1,
                    "KILL" => kl += 1,
                    _ => inc += 1,
                }
            }
            let s = sims as f64;
            println!(
                "{},{tt},{n},{lf:.6e},{rho},{:.1},{:.3},{:.3},{:.3},{:.3}",
                if lf == lam_f { "observed-rate" } else { "conservative-rate" },
                tt as f64 * n as f64 * lf,
                pr as f64 / s,
                kl as f64 / s,
                inc as f64 / s,
                under as f64 / s
            );
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("analyze") if args.len() == 7 => analyze(&args),
        Some("power") => power(&args),
        _ => {
            eprintln!("usage: l1p_analyze analyze TARGETS_TSV AA_TSV BOOT_SEED CONTROLS_KV OUT_JSON | power [SIMS] [BOOT]");
            std::process::exit(2);
        }
    }
}
