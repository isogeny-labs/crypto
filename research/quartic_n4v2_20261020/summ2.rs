// N4 v2 aggregation from recorded counts: priced costs, section-5a band, bootstrap intervals,
// exponent fits on three ranges, bend classification, crossovers (M-eq and wall), improvements
// against B0, rho calibration. Reads records only; writes summary JSONL and prints Markdown.
#![allow(dead_code)]
use crate::common::Rng;
use std::collections::{BTreeMap, HashMap};

pub type Rec = HashMap<String, String>;

pub fn parse_line(l: &str) -> Rec {
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
        } else if v.starts_with('[') {
            let e = v.find(']').unwrap();
            (v[..e + 1].to_string(), &v[e + 1..])
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
fn s<'a>(r: &'a Rec, k: &str) -> &'a str {
    r.get(k).map(|x| x.as_str()).unwrap_or("")
}

pub const PHASES: [&str; 7] = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"];
pub const IC_ARMS: [&str; 8] = ["B0", "S", "L0", "L1", "L", "D", "DL1", "DL"];
pub const RHO_ARMS: [&str; 2] = ["plain", "neg"];
const W: [f64; 3] = [0.0, 0.1, 1.0];

fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 { v[n / 2] } else { 0.5 * (v[n / 2 - 1] + v[n / 2]) }
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn pct(v: &[f64], p: f64) -> f64 {
    let mut v: Vec<f64> = v.iter().copied().filter(|x| x.is_finite()).collect();
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() as f64 - 1.0) * p).round() as usize]
}
pub fn slope(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let b = sxy / sxx;
    (b, my - b * mx)
}

#[derive(Clone)]
struct Unit {
    seed: u64,
    ok: bool,
    attempts: usize,
    meq: f64,
    unp: f64,
    wall: f64,
    ph: [f64; 7],
    phw: [f64; 7],
    stats: BTreeMap<String, f64>,
}
impl Unit {
    fn v(&self, gae: f64) -> [f64; 4] {
        [self.meq, self.meq + 0.1 * gae * self.unp, self.meq + gae * self.unp, self.wall]
    }
}
#[derive(Clone)]
struct Run {
    seed: u64,
    ok: bool,
    meq: f64,
    unp: f64,
    wall: f64,
    steps: f64,
    sqrt_ell: f64,
}
impl Run {
    fn v(&self, gae: f64) -> [f64; 4] {
        [self.meq, self.meq + 0.1 * gae * self.unp, self.meq + gae * self.unp, self.wall]
    }
}

struct Size {
    q: u64,
    r_inv: f64,
    r_l: f64,
    gae: f64,
    sqrt_ell: f64,
    load: f64,
}

fn kof(q: u64) -> u32 {
    64 - q.leading_zeros()
}

