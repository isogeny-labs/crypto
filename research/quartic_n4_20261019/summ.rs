// Aggregation of the flat JSONL records of `n4 unit`: priced costs, the section-5a band,
// bootstrap intervals, and exponent fits. Reads recorded counts only.
#![allow(dead_code)]
use crate::common::Rng;
use std::collections::{BTreeMap, HashMap};

pub type Rec = HashMap<String, String>;

pub fn parse_line(l: &str) -> Rec {
    // flat object: "key":value pairs, values are numbers, true/false or "strings" without commas
    let mut m = Rec::new();
    let l = l.trim().trim_start_matches('{').trim_end_matches('}');
    let mut rest = l;
    while let Some(i) = rest.find("\":") {
        let kstart = rest[..i].rfind('"').unwrap();
        let key = rest[kstart + 1..i].to_string();
        let v = &rest[i + 2..];
        let (val, next) = if v.starts_with('"') {
            let e = v[1..].find('"').unwrap();
            (v[1..1 + e].to_string(), &v[e + 2..])
        } else {
            let e = v.find(',').unwrap_or(v.len());
            (v[..e].to_string(), &v[e..])
        };
        m.insert(key, val);
        rest = next.trim_start_matches(',');
    }
    m
}
fn f(r: &Rec, k: &str) -> f64 {
    r.get(k).and_then(|v| v.parse().ok()).unwrap_or(0.0)
}

const PHASES: [&str; 7] = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"];

fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 { v[n / 2] } else { 0.5 * (v[n / 2 - 1] + v[n / 2]) }
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
pub fn slope(xs: &[f64], ys: &[f64]) -> (f64, f64, f64) {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let b = sxy / sxx;
    let a = my - b * mx;
    let rss: f64 = xs.iter().zip(ys).map(|(x, y)| (y - a - b * x).powi(2)).sum();
    let se = if xs.len() > 2 { (rss / (n - 2.0) / sxx).sqrt() } else { f64::NAN };
    (b, se, a)
}
fn tcrit(df: usize) -> f64 {
    let t = [0.0, 12.706, 4.303, 3.182, 2.776, 2.571, 2.447, 2.365, 2.306, 2.262, 2.228, 2.201, 2.179, 2.160, 2.145, 2.131];
    if df < t.len() { t[df] } else { 1.96 }
}
fn pct(v: &mut Vec<f64>, p: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() as f64 - 1.0) * p).round() as usize]
}

struct Unit {
    meq_phase: [f64; 7],
    unpriced: f64,
    attempts: usize,
    ok: bool,
}

