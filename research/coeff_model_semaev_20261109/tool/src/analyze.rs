//! Native analysis (PROTOCOL.md section 5; amendment A1 ports tool/analyze.py
//! unchanged in statistics: per-arm bootstrap of mean normalized yield, ratio
//! contrasts with 95% percentile CIs and two-sided bootstrap p, Holm, and a
//! permutation test of between-arm variance).
//!
//! cmsem analyze <rows.jsonl> <cell-name> <mode s3|s4> <out.json>
use crate::Rng;
use std::collections::BTreeMap;
use std::fmt::Write as _;

const NBOOT: usize = 10000;
const NPERM: usize = 10000;

fn field<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{}\":", key);
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = rest.find(|c| c == ',' || c == '}').unwrap_or(rest.len());
    Some(rest[..end].trim_matches('"'))
}
fn num(line: &str, key: &str) -> Option<f64> {
    field(line, key).and_then(|v| if v == "null" { None } else { v.parse().ok() })
}

struct ArmData {
    fx: f64,
    fpts: f64,
    ng: f64,
    y: Vec<f64>,
    yn: Vec<f64>,
    muls: Vec<f64>,
    ns: Vec<f64>,
    sqrt: Vec<f64>,
    jac: Vec<f64>,
    planted: usize,
    xc: usize,
    mism: usize,
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn sd(v: &[f64]) -> f64 {
    let m = mean(v);
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() as f64 - 1.0)).sqrt()
}
fn quantile(sorted: &[f64], q: f64) -> f64 {
    let pos = (sorted.len() - 1) as f64 * q;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    sorted[lo] + (sorted[hi] - sorted[lo]) * (pos - lo as f64)
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    quantile(&s, 0.5)
}

