// Analysis: reads results/*.tsv and gp_verify.txt, writes results.json and results/auto_tables.md
use crate::util::*;
use std::fmt::Write as FW;

#[derive(Clone, Debug)]
pub enum J {
    N(f64),
    S(String),
    B(bool),
    A(Vec<J>),
    O(Vec<(String, J)>),
}
impl J {
    pub fn s(&self, out: &mut String) {
        match self {
            J::N(v) => {
                if v.is_finite() {
                    if v.fract() == 0.0 && v.abs() < 1e15 {
                        write!(out, "{}", *v as i64).unwrap()
                    } else {
                        write!(out, "{:.9e}", v).unwrap()
                    }
                } else {
                    out.push_str("null")
                }
            }
            J::S(t) => {
                out.push('"');
                out.push_str(&t.replace('\\', "\\\\").replace('"', "\\\""));
                out.push('"');
            }
            J::B(b) => out.push_str(if *b { "true" } else { "false" }),
            J::A(a) => {
                out.push('[');
                for (i, x) in a.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    x.s(out);
                }
                out.push(']');
            }
            J::O(o) => {
                out.push('{');
                for (i, (k, v)) in o.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push('"');
                    out.push_str(k);
                    out.push_str("\":");
                    v.s(out);
                }
                out.push('}');
            }
        }
    }
}
fn n(v: f64) -> J {
    J::N(v)
}
fn st(v: &str) -> J {
    J::S(v.to_string())
}

fn sum(rs: &[Rec], k: &str) -> f64 {
    rs.iter().map(|r| r.get(k)).sum()
}
fn nat(r: &Rec, p: &str) -> f64 {
    ["s", "m", "t", "x", "l", "la"].iter().map(|f| r.get(&format!("{}_{}", p, f))).sum()
}
fn natsum(rs: &[Rec], p: &str) -> f64 {
    rs.iter().map(|r| nat(r, p)).sum()
}

// exact Clopper-Pearson via bisection on the binomial CDF (log-space)
fn lbinom_tail_le(x: u64, nn: u64, p: f64) -> f64 {
    // P(X <= x)
    let mut lf = vec![0f64; (nn + 1).min(2_000_000) as usize];
    let mut s = 0.0;
    let ln = |i: u64| -> f64 { (i as f64).ln() };
    let _ = &mut lf;
    let mut logc = 0.0; // log C(nn,0)
    let lp = p.ln();
    let lq = (1.0 - p).ln();
    let mut tot = 0.0;
    for i in 0..=x {
        if i > 0 {
            logc += ln(nn - i + 1) - ln(i);
        }
        s = logc + i as f64 * lp + (nn - i) as f64 * lq;
        tot += s.exp();
    }
    let _ = s;
    tot
}
pub fn cp95(x: u64, nn: u64) -> (f64, f64) {
    if nn == 0 {
        return (0.0, 1.0);
    }
    let hi = if x >= nn {
        1.0
    } else {
        let (mut a, mut b) = (0.0f64, 1.0f64);
        for _ in 0..80 {
            let mid = 0.5 * (a + b);
            if lbinom_tail_le(x, nn, mid) > 0.025 { a = mid } else { b = mid }
        }
        0.5 * (a + b)
    };
    let lo = if x == 0 {
        0.0
    } else {
        let (mut a, mut b) = (0.0f64, 1.0f64);
        for _ in 0..80 {
            let mid = 0.5 * (a + b);
            if 1.0 - lbinom_tail_le(x - 1, nn, mid) < 0.025 { a = mid } else { b = mid }
        }
        0.5 * (a + b)
    };
    (lo, hi)
}

fn boot_ratio(num: &[(f64, f64)], den: &[(f64, f64)], c: f64, nb: usize) -> (f64, f64, f64) {
    // each entry (G, N) per curve; ratio of sums of (G + c N)
    let m = num.len();
    let val = |idx: &[usize]| -> f64 {
        let a: f64 = idx.iter().map(|&i| num[i].0 + c * num[i].1).sum();
        let b: f64 = idx.iter().map(|&i| den[i].0 + c * den[i].1).sum();
        a / b
    };
    let all: Vec<usize> = (0..m).collect();
    let pt = val(&all);
    let mut rng = Rng(MSEED);
    let mut v = vec![];
    for _ in 0..nb {
        let idx: Vec<usize> = (0..m).map(|_| rng.below(m as u64) as usize).collect();
        v.push(val(&idx));
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (pt, v[(0.025 * nb as f64) as usize], v[((0.975 * nb as f64) as usize).min(nb - 1)])
}

fn rank3(a: [f64; 3], b: [f64; 3], eq_g: bool) -> &'static str {
    // A vs B: "A<B", "B<A", "unordered"
    let strict_ab = a[1] < b[1] && a[2] < b[2] && (a[0] < b[0] || (eq_g && a[0] <= b[0]));
    let strict_ba = b[1] < a[1] && b[2] < a[2] && (b[0] < a[0] || (eq_g && b[0] <= a[0]));
    if strict_ab {
        if a[0] == b[0] { "A<B (weak: tie at c=0)" } else { "A<B" }
    } else if strict_ba {
        if a[0] == b[0] { "B<A (weak: tie at c=0)" } else { "B<A" }
    } else {
        "unordered across the band"
    }
}

const REG: [(usize, u32, f64, f64, f64, f64, f64); 9] = [
    // m, k, a, h1, h2, R_registered, lambda
    (2, 17, 27.0, 38.0, 54.0, 1.333, 1.0),
    (2, 19, 30.0, 43.0, 61.0, 1.330, 1.0),
    (2, 23, 35.0, 52.0, 74.0, 1.321, 1.0),
    (2, 29, 41.0, 64.0, 92.0, 1.308, 1.0),
    (2, 31, 44.0, 69.0, 99.0, 1.308, 1.0),
    (3, 17, 27.0, 38.0, 60.0, 1.724, 1.0 / 3.0),
    (3, 19, 30.0, 43.0, 67.0, 1.723, 2.0 / 3.0),
    (3, 23, 35.0, 52.0, 82.0, 1.707, 1.0 / 3.0),
    (3, 29, 41.0, 64.0, 102.0, 1.688, 1.0 / 3.0),
];

