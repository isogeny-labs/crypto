//! L1 x C1 descent-cost protocol: frozen analysis program (std only; no crates).
//! Lane research/extension_ic_graph_20261115/leads-20261118/l1c1-cost. [pcc:unverified]
//!
//!   l1c1_analyze analyze TARGETS_TSV AA_TSV BOOT_SEED CONTROLS_KV OUT_JSON
//!   l1c1_analyze power SIMS BOOT OUT_CSV      (design-time power table; reads no data)
//!   l1c1_analyze order SEED                   (wall-arm order of one target)
//!
//! TARGETS_TSV / AA_TSV: one header, then one `l1c1_cost collect` line per target
//! (tag primary / aa). CONTROLS_KV: key=value lines written by run.sh.
//! The decision rules are PROTOCOL.md section 2, implemented in `verdict_*` and
//! `analyze`; nothing here is tuned after data.
use std::collections::HashMap;
use std::fmt::Write as _;

// ---------- deterministic RNG (SplitMix64), as l1p_analyze.rs ----------
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
        let zone = u64::MAX - u64::MAX % n;
        loop {
            let x = self.next();
            if x < zone {
                return x % n;
            }
        }
    }
    fn unit(&mut self) -> f64 {
        ((self.next() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
    }
    fn normal(&mut self) -> f64 {
        let (u1, u2) = (self.unit(), self.unit());
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
    /// Failures before the first success, success probability q (0 < q <= 1).
    fn geometric(&mut self, q: f64) -> u64 {
        if q >= 1.0 {
            return 0;
        }
        (self.unit().ln() / (1.0 - q).ln()).floor() as u64
    }
}

// ---------- preregistered constants (PROTOCOL.md section 2) ----------
const B_BOOT: usize = 20_000;
const LO95: usize = 500; // sorted index of the 2.5% point at B = 20,000
const HI95: usize = 19_499;
const LO99: usize = 100;
const HI99: usize = 19_899;
const REDUCE_LINE: f64 = 1.0;
const MATERIAL_LINE: f64 = 0.8;
const EQUIV_MARGIN: f64 = 0.10; // multiplicative composition: I within [0.90, 1.10]
const AA_WALL_BAND: f64 = 0.05; // wall noise floor: A/A 99% interval within [0.95, 1.05]
const T_MIN: usize = 50; // >= 100 accepted successes per arm (K = 2)

/// Counted / wall effect verdict on a cost ratio (arm / reference).
fn verdict_effect(lo: f64, hi: f64) -> &'static str {
    if hi < MATERIAL_LINE {
        "MATERIAL-REDUCTION"
    } else if hi < REDUCE_LINE {
        "REDUCES"
    } else if lo > REDUCE_LINE {
        "INCREASES"
    } else {
        "INCONCLUSIVE"
    }
}
/// Interaction I = R_L1C1 / (R_L1 R_C1).
fn verdict_interaction(lo: f64, hi: f64) -> String {
    let within = lo >= 1.0 - EQUIV_MARGIN && hi <= 1.0 + EQUIV_MARGIN;
    let dir = if lo > 1.0 {
        "SUB-MULTIPLICATIVE"
    } else if hi < 1.0 {
        "SUPER-MULTIPLICATIVE"
    } else {
        "NOT-DISTINGUISHED-FROM-MULTIPLICATIVE"
    };
    if within {
        format!("{dir}; MULTIPLICATIVE-WITHIN-10%")
    } else {
        format!("{dir}; NOT-SHOWN-WITHIN-10%")
    }
}
/// Composition: the combination is cheaper than each single factor.
fn verdict_compose(c1_given_l1: (f64, f64), l1_given_c1: (f64, f64)) -> &'static str {
    if c1_given_l1.1 < 1.0 && l1_given_c1.1 < 1.0 {
        "COMPOSES"
    } else if c1_given_l1.0 > 1.0 || l1_given_c1.0 > 1.0 {
        "DOES-NOT-COMPOSE"
    } else {
        "INCONCLUSIVE"
    }
}

// ---------- statistics ----------
/// The six ratios of one (re)sample: R_L1, R_C1, R_L1C1, I, S_C1|L1, S_L1|C1.
fn ratios(base: f64, l1: f64, c1: f64, l1c1: f64) -> [f64; 6] {
    let (rl, rc, r11) = (l1 / base, c1 / base, l1c1 / base);
    [rl, rc, r11, r11 / (rl * rc), l1c1 / l1, l1c1 / c1]
}
const RNAMES: [&str; 6] = ["R_L1", "R_C1", "R_L1C1", "I", "S_C1_given_L1", "S_L1_given_C1"];