pub fn cmd_analyze(args: &[String]) {
    let text = std::fs::read_to_string(&args[0]).unwrap();
    let cell = args[1].clone();
    let mode = args[2].clone();
    let outp = args[3].clone();
    let mut arms: BTreeMap<String, ArmData> = BTreeMap::new();
    let mut rows = 0usize;
    let mut tgt: Vec<(String, usize, String)> = vec![];
    for l in text.lines() {
        let kind = field(l, "kind").unwrap();
        let arm = field(l, "arm").unwrap().to_string();
        if kind == "base" {
            arms.insert(arm, ArmData {
                fx: num(l, "fx").unwrap(), fpts: num(l, "fpts").unwrap(), ng: num(l, "ngroup").unwrap(),
                y: vec![], yn: vec![], muls: vec![], ns: vec![], sqrt: vec![], jac: vec![], planted: 0, xc: 0, mism: 0,
            });
        } else {
            rows += 1;
            tgt.push((arm, num(l, "t").unwrap() as usize, l.to_string()));
        }
    }
    tgt.sort_by(|a, b| (a.0.clone(), a.1).cmp(&(b.0.clone(), b.1)));
    for (arm, _, l) in tgt.iter() {
        let d = arms.get_mut(arm).unwrap();
        let yrf = num(l, "y_rf");
        let ygl = num(l, "y_gl");
        let y = if mode == "s4" { num(l, "y4").unwrap() } else { yrf.or(ygl).unwrap() };
        let norm = if mode == "s4" { d.ng / d.fpts.powi(3) } else { d.ng / d.fpts.powi(2) };
        if let (Some(a), Some(b)) = (yrf, ygl) {
            d.xc += 1;
            if a != b {
                d.mism += 1;
            }
        }
        d.y.push(y);
        d.yn.push(y * norm);
        d.muls.push(num(l, "muls").unwrap());
        d.ns.push(num(l, "ns").unwrap());
        d.sqrt.push(num(l, "sqrt").unwrap());
        d.jac.push(num(l, "jacobi").unwrap());
        if field(l, "planted") == Some("true") {
            d.planted += 1;
        }
    }
    let names: Vec<String> = arms.keys().cloned().collect();
    let mut rng = Rng(20261109);
    // bootstrap means per arm
    let mut boots: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for a in names.iter() {
        let v = &arms[a].yn;
        let n = v.len();
        let mut bs = Vec::with_capacity(NBOOT);
        for _ in 0..NBOOT {
            let mut s = 0.0;
            for _ in 0..n {
                s += v[rng.below(n as u64) as usize];
            }
            bs.push(s / n as f64);
        }
        boots.insert(a.clone(), bs);
    }
    let gmean = |g: &[&str]| -> f64 { g.iter().map(|a| mean(&arms[*a].yn)).sum::<f64>() / g.len() as f64 };
    let contrast = |num_: &[&str], den: &[&str]| -> (f64, f64, f64, f64) {
        let est = gmean(num_) / gmean(den);
        let mut r: Vec<f64> = (0..NBOOT)
            .map(|i| {
                let bn = num_.iter().map(|a| boots[*a][i]).sum::<f64>() / num_.len() as f64;
                let bd = den.iter().map(|a| boots[*a][i]).sum::<f64>() / den.len() as f64;
                bn / bd
            })
            .filter(|x| x.is_finite())
            .collect();
        r.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let le = r.iter().filter(|&&x| x <= 1.0).count() as f64 / r.len() as f64;
        let ge = r.iter().filter(|&&x| x >= 1.0).count() as f64 / r.len() as f64;
        (est, quantile(&r, 0.025), quantile(&r, 0.975), (2.0 * le.min(ge)).min(1.0))
    };
    let cjson = |num_: &[&str], den: &[&str], c: (f64, f64, f64, f64)| -> String {
        format!("{{\"num\":{:?},\"den\":{:?},\"ratio\":{:.6},\"ci95\":[{:.6},{:.6}],\"p_boot\":{:.5}", num_, den, c.0, c.1, c.2, c.3)
    };
    let mut o = String::new();
    let xc: usize = arms.values().map(|d| d.xc).sum();
    let mism: usize = arms.values().map(|d| d.mism).sum();
    write!(o, "{{\"cell\":\"{}\",\"mode\":\"{}\",\"generator\":\"cmsem analyze (native)\",\"rows\":{},\"crosscheck_targets\":{},\"crosscheck_mismatches\":{},\"cell_void\":{}", cell, mode, rows, xc, mism, mism > 0).unwrap();
    // per arm
    let mut pa = vec![];
    let mut mpe: Vec<f64> = vec![];
    for a in names.iter() {
        let d = &arms[a];
        let k = if mode == "s4" { 3 } else { 2 };
        let mpe_a = if mode == "s3" && mean(&d.muls) > 0.0 { Some(mean(&d.muls) / d.fx) } else { None };
        if let Some(x) = mpe_a {
            mpe.push(x);
        }
        let mn = d.muls.iter().cloned().fold(f64::INFINITY, f64::min);
        let mx = d.muls.iter().cloned().fold(0.0, f64::max);
        pa.push(format!(
            "\"{}\":{{\"targets\":{},\"planted_targets\":{},\"fx\":{},\"fpts\":{},\"expected_y\":{:.6},\"y_total\":{},\"y_mean\":{:.6},\"ynorm_mean\":{:.6},\"ynorm_se\":{:.6},\"muls_per_target_mean\":{:.1},\"muls_per_target_min\":{},\"muls_per_target_max\":{},\"muls_per_base_element\":{},\"sqrt_per_target_mean\":{:.2},\"jacobi_per_target_mean\":{:.2},\"ns_per_target_median_descriptive\":{:.0},\"ns_per_base_element_median_descriptive\":{}}}",
            a, d.y.len(), d.planted, d.fx, d.fpts, d.fpts.powi(k) / d.ng, d.y.iter().sum::<f64>(), mean(&d.y), mean(&d.yn), sd(&d.yn) / (d.yn.len() as f64).sqrt(),
            mean(&d.muls), mn, mx, mpe_a.map(|x| format!("{:.4}", x)).unwrap_or("null".into()),
            mean(&d.sqrt), mean(&d.jac), median(&d.ns),
            if mode == "s3" { format!("{:.3}", median(&d.ns) / d.fx) } else { "null".into() }
        ));
    }
    write!(o, ",\"per_arm\":{{{}}}", pa.join(",")).unwrap();
    let has = |x: &str| arms.contains_key(x);
    let nm: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
    let mut fam: Vec<(String, String, f64)> = vec![]; // key, json-prefix, p
    let side = has("W");
    let mut pc_detected = true;
    let mut p1ci = (0.0, 0.0);
    if side {
        let refg: Vec<&str> = nm.iter().cloned().filter(|a| *a == "W" || *a == "W_AA").collect();
        for a in nm.iter() {
            if *a == "W" {
                continue;
            }
            let den: Vec<&str> = if *a == "W_AA" { vec!["W"] } else { refg.clone() };
            let c = contrast(&[a], &den);
            fam.push((format!("{}_vs_W", a), cjson(&[a], &den, c), c.3));
        }
    } else {
        let rand: Vec<&str> = nm.iter().cloned().filter(|a| a.starts_with("RAND")).collect();
        let small: Vec<&str> = nm.iter().cloned().filter(|a| a.starts_with("SMALL") && !a.ends_with("_ISO")).collect();
        if !small.is_empty() && !rand.is_empty() {
            let c = contrast(&small, &rand);
            p1ci = (c.1, c.2);
            write!(o, ",\"P1_small_vs_rand\":{}}}", cjson(&small, &rand, c)).unwrap();
        }
        for a in nm.iter() {
            if a.starts_with("SMALL") || a.starts_with("A3_") || a.starts_with("E0_ISO") || *a == "E0" {
                let c = contrast(&[a], &rand);
                fam.push((format!("{}_vs_RAND", a), cjson(&[a], &rand, c), c.3));
            }
        }
        for a in nm.iter() {
            let iso = format!("{}_ISO", a);
            if a.starts_with("SMALL") && !a.ends_with("_ISO") && has(&iso) {
                let c = contrast(&[a], &[iso.as_str()]);
                fam.push((format!("P3_{}_vs_ISO", a), cjson(&[a], &[iso.as_str()], c), c.3));
            }
        }
        if has("E0_AA") {
            let c = contrast(&["E0_AA"], &["E0"]);
            write!(o, ",\"AA\":{}}}", cjson(&["E0_AA"], &["E0"], c)).unwrap();
        }
        if has("PLANTED") {
            let c = contrast(&["PLANTED"], &["E0"]);
            pc_detected = c.1 > 1.0;
            write!(o, ",\"positive_control\":{},\"detected\":{}}}", cjson(&["PLANTED"], &["E0"], c), pc_detected).unwrap();
        }
    }
    // Holm
    let m = fam.len();
    let mut idx: Vec<usize> = (0..m).collect();
    idx.sort_by(|&i, &j| fam[i].2.partial_cmp(&fam[j].2).unwrap());
    let mut adj = vec![0.0; m];
    let mut run: f64 = 0.0;
    for (rank, &i) in idx.iter().enumerate() {
        run = run.max(((m - rank) as f64 * fam[i].2).min(1.0));
        adj[i] = run;
    }
    let fj: Vec<String> = fam.iter().zip(adj.iter()).map(|(f, a)| format!("\"{}\":{},\"p_holm\":{:.5}}}", f.0, f.1, a)).collect();
    write!(o, ",\"family\":{{{}}}", fj.join(",")).unwrap();
    // heterogeneity
    let het: Vec<&str> = nm.iter().cloned().filter(|a| *a != "PLANTED").collect();
    let mut allv: Vec<f64> = vec![];
    let mut sizes = vec![];
    for a in het.iter() {
        allv.extend(arms[*a].yn.iter());
        sizes.push(arms[*a].yn.len());
    }
    let bvar = |v: &[f64]| -> f64 {
        let mut ms = vec![];
        let mut i = 0;
        for &s in sizes.iter() {
            ms.push(mean(&v[i..i + s]));
            i += s;
        }
        let mm = mean(&ms);
        ms.iter().map(|x| (x - mm) * (x - mm)).sum::<f64>() / ms.len() as f64
    };
    let obs = bvar(&allv);
    let mut cnt = 0usize;
    let mut perm = allv.clone();
    for _ in 0..NPERM {
        for i in (1..perm.len()).rev() {
            let j = rng.below(i as u64 + 1) as usize;
            perm.swap(i, j);
        }
        if bvar(&perm) >= obs {
            cnt += 1;
        }
    }
    write!(o, ",\"heterogeneity\":{{\"arms\":{:?},\"between_arm_var\":{:.6e},\"p_perm\":{:.5}}}", het, obs, (cnt + 1) as f64 / (NPERM + 1) as f64).unwrap();
    if !mpe.is_empty() {
        let lo = mpe.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = mpe.iter().cloned().fold(0.0, f64::max);
        write!(o, ",\"muls_per_element_range\":[{:.4},{:.4}]", lo, hi).unwrap();
    }
    let margin = match cell.as_str() {
        "C40-S3" => Some(0.10),
        "C48-S3" => Some(0.15),
        _ => None,
    };
    if let Some(mg) = margin {
        let all_holm = adj.iter().all(|&a| a >= 0.05);
        let flat = p1ci.0 >= 1.0 - mg && p1ci.1 <= 1.0 + mg && all_holm && pc_detected && mism == 0;
        let effect = (p1ci.0 > 1.0 || p1ci.1 < 1.0) && mism == 0;
        let dec = if flat { "flat" } else if effect { "effect (needs replication)" } else { "inconclusive" };
        write!(o, ",\"margin\":{},\"decision\":\"{}\"", mg, dec).unwrap();
    }
    o.push('}');
    std::fs::write(&outp, &o).unwrap();
    println!("{}", o.len());
}