pub fn analyze(dir: &str) {
    let mut md = String::new();
    let mut jroot: Vec<(String, J)> = vec![];
    let cprices = [0.0f64, 0.1, 1.0];
    let mut f1_total_viol = 0.0;
    let mut f2_reject = 0;
    let mut f2_sd_bad = false;
    let mut f3_out = 0;
    let mut f3_ci_bad = false;
    let mut f4_mis = 0.0;
    let mut f5_bad = false;
    let mut f5_mag_bad = false;
    let mut f9_bad = 0.0;
    let mut cells_json = vec![];
    writeln!(md, "## A. Binary arms (H1-H4): per cell counters\n").unwrap();
    writeln!(md, "| m | k | l | curves | targets | unique sol. | per target (pred) | flagged tg | violations (non-D twist coord.) | 95% upper bound | D-family sol. | all-E | F4 U!=F | avoided lifts (non-D) | lift-fail sol. = twist sol. | powered |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    struct CellAgg {
        m: usize,
        k: u32,
        recs: Vec<Rec>,
    }
    let mut cells: Vec<CellAgg> = vec![];
    for &(m, k, ..) in REG.iter() {
        let recs = read_tsv(&format!("{}/results/arms_B_m{}_k{}.tsv", dir, m, k));
        if recs.is_empty() {
            continue;
        }
        cells.push(CellAgg { m, k, recs });
    }
    let mut h2rows = String::new();
    let mut h3rows = String::new();
    let mut gaerows = String::new();
    writeln!(h2rows, "| m | k | l | N_z | mean z | sd z | reject (alpha=0.01) | mean f(twist) arm V | 1-mu_E mean | call ratio U/F (mean per curve) | ref | mean z_call | sd z_call | within band (declared rule) | known-sd variant (|mean z| <= 2.576/sqrt(curves)) |").unwrap();
    writeln!(h2rows, "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    writeln!(h3rows, "| m | k | R_registered | R_instrument-model(a,h from measured) | R_hat native (U/F, pre amortised) | in +-10% | cost ratio c=0.1 [95% CI] | cost ratio c=1 [95% CI] | CI excludes 1 (c>=0.1) | a_hat | h1_hat | h2_hat(mem incl.) | S/U native | S/U in [1,1.05] |").unwrap();
    writeln!(h3rows, "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    writeln!(gaerows, "| m | k | arm | G/target | N/target | cost c=0 | cost c=0.1 | cost c=1 | job wall s per target (whole job: U+F+S+census+verification; contention-affected, secondary) |").unwrap();
    writeln!(gaerows, "|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut rankrows = String::new();
    writeln!(rankrows, "| m | k | F vs U | S vs U |").unwrap();
    writeln!(rankrows, "|---|---|---|---|").unwrap();
    for ca in &cells {
        let (m, k) = (ca.m, ca.k);
        let rs = &ca.recs;
        let reg = REG.iter().find(|r| r.0 == m && r.1 == k).unwrap();
        let tg = sum(rs, "u_targets");
        let uniq = sum(rs, "u_uniq");
        let pure_nd = sum(rs, "u_cls_pure_nd");
        let mix_nd = sum(rs, "u_cls_mix_nd");
        let d_sol = sum(rs, "u_cls_pure_d") + sum(rs, "u_cls_mix_d");
        let alle = sum(rs, "u_cls_alle");
        let viol = pure_nd + mix_nd;
        f1_total_viol += viol;
        let (_lo, hi) = cp95(viol as u64, uniq as u64);
        let unlift = sum(rs, "l_rel_unlift");
        let f4 = sum(rs, "u_f4_mis");
        f4_mis += f4;
        f9_bad += sum(rs, "l_rel_bad");
        let avoided_nond = if viol == 0.0 { 0.0 } else { sum(rs, "s_s_avoid_lifts") };
        if avoided_nond > 0.0 {
            f5_bad = true;
        }
        writeln!(md, "| {} | {} | {} | {} | {} | {} | {:.3} ({:.3}) | {} | {} | {:.2e} | {} | {} | {} | {} | {} | {} |", m, k, rs[0].get("l"), rs.len(), tg, uniq, uniq / tg, reg.6, sum(rs, "u_flagged"), viol, hi, d_sol, alle, f4, avoided_nond, if unlift == viol + d_sol { "consistent".to_string() } else { format!("MISMATCH {} vs {}", unlift, viol + d_sol) }, if uniq >= 1000.0 { "yes" } else { "UNDERPOWERED" }).unwrap();
        // H2: pooled z
        let mut zs_n = 0.0;
        let mut zs_s = 0.0;
        let mut zs_s2 = 0.0;
        for r in rs {
            let z = r.get("z_arm");
            zs_n += 1.0 + r.get("cz_n");
            zs_s += z + r.get("cz_sum");
            zs_s2 += z * z + r.get("cz_sum2");
        }
        let zm = zs_s / zs_n;
        let zsd = ((zs_s2 - zs_n * zm * zm) / (zs_n - 1.0)).sqrt();
        let rej = zm.abs() > 2.576 * zsd / zs_n.sqrt();
        if rej {
            f2_reject += 1;
        }
        if zsd < 0.8 || zsd > 1.25 {
            f2_sd_bad = true;
        }
        let mean_f: f64 = rs.iter().map(|r| r.get("tw_arm") / r.get("s")).sum::<f64>() / rs.len() as f64;
        let mean_1mu: f64 = rs.iter().map(|r| 1.0 - r.get("mu")).sum::<f64>() / rs.len() as f64;
        // call ratio z per curve
        let refr = if m == 2 { 2.0 } else { 4.0 };
        let mut zc = vec![];
        let mut crs = vec![];
        for r in rs {
            let ratio = r.get("u_st1") / r.get("f_st1");
            crs.push(ratio);
            let s = r.get("s");
            let mu = r.get("mu");
            let sdf = (mu * (1.0 - mu) / s).sqrt();
            let sdr = if m == 2 { 4.0 * sdf } else { 16.0 * sdf };
            zc.push((ratio - refr) / sdr);
        }
        let (zcm, zcs) = mean_sd(&zc);
        let call_ok = zcm.abs() <= 2.576 * zcs / (zc.len() as f64).sqrt();
        writeln!(h2rows, "| {} | {} | {} | {} | {:+.4} | {:.4} | {} | {:.5} | {:.5} | {:.4} | {} | {:+.3} | {:.3} | {} | {} |", m, k, rs[0].get("l"), zs_n, zm, zsd, if rej { "REJECT" } else { "no" }, mean_f, mean_1mu, crs.iter().sum::<f64>() / crs.len() as f64, refr, zcm, zcs, if call_ok { "yes" } else { "NO" }, if zcm.abs() <= 2.576 / (zc.len() as f64).sqrt() { "yes" } else { "NO" }).unwrap();
        // H3
        let un = natsum(rs, "u");
        let fnn = natsum(rs, "f");
        let rhat = un / fnn;
        let in10 = (rhat / reg.5 - 1.0).abs() <= 0.10;
        if !in10 {
            f3_out += 1;
        }
        let gcommon: Vec<f64> = rs.iter().map(|r| r.get("c_g") + r.get("l_g")).collect();
        let numv: Vec<(f64, f64)> = rs.iter().zip(&gcommon).map(|(r, g)| (*g, nat(r, "u"))).collect();
        let denv: Vec<(f64, f64)> = rs.iter().zip(&gcommon).map(|(r, g)| (*g, nat(r, "f"))).collect();
        let b01 = boot_ratio(&numv, &denv, 0.1, 10000);
        let b1 = boot_ratio(&numv, &denv, 1.0, 10000);
        let ci_ok = b01.1 > 1.0 && b1.1 > 1.0;
        if !ci_ok {
            f3_ci_bad = true;
        }
        let a_hat = sum(rs, "u_ops_fail") / sum(rs, "u_n_fail");
        let h1_hat = sum(rs, "u_ops_pass") / sum(rs, "u_n_pass") - a_hat;
        let mem = 2.0 * (k as f64 - rs[0].get("l"));
        let h2_hat = h1_hat + mem;
        let su = (un + natsum(rs, "s")) / un;
        if su < 1.0 || su > 1.05 {
            f5_mag_bad = true;
        }
        let pre_share = {
            let pre: f64 = rs.iter().map(|r| r.get("s")).sum::<f64>(); // placeholder replaced below
            let _ = pre;
            0.0
        };
        let _ = pre_share;
        writeln!(h3rows, "| {} | {} | {:.3} | {:.3} | {:.4} | {} | {:.4} [{:.4}, {:.4}] | {:.4} [{:.4}, {:.4}] | {} | {:.2} | {:.2} | {:.2} | {:.5} | {} |", m, k, reg.5, if m == 2 { (2.0 * a_hat + h2_hat) / (a_hat + h2_hat) } else { (8.0 * a_hat + 2.0 * h1_hat + 2.0 * h2_hat) / (3.0 * a_hat + h1_hat + 2.0 * h2_hat) }, rhat, if in10 { "yes" } else { "NO" }, b01.0, b01.1, b01.2, b1.0, b1.1, b1.2, if ci_ok { "yes" } else { "NO" }, a_hat, h1_hat, h2_hat, su, if su >= 1.0 && su <= 1.05 { "yes" } else { "NO" }).unwrap();
        // GAE
        let wall: f64 = sum(rs, "wall_s");
        let gper = gcommon.iter().sum::<f64>() / tg;
        let arms = [("U", un), ("F", fnn), ("S", un + natsum(rs, "s"))];
        let mut costs = vec![];
        for (nm, nn) in &arms {
            let np = nn / tg;
            let c3 = [gper, gper + 0.1 * np, gper + np];
            costs.push(c3);
            writeln!(gaerows, "| {} | {} | {} | {:.3} | {:.1} | {:.3} | {:.3} | {:.3} | {:.5} |", m, k, nm, gper, np, c3[0], c3[1], c3[2], wall / tg).unwrap();
        }
        writeln!(rankrows, "| {} | {} | {} | {} |", m, k, rank3(costs[1], costs[0], true).replace("A<B", "F<U").replace("B<A", "U<F"), rank3(costs[0], costs[2], true).replace("A<B", "U<S").replace("B<A", "S<U")).unwrap();
        cells_json.push(J::O(vec![
            ("m".into(), n(m as f64)),
            ("k".into(), n(k as f64)),
            ("targets".into(), n(tg)),
            ("unique_solutions".into(), n(uniq)),
            ("violations".into(), n(viol)),
            ("violation_upper95".into(), n(hi)),
            ("d_family".into(), n(d_sol)),
            ("f4_mismatch_targets".into(), n(f4)),
            ("z_mean".into(), n(zm)),
            ("z_sd".into(), n(zsd)),
            ("z_reject".into(), J::B(rej)),
            ("call_ratio_mean".into(), n(crs.iter().sum::<f64>() / crs.len() as f64)),
            ("call_ratio_ref".into(), n(refr)),
            ("R_registered".into(), n(reg.5)),
            ("R_hat_native".into(), n(rhat)),
            ("R_hat_cost_c0.1".into(), J::A(vec![n(b01.0), n(b01.1), n(b01.2)])),
            ("R_hat_cost_c1".into(), J::A(vec![n(b1.0), n(b1.1), n(b1.2)])),
            ("S_over_U".into(), n(su)),
            ("native_U".into(), n(un)),
            ("native_F".into(), n(fnn)),
            ("G_common_per_target".into(), n(gper)),
            ("z_arm_list".into(), J::A(rs.iter().map(|r| n(r.get("z_arm"))).collect())),
            ("cost_per_target_U".into(), J::A(costs[0].iter().map(|x| n(*x)).collect())),
            ("cost_per_target_F".into(), J::A(costs[1].iter().map(|x| n(*x)).collect())),
            ("cost_per_target_S".into(), J::A(costs[2].iter().map(|x| n(*x)).collect())),
        ]));
    }
    md.push_str("\n_Violations = unique solutions with a twist-type coordinate that do not match the degenerate family D. Coin-flip model would give 0.75 (m=2) / 0.875 (m=3) of solutions as non-liftable. 'F4 U!=F' counts unflagged targets whose unique solution sets differ between arms U and F._\n");
    writeln!(md, "\n## B. H2: factor-base column census (z = (f - (1-mu_E))/sd, sd = sqrt(mu(1-mu)/s)); call ratio check\n").unwrap();
    md.push_str(&h2rows);
    writeln!(md, "\n## C. H3/H4: counted cost ratios (native operations; G common to both arms enters the cost columns)\n").unwrap();
    md.push_str(&h3rows);
    writeln!(md, "\n## D. GAE band per target (cost(c) = G + c N; N = S+M+T+X+L native operations of the solver incl. precompute; lifts and LA excluded)\n").unwrap();
    md.push_str(&gaerows);
    writeln!(md, "\n### Ranking across the band (section 3 rule)\n").unwrap();
    md.push_str(&rankrows);
    jroot.push(("binary_cells".into(), J::A(cells_json)));

    // ---------- prime control ----------
    writeln!(md, "\n## E. Prime-field control (H5)\n").unwrap();
    writeln!(md, "| m | k | l | targets | unique sol. | violations (non-D) | 95% upper | D-family | all-E | F4 U!=F | call ratio mean (ref) | mean z_call | sd z_call | within band (declared rule) | known-sd variant | R_hat native | R registered-formula from measured a,h | z pooled mean (sd) |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut f6_viol = 0.0;
    let mut f6_call_bad = 0;
    let mut pjson = vec![];
    for &(m, k) in &[(2usize, 17u32), (2, 19), (2, 23), (2, 29), (2, 31), (3, 17), (3, 19), (3, 23)] {
        let rs = read_tsv(&format!("{}/results/arms_P_m{}_k{}.tsv", dir, m, k));
        if rs.is_empty() {
            continue;
        }
        let tg = sum(&rs, "u_targets");
        let uniq = sum(&rs, "u_uniq");
        let viol = sum(&rs, "u_cls_pure_nd") + sum(&rs, "u_cls_mix_nd");
        f6_viol += viol;
        f4_mis += sum(&rs, "u_f4_mis");
        f9_bad += sum(&rs, "l_rel_bad");
        let (_, hi) = cp95(viol as u64, uniq as u64);
        let refr = if m == 2 { 2.0 } else { 4.0 };
        let mut zc = vec![];
        let mut crs = vec![];
        for r in &rs {
            let ratio = r.get("u_st1") / r.get("f_st1");
            crs.push(ratio);
            let s = r.get("s");
            let mu = r.get("mu");
            let sdf = (mu * (1.0 - mu) / s).sqrt();
            zc.push((ratio - refr) / (if m == 2 { 4.0 * sdf } else { 16.0 * sdf }));
        }
        let (zcm, zcs) = mean_sd(&zc);
        let call_ok = zcm.abs() <= 2.576 * zcs / (zc.len() as f64).sqrt();
        if !call_ok {
            f6_call_bad += 1;
        }
        let un = natsum(&rs, "u");
        let fnn = natsum(&rs, "f");
        let a_hat = sum(&rs, "u_ops_fail") / sum(&rs, "u_n_fail");
        let h1_hat = sum(&rs, "u_ops_pass") / sum(&rs, "u_n_pass") - a_hat;
        let h2_hat = h1_hat + 2.0; // two L lookups
        let rform = if m == 2 { (2.0 * a_hat + h2_hat) / (a_hat + h2_hat) } else { (8.0 * a_hat + 2.0 * h1_hat + 2.0 * h2_hat) / (3.0 * a_hat + h1_hat + 2.0 * h2_hat) };
        let mut zn = 0.0;
        let mut zs = 0.0;
        let mut zs2 = 0.0;
        for r in &rs {
            let z = r.get("z_arm");
            zn += 1.0 + r.get("cz_n");
            zs += z + r.get("cz_sum");
            zs2 += z * z + r.get("cz_sum2");
        }
        let zm = zs / zn;
        let zsd = ((zs2 - zn * zm * zm) / (zn - 1.0)).sqrt();
        writeln!(md, "| {} | {} | {} | {} | {} | {} | {:.2e} | {} | {} | {} | {:.4} ({}) | {:+.3} | {:.3} | {} | {} | {:.4} | {:.4} | {:+.3} ({:.3}) |", m, k, rs[0].get("l"), tg, uniq, viol, hi, sum(&rs, "u_cls_pure_d") + sum(&rs, "u_cls_mix_d"), sum(&rs, "u_cls_alle"), sum(&rs, "u_f4_mis"), crs.iter().sum::<f64>() / crs.len() as f64, refr, zcm, zcs, if call_ok { "yes" } else { "NO" }, if zcm.abs() <= 2.576 / (zc.len() as f64).sqrt() { "yes" } else { "NO" }, un / fnn, rform, zm, zsd).unwrap();
        pjson.push(J::O(vec![("m".into(), n(m as f64)), ("k".into(), n(k as f64)), ("targets".into(), n(tg)), ("unique".into(), n(uniq)), ("violations".into(), n(viol)), ("call_ratio_mean".into(), n(crs.iter().sum::<f64>() / crs.len() as f64)), ("R_hat_native".into(), n(un / fnn)), ("R_formula".into(), n(rform)), ("a_hat".into(), n(a_hat)), ("h1_hat".into(), n(h1_hat))]));
    }
    jroot.push(("prime_cells".into(), J::A(pjson)));

    // ---------- controls ----------
    writeln!(md, "\n## F. Instrument validity: positive controls (F7)\n").unwrap();
    let mut f7_fail: Vec<String> = vec![];
    let pca = read_tsv(&format!("{}/results/pca.tsv", dir));
    let gpv = std::fs::read_to_string(format!("{}/results/gp_verify.txt", dir)).unwrap_or_default();
    let mut pca_gp_mis = 0u64;
    let mut pca_gp_n = 0u64;
    let mut samp_lmis = 0u64;
    let mut samp_rfail = 0u64;
    let mut samp_n = 0u64;
    let mut samp_rel = 0u64;
    for l in gpv.lines() {
        let get = |key: &str| -> u64 { l.split_whitespace().find(|t| t.starts_with(key)).and_then(|t| t[key.len()..].parse().ok()).unwrap_or(0) };
        if l.starts_with("PCA") {
            pca_gp_n += get("n=");
            pca_gp_mis += get("mismatch=");
        } else if l.starts_with("SAMPLE") {
            samp_n += get("n=");
            samp_rel += get("relchecked=");
            samp_lmis += get("label_mismatch=");
            samp_rfail += get("rel_fail=");
        }
    }
    let pca_rust_n = sum(&pca, "n");
    let pca_rust_agree = sum(&pca, "agree_lift");
    writeln!(md, "* PC-A: Rust classifier vs Rust explicit lift: {}/{} agree; Rust labels vs GP `ellordinate`: {} points, {} mismatches (binary and prime, k in 17,19,23,29,31).", pca_rust_agree, pca_rust_n, pca_gp_n, pca_gp_mis).unwrap();
    if pca_rust_agree != pca_rust_n || pca_gp_mis != 0 || pca_gp_n == 0 {
        f7_fail.push("PC-A".into());
    }
    let pcb = read_tsv(&format!("{}/results/pcb.tsv", dir));
    let mut pcb_rows = String::new();
    writeln!(pcb_rows, "| field | m | k | curves | targets | solutions | all twist | mixed | with E coords only | waste fraction |").unwrap();
    writeln!(pcb_rows, "|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut pcb_json = vec![];
    for field in 0..2 {
        for &m in &[2usize, 3] {
            for &k in &[17u32, 19, 23] {
                let rs: Vec<Rec> = pcb.iter().filter(|r| r.get("field") == field as f64 && r.get("m") == m as f64 && r.get("k") == k as f64).cloned().collect();
                if rs.is_empty() {
                    continue;
                }
                let ns = sum(&rs, "nsol");
                let tw = sum(&rs, "n_all_tw");
                let mx = sum(&rs, "n_mixed");
                let ae = sum(&rs, "n_all_e");
                let waste = (tw + mx) / ns;
                if waste < 0.99 || ae > 0.0 || mx > 0.0 {
                    f7_fail.push(format!("PC-B field{} m{} k{}", field, m, k));
                }
                writeln!(pcb_rows, "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.4} |", if field == 0 { "binary" } else { "prime" }, m, k, rs.len(), sum(&rs, "targets"), ns, tw, mx, ae, waste).unwrap();
                pcb_json.push(J::O(vec![("field".into(), n(field as f64)), ("m".into(), n(m as f64)), ("k".into(), n(k as f64)), ("solutions".into(), n(ns)), ("all_twist".into(), n(tw)), ("mixed".into(), n(mx)), ("all_E".into(), n(ae)), ("waste".into(), n(waste))]));
            }
        }
    }
    writeln!(md, "\n**PC-B (twist-type targets; every non-degenerate solution must have all coordinates twist-type, blindness criterion waste < 0.99):**\n").unwrap();
    md.push_str(&pcb_rows);
    let pcc = read_tsv(&format!("{}/results/pcc.tsv", dir));
    writeln!(md, "\n**PC-C (injection into stored U solutions):**\n").unwrap();
    writeln!(md, "| field | m | k | rho | records | injected | TP | FP | FN | TN | sensitivity | specificity | observed waste | expected |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    for r in &pcc {
        let (tp, fp, fnn, tn) = (r.get("tp"), r.get("fp"), r.get("fn"), r.get("tn"));
        let sens = tp / (tp + fnn);
        let spec = tn / (tn + fp);
        if sens != 1.0 || spec != 1.0 || (r.get("observed_waste") - r.get("expected_waste")).abs() > 1e-12 {
            f7_fail.push("PC-C".into());
        }
        writeln!(md, "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.1} | {:.1} | {:.6} | {:.6} |", if r.get("field") == 0.0 { "binary" } else { "prime" }, r.get("m"), r.get("k"), r.get("rho"), r.get("nrec"), r.get("ninj"), tp, fp, fnn, tn, sens, spec, r.get("observed_waste"), r.get("expected_waste")).unwrap();
    }
    let pcd = read_tsv(&format!("{}/results/pcd.tsv", dir));
    writeln!(md, "\n**PC-D (degenerate family, m=3, x(R) forced into V; predicted mixed D-multisets = |V_tw|, nothing else mixed, no pure-twist):**\n").unwrap();
    writeln!(md, "| field | k | curves | targets | sum |V_tw| | mixed D found | mixed non-D | pure twist | targets with exact count |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|").unwrap();
    for field in 0..2 {
        for &k in &[17u32, 19, 23] {
            let rs: Vec<Rec> = pcd.iter().filter(|r| r.get("field") == field as f64 && r.get("k") == k as f64).cloned().collect();
            if rs.is_empty() {
                continue;
            }
            let vt = sum(&rs, "vtw_sum");
            let md_ = sum(&rs, "mixed_D");
            let mnd = sum(&rs, "mixed_nonD");
            let pt = sum(&rs, "pure_tw");
            if vt != md_ || mnd != 0.0 || pt != 0.0 || sum(&rs, "targets_exact") != sum(&rs, "targets") {
                f7_fail.push(format!("PC-D field{} k{}", field, k));
            }
            writeln!(md, "| {} | {} | {} | {} | {} | {} | {} | {} | {}/{} |", if field == 0 { "binary" } else { "prime" }, k, rs.len(), sum(&rs, "targets"), vt, md_, mnd, pt, sum(&rs, "targets_exact"), sum(&rs, "targets")).unwrap();
        }
    }
    writeln!(md, "\n* GP independent cross-check of stored samples: {} unique solutions checked ({} full relation checks by `elladd`, remainder label-only for degenerate/twist coordinates); label mismatches {}; relation failures {}.", samp_n, samp_rel, samp_lmis, samp_rfail).unwrap();
    if samp_lmis != 0 || samp_rfail != 0 || samp_n == 0 {
        f7_fail.push("GP sample cross-check".into());
    }
    jroot.push(("pcb".into(), J::A(pcb_json)));

    // ---------- rho ----------
    writeln!(md, "\n## G. Matched Pollard rho (negation map), per size\n").unwrap();
    writeln!(md, "| k | curves x instances | solved | mean G | mean G / (sqrt(pi n)/2) | F8 band [0.8,1.6] | mean N (native) | mean lookups | steps/instance | walks/instance | cost c=0 | c=0.1 | c=1 |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut rho_cost: Vec<(u32, [f64; 3], bool)> = vec![];
    let mut f8_out: Vec<u32> = vec![];
    let mut rjson = vec![];
    for &k in &[17u32, 19, 23, 29, 31] {
        let rs = read_tsv(&format!("{}/results/rho_k{}.tsv", dir, k));
        if rs.is_empty() {
            continue;
        }
        let ni = sum(&rs, "ninst");
        let g = rs.iter().map(|r| r.get("g_mean") * r.get("ninst")).sum::<f64>() / ni;
        let nn = rs.iter().map(|r| r.get("nat_mean") * r.get("ninst")).sum::<f64>() / ni;
        let lk = rs.iter().map(|r| r.get("lookups_mean") * r.get("ninst")).sum::<f64>() / ni;
        let st_ = rs.iter().map(|r| r.get("steps_mean") * r.get("ninst")).sum::<f64>() / ni;
        let wk = rs.iter().map(|r| r.get("walks_mean") * r.get("ninst")).sum::<f64>() / ni;
        let ref_ = rs.iter().map(|r| r.get("sqrt_pi_n_over_2") * r.get("ninst")).sum::<f64>() / ni;
        let ratio = g / ref_;
        let inband = ratio >= 0.8 && ratio <= 1.6;
        if !inband {
            f8_out.push(k);
        }
        let ntot = nn + lk;
        let c3 = [g, g + 0.1 * ntot, g + ntot];
        rho_cost.push((k, c3, inband));
        writeln!(md, "| {} | {}x{} | {}/{} | {:.1} | {:.3} | {} | {:.1} | {:.1} | {:.1} | {:.2} | {:.1} | {:.1} | {:.1} |", k, rs.len(), rs[0].get("ninst"), sum(&rs, "solved"), ni, g, ratio, if inband { "in band" } else { "OUTSIDE (flagged)" }, nn, lk, st_, wk, c3[0], c3[1], c3[2]).unwrap();
        rjson.push(J::O(vec![("k".into(), n(k as f64)), ("g_mean".into(), n(g)), ("ratio_to_sqrt_pi_n_over_2".into(), n(ratio)), ("in_band".into(), J::B(inband)), ("cost_c0".into(), n(c3[0])), ("cost_c0.1".into(), n(c3[1])), ("cost_c1".into(), n(c3[2]))]));
    }
    jroot.push(("rho".into(), J::A(rjson)));

    // ---------- IC end-to-end ----------
    writeln!(md, "\n## H. Index calculus end-to-end vs matched rho (distance remaining)\n").unwrap();
    writeln!(md, "Gap = cost_IC / cost_rho at c = 0, 0.1, 1. Ranking only where the order holds at all three prices.\n").unwrap();
    writeln!(md, "| m | k | source | arm | N_rel | G | N | cost c=0 | c=0.1 | c=1 | gap c=0 | gap c=0.1 | gap c=1 | vs rho (rank rule) | rho F8 |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut fulljson = vec![];
    let mut fsrows = String::new();
    writeln!(fsrows, "| m | k | instances | registered stop rule (full column rank): success | d determined (exploratory) | d correct (dP=Q) | mean unknowns u | mean N_need | mean relations at d-determined | mean relations at registered stop/fail | mean targets at d-determined | F4/F9 failures |").unwrap();
    writeln!(fsrows, "|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let ic_cells = [(2usize, 17u32), (2, 19), (2, 23), (2, 29), (2, 31), (3, 17), (3, 19), (3, 23), (3, 29)];
    let mut dd_ratio: std::collections::HashMap<usize, f64> = std::collections::HashMap::new();
    for &(m, k) in &ic_cells {
        let fr = read_tsv(&format!("{}/results/full_m{}_k{}.tsv", dir, m, k));
        let rho = rho_cost.iter().find(|r| r.0 == k).cloned();
        let rank_vs = |c: [f64; 3]| -> String {
            match &rho {
                Some((_, rc, inb)) => {
                    let r = rank3(c, *rc, false);
                    let r = r.replace("A<B", "IC<rho").replace("B<A", "rho<IC");
                    if *inb { r } else { format!("{} (rho flagged by F8: not reported)", if r == "unordered across the band" { r } else { "withheld".to_string() }) }
                }
                None => "n/a".into(),
            }
        };
        let gap = |c: [f64; 3]| -> [f64; 3] {
            match &rho {
                Some((_, rc, _)) => [c[0] / rc[0], c[1] / rc[1], c[2] / rc[2]],
                None => [f64::NAN; 3],
            }
        };
        let rf = match &rho {
            Some((_, _, inb)) => if *inb { "in band" } else { "flagged" },
            None => "n/a",
        };
        if !fr.is_empty() {
            let ninst = fr.len() as f64;
            let reg_ok = sum(&fr, "reg_success");
            let dd = sum(&fr, "dd_found");
            let dok = sum(&fr, "d_ok");
            f9_bad += sum(&fr, "rel_bad") + (dd - dok).max(0.0);
            f4_mis += sum(&fr, "f4_mis");
            let mu = sum(&fr, "u") / ninst;
            let mn = sum(&fr, "nneed") / ninst;
            let ddr: Vec<&Rec> = fr.iter().filter(|r| r.get("dd_found") == 1.0).collect();
            let m_rc_dd = ddr.iter().map(|r| r.get("dd_rc")).sum::<f64>() / ddr.len().max(1) as f64;
            let m_tg_dd = ddr.iter().map(|r| r.get("dd_targets")).sum::<f64>() / ddr.len().max(1) as f64;
            let m_rc_reg = fr.iter().map(|r| if r.get("reg_success") == 1.0 { r.get("reg_rc") } else { r.get("rc_total") }).sum::<f64>() / ninst;
            dd_ratio.insert(m, m_rc_dd / mu);
            writeln!(fsrows, "| {} | {} | {} | {}/{} | {}/{} | {}/{} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {} |", m, k, fr.len(), reg_ok, ninst, dd, ninst, dok, ninst, mu, mn, m_rc_dd, m_rc_reg, m_tg_dd, sum(&fr, "rel_bad") + sum(&fr, "f4_mis")).unwrap();
            for (arm, key) in &[("U", "u_native"), ("F", "f_native")] {
                // exploratory (d-determined) snapshot
                let mut g_s = 0.0;
                let mut n_s = 0.0;
                let mut cnt = 0.0;
                for r in &ddr {
                    g_s += r.get("dd_setup_g") + r.get("dd_walk_g") + r.get("dd_l_g") + r.get("final_g");
                    n_s += r.get(&format!("dd_{}", key)) + r.get("dd_l_native") + r.get("dd_la_madds");
                    cnt += 1.0;
                }
                if cnt > 0.0 {
                    let (g_, n_) = (g_s / cnt, n_s / cnt);
                    let c3 = [g_, g_ + 0.1 * n_, g_ + n_];
                    let gp = gap(c3);
                    writeln!(md, "| {} | {} | measured, d-determined (exploratory stop rule) | {} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.1} | {:.1} | {:.1} | {} | {} |", m, k, arm, m_rc_dd, g_, n_, c3[0], c3[1], c3[2], gp[0], gp[1], gp[2], rank_vs(c3), rf).unwrap();
                    fulljson.push(J::O(vec![("m".into(), n(m as f64)), ("k".into(), n(k as f64)), ("arm".into(), st(arm)), ("source".into(), st("measured_d_determined")), ("G".into(), n(g_)), ("N".into(), n(n_)), ("cost".into(), J::A(c3.iter().map(|x| n(*x)).collect())), ("gap".into(), J::A(gp.iter().map(|x| n(*x)).collect()))]));
                }
                // registered-rule snapshot where it succeeded
                let sr: Vec<&Rec> = fr.iter().filter(|r| r.get("reg_success") == 1.0).collect();
                if !sr.is_empty() {
                    let mut g_s = 0.0;
                    let mut n_s = 0.0;
                    for r in &sr {
                        g_s += r.get("reg_setup_g") + r.get("reg_walk_g") + r.get("reg_l_g") + r.get("final_g");
                        n_s += r.get(&format!("reg_{}", key)) + r.get("reg_l_native") + r.get("reg_la_madds");
                    }
                    let c_ = sr.len() as f64;
                    let (g_, n_) = (g_s / c_, n_s / c_);
                    let c3 = [g_, g_ + 0.1 * n_, g_ + n_];
                    let gp = gap(c3);
                    writeln!(md, "| {} | {} | measured, registered stop rule (mean over the {} of {} instances that succeeded) | {} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.1} | {:.1} | {:.1} | {} | {} |", m, k, sr.len(), fr.len(), arm, sr.iter().map(|r| r.get("reg_rc")).sum::<f64>() / c_, g_, n_, c3[0], c3[1], c3[2], gp[0], gp[1], gp[2], rank_vs(c3), rf).unwrap();
                }
            }
        }
        // extrapolation from arms cell (all cells): per-relation measured cost x N_need (registered) and x N_dd (ratio from measured full solves)
        let arms = read_tsv(&format!("{}/results/arms_B_m{}_k{}.tsv", dir, m, k));
        if arms.is_empty() {
            continue;
        }
        let uniq = sum(&arms, "u_uniq");
        let ve_mean = sum(&arms, "ve") / arms.len() as f64;
        let setup = sum(&arms, "setup_g") / arms.len() as f64;
        let g_rel = (sum(&arms, "c_g") - sum(&arms, "setup_g") + sum(&arms, "l_g")) / uniq;
        let nl = natsum(&arms, "l") / uniq;
        let nneed = ve_mean + 1.0 + 10.0;
        let ratios_dd = dd_ratio.get(&m).cloned();
        for (arm, p) in &[("U", "u"), ("F", "f")] {
            let n_rel = natsum(&arms, p) / uniq + nl;
            let mut variants: Vec<(String, f64)> = vec![("extrapolated, N_need = |V_E|+11 (registered)".into(), nneed)];
            if let Some(rr) = ratios_dd {
                variants.push((format!("extrapolated, N_dd = {:.2} |V_E| (ratio measured at full-solve cells, exploratory)", rr), rr * ve_mean));
            }
            for (lbl, nrel) in variants {
                let la = nrel * nrel * nrel / 3.0 * if lbl.contains("N_dd") { 0.0 } else { 1.0 };
                let la = if lbl.contains("N_dd") {
                    // LA measured per madds at full-solve cells is small; model with the same cubic formula on N_dd as an upper bound
                    nrel * nrel * nrel / 3.0
                } else {
                    la
                };
                let g_ = setup + nrel * g_rel;
                let n_ = nrel * n_rel + la;
                let c3 = [g_, g_ + 0.1 * n_, g_ + n_];
                let gp = gap(c3);
                writeln!(md, "| {} | {} | {} | {} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.1} | {:.1} | {:.1} | {} | {} |", m, k, lbl, arm, nrel, g_, n_, c3[0], c3[1], c3[2], gp[0], gp[1], gp[2], rank_vs(c3), rf).unwrap();
                fulljson.push(J::O(vec![("m".into(), n(m as f64)), ("k".into(), n(k as f64)), ("arm".into(), st(arm)), ("source".into(), st(&lbl)), ("G".into(), n(g_)), ("N".into(), n(n_)), ("cost".into(), J::A(c3.iter().map(|x| n(*x)).collect())), ("gap".into(), J::A(gp.iter().map(|x| n(*x)).collect()))]));
            }
        }
    }
    md.push_str("\n_Extrapolated rows: relation-phase cost = measured per-unique-solution cost of the arms cell (G: target walk + sign checks; N: solver + lifts + hash lookups) x N_rel; LA = N_rel^3/3 multiply-adds (formula model, not measured); setup G measured; final dP=Q check (about 1.5k G) not added. Never a measured full solve._\n");
    writeln!(md, "\n**Complete-solve certificate cells (measured; 5 curves x 2 instances at k=17,19, reduced at larger k, see DEVIATIONS.md):**\n").unwrap();
    md.push_str(&fsrows);
    jroot.push(("ic_vs_rho".into(), J::A(fulljson.clone())));

    // ---------- verdicts ----------
    writeln!(md, "\n## I. Falsification criteria (registered thresholds, mechanically evaluated)\n").unwrap();
    let f7 = f7_fail.is_empty();
    let rows = vec![
        ("F7 instrument validity".to_string(), f7, if f7 { "all controls pass".to_string() } else { format!("FAILED: {}", f7_fail.join("; ")) }),
        ("F1 H1 (zero waste, exact)".into(), f1_total_viol == 0.0, format!("{} non-degenerate solutions with a twist coordinate over all binary cells", f1_total_viol)),
        ("F2 H2 (column density)".into(), f2_reject < 2 && !f2_sd_bad, format!("{} of {} cells reject; pooled-sd out of [0.8,1.25]: {}", f2_reject, cells.len(), f2_sd_bad)),
        ("F3 H3 (cost ratio)".into(), f3_out < 3 && !f3_ci_bad, format!("{} of {} cells outside +-10% of the registered R; CI contains 1 at c>=0.1 in some cell: {}", f3_out, cells.len(), f3_ci_bad)),
        ("F4 set equality U vs F".into(), f4_mis == 0.0, format!("{} unflagged targets with differing solution sets (all cells, both fields, full-solve)", f4_mis)),
        ("F5 H4 (solution-side filter)".into(), !f5_bad, format!("avoided non-degenerate lifts > 0: {}; S/U outside [1,1.05] in some cell (magnitude only): {}", f5_bad, f5_mag_bad)),
        ("F6 H5 (prime control)".into(), f6_viol == 0.0 && f6_call_bad == 0, format!("{} non-degenerate prime waste solutions; {} prime cells outside the call-ratio band", f6_viol, f6_call_bad)),
        ("F8 rho baseline band".into(), f8_out.is_empty(), format!("sizes outside [0.8,1.6]: {:?}", f8_out)),
        ("F9 verification".into(), f9_bad == 0.0, format!("{} failed group-law checks or wrong recovered d", f9_bad)),
    ];
    writeln!(md, "| criterion | not falsified / valid | detail |").unwrap();
    writeln!(md, "|---|---|---|").unwrap();
    let mut vj = vec![];
    for (a, b, c) in &rows {
        writeln!(md, "| {} | {} | {} |", a, if *b { "yes" } else { "NO" }, c).unwrap();
        vj.push(J::O(vec![("criterion".into(), st(a)), ("holds".into(), J::B(*b)), ("detail".into(), st(c))]));
    }
    // H6 (qualitative): gap at c=1 and unordered cells, from the recorded IC-vs-rho rows
    {
        let mut min_gap1 = f64::INFINITY;
        let mut unordered: Vec<String> = vec![];
        for r in &fulljson {
            if let J::O(o) = r {
                let get = |k: &str| o.iter().find(|(a, _)| a == k).map(|(_, v)| v.clone());
                if let (Some(J::A(g)), Some(J::S(src)), Some(J::N(kk)), Some(J::N(mm)), Some(J::S(arm))) = (get("gap"), get("source"), get("k"), get("m"), get("arm")) {
                    if let Some(J::N(g1)) = g.get(2) {
                        if *g1 < min_gap1 {
                            min_gap1 = *g1;
                        }
                    }
                    if let (Some(J::N(g0)), Some(J::N(g1)), Some(J::N(g2))) = (g.get(0), g.get(1), g.get(2)) {
                        let mixed = (*g0 < 1.0) != (*g2 < 1.0) || (*g1 < 1.0) != (*g2 < 1.0);
                        if mixed {
                            unordered.push(format!("m={} k={} {} [{}]", mm, kk, arm, src));
                        }
                    }
                }
            }
        }
        writeln!(md, "\n**H6 (qualitative, distance remaining):** smallest gap cost_IC/cost_rho at c = 1 over every IC row of section H: {:.1}. Rows whose order flips inside the band (IC cheaper at some prices, rho at others): {}.", min_gap1, if unordered.is_empty() { "none".to_string() } else { unordered.join("; ") }).unwrap();
    }
    jroot.push(("verdicts".into(), J::A(vj)));
    let mut s = String::new();
    J::O(jroot).s(&mut s);
    std::fs::write(format!("{}/results.json", dir), s).unwrap();
    std::fs::write(format!("{}/results/auto_tables.md", dir), md).unwrap();
}