struct Boot {
    point: [f64; 6],
    ci95: [(f64, f64); 6],
    ci99: [(f64, f64); 6],
}
/// Cluster (target) percentile bootstrap of the ratio-of-sums statistics.
fn boot(cols: &[[f64; 4]], seed: u64, b: usize) -> Boot {
    let n = cols.len();
    let sum = |idx: &mut dyn Iterator<Item = usize>| {
        let mut s = [0f64; 4];
        for i in idx {
            for k in 0..4 {
                s[k] += cols[i][k];
            }
        }
        s
    };
    let s = sum(&mut (0..n));
    let point = ratios(s[0], s[1], s[2], s[3]);
    let mut rng = Rng(seed);
    let mut draws: Vec<[f64; 6]> = Vec::with_capacity(b);
    for _ in 0..b {
        let mut s = [0f64; 4];
        for _ in 0..n {
            let i = rng.below(n as u64) as usize;
            for k in 0..4 {
                s[k] += cols[i][k];
            }
        }
        draws.push(ratios(s[0], s[1], s[2], s[3]));
    }
    let (lo95, hi95, lo99, hi99) = scaled(b);
    let mut ci95 = [(0.0, 0.0); 6];
    let mut ci99 = [(0.0, 0.0); 6];
    for k in 0..6 {
        let mut v: Vec<f64> = draws.iter().map(|d| d[k]).collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ci95[k] = (v[lo95], v[hi95]);
        ci99[k] = (v[lo99], v[hi99]);
    }
    Boot { point, ci95, ci99 }
}
fn scaled(b: usize) -> (usize, usize, usize, usize) {
    if b == B_BOOT {
        (LO95, HI95, LO99, HI99)
    } else {
        let f = |q: f64| ((q * b as f64).round() as usize).min(b - 1);
        (f(0.025), f(0.975) - 1, f(0.005), f(0.995) - 1)
    }
}
/// Bootstrap of a ratio of sums of two paired columns (A/A), 99% interval.
fn boot_pair(x: &[f64], y: &[f64], seed: u64) -> (f64, (f64, f64)) {
    let n = x.len();
    let point = x.iter().sum::<f64>() / y.iter().sum::<f64>();
    let mut rng = Rng(seed);
    let mut v = Vec::with_capacity(B_BOOT);
    for _ in 0..B_BOOT {
        let (mut a, mut b) = (0.0, 0.0);
        for _ in 0..n {
            let i = rng.below(n as u64) as usize;
            a += x[i];
            b += y[i];
        }
        v.push(a / b);
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (point, (v[LO99], v[HI99]))
}

// ---------- input ----------
struct Table {
    head: Vec<String>,
    rows: Vec<Vec<String>>,
}
impl Table {
    fn read(path: &str) -> Table {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let mut lines = text.lines();
        let head: Vec<String> = lines.next().unwrap_or("").split('\t').map(String::from).collect();
        let rows = lines.filter(|l| !l.is_empty() && !l.starts_with("tag\t")).map(|l| l.split('\t').map(String::from).collect()).collect();
        Table { head, rows }
    }
    fn col(&self, name: &str) -> usize {
        self.head.iter().position(|h| h == name).unwrap_or_else(|| panic!("column {name} missing"))
    }
    fn get<'a>(&self, r: &'a [String], name: &str) -> &'a str {
        &r[self.col(name)]
    }
    fn num(&self, r: &[String], name: &str) -> f64 {
        self.get(r, name).parse::<f64>().unwrap_or(f64::NAN)
    }
}
fn kv(path: &str) -> HashMap<String, String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.split_once('=').map(|(a, b)| (a.trim().to_string(), b.trim().to_string())))
        .collect()
}
fn jf(x: f64) -> String {
    if x.is_finite() {
        format!("{x:.6}")
    } else {
        "null".into()
    }
}
fn jci(c: (f64, f64)) -> String {
    format!("[{},{}]", jf(c.0), jf(c.1))
}