pub fn run(args: &[String]) {
    let out_path = args[0].clone();
    let mut recs: Vec<Rec> = Vec::new();
    for p in &args[1..] {
        let txt = std::fs::read_to_string(p).unwrap_or_default();
        for l in txt.lines() {
            if l.starts_with('{') {
                recs.push(parse_line(l));
            }
        }
    }
    // sizes and ratios
    let mut by_q: BTreeMap<u64, Vec<&Rec>> = BTreeMap::new();
    for r in recs.iter().filter(|r| s(r, "rec") == "ratios") {
        by_q.entry(r["q"].parse().unwrap()).or_default().push(r);
    }
    let mut cal: BTreeMap<u64, Vec<&Rec>> = BTreeMap::new();
    for r in recs.iter().filter(|r| s(r, "rec") == "calib") {
        cal.entry(r["q"].parse().unwrap()).or_default().push(r);
    }
    // fixed conversion constants (sensitivity): env N4V2_FIXED="r_inv,r_lmul"
    let fixed: Option<(f64, f64)> = std::env::var("N4V2_FIXED").ok().map(|v| {
        let p: Vec<f64> = v.split(',').map(|x| x.parse().unwrap()).collect();
        (p[0], p[1])
    });
    let mut sizes: BTreeMap<u64, Size> = BTreeMap::new();
    for (&q, rr) in &by_q {
        let src: &Vec<&Rec> = cal.get(&q).unwrap_or(rr);
        let mut r_inv = median(&src.iter().map(|r| f(r, "r_inv")).collect::<Vec<_>>());
        let mut r_l = median(&src.iter().map(|r| f(r, "r_lmul")).collect::<Vec<_>>());
        if let Some((a, b)) = fixed {
            r_inv = a;
            r_l = b;
        }
        let gae = median(&rr.iter().map(|r| f(r, "gae_mul")).collect::<Vec<_>>()) + r_inv * median(&rr.iter().map(|r| f(r, "gae_inv")).collect::<Vec<_>>());
        let sqrt_ell = mean(&rr.iter().map(|r| f(r, "ell").sqrt()).collect::<Vec<_>>());
        let load = mean(&rr.iter().map(|r| f(r, "load1")).collect::<Vec<_>>());
        sizes.insert(q, Size { q, r_inv, r_l, gae, sqrt_ell, load });
    }
    // IC units
    let mut ic: BTreeMap<String, BTreeMap<u64, Vec<Unit>>> = BTreeMap::new();
    {
        let mut tmp: BTreeMap<(String, u64, u64), Unit> = BTreeMap::new();
        for r in recs.iter().filter(|r| s(r, "rec") == "ic") {
            let q: u64 = r["q"].parse().unwrap();
            let sz = match sizes.get(&q) {
                Some(x) => x,
                None => continue,
            };
            let key = (r["arm"].clone(), q, r["seed"].parse().unwrap());
            let u = tmp.entry(key.clone()).or_insert(Unit { seed: key.2, ok: false, attempts: 0, meq: 0.0, unp: 0.0, wall: 0.0, ph: [0.0; 7], phw: [0.0; 7], stats: BTreeMap::new() });
            u.attempts += 1;
            u.ok = s(r, "ok") == "true";
            for (i, p) in PHASES.iter().enumerate() {
                let m = f(r, &format!("{p}.mul")) + sz.r_inv * f(r, &format!("{p}.inv")) + sz.r_l * f(r, &format!("{p}.lmul"));
                u.ph[i] += m;
                u.meq += m;
                u.unp += f(r, &format!("{p}.hash")) + f(r, &format!("{p}.cmp"));
                u.phw[i] += f(r, &format!("{p}.wall"));
                u.wall += f(r, &format!("{p}.wall"));
            }
            for (k, v) in r.iter() {
                if let Some(st) = k.strip_prefix("s.") {
                    if let Ok(x) = v.parse::<f64>() {
                        u.stats.insert(st.to_string(), x); // last attempt's value
                    }
                }
            }
        }
        for ((arm, q, _), u) in tmp {
            ic.entry(arm).or_default().entry(q).or_default().push(u);
        }
    }
    let mut rho: BTreeMap<String, BTreeMap<u64, Vec<Run>>> = BTreeMap::new();
    for r in recs.iter().filter(|r| s(r, "rec") == "rho") {
        let q: u64 = r["q"].parse().unwrap();
        let sz = match sizes.get(&q) {
            Some(x) => x,
            None => continue,
        };
        let arm = if r.contains_key("arm") { r["arm"].clone() } else { "plain".into() };
        let meq = f(r, "c.mul") + sz.r_inv * f(r, "c.inv") + sz.r_l * f(r, "c.lmul");
        rho.entry(arm).or_default().entry(q).or_default().push(Run {
            seed: r["seed"].parse().unwrap(),
            ok: s(r, "ok") == "true",
            meq,
            unp: f(r, "c.hash") + f(r, "c.cmp"),
            wall: f(r, "wall"),
            steps: f(r, "steps"),
            sqrt_ell: f(r, "ell").sqrt(),
        });
    }
    let mut js: Vec<String> = Vec::new();
    let nb = 2000;
    // ---------------------------------------------------------------- per size tables
    println!("## Cold single-target cost per arm and size (successful units; M-eq at w = 0)\n");
    println!("| arm | q | k | units ok/total | attempts | M-eq w=0 | w=0.1 | w=1 | wall s | phase shares fb/rel/mat/LA/logs/desc/ver (%) |");
    println!("|:--|--:|--:|--:|--:|--:|--:|--:|--:|:--|");
    for arm in IC_ARMS {
        let Some(m) = ic.get(arm) else { continue };
        for (&q, us) in m {
            let sz = &sizes[&q];
            let ok: Vec<&Unit> = us.iter().filter(|u| u.ok).collect();
            if ok.is_empty() {
                println!("| {} | {} | {} | 0/{} | | | | | | |", arm, q, kof(q), us.len());
                continue;
            }
            let vm: [f64; 4] = [0, 1, 2, 3].map(|b| mean(&ok.iter().map(|u| u.v(sz.gae)[b]).collect::<Vec<_>>()));
            let phm: [f64; 7] = [0, 1, 2, 3, 4, 5, 6].map(|i| mean(&ok.iter().map(|u| u.ph[i]).collect::<Vec<_>>()));
            let phwm: [f64; 7] = [0, 1, 2, 3, 4, 5, 6].map(|i| mean(&ok.iter().map(|u| u.phw[i]).collect::<Vec<_>>()));
            let tot: f64 = phm.iter().sum();
            let att = mean(&us.iter().map(|u| u.attempts as f64).collect::<Vec<_>>());
            let failed_cost: Vec<f64> = us.iter().filter(|u| !u.ok).map(|u| u.meq).collect();
            println!(
                "| {} | {} | {} | {}/{} | {:.2} | {:.3e} | {:.3e} | {:.3e} | {:.2} | {} |",
                arm, q, kof(q), ok.len(), us.len(), att, vm[0], vm[1], vm[2], vm[3],
                phm.iter().map(|x| format!("{:.0}", 100.0 * x / tot)).collect::<Vec<_>>().join("/")
            );
            // stats means
            let mut stm: BTreeMap<String, f64> = BTreeMap::new();
            for u in &ok {
                for (k, v) in &u.stats {
                    *stm.entry(k.clone()).or_insert(0.0) += v / ok.len() as f64;
                }
            }
            js.push(format!(
                "{{\"kind\":\"ic_size\",\"arm\":\"{}\",\"q\":{},\"k\":{},\"units\":{},\"units_ok\":{},\"attempts_mean\":{:.3},\"meq\":[{:.6e},{:.6e},{:.6e}],\"wall_s\":{:.4},\"phase_meq\":[{}],\"phase_wall\":[{}],\"failed_unit_meq\":[{}],\"r_inv\":{:.4},\"r_lmul\":{:.4},\"gae\":{:.3},\"load1\":{:.1},\"stats\":{{{}}}}}",
                arm, q, kof(q), us.len(), ok.len(), att, vm[0], vm[1], vm[2], vm[3],
                phm.iter().map(|x| format!("{:.5e}", x)).collect::<Vec<_>>().join(","),
                phwm.iter().map(|x| format!("{:.5}", x)).collect::<Vec<_>>().join(","),
                failed_cost.iter().map(|x| format!("{:.4e}", x)).collect::<Vec<_>>().join(","),
                sz.r_inv, sz.r_l, sz.gae, sz.load,
                stm.iter().map(|(k, v)| format!("\"{}\":{:.4}", k, v)).collect::<Vec<_>>().join(",")
            ));
        }
    }
    println!("\n## rho per size\n");
    println!("| rho | q | k | runs ok/total | M-eq w=0 | w=1 | wall s | steps/sqrt(ell) | M-eq per step | model M-eq (pooled calibration) |");
    println!("|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let mut calib: BTreeMap<String, f64> = BTreeMap::new();
    for arm in RHO_ARMS {
        let Some(m) = rho.get(arm) else { continue };
        let all: Vec<f64> = m.values().flatten().filter(|r| kof(r.sqrt_ell as u64) > 0).map(|r| r.steps / r.sqrt_ell).collect();
        let c = mean(&all);
        calib.insert(arm.to_string(), c);
        for (&q, rs) in m {
            let sz = &sizes[&q];
            let vm: [f64; 4] = [0, 1, 2, 3].map(|b| mean(&rs.iter().map(|r| r.v(sz.gae)[b]).collect::<Vec<_>>()));
            let spq = mean(&rs.iter().map(|r| r.steps / r.sqrt_ell).collect::<Vec<_>>());
            let per_step = rs.iter().map(|r| r.meq).sum::<f64>() / rs.iter().map(|r| r.steps).sum::<f64>();
            let model = per_step * c * sz.sqrt_ell;
            let nok = rs.iter().filter(|r| r.ok).count();
            println!("| {} | {} | {} | {}/{} | {:.3e} | {:.3e} | {:.2} | {:.3} | {:.1} | {:.3e} |", arm, q, kof(q), nok, rs.len(), vm[0], vm[2], vm[3], spq, per_step, model);
            js.push(format!(
                "{{\"kind\":\"rho_size\",\"arm\":\"{}\",\"q\":{},\"k\":{},\"runs\":{},\"runs_ok\":{},\"meq\":[{:.6e},{:.6e},{:.6e}],\"wall_s\":{:.4},\"steps_over_sqrt_ell\":{:.4},\"meq_per_step\":{:.3},\"model_meq\":{:.6e},\"gae\":{:.3}}}",
                arm, q, kof(q), rs.len(), nok, vm[0], vm[1], vm[2], vm[3], spq, per_step, model, sz.gae
            ));
        }
        js.push(format!("{{\"kind\":\"rho_calibration\",\"arm\":\"{}\",\"pooled_steps_over_sqrt_ell\":{:.4},\"runs\":{}}}", arm, c, all.len()));
    }
    // helpers: per size value lists for an arm (IC successful units or rho runs), index b of v()
    let ic_vals = |arm: &str, q: u64| -> Vec<[f64; 4]> {
        ic.get(arm).and_then(|m| m.get(&q)).map(|us| us.iter().filter(|u| u.ok).map(|u| u.v(sizes[&q].gae)).collect()).unwrap_or_default()
    };
    let rho_vals = |arm: &str, q: u64| -> Vec<[f64; 4]> { rho.get(arm).and_then(|m| m.get(&q)).map(|rs| rs.iter().map(|r| r.v(sizes[&q].gae)).collect()).unwrap_or_default() };
    let vals = |arm: &str, q: u64| -> Vec<[f64; 4]> { if RHO_ARMS.contains(&arm) { rho_vals(arm, q) } else { ic_vals(arm, q) } };
    let resample = |v: &Vec<[f64; 4]>, rng: &mut Rng, b: usize| -> f64 {
        let n = v.len();
        let mut acc = 0.0;
        for _ in 0..n {
            acc += v[rng.below(n as u64) as usize][b];
        }
        acc / n as f64
    };
    // ---------------------------------------------------------------- ratios IC / rho
    println!("\n## IC / rho per size (95% bootstrap), band w = 0 / 0.1 / 1 GAE, and wall\n");
    println!("| IC arm | rho | q | w=0 | w=0.1 | w=1 | resolved | wall ratio |");
    println!("|:--|:--|--:|:--|:--|:--|:--|:--|");
    // ratio bootstrap samples kept for crossovers: (ic arm, rho arm) -> q -> [b] -> samples
    let mut rsamp: BTreeMap<(String, String), BTreeMap<u64, [Vec<f64>; 4]>> = BTreeMap::new();
    let mut rmean: BTreeMap<(String, String), BTreeMap<u64, [f64; 4]>> = BTreeMap::new();
    for a in IC_ARMS {
        for r in RHO_ARMS {
            for &q in sizes.keys() {
                let iv = ic_vals(a, q);
                let rv = rho_vals(r, q);
                if iv.is_empty() || rv.is_empty() {
                    continue;
                }
                let mut rng = Rng::new(0xB007 ^ q ^ ((a.len() as u64) << 40) ^ ((r.len() as u64) << 48));
                let mut sm: [Vec<f64>; 4] = [vec![], vec![], vec![], vec![]];
                for _ in 0..nb {
                    for b in 0..4 {
                        let x = resample(&iv, &mut rng, b);
                        let y = resample(&rv, &mut rng, b);
                        sm[b].push(x / y);
                    }
                }
                let pm: [f64; 4] = [0, 1, 2, 3].map(|b| mean(&iv.iter().map(|v| v[b]).collect::<Vec<_>>()) / mean(&rv.iter().map(|v| v[b]).collect::<Vec<_>>()));
                let ci: Vec<(f64, f64)> = (0..4).map(|b| (pct(&sm[b], 0.025), pct(&sm[b], 0.975))).collect();
                let resolved = if (0..3).all(|b| ci[b].1 < 1.0) { "IC lower" } else if (0..3).all(|b| ci[b].0 > 1.0) { "rho lower" } else { "unresolved" };
                println!(
                    "| {} | {} | {} | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | {} | {:.3} [{:.3},{:.3}] |",
                    a, r, q, pm[0], ci[0].0, ci[0].1, pm[1], ci[1].0, ci[1].1, pm[2], ci[2].0, ci[2].1, resolved, pm[3], ci[3].0, ci[3].1
                );
                js.push(format!(
                    "{{\"kind\":\"ratio\",\"ic\":\"{}\",\"rho\":\"{}\",\"q\":{},\"ratio\":[{:.5},{:.5},{:.5}],\"lo\":[{:.5},{:.5},{:.5}],\"hi\":[{:.5},{:.5},{:.5}],\"resolved\":\"{}\",\"wall_ratio\":[{:.5},{:.5},{:.5}]}}",
                    a, r, q, pm[0], pm[1], pm[2], ci[0].0, ci[1].0, ci[2].0, ci[0].1, ci[1].1, ci[2].1, resolved, pm[3], ci[3].0, ci[3].1
                ));
                rsamp.entry((a.into(), r.into())).or_default().insert(q, sm);
                rmean.entry((a.into(), r.into())).or_default().insert(q, pm);
            }
        }
    }
    // ---------------------------------------------------------------- paired improvements
    println!("\n## Paired arm ratios on the same instances (bootstrap over seeds), M-eq w = 0 / 0.1 / 1, wall\n");
    println!("| arm / reference | q | w=0 | w=0.1 | w=1 | resolved | wall |");
    println!("|:--|--:|:--|:--|:--|:--|:--|");
    let pairs = [("S", "B0"), ("L0", "S"), ("L1", "S"), ("L", "S"), ("L", "L1"), ("L", "B0"), ("D", "S"), ("D", "B0"), ("DL1", "D"), ("DL", "D"), ("DL", "B0"), ("DL", "S")];
    for (a, b) in pairs {
        let (Some(ma), Some(mb)) = (ic.get(a), ic.get(b)) else { continue };
        for (&q, ua) in ma {
            let Some(ub) = mb.get(&q) else { continue };
            let sz = &sizes[&q];
            let pr: Vec<([f64; 4], [f64; 4])> = ua.iter().filter(|u| u.ok).filter_map(|u| ub.iter().find(|v| v.seed == u.seed && v.ok).map(|v| (u.v(sz.gae), v.v(sz.gae)))).collect();
            if pr.is_empty() {
                continue;
            }
            let mut rng = Rng::new(0xA11 ^ q);
            let mut sm: [Vec<f64>; 4] = [vec![], vec![], vec![], vec![]];
            for _ in 0..nb {
                let idx: Vec<usize> = (0..pr.len()).map(|_| rng.below(pr.len() as u64) as usize).collect();
                for bb in 0..4 {
                    let x: f64 = idx.iter().map(|&i| pr[i].0[bb]).sum();
                    let y: f64 = idx.iter().map(|&i| pr[i].1[bb]).sum();
                    sm[bb].push(x / y);
                }
            }
            let pm: [f64; 4] = [0, 1, 2, 3].map(|bb| pr.iter().map(|p| p.0[bb]).sum::<f64>() / pr.iter().map(|p| p.1[bb]).sum::<f64>());
            let ci: Vec<(f64, f64)> = (0..4).map(|bb| (pct(&sm[bb], 0.025), pct(&sm[bb], 0.975))).collect();
            let resolved = if (0..3).all(|bb| ci[bb].1 < 1.0) { "lower" } else if (0..3).all(|bb| ci[bb].0 > 1.0) { "higher" } else { "unresolved" };
            println!("| {} / {} | {} | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | {} | {:.3} [{:.3},{:.3}] |", a, b, q, pm[0], ci[0].0, ci[0].1, pm[1], ci[1].0, ci[1].1, pm[2], ci[2].0, ci[2].1, resolved, pm[3], ci[3].0, ci[3].1);
            js.push(format!(
                "{{\"kind\":\"paired\",\"arm\":\"{}\",\"ref\":\"{}\",\"q\":{},\"n\":{},\"ratio\":[{:.5},{:.5},{:.5}],\"lo\":[{:.5},{:.5},{:.5}],\"hi\":[{:.5},{:.5},{:.5}],\"resolved\":\"{}\",\"wall\":[{:.5},{:.5},{:.5}]}}",
                a, b, q, pr.len(), pm[0], pm[1], pm[2], ci[0].0, ci[1].0, ci[2].0, ci[0].1, ci[1].1, ci[2].1, resolved, pm[3], ci[3].0, ci[3].1
            ));
        }
    }
    // ---------------------------------------------------------------- exponent fits
    println!("\n## Exponent fits ln cost = a + e ln q (95% bootstrap over units and runs)\n");
    println!("| arm | range | sizes | e (w=0) | e (w=0.1) | e (w=1) | e (wall) | bend (w=0) | bend (w=1) |");
    println!("|:--|:--|--:|:--|:--|:--|:--|:--|:--|");
    let ranges: [(&str, u32, u32); 3] = [("full", 12, 22), ("low", 12, 16), ("high", 17, 22)];
    let mut fitstore: BTreeMap<(String, String), ([f64; 4], [Vec<f64>; 4], [f64; 4])> = BTreeMap::new(); // point slope, samples, intercept
    let all_arms: Vec<&str> = IC_ARMS.iter().chain(RHO_ARMS.iter()).copied().collect();
    for arm in &all_arms {
        for (rn, k0, k1) in ranges {
            let qs: Vec<u64> = sizes.keys().copied().filter(|&q| kof(q) >= k0 && kof(q) <= k1 && !vals(arm, q).is_empty()).collect();
            if qs.len() < 3 {
                continue;
            }
            let xs: Vec<f64> = qs.iter().map(|&q| (q as f64).ln()).collect();
            let vv: Vec<Vec<[f64; 4]>> = qs.iter().map(|&q| vals(arm, q)).collect();
            let mut pt = [0.0; 4];
            let mut ic0 = [0.0; 4];
            for b in 0..4 {
                let ys: Vec<f64> = vv.iter().map(|v| mean(&v.iter().map(|x| x[b]).collect::<Vec<_>>()).ln()).collect();
                let (e, a0) = slope(&xs, &ys);
                pt[b] = e;
                ic0[b] = a0;
            }
            let mut rng = Rng::new(0xFEED ^ (k0 as u64) << 8 ^ (arm.len() as u64));
            let mut sm: [Vec<f64>; 4] = [vec![], vec![], vec![], vec![]];
            for _ in 0..nb {
                let means: Vec<[f64; 4]> = vv.iter().map(|v| {
                    let idx: Vec<usize> = (0..v.len()).map(|_| rng.below(v.len() as u64) as usize).collect();
                    [0, 1, 2, 3].map(|b| idx.iter().map(|&i| v[i][b]).sum::<f64>() / idx.len() as f64)
                }).collect();
                for b in 0..4 {
                    let ys: Vec<f64> = means.iter().map(|m| m[b].ln()).collect();
                    sm[b].push(slope(&xs, &ys).0);
                }
            }
            let ci = |b: usize| (pct(&sm[b], 0.025), pct(&sm[b], 0.975));
            let bend = |b: usize| -> &'static str {
                let (lo, hi) = ci(b);
                let c1 = lo <= 1.0 && hi >= 1.0;
                let c12 = lo <= 1.2 && hi >= 1.2;
                if c1 && c12 { "unresolved" } else if c1 { "toward 1" } else if c12 { "toward 6/5" } else if hi < 1.0 { "below 1" } else if lo > 1.2 { "above 6/5" } else { "between 1 and 6/5" }
            };
            let isic = IC_ARMS.contains(arm);
            println!(
                "| {} | {} (k {}..{}) | {} | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | {} | {} |",
                arm, rn, k0, k1, qs.len(), pt[0], ci(0).0, ci(0).1, pt[1], ci(1).0, ci(1).1, pt[2], ci(2).0, ci(2).1, pt[3], ci(3).0, ci(3).1,
                if isic { bend(0) } else { "" }, if isic { bend(2) } else { "" }
            );
            js.push(format!(
                "{{\"kind\":\"fit\",\"arm\":\"{}\",\"range\":\"{}\",\"k0\":{},\"k1\":{},\"sizes\":{},\"e\":[{:.4},{:.4},{:.4},{:.4}],\"lo\":[{:.4},{:.4},{:.4},{:.4}],\"hi\":[{:.4},{:.4},{:.4},{:.4}],\"a\":[{:.4},{:.4},{:.4},{:.4}],\"bend_w0\":\"{}\",\"bend_w1\":\"{}\"}}",
                arm, rn, k0, k1, qs.len(), pt[0], pt[1], pt[2], pt[3], ci(0).0, ci(1).0, ci(2).0, ci(3).0, ci(0).1, ci(1).1, ci(2).1, ci(3).1, ic0[0], ic0[1], ic0[2], ic0[3],
                if isic { bend(0) } else { "" }, if isic { bend(2) } else { "" }
            ));
            fitstore.insert((arm.to_string(), rn.to_string()), (pt, sm, ic0));
        }
        // curvature high - low
        if let (Some(h), Some(l)) = (fitstore.get(&(arm.to_string(), "high".into())), fitstore.get(&(arm.to_string(), "low".into()))) {
            let d: Vec<Vec<f64>> = (0..4).map(|b| h.1[b].iter().zip(&l.1[b]).map(|(x, y)| x - y).collect()).collect();
            println!(
                "| {} | curvature e_high - e_low | | {:.3} [{:.3},{:.3}] | | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] | | |",
                arm, h.0[0] - l.0[0], pct(&d[0], 0.025), pct(&d[0], 0.975), h.0[2] - l.0[2], pct(&d[2], 0.025), pct(&d[2], 0.975), h.0[3] - l.0[3], pct(&d[3], 0.025), pct(&d[3], 0.975)
            );
            js.push(format!(
                "{{\"kind\":\"curvature\",\"arm\":\"{}\",\"d\":[{:.4},{:.4},{:.4}],\"lo\":[{:.4},{:.4},{:.4}],\"hi\":[{:.4},{:.4},{:.4}]}}",
                arm, h.0[0] - l.0[0], h.0[2] - l.0[2], h.0[3] - l.0[3], pct(&d[0], 0.025), pct(&d[2], 0.025), pct(&d[3], 0.025), pct(&d[0], 0.975), pct(&d[2], 0.975), pct(&d[3], 0.975)
            ));
        }
    }
    // exponent gap rho-neg - IC (full range)
    println!("\n## Exponent gap e_rho - e_IC on the full range (95% bootstrap)\n");
    println!("| IC arm | rho | gap w=0 | gap w=1 |");
    println!("|:--|:--|:--|:--|");
    for a in IC_ARMS {
        for r in RHO_ARMS {
            if let (Some(fa), Some(fr)) = (fitstore.get(&(a.to_string(), "full".into())), fitstore.get(&(r.to_string(), "full".into()))) {
                let g: Vec<Vec<f64>> = (0..3).map(|b| fr.1[b].iter().zip(&fa.1[b]).map(|(x, y)| x - y).collect()).collect();
                println!("| {} | {} | {:.3} [{:.3},{:.3}] | {:.3} [{:.3},{:.3}] |", a, r, fr.0[0] - fa.0[0], pct(&g[0], 0.025), pct(&g[0], 0.975), fr.0[2] - fa.0[2], pct(&g[2], 0.025), pct(&g[2], 0.975));
                js.push(format!(
                    "{{\"kind\":\"gap\",\"ic\":\"{}\",\"rho\":\"{}\",\"gap\":[{:.4},{:.4}],\"lo\":[{:.4},{:.4}],\"hi\":[{:.4},{:.4}]}}",
                    a, r, fr.0[0] - fa.0[0], fr.0[2] - fa.0[2], pct(&g[0], 0.025), pct(&g[2], 0.025), pct(&g[0], 0.975), pct(&g[2], 0.975)
                ));
            }
        }
    }
    // ---------------------------------------------------------------- crossovers
    println!("\n## Crossovers (M-eq at w = 0 / 0.1 / 1 GAE, and wall time)\n");
    println!("| IC arm | rho | measure | resolved crossover (smallest grid q from which IC is band-resolved lower) | interpolated q* [95% boot] | note |");
    println!("|:--|:--|:--|:--|:--|:--|");
    let qmin = *sizes.keys().next().unwrap_or(&1) as f64;
    for ((a, r), m) in &rmean {
        let qs: Vec<u64> = m.keys().copied().collect();
        let samp = &rsamp[&(a.clone(), r.clone())];
        for (bi, bname) in [(0usize, "M-eq w=0"), (1, "M-eq w=0.1"), (2, "M-eq w=1"), (3, "wall")] {
            // resolved: per size CI upper < 1 (for M-eq: at all three weights)
            let res_lower = |q: u64| -> bool {
                let sm = &samp[&q];
                if bi == 3 { pct(&sm[3], 0.975) < 1.0 } else { (0..3).all(|b| pct(&sm[b], 0.975) < 1.0) }
            };
            let mut rc: Option<u64> = None;
            for (t, &q) in qs.iter().enumerate() {
                if qs[t..].iter().all(|&qq| res_lower(qq)) {
                    rc = Some(q);
                    break;
                }
            }
            let cross = |vals: &Vec<f64>| -> f64 {
                // highest crossing from >= 1 to < 1, interpolated in ln q; -inf if always < 1, +inf if never < 1 at the top
                let n = vals.len();
                if vals[n - 1] >= 1.0 {
                    return f64::INFINITY;
                }
                let mut t = n - 1;
                while t > 0 && vals[t - 1] < 1.0 {
                    t -= 1;
                }
                if t == 0 {
                    return f64::NEG_INFINITY;
                }
                let (x0, x1) = ((qs[t - 1] as f64).ln(), (qs[t] as f64).ln());
                let (y0, y1) = (vals[t - 1].ln(), vals[t].ln());
                (x0 + (0.0 - y0) * (x1 - x0) / (y1 - y0)).exp()
            };
            let pmv: Vec<f64> = qs.iter().map(|q| m[q][bi]).collect();
            let qstar = cross(&pmv);
            let mut bs: Vec<f64> = Vec::new();
            for i in 0..nb {
                let v: Vec<f64> = qs.iter().map(|q| samp[q][bi][i]).collect();
                bs.push(cross(&v));
            }
            let below = bs.iter().filter(|x| **x == f64::NEG_INFINITY).count() as f64 / nb as f64;
            let above = bs.iter().filter(|x| **x == f64::INFINITY).count() as f64 / nb as f64;
            let fin: Vec<f64> = bs.iter().copied().filter(|x| x.is_finite()).collect();
            let qs_txt = if qstar.is_finite() {
                format!("{:.0} [{:.0},{:.0}]", qstar, pct(&fin, 0.025), pct(&fin, 0.975))
            } else if qstar == f64::NEG_INFINITY {
                format!("below q_min = {}", qmin)
            } else {
                "above q_max".into()
            };
            // extrapolation from full-range fits when outside
            let mut note = format!("boot: {:.0}% below grid, {:.0}% above", 100.0 * below, 100.0 * above);
            if !qstar.is_finite() {
                if let (Some(fa), Some(fr)) = (fitstore.get(&(a.clone(), "full".into())), fitstore.get(&(r.clone(), "full".into()))) {
                    let bb = bi.min(3);
                    let de = fa.0[bb] - fr.0[bb];
                    if de.abs() > 1e-6 {
                        let lq = (fr.2[bb] - fa.2[bb]) / de;
                        note += &format!("; extrapolated from full-range fits (extrapolation): q* ~ {:.0}", lq.exp());
                    }
                }
            }
            println!("| {} | {} | {} | {} | {} | {} |", a, r, bname, rc.map(|x| x.to_string()).unwrap_or("none in grid".into()), qs_txt, note);
            js.push(format!(
                "{{\"kind\":\"crossover\",\"ic\":\"{}\",\"rho\":\"{}\",\"measure\":\"{}\",\"resolved_q\":{},\"qstar\":{},\"qstar_lo\":{},\"qstar_hi\":{},\"boot_below\":{:.4},\"boot_above\":{:.4},\"note\":\"{}\"}}",
                a, r, bname, rc.map(|x| x.to_string()).unwrap_or("null".into()),
                if qstar.is_finite() { format!("{:.1}", qstar) } else if qstar < 0.0 { "\"below\"".into() } else { "\"above\"".into() },
                if fin.is_empty() { "null".into() } else { format!("{:.1}", pct(&fin, 0.025)) },
                if fin.is_empty() { "null".into() } else { format!("{:.1}", pct(&fin, 0.975)) },
                below, above, note
            ));
        }
    }
    // rho-neg / plain pooled (k >= 12): geometric mean of per-size ratios
    if let (Some(mn), Some(mp)) = (rho.get("neg"), rho.get("plain")) {
        let qs: Vec<u64> = mn.keys().copied().filter(|q| kof(*q) >= 12 && mp.contains_key(q)).collect();
        if !qs.is_empty() {
            let mut rng = Rng::new(0x4E47);
            let pt: f64 = (qs.iter().map(|q| (mean(&rho_vals("neg", *q).iter().map(|v| v[0]).collect::<Vec<_>>()) / mean(&rho_vals("plain", *q).iter().map(|v| v[0]).collect::<Vec<_>>())).ln()).sum::<f64>() / qs.len() as f64).exp();
            let mut bs = Vec::new();
            for _ in 0..nb {
                let g: f64 = qs.iter().map(|q| (resample(&rho_vals("neg", *q), &mut rng, 0) / resample(&rho_vals("plain", *q), &mut rng, 0)).ln()).sum::<f64>() / qs.len() as f64;
                bs.push(g.exp());
            }
            println!("\nrho-neg / rho-plain M-eq (w = 0), geometric mean over {} sizes k >= 12: {:.3} [{:.3}, {:.3}]", qs.len(), pt, pct(&bs, 0.025), pct(&bs, 0.975));
            js.push(format!("{{\"kind\":\"neg_over_plain\",\"sizes\":{},\"ratio\":{:.4},\"lo\":{:.4},\"hi\":{:.4}}}", qs.len(), pt, pct(&bs, 0.025), pct(&bs, 0.975)));
        }
    }
    // phase exponents per arm (full range)
    println!("\n## Phase exponents (full range, fits of phase means)\n");
    for arm in IC_ARMS {
        let Some(m) = ic.get(arm) else { continue };
        let qs: Vec<u64> = m.keys().copied().filter(|&q| kof(q) >= 12 && m[&q].iter().any(|u| u.ok)).collect();
        if qs.len() < 3 {
            continue;
        }
        let xs: Vec<f64> = qs.iter().map(|&q| (q as f64).ln()).collect();
        let mut line = format!("- {}:", arm);
        let mut pj = Vec::new();
        for i in 0..7 {
            let ys: Vec<f64> = qs.iter().map(|q| mean(&m[q].iter().filter(|u| u.ok).map(|u| u.ph[i]).collect::<Vec<_>>()).max(1.0).ln()).collect();
            if ys.iter().all(|y| *y > 100f64.ln()) {
                let e = slope(&xs, &ys).0;
                line += &format!(" {}={:.2}", PHASES[i], e);
                pj.push(format!("\"{}\":{:.4}", PHASES[i], e));
            }
        }
        println!("{}", line);
        js.push(format!("{{\"kind\":\"phase_exponents\",\"arm\":\"{}\",{}}}", arm, pj.join(",")));
    }
    std::fs::write(out_path, js.join("\n") + "\n").unwrap();
}

