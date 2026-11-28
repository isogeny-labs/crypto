//! A4 (P3.3/Q099): basic KLPT against the exact least ell-power norm.
//! Usage: a4_klpt <csv> <out_dir> [runs_per_instance] [seed]

mod arith;
mod klpt;
mod quat;

use arith::*;
use num_traits::ToPrimitive;
use quat::*;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::time::Instant;

/// RFC 4180 CSV parser (quoted fields, doubled quotes, commas inside quotes).
fn parse_csv(text: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut inq = false;
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if inq {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    field.push('"');
                    chars.next();
                } else {
                    inq = false;
                }
            } else {
                field.push(ch);
            }
        } else {
            match ch {
                '"' => inq = true,
                ',' => row.push(std::mem::take(&mut field)),
                '\r' => {}
                '\n' => {
                    row.push(std::mem::take(&mut field));
                    rows.push(std::mem::take(&mut row));
                }
                _ => field.push(ch),
            }
        }
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

struct Inst {
    line: usize,
    instance: String,
    p: Z,
    p_bits: u32,
    ellp: Z,
    alpha: [Z; 4],
    unconstrained: Z,
    opt2: Option<Z>,
    opt3: Option<Z>,
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let h = (sorted.len() - 1) as f64 * q;
    let lo = h.floor() as usize;
    let hi = h.ceil() as usize;
    sorted[lo] + (h - lo as f64) * (sorted[hi] - sorted[lo])
}

fn stats3(v: &[f64]) -> (f64, f64, f64) {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (quantile(&s, 0.5), quantile(&s, 0.25), quantile(&s, 0.75))
}

fn ols(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let sxx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
    let slope = sxy / sxx;
    (slope, my - slope * mx)
}

struct Run {
    inst: usize,
    ell: u32,
    run: u32,
    ok: bool,
    err: String,
    n: String,
    e0r: u32,
    e1r: u32,
    e0: u32,
    e1: u32,
    st: klpt::Stats,
    c: Option<klpt::Controls>,
    x: String,
    secs: f64,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let csv = &args[1];
    let out = &args[2];
    let runs: u32 = args.get(3).map(|s| s.parse().unwrap()).unwrap_or(5);
    let seed: u64 = args.get(4).map(|s| s.parse().unwrap()).unwrap_or(20261123);
    let t_all = Instant::now();
    let text = std::fs::read_to_string(csv).unwrap();
    let rows = parse_csv(&text);
    let hdr = &rows[0];
    let col = |name: &str| hdr.iter().position(|h| h == name).unwrap_or_else(|| panic!("{}", name));
    let (ci, cp, cb, cl, ca, cu, c2, c3) = (
        col("instance"),
        col("p"),
        col("p_bits"),
        col("input_ideal_norm"),
        col("alpha_order_coordinates"),
        col("unconstrained_norm"),
        col("power2_norm"),
        col("power3_norm"),
    );
    let mut insts = Vec::new();
    for (k, r) in rows.iter().enumerate().skip(1) {
        assert_eq!(r.len(), hdr.len(), "line {} has {} fields", k + 1, r.len());
        let a: Vec<Z> = r[ca].split(';').map(|s| s.parse().unwrap()).collect();
        let opt = |s: &str| if s.is_empty() { None } else { Some(s.parse::<Z>().unwrap()) };
        insts.push(Inst {
            line: k + 1,
            instance: r[ci].clone(),
            p: r[cp].parse().unwrap(),
            p_bits: r[cb].parse().unwrap(),
            ellp: r[cl].parse().unwrap(),
            alpha: [a[0].clone(), a[1].clone(), a[2].clone(), a[3].clone()],
            unconstrained: r[cu].parse().unwrap(),
            opt2: opt(&r[c2]),
            opt3: opt(&r[c3]),
        });
    }
    eprintln!("parsed {} instances", insts.len());
    let small = small_primes(1000);

    // ---- Control 1: ideal reconstruction + exact SVP on all rows ----
    let t1 = Instant::now();
    let built: Vec<(Ideal, [[Z; 4]; 4], Z, bool, bool)> = insts
        .par_iter()
        .map(|it| {
            let id = prime_ideal(&it.p, &it.ellp, &it.alpha);
            let closed = id.left_closed(&it.p);
            let (q, _v, _n, red) = exact_svp(&it.p, &id, &it.ellp);
            let m = q == it.unconstrained;
            (id, red, q, closed, m)
        })
        .collect();
    let c1_secs = t1.elapsed().as_secs_f64();
    let c1_match = built.iter().filter(|b| b.4).count();
    let c1_closed = built.iter().filter(|b| b.3).count();
    let mut c1_lines = String::from("csv_line,instance,p,input_ideal_norm,our_min_q,upstream_unconstrained_norm,match,left_closed\n");
    for (it, b) in insts.iter().zip(built.iter()) {
        writeln!(c1_lines, "{},{},{},{},{},{},{},{}", it.line, it.instance, it.p, it.ellp, b.2, it.unconstrained, b.4 as u8, b.3 as u8).unwrap();
    }
    std::fs::write(format!("{}/control1.csv", out), c1_lines).unwrap();
    eprintln!("control 1: {}/{} match, {}/{} left-closed, {:.2}s", c1_match, insts.len(), c1_closed, insts.len(), c1_secs);
    if c1_match != insts.len() || c1_closed != insts.len() {
        eprintln!("CONTROL 1 FAILED: stopping");
        std::process::exit(2);
    }