fn analyze(args: &[String]) {
    let (tp, ap, seed, cp, out) = (&args[0], &args[1], args[2].parse::<u64>().expect("seed"), &args[3], &args[4]);
    let t = Table::read(tp);
    let aa = Table::read(ap);
    let ctl = kv(cp);
    let prim: Vec<&Vec<String>> = t.rows.iter().filter(|r| t.get(r, "tag") == "primary").collect();
    let mut gates: Vec<String> = Vec::new();
    for key in ["pc1", "pc1b", "pc1c", "pc2", "dc", "unseen"] {
        if ctl.get(key).map(String::as_str) != Some("pass") {
            gates.push(format!("control:{key}"));
        }
    }
    let mut kill_l1 = false;
    let mut kill_c1 = false;
    for r in &prim {
        let g = t.get(r, "gates_failed");
        if g != "-" {
            gates.push(format!("{}:{g}", &t.get(r, "public_sha256")[..12]));
        }
        kill_l1 |= t.get(r, "kill_l1") == "1";
        kill_c1 |= t.get(r, "kill_c1") == "1";
    }
    let arms = ["base", "l1", "c1", "l1c1"];
    let cols_of = |tab: &Table, rows: &[&Vec<String>], prefix: &str| -> Vec<[f64; 4]> {
        rows.iter().map(|r| core::array::from_fn(|k| tab.num(r, &format!("{prefix}_{}", arms[k])))).collect()
    };
    let n = prim.len();
    let counted = cols_of(&t, &prim, "C");
    let counted_nl = cols_of(&t, &prim, "Cn");
    let k1 = cols_of(&t, &prim, "K1");
    let censored = prim.iter().filter(|r| t.get(r, "censored") == "1").count();
    let mut j = String::new();
    write!(j, "{{\"schema\":\"l1c1-cost.analysis/v1\",\"pcc\":\"unverified\",\"targets\":{n},\"censored_base\":{censored}").unwrap();
    let invalid = !gates.is_empty();
    let underpowered = n < T_MIN;
    // ---- counted (primary) ----
    let bc = boot(&counted, seed, B_BOOT);
    // ---- counted A/A on the independent stream ----
    let pidx: HashMap<&str, usize> = prim.iter().enumerate().map(|(i, r)| (t.get(r, "public_sha256"), i)).collect();
    let aarows: Vec<&Vec<String>> = aa.rows.iter().filter(|r| aa.get(r, "tag") == "aa").collect();
    let mut aa_base = (Vec::new(), Vec::new());
    let mut aa_l1 = (Vec::new(), Vec::new());
    for r in &aarows {
        if aa.get(r, "gates_failed") != "-" {
            gates.push(format!("aa:{}:{}", &aa.get(r, "public_sha256")[..12], aa.get(r, "gates_failed")));
        }
        if let Some(&i) = pidx.get(aa.get(r, "public_sha256")) {
            aa_base.0.push(aa.num(r, "C_base"));
            aa_base.1.push(counted[i][0]);
            aa_l1.0.push(aa.num(r, "C_l1"));
            aa_l1.1.push(counted[i][1]);
        }
    }
    let (aab, aab99) = if aa_base.0.len() >= 2 { boot_pair(&aa_base.0, &aa_base.1, seed ^ 0xAA01) } else { (f64::NAN, (f64::NAN, f64::NAN)) };
    // A/A for R_L1: (sum C_l1^aa / sum C_base^aa) / (sum C_l1 / sum C_base) via the ratio of the two ratio-of-sums
    let aa_rl = if aa_base.0.len() >= 2 {
        let m = aa_base.0.len();
        let point = (aa_l1.0.iter().sum::<f64>() / aa_base.0.iter().sum::<f64>()) / (aa_l1.1.iter().sum::<f64>() / aa_base.1.iter().sum::<f64>());
        let mut rng = Rng(seed ^ 0xAA02);
        let mut v = Vec::with_capacity(B_BOOT);
        for _ in 0..B_BOOT {
            let (mut a, mut b, mut c, mut d) = (0.0, 0.0, 0.0, 0.0);
            for _ in 0..m {
                let i = rng.below(m as u64) as usize;
                a += aa_l1.0[i];
                b += aa_base.0[i];
                c += aa_l1.1[i];
                d += aa_base.1[i];
            }
            v.push((a / b) / (c / d));
        }
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        (point, (v[LO99], v[HI99]))
    } else {
        (f64::NAN, (f64::NAN, f64::NAN))
    };
    let aa_pass = aa_base.0.len() >= 2 && aab99.0 <= 1.0 && aab99.1 >= 1.0 && aa_rl.1 .0 <= 1.0 && aa_rl.1 .1 >= 1.0;
    if !aa_pass {
        gates.push("counted_aa".into());
    }
    let invalid = invalid || !aa_pass;
    write!(j, ",\"counted_aa\":{{\"targets\":{},\"base_ratio\":{},\"base_ratio_ci99\":{},\"R_L1_ratio\":{},\"R_L1_ratio_ci99\":{},\"pass\":{}}}", aa_base.0.len(), jf(aab), jci(aab99), jf(aa_rl.0), jci(aa_rl.1), aa_pass).unwrap();
    // ---- verdicts (order: INVALID, correctness KILL per factor, underpowered, rules) ----
    let v_l1 = if invalid {
        "INVALID".to_string()
    } else if kill_l1 {
        "KILL-CORRECTNESS".into()
    } else if underpowered {
        "INCONCLUSIVE-UNDERPOWERED".into()
    } else {
        verdict_effect(bc.ci95[0].0, bc.ci95[0].1).into()
    };
    let v_c1 = if invalid {
        "INVALID".to_string()
    } else if kill_c1 {
        "KILL-CORRECTNESS".into()
    } else if underpowered {
        "INCONCLUSIVE-UNDERPOWERED".into()
    } else {
        verdict_effect(bc.ci95[1].0, bc.ci95[1].1).into()
    };
    let (v_int, v_comp) = if invalid {
        ("INVALID".to_string(), "INVALID".to_string())
    } else if kill_l1 || kill_c1 {
        ("KILL-CORRECTNESS".into(), "KILL-CORRECTNESS".into())
    } else if underpowered {
        ("INCONCLUSIVE-UNDERPOWERED".into(), "INCONCLUSIVE-UNDERPOWERED".into())
    } else {
        (verdict_interaction(bc.ci95[3].0, bc.ci95[3].1), verdict_compose(bc.ci95[4], bc.ci95[5]).to_string())
    };
    write!(j, ",\"gates_failed\":[{}]", gates.iter().map(|g| format!("\"{g}\"")).collect::<Vec<_>>().join(",")).unwrap();
    let block = |b: &Boot| -> String {
        let mut s = String::from("{");
        for k in 0..6 {
            if k > 0 {
                s.push(',');
            }
            write!(s, "\"{}\":{{\"point\":{},\"ci95\":{},\"ci99\":{}}}", RNAMES[k], jf(b.point[k]), jci(b.ci95[k]), jci(b.ci99[k])).unwrap();
        }
        s.push('}');
        s
    };
    write!(
        j,
        ",\"counted\":{{\"unit\":\"bitop/v1 counted lower bound (learning RREF at its upper bound)\",\"ratios\":{},\"verdict\":{{\"L1\":\"{v_l1}\",\"C1\":\"{v_c1}\",\"interaction\":\"{v_int}\",\"composition\":\"{v_comp}\"}}}}",
        block(&bc)
    )
    .unwrap();
    // ---- wall (indicative) ----
    let wrows: Vec<&Vec<String>> = prim.iter().copied().filter(|r| t.get(r, "wall_run") == "1").collect();
    if wrows.len() >= 2 {
        let wall = cols_of(&t, &wrows, "W");
        let wb = boot(&wall, seed ^ 0x3A11, B_BOOT);
        let x: Vec<f64> = wrows.iter().map(|r| t.num(r, "W_base")).collect();
        let y: Vec<f64> = wrows.iter().map(|r| t.num(r, "W_base_aa")).collect();
        let (aw, aw99) = boot_pair(&x, &y, seed ^ 0x3A12);
        let aa_ok = aw99.0 <= 1.0 && aw99.1 >= 1.0;
        let floor_ok = aw99.0 >= 1.0 - AA_WALL_BAND && aw99.1 <= 1.0 + AA_WALL_BAND;
        let isolated = ctl.get("host_isolated").map(String::as_str) == Some("1");
        let label = if isolated { "isolated host" } else { "INDICATIVE (non-isolated host)" };
        let wv = |lo: f64, hi: f64| -> String {
            if invalid {
                "INVALID".into()
            } else if !aa_ok {
                "INVALID-WALL-AA".into()
            } else if !floor_ok {
                "NO-WALL-VERDICT-NOISE-FLOOR".into()
            } else {
                verdict_effect(lo, hi).into()
            }
        };
        let wi = if invalid || !aa_ok || !floor_ok { "NO-WALL-VERDICT".to_string() } else { verdict_interaction(wb.ci95[3].0, wb.ci95[3].1) };
        let wc = if invalid || !aa_ok || !floor_ok { "NO-WALL-VERDICT".to_string() } else { verdict_compose(wb.ci95[4], wb.ci95[5]).to_string() };
        write!(
            j,
            ",\"wall\":{{\"label\":\"{label}\",\"metric\":\"thread CPU ns of the arm window\",\"targets\":{},\"aa_base_ratio\":{},\"aa_ci99\":{},\"aa_pass\":{aa_ok},\"noise_floor_within_5pct\":{floor_ok},\"ratios\":{},\"verdict\":{{\"L1\":\"{}\",\"C1\":\"{}\",\"interaction\":\"{wi}\",\"composition\":\"{wc}\"}}}}",
            wrows.len(),
            jf(aw),
            jci(aw99),
            block(&wb),
            wv(wb.ci95[0].0, wb.ci95[0].1),
            wv(wb.ci95[1].0, wb.ci95[1].1)
        )
        .unwrap();
    }
    // ---- secondary (descriptive; decides nothing) ----
    let bn = boot(&counted_nl, seed ^ 0x5EC1, B_BOOT);
    let bk = boot(&k1, seed ^ 0x5EC2, B_BOOT);
    let mean = |name: &str| prim.iter().map(|r| t.num(r, name) + 1.0).sum::<f64>() / n.max(1) as f64;
    let geo = |a: usize, b: usize| -> f64 { (counted.iter().map(|c| (c[a] / c[b]).ln()).sum::<f64>() / n.max(1) as f64).exp() };
    let l1share = prim.iter().map(|r| t.num(r, "L1work_l1")).sum::<f64>() / prim.iter().map(|r| t.num(r, "C_l1")).sum::<f64>();
    let fsum: f64 = prim.iter().map(|r| t.num(r, "F_audit")).sum();
    let msum: f64 = prim.iter().map(|r| t.num(r, "M_audit")).sum();
    let rej: f64 = prim.iter().map(|r| t.num(r, "rejected_c1") + t.num(r, "rejected_l1c1")).sum();
    write!(
        j,
        ",\"secondary\":{{\"counted_without_learning_bound\":{},\"first_success_K1\":{},\"geomean_paired_ratio\":{{\"R_L1\":{},\"R_C1\":{},\"R_L1C1\":{}}},\"mean_residuals_to_K_base\":{},\"mean_residuals_to_K_l1\":{},\"l1_added_work_share_of_l1_arm\":{},\"audit_F\":{fsum},\"audit_M\":{msum},\"audit_rho_hat\":{},\"trace_rejections\":{rej}}}",
        block(&bn),
        block(&bk),
        jf(geo(1, 0)),
        jf(geo(2, 0)),
        jf(geo(3, 0)),
        jf(mean("tau_base")),
        jf(mean("tau_l1")),
        jf(l1share),
        jf(msum / fsum)
    )
    .unwrap();
    write!(
        j,
        ",\"controls\":{{{}}},\"rho_distance\":\"not computed: component (descent-phase) measurement; no leaderboard row (critic F3/F5)\"}}",
        {
            let mut k: Vec<_> = ctl.iter().collect();
            k.sort();
            k.iter().map(|(a, b)| format!("\"{a}\":\"{b}\"")).collect::<Vec<_>>().join(",")
        }
    )
    .unwrap();
    std::fs::write(out, &j).expect("write analysis");
    println!("{j}");
}