pub fn run(args: &[String]) {
    let out_path = args[0].clone();
    let minq: u64 = args[1].parse().unwrap();
    let mut recs: Vec<Rec> = Vec::new();
    for p in &args[2..] {
        for l in std::fs::read_to_string(p).unwrap().lines() {
            if l.starts_with('{') {
                recs.push(parse_line(l));
            }
        }
    }
    // group by (arm, q)
    let mut sizes: BTreeMap<(String, u64), Vec<u64>> = BTreeMap::new(); // seeds
    for r in &recs {
        if r.get("rec").map(|s| s.as_str()) == Some("ratios") {
            sizes.entry((r["arm"].clone(), r["q"].parse().unwrap())).or_default().push(r["seed"].parse().unwrap());
        }
    }
    let band = [0.0f64, 0.1, 1.0];
    let mut js = Vec::new();
    // per arm: series of (ln q, mean ic by band, mean rho by band, phase means)
    let mut series: BTreeMap<String, Vec<(f64, f64, [f64; 3], [f64; 3], [f64; 7], f64, f64)>> = BTreeMap::new();
    let mut boot_inputs: BTreeMap<String, Vec<(f64, Vec<[f64; 3]>, Vec<[f64; 3]>)>> = BTreeMap::new();
    println!("| arm | q | units ok/total | rho runs | IC M-eq | rho M-eq | IC/rho [95% boot] w=0 | w=0.1 | w=1 | resolved |");
    println!("|:--|--:|--:|--:|--:|--:|:--|:--|:--|:--|");
    for ((arm, q), seeds) in &sizes {
        let rr: Vec<&Rec> = recs.iter().filter(|r| r.get("rec").map(|s| s.as_str()) == Some("ratios") && r["arm"] == *arm && r["q"] == q.to_string()).collect();
        let mut ri: Vec<f64> = rr.iter().map(|r| f(r, "r_inv")).collect();
        let mut rl: Vec<f64> = rr.iter().map(|r| f(r, "r_lmul")).collect();
        let mut gm: Vec<f64> = rr.iter().map(|r| f(r, "gae_mul")).collect();
        let mut gi: Vec<f64> = rr.iter().map(|r| f(r, "gae_inv")).collect();
        let r_inv = median(&mut ri);
        let r_l = median(&mut rl);
        let gae = median(&mut gm) + r_inv * median(&mut gi);
        let meq = |r: &Rec, p: &str| f(r, &format!("{p}.mul")) + r_inv * f(r, &format!("{p}.inv")) + r_l * f(r, &format!("{p}.lmul"));
        let unp = |r: &Rec, p: &str| f(r, &format!("{p}.hash")) + f(r, &format!("{p}.cmp"));
        let mut units: Vec<Unit> = Vec::new();
        let mut rho_runs: Vec<[f64; 3]> = Vec::new(); // per run: meq, unpriced, steps
        let mut nrho_ok = 0;
        for sd in seeds {
            let mut u = Unit { meq_phase: [0.0; 7], unpriced: 0.0, attempts: 0, ok: false };
            for r in recs.iter().filter(|r| r.get("rec").map(|s| s.as_str()) == Some("ic") && r["arm"] == *arm && r["q"] == q.to_string() && r["seed"] == sd.to_string()) {
                u.attempts += 1;
                u.ok = r["ok"] == "true";
                for (i, p) in PHASES.iter().enumerate() {
                    u.meq_phase[i] += meq(r, p);
                    u.unpriced += unp(r, p);
                }
            }
            units.push(u);
            if arm == "B0" || true {
                for r in recs.iter().filter(|r| r.get("rec").map(|s| s.as_str()) == Some("rho") && r["q"] == q.to_string() && r["seed"] == sd.to_string()) {
                    if r["ok"] == "true" {
                        nrho_ok += 1;
                    }
                    rho_runs.push([meq(r, "c"), unp(r, "c"), f(r, "steps")]);
                }
            }
        }
        let ok_units = units.iter().filter(|u| u.ok).count();
        let ic_vals: Vec<[f64; 3]> = units.iter().filter(|u| u.ok).map(|u| {
            let m: f64 = u.meq_phase.iter().sum();
            [m, m + 0.1 * gae * u.unpriced, m + gae * u.unpriced]
        }).collect();
        let rho_vals: Vec<[f64; 3]> = rho_runs.iter().map(|x| [x[0], x[0] + 0.1 * gae * x[1], x[0] + gae * x[1]]).collect();
        if ic_vals.is_empty() || rho_vals.is_empty() {
            continue;
        }
        let ic_wall = {
            let mut tot = 0.0;
            let mut n = 0.0;
            for sd in seeds {
                let mut w = 0.0;
                let mut any = false;
                for r in recs.iter().filter(|r| r.get("rec").map(|s| s.as_str()) == Some("ic") && r["arm"] == *arm && r["q"] == q.to_string() && r["seed"] == sd.to_string()) {
                    any = true;
                    for p in PHASES.iter() {
                        w += f(r, &format!("{p}.wall"));
                    }
                }
                if any { tot += w; n += 1.0; }
            }
            tot / n
        };
        let rho_wall = {
            let v: Vec<f64> = recs.iter().filter(|r| r.get("rec").map(|s| s.as_str()) == Some("rho") && r["q"] == q.to_string() && seeds.iter().any(|s| r["seed"] == s.to_string())).map(|r| f(r, "wall")).collect();
            mean(&v)
        };
        let ell_mean = mean(&rr.iter().map(|r| f(r, "ell").sqrt()).collect::<Vec<_>>());
        let ic_m: [f64; 3] = [0, 1, 2].map(|b| mean(&ic_vals.iter().map(|v| v[b]).collect::<Vec<_>>()));
        let rho_m: [f64; 3] = [0, 1, 2].map(|b| mean(&rho_vals.iter().map(|v| v[b]).collect::<Vec<_>>()));
        let mut phase_m = [0.0; 7];
        for u in units.iter().filter(|u| u.ok) {
            for i in 0..7 {
                phase_m[i] += u.meq_phase[i] / ok_units as f64;
            }
        }
        // bootstrap ratio per band
        let mut rng = Rng::new(0xB007 ^ q);
        let mut ratios: [Vec<f64>; 3] = [vec![], vec![], vec![]];
        for _ in 0..2000 {
            let mi: Vec<usize> = (0..ic_vals.len()).map(|_| rng.below(ic_vals.len() as u64) as usize).collect();
            let mr: Vec<usize> = (0..rho_vals.len()).map(|_| rng.below(rho_vals.len() as u64) as usize).collect();
            for b in 0..3 {
                let a = mean(&mi.iter().map(|&i| ic_vals[i][b]).collect::<Vec<_>>());
                let c = mean(&mr.iter().map(|&i| rho_vals[i][b]).collect::<Vec<_>>());
                ratios[b].push(a / c);
            }
        }
        let ci: Vec<(f64, f64)> = (0..3).map(|b| (pct(&mut ratios[b].clone(), 0.025), pct(&mut ratios[b], 0.975))).collect();
        let resolved = if ci.iter().all(|c| c.1 < 1.0) { "IC lower" } else if ci.iter().all(|c| c.0 > 1.0) { "rho lower" } else { "unresolved" };
        println!(
            "| {} | {} | {}/{} | {} | {:.3e} | {:.3e} | {:.2} [{:.2},{:.2}] | {:.2} [{:.2},{:.2}] | {:.2} [{:.2},{:.2}] | {} |",
            arm, q, ok_units, units.len(), rho_vals.len(), ic_m[0], rho_m[0],
            ic_m[0] / rho_m[0], ci[0].0, ci[0].1, ic_m[1] / rho_m[1], ci[1].0, ci[1].1, ic_m[2] / rho_m[2], ci[2].0, ci[2].1, resolved
        );
        js.push(format!(
            "{{\"arm\":\"{}\",\"q\":{},\"units\":{},\"units_ok\":{},\"rho_runs\":{},\"rho_ok\":{},\"r_inv\":{:.4},\"r_lmul\":{:.4},\"gae_meq\":{:.3},\"ic_meq\":[{:.6e},{:.6e},{:.6e}],\"rho_meq\":[{:.6e},{:.6e},{:.6e}],\"ratio\":[{:.5},{:.5},{:.5}],\"ratio_lo\":[{:.5},{:.5},{:.5}],\"ratio_hi\":[{:.5},{:.5},{:.5}],\"resolved\":\"{}\",\"phase_meq\":[{}],\"ic_wall_s\":{:.3},\"rho_wall_s\":{:.3},\"ic_meq_descent_free\":{:.6e},\"ic_meq_descent_div20\":{:.6e},\"s_ic\":{:.4},\"s_rho\":{:.4},\"rho_steps_mean\":{:.1},\"rho_steps_over_sqrt_ell\":{:.4},\"attempts_mean\":{:.2}}}",
            arm, q, units.len(), ok_units, rho_vals.len(), nrho_ok, r_inv, r_l, gae,
            ic_m[0], ic_m[1], ic_m[2], rho_m[0], rho_m[1], rho_m[2],
            ic_m[0] / rho_m[0], ic_m[1] / rho_m[1], ic_m[2] / rho_m[2],
            ci[0].0, ci[1].0, ci[2].0, ci[0].1, ci[1].1, ci[2].1, resolved,
            phase_m.iter().map(|x| format!("{:.5e}", x)).collect::<Vec<_>>().join(","),
            ic_wall, rho_wall, ic_m[0] - phase_m[5], ic_m[0] - phase_m[5] * 0.95, ic_m[0] / (gae * ell_mean), rho_m[0] / (gae * ell_mean),
            mean(&rho_runs.iter().map(|x| x[2]).collect::<Vec<_>>()),
            mean(&rho_runs.iter().map(|x| x[2]).collect::<Vec<_>>()) / ell_mean,
            mean(&units.iter().map(|u| u.attempts as f64).collect::<Vec<_>>())
        ));
        if *q < minq {
            continue;
        }
        series.entry(arm.clone()).or_default().push(((*q as f64).ln(), ic_m[0], ic_m, rho_m, phase_m, 0.0, 0.0));
        boot_inputs.entry(arm.clone()).or_default().push(((*q as f64).ln(), ic_vals.clone(), rho_vals.clone()));
    }
    // exponent fits (band w = 0 and w = 1), bootstrap CI over units / rho runs
    println!("\n| arm | sizes | IC exponent [95% boot] (w=0) | rho exponent [95% boot] (w=0) | exponent gap rho-IC [95% boot] | IC (w=1) | rho (w=1) |");
    println!("|:--|--:|:--|:--|:--|:--|:--|");
    for (arm, bi) in &boot_inputs {
        let n = bi.len();
        if n < 3 {
            continue;
        }
        let xs: Vec<f64> = bi.iter().map(|b| b.0).collect();
        let fit = |band: usize, which: usize, sample: &Vec<Vec<usize>>, sample_r: &Vec<Vec<usize>>| -> f64 {
            let ys: Vec<f64> = (0..n).map(|i| {
                if which == 0 { mean(&sample[i].iter().map(|&j| bi[i].1[j][band]).collect::<Vec<_>>()).ln() }
                else { mean(&sample_r[i].iter().map(|&j| bi[i].2[j][band]).collect::<Vec<_>>()).ln() }
            }).collect();
            slope(&xs, &ys).0
        };
        let ident: Vec<Vec<usize>> = (0..n).map(|i| (0..bi[i].1.len()).collect()).collect();
        let ident_r: Vec<Vec<usize>> = (0..n).map(|i| (0..bi[i].2.len()).collect()).collect();
        let mut rng = Rng::new(0xFEED);
        let mut res: Vec<[Vec<f64>; 3]> = vec![[vec![], vec![], vec![]], [vec![], vec![], vec![]]]; // band 0 ic, rho, gap ; band 2 ic, rho, gap
        for _ in 0..2000 {
            let s: Vec<Vec<usize>> = (0..n).map(|i| (0..bi[i].1.len()).map(|_| rng.below(bi[i].1.len() as u64) as usize).collect()).collect();
            let sr: Vec<Vec<usize>> = (0..n).map(|i| (0..bi[i].2.len()).map(|_| rng.below(bi[i].2.len() as u64) as usize).collect()).collect();
            for (bi_, band) in [(0usize, 0usize), (1, 2)] {
                let a = fit(band, 0, &s, &sr);
                let b = fit(band, 1, &s, &sr);
                res[bi_][0].push(a);
                res[bi_][1].push(b);
                res[bi_][2].push(b - a);
            }
        }
        let pt = |band: usize| (fit(band, 0, &ident, &ident_r), fit(band, 1, &ident, &ident_r));
        let (a0, r0) = pt(0);
        let (a2, r2) = pt(2);
        let c = |v: &Vec<f64>| (pct(&mut v.clone(), 0.025), pct(&mut v.clone(), 0.975));
        let ci = |bnd: usize, k: usize| c(&res[bnd][k]);
        println!(
            "| {} | {} | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] |",
            arm, n, a0, ci(0, 0).0, ci(0, 0).1, r0, ci(0, 1).0, ci(0, 1).1, r0 - a0, ci(0, 2).0, ci(0, 2).1, a2, ci(1, 0).0, ci(1, 0).1, r2, ci(1, 1).0, ci(1, 1).1
        );
        js.push(format!(
            "{{\"fit\":\"{}\",\"sizes\":{},\"ic_exp_w0\":[{:.4},{:.4},{:.4}],\"rho_exp_w0\":[{:.4},{:.4},{:.4}],\"gap_w0\":[{:.4},{:.4},{:.4}],\"ic_exp_w1\":[{:.4},{:.4},{:.4}],\"rho_exp_w1\":[{:.4},{:.4},{:.4}],\"gap_w1\":[{:.4},{:.4},{:.4}]}}",
            arm, n, a0, ci(0, 0).0, ci(0, 0).1, r0, ci(0, 1).0, ci(0, 1).1, r0 - a0, ci(0, 2).0, ci(0, 2).1,
            a2, ci(1, 0).0, ci(1, 0).1, r2, ci(1, 1).0, ci(1, 1).1, r2 - a2, ci(1, 2).0, ci(1, 2).1
        ));
        // phase exponents
        if let Some(sr) = series.get(arm) {
            let names = PHASES;
            let mut line = format!("phase exponents ({}):", arm);
            let mut pj = Vec::new();
            for i in 0..7 {
                let ys: Vec<f64> = sr.iter().map(|x| x.4[i].max(1.0).ln()).collect();
                let xs2: Vec<f64> = sr.iter().map(|x| x.0).collect();
                if sr.iter().all(|x| x.4[i] > 100.0) {
                    let b = slope(&xs2, &ys).0;
                    line += &format!(" {}={:.2}", names[i], b);
                    pj.push(format!("\"{}\":{:.4}", names[i], b));
                }
            }
            println!("{}", line);
            js.push(format!("{{\"phase_exponents\":{{{}}},\"arm\":\"{}\"}}", pj.join(","), arm));
        }
    }
    std::fs::write(out_path, js.join("\n") + "\n").unwrap();
}