/// Replay comparison of two record files: every field except wall times, load, timing ratios.
pub fn replay_cmp(a: &str, b: &str) -> bool {
    let rd = |p: &str| -> Vec<Rec> { std::fs::read_to_string(p).unwrap().lines().filter(|l| l.starts_with('{')).map(parse_line).filter(|r| s(r, "rec") != "ratios").collect() };
    let (ra, rb) = (rd(a), rd(b));
    let skip = |k: &str| k.ends_with("wall") || k == "load1" || k == "s.wall_total" || k == "r_inv" || k == "r_lmul" || k == "search_s";
    if ra.len() != rb.len() {
        println!("record count differs: {} vs {}", ra.len(), rb.len());
        return false;
    }
    let mut same = true;
    for (x, y) in ra.iter().zip(&rb) {
        let mut keys: Vec<&String> = x.keys().chain(y.keys()).filter(|k| !skip(k)).collect();
        keys.sort();
        keys.dedup();
        for k in keys {
            if x.get(k) != y.get(k) {
                println!("differs: arm={} attempt={} key={} : {:?} vs {:?}", s(x, "arm"), s(x, "attempt"), k, x.get(k), y.get(k));
                same = false;
            }
        }
    }
    println!("{}", if same { "REPLAY IDENTICAL" } else { "REPLAY DIFFERS" });
    same
}