    // ---- KLPT runs ----
    let usable: Vec<usize> = (0..insts.len()).filter(|&k| insts[k].opt2.is_some() && insts[k].opt3.is_some()).collect();
    eprintln!("usable rows: {}", usable.len());
    let mut jobs = Vec::new();
    for &k in usable.iter() {
        for ell in [2u32, 3] {
            for r in 0..runs {
                jobs.push((k, ell, r));
            }
        }
    }
    let results: Vec<Run> = jobs
        .par_iter()
        .map(|&(k, ell, r)| {
            let it = &insts[k];
            let (id, red) = (&built[k].0, &built[k].1);
            let s = seed
                .wrapping_mul(1_000_003)
                .wrapping_add((it.line as u64) * 10_007 + (ell as u64) * 101 + r as u64);
            let mut rng = ChaCha20Rng::seed_from_u64(s);
            let t0 = Instant::now();
            let res = klpt::klpt(&mut rng, &it.p, ell, id, &it.ellp, red, &small);
            match res {
                Ok(o) => {
                    let e = o.e0 + o.e1;
                    let c = klpt::controls(&it.p, id, &it.ellp, &o.x, ell, e);
                    let secs = t0.elapsed().as_secs_f64();
                    Run {
                        inst: k, ell, run: r, ok: true, err: String::new(),
                        n: o.n_prime.to_string(), e0r: o.e0_rule, e1r: o.e1_rule, e0: o.e0, e1: o.e1,
                        st: o.stats, c: Some(c),
                        x: o.x.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(";"),
                        secs,
                    }
                }
                Err(msg) => Run {
                    inst: k, ell, run: r, ok: false, err: msg, n: String::new(), e0r: 0, e1r: 0, e0: 0, e1: 0,
                    st: klpt::Stats::default(), c: None, x: String::new(), secs: t0.elapsed().as_secs_f64(),
                },
            }
        })
        .collect();