// ---------- design-time power (reads no data) ----------
/// Model [pcc:unverified]: per target, arm A success per residual pA_t = pA * LN(0, 0.15)
/// (pA = 554/393,216, l1-paired S_A); an arm B success event per residual at rate
/// g * pA_t, of which a fraction 1/g are arm A events. tau = index of the 2nd event,
/// censored at NCAP = 8192. Cost in production-residual units: base (tau_A+1);
/// L1 (tau_B+1)(1+delta); C1 (tau_A+1) r + lambda; L1+C1 (tau_B+1)(r+delta) + lambda,
/// with r = 2,483,262,669,096 / 4,358,996,976,408 (descent-walltime, seed 389) and
/// lambda = 112,775,271,840 / (4,358,996,976,408 / 3,415) (learning bound, residual units).
fn power(args: &[String]) {
    let sims: usize = args[0].parse().expect("SIMS");
    let b: usize = args[1].parse().expect("BOOT");
    let out = &args[2];
    let pa = 554.0 / 393_216.0;
    let r = 2_483_262_669_096.0 / 4_358_996_976_408.0;
    let lambda = 112_775_271_840.0 / (4_358_996_976_408.0 / 3415.0);
    let ncap = 8192u64;
    let mut csv = String::from("g_true,delta,T,sims,boot,P_L1_reduces,P_L1_material,P_L1_increases,P_C1_material,mean_I_hat,P_I_within_10pct,P_I_sub_multiplicative,P_composes,mean_R_L1_hat,mean_R_C1_hat\n");
    let mut rng = Rng(0x00C1_C1C0_57D3_5E6E);
    for &(g, delta) in &[(1.0, 0.01), (1.15, 0.01), (1.3, 0.01), (1.6, 0.01), (2.42, 0.01), (2.42, 0.05)] {
        for &tt in &[50usize, 96, 128] {
            let mut cnt = [0usize; 8];
            let (mut si, mut srl, mut src) = (0.0, 0.0, 0.0);
            for _ in 0..sims {
                let mut cols = Vec::with_capacity(tt);
                for _ in 0..tt {
                    let pat = (pa * (0.15 * rng.normal() - 0.5 * 0.0225).exp()).min(1.0);
                    let qb = (g * pat).min(1.0);
                    // walk the B events; each is an A event with probability 1/g
                    let (mut pos, mut nb, mut na) = (0u64, 0u32, 0u32);
                    let (mut tb, mut ta) = (None, None);
                    loop {
                        pos += rng.geometric(qb);
                        if pos >= ncap {
                            break;
                        }
                        nb += 1;
                        if nb == 2 && tb.is_none() {
                            tb = Some(pos);
                        }
                        if rng.unit() < 1.0 / g {
                            na += 1;
                            if na == 2 {
                                ta = Some(pos);
                                break;
                            }
                        }
                        pos += 1;
                    }
                    let na_ = (ta.unwrap_or(ncap - 1) + 1) as f64;
                    let nb_ = (tb.unwrap_or(ncap - 1).min(ta.unwrap_or(ncap - 1)) + 1) as f64;
                    let l1 = if g == 1.0 { na_ * (1.0 + delta) } else { nb_ * (1.0 + delta) };
                    let l1c1 = if g == 1.0 { na_ * (r + delta) + lambda } else { nb_ * (r + delta) + lambda };
                    cols.push([na_, l1, na_ * r + lambda, l1c1]);
                }
                let bt = boot(&cols, rng.next(), b);
                let (lo, hi) = bt.ci95[0];
                cnt[0] += usize::from(hi < 1.0);
                cnt[1] += usize::from(hi < MATERIAL_LINE);
                cnt[2] += usize::from(lo > 1.0);
                cnt[3] += usize::from(bt.ci95[1].1 < MATERIAL_LINE);
                let (ilo, ihi) = bt.ci95[3];
                cnt[4] += usize::from(ilo >= 1.0 - EQUIV_MARGIN && ihi <= 1.0 + EQUIV_MARGIN);
                cnt[5] += usize::from(ilo > 1.0);
                cnt[6] += usize::from(bt.ci95[4].1 < 1.0 && bt.ci95[5].1 < 1.0);
                si += bt.point[3];
                srl += bt.point[0];
                src += bt.point[1];
            }
            let f = |c: usize| c as f64 / sims as f64;
            writeln!(
                csv,
                "{g},{delta},{tt},{sims},{b},{:.3},{:.3},{:.3},{:.3},{:.4},{:.3},{:.3},{:.3},{:.4},{:.4}",
                f(cnt[0]),
                f(cnt[1]),
                f(cnt[2]),
                f(cnt[3]),
                si / sims as f64,
                f(cnt[4]),
                f(cnt[5]),
                f(cnt[6]),
                srl / sims as f64,
                src / sims as f64
            )
            .unwrap();
        }
    }
    std::fs::write(out, &csv).expect("write power csv");
    print!("{csv}");
}

/// Wall-arm order of one target: a Fisher-Yates permutation from the target's
/// derived `l1c1-order` seed. `base-aa` is the A/A repeat of the base arm.
fn order(args: &[String]) {
    let mut rng = Rng(args[0].parse().expect("SEED"));
    let mut v = ["base", "l1", "c1", "l1c1", "base-aa"];
    for i in (1..v.len()).rev() {
        let j = rng.below(i as u64 + 1) as usize;
        v.swap(i, j);
    }
    println!("{}", v.join(" "));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("analyze") if args.len() == 7 => analyze(&args[2..]),
        Some("power") if args.len() == 5 => power(&args[2..]),
        Some("order") if args.len() == 3 => order(&args[2..]),
        _ => {
            eprintln!("usage: l1c1_analyze analyze TARGETS AA SEED CONTROLS OUT | power SIMS BOOT OUT_CSV | order SEED");
            std::process::exit(2);
        }
    }
}