    // ---- runs.csv ----
    let mut csvout = String::from("csv_line,instance,p,p_bits,input_ideal_norm,ell,run,status,N,log_p_N,delta_box,e0_rule,e1_rule,e0,e1,e_klpt,log_p_ell_e,optimum,log_p_optimum,ratio,log_p_ratio,retries,gamma_pairs_tried,gamma_found,rej_l2_parity,rej_gamma_nonprimitive,rej_no_cd,rej_cd_isotropic,rej_residuosity,rej_e1_cap,e0_bumps,e1_bumps,e1_parity_bumps,sa_candidates,sa_parity_skips,sa_negative,sa_no_sos,nonprimitive_rejects,ctl_beta_in_I,ctl_norm_exact,ctl_ideal_left_O0_norm,primitive,seconds,beta_order_coordinates,error\n");
    let mut good: HashMap<(u32, u32), Vec<(f64, f64, f64, f64, f64, f64)>> = HashMap::new(); // (bits, ell) -> (log_p ell^e, log_p ratio, log2 p, log_p N)
    let mut fails_klpt = 0usize;
    let mut fails_ctl = 0usize;
    let (mut f_in, mut f_norm, mut f_ideal, mut nonprim) = (0usize, 0usize, 0usize, 0usize);
    let mut per_ell_xy: HashMap<u32, (Vec<f64>, Vec<f64>, Vec<f64>)> = HashMap::new();
    let mut total_retries: HashMap<u32, u64> = HashMap::new();
    let mut maxsecs = 0f64;
    for r in results.iter() {
        let it = &insts[r.inst];
        let lnp = ln_big(&it.p);
        let opt = if r.ell == 2 { it.opt2.clone().unwrap() } else { it.opt3.clone().unwrap() };
        let lnopt = ln_big(&opt);
        maxsecs = maxsecs.max(r.secs);
        let mut status = "ok".to_string();
        if !r.ok {
            fails_klpt += 1;
            status = "klpt_fail".into();
        } else {
            let c = r.c.as_ref().unwrap();
            if !c.in_i { f_in += 1; }
            if !c.norm_ok { f_norm += 1; }
            if !c.ideal_ok { f_ideal += 1; }
            if !c.primitive { nonprim += 1; }
            if !(c.in_i && c.norm_ok && c.ideal_ok) {
                fails_ctl += 1;
                status = "control_fail".into();
            }
        }
        let e = r.e0 + r.e1;
        let lpe = e as f64 * (r.ell as f64).ln() / lnp;
        let lpr = (e as f64 * (r.ell as f64).ln() - lnopt) / lnp;
        let ratio = (e as f64 * (r.ell as f64).ln() - lnopt).exp();
        let lpn = if r.ok { ln_big(&r.n.parse::<Z>().unwrap()) / lnp } else { f64::NAN };
        if status == "ok" {
            good.entry((it.p_bits, r.ell)).or_default().push((lpe, lpr, lnp / std::f64::consts::LN_2, lpn, r.e0 as f64 * (r.ell as f64).ln() / lnp, r.e1 as f64 * (r.ell as f64).ln() / lnp));
            let ent = per_ell_xy.entry(r.ell).or_default();
            ent.0.push(lnp / std::f64::consts::LN_2);
            ent.1.push((lpr * lnp) / std::f64::consts::LN_2);
            ent.2.push((lpe * lnp) / std::f64::consts::LN_2);
            *total_retries.entry(r.ell).or_default() += r.st.retries();
        }
        let s = &r.st;
        let c = r.c.as_ref();
        let b = |f: fn(&klpt::Controls) -> bool| c.map_or(String::new(), |c| (f(c) as u8).to_string());
        let (es, lpes, rs, lprs, lpns) = if r.ok {
            (e.to_string(), format!("{:.6}", lpe), format!("{:.6e}", ratio), format!("{:.6}", lpr), format!("{:.6}", lpn))
        } else {
            Default::default()
        };
        writeln!(
            csvout,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.6},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.4},{},\"{}\"",
            it.line, it.instance, it.p, it.p_bits, it.ellp, r.ell, r.run, status, r.n, lpns, s.delta_box,
            r.e0r, r.e1r, r.e0, r.e1, es, lpes, opt, lnopt / lnp, rs, lprs, s.retries(), s.gamma_pairs_tried,
            s.gamma_found, s.gamma_rejected_l2_parity, s.gamma_rejected_nonprimitive, s.gamma_rejected_no_cd, s.gamma_rejected_cd_isotropic,
            s.gamma_rejected_residuosity, s.gamma_rejected_e1_cap, s.e0_bumps, s.e1_bumps, s.e1_parity_bumps,
            s.sa_candidates, s.sa_parity_skips, s.sa_negative, s.sa_no_sos, s.nonprimitive_rejects,
            b(|c| c.in_i), b(|c| c.norm_ok), b(|c| c.ideal_ok), b(|c| c.primitive), r.secs, r.x, r.err
        )
        .unwrap();
    }
    std::fs::write(format!("{}/runs.csv", out), csvout).unwrap();

    // ---- summary.json ----
    let mut j = String::from("{\n");
    writeln!(j, "  \"experiment\": \"A4 P3.3/Q099 basic KLPT vs exact least ell-power norm\",").unwrap();
    writeln!(j, "  \"seed\": {},\n  \"runs_per_instance\": {},", seed, runs).unwrap();
    writeln!(j, "  \"control1\": {{\"rows\": {}, \"svp_matches_unconstrained_norm\": {}, \"left_closed\": {}, \"seconds\": {:.3}}},", insts.len(), c1_match, c1_closed, c1_secs).unwrap();
    writeln!(j, "  \"usable_rows\": {},\n  \"klpt_runs\": {},", usable.len(), results.len()).unwrap();
    writeln!(j, "  \"klpt_failures\": {},\n  \"control_failures\": {{\"total\": {}, \"beta_not_in_I\": {}, \"norm_not_ell_e\": {}, \"equivalent_ideal_bad\": {}}},", fails_klpt, fails_ctl, f_in, f_norm, f_ideal).unwrap();
    writeln!(j, "  \"nonprimitive_outputs\": {},", nonprim).unwrap();
    j.push_str("  \"bands\": [\n");
    let mut first = true;
    for bits in [12u32, 20, 28] {
        for ell in [2u32, 3] {
            let v = good.get(&(bits, ell)).cloned().unwrap_or_default();
            let le: Vec<f64> = v.iter().map(|t| t.0).collect();
            let lr: Vec<f64> = v.iter().map(|t| t.1).collect();
            let ln: Vec<f64> = v.iter().map(|t| t.3).collect();
            let (m1, a1, b1) = stats3(&le);
            let (m2, a2, b2) = stats3(&lr);
            let (m3, _, _) = stats3(&ln);
            let (m4, _, _) = stats3(&v.iter().map(|t| t.4).collect::<Vec<_>>());
            let (m5, _, _) = stats3(&v.iter().map(|t| t.5).collect::<Vec<_>>());
            if !first { j.push_str(",\n"); }
            first = false;
            write!(j, "    {{\"p_bits\": {}, \"ell\": {}, \"n_runs\": {}, \"log_p_ell_e\": {{\"median\": {:.4}, \"q1\": {:.4}, \"q3\": {:.4}, \"iqr\": {:.4}}}, \"log_p_ratio\": {{\"median\": {:.4}, \"q1\": {:.4}, \"q3\": {:.4}, \"iqr\": {:.4}}}, \"median_log_p_N\": {:.4}, \"median_log_p_ell_e0\": {:.4}, \"median_log_p_ell_e1\": {:.4}, \"h4_in_3.0_3.7\": {}, \"lead_median_below_2.5\": {}}}",
                bits, ell, v.len(), m1, a1, b1, b1 - a1, m2, a2, b2, b2 - a2, m3, m4, m5, (3.0..=3.7).contains(&m1), m1 < 2.5).unwrap();
        }
    }
    j.push_str("\n  ],\n  \"slopes_vs_log2_p\": [\n");
    let mut first = true;
    for ell in [2u32, 3] {
        if let Some((x, yr, ye)) = per_ell_xy.get(&ell) {
            let (sr, ir) = ols(x, yr);
            let (se, ie) = ols(x, ye);
            if !first { j.push_str(",\n"); }
            first = false;
            write!(j, "    {{\"ell\": {}, \"n\": {}, \"log2_ratio_slope\": {:.4}, \"log2_ratio_intercept\": {:.4}, \"log2_ell_e_slope\": {:.4}, \"log2_ell_e_intercept\": {:.4}, \"lead_ratio_slope_below_2\": {}, \"total_retries\": {}}}",
                ell, x.len(), sr, ir, se, ie, sr < 2.0, total_retries.get(&ell).unwrap_or(&0)).unwrap();
        }
    }
    j.push_str("\n  ],\n  \"retry_totals\": [\n");
    for (k, ell) in [2u32, 3].iter().enumerate() {
        let rs: Vec<&klpt::Stats> = results.iter().filter(|r| r.ell == *ell && r.ok).map(|r| &r.st).collect();
        let sum = |f: fn(&klpt::Stats) -> u64| rs.iter().map(|s| f(s)).sum::<u64>();
        let with_retry = rs.iter().filter(|s| s.retries() > 0).count();
        let bumped = results.iter().filter(|r| r.ell == *ell && r.ok && (r.e0 != r.e0r || r.e1 != r.e1r)).count();
        let maxe = results.iter().filter(|r| r.ell == *ell && r.ok).map(|r| (r.e0 + r.e1) as i64 - (r.e0r + r.e1r) as i64).max().unwrap_or(0);
        if k > 0 { j.push_str(",\n"); }
        write!(j, "    {{\"ell\": {}, \"runs\": {}, \"runs_with_any_retry\": {}, \"runs_with_e_above_rule\": {}, \"max_e_minus_rule\": {}, \"retries\": {}, \"gamma_pairs_tried\": {}, \"rej_l2_parity\": {}, \"rej_gamma_nonprimitive\": {}, \"rej_no_cd\": {}, \"rej_cd_isotropic\": {}, \"rej_residuosity\": {}, \"rej_e1_cap\": {}, \"e0_bumps\": {}, \"e1_bumps\": {}, \"e1_parity_bumps\": {}, \"nonprimitive_beta_rejects\": {}, \"sa_candidates\": {}, \"sa_parity_skips\": {}, \"sa_no_sos\": {}, \"delta_box_gt1\": {}}}",
            ell, rs.len(), with_retry, bumped, maxe, sum(|s| s.retries()), sum(|s| s.gamma_pairs_tried), sum(|s| s.gamma_rejected_l2_parity),
            sum(|s| s.gamma_rejected_nonprimitive), sum(|s| s.gamma_rejected_no_cd), sum(|s| s.gamma_rejected_cd_isotropic),
            sum(|s| s.gamma_rejected_residuosity), sum(|s| s.gamma_rejected_e1_cap), sum(|s| s.e0_bumps), sum(|s| s.e1_bumps),
            sum(|s| s.e1_parity_bumps), sum(|s| s.nonprimitive_rejects), sum(|s| s.sa_candidates), sum(|s| s.sa_parity_skips),
            sum(|s| s.sa_no_sos), rs.iter().filter(|s| s.delta_box > 1).count()).unwrap();
    }
    writeln!(j, "\n  ],\n  \"max_run_seconds\": {:.4},\n  \"total_seconds\": {:.3}\n}}", maxsecs, t_all.elapsed().as_secs_f64()).unwrap();
    std::fs::write(format!("{}/summary.json", out), &j).unwrap();
    println!("{}", j);
    let _ = (0u8).to_f64();
}
