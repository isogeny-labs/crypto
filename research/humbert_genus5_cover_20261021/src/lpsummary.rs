//! `humbert summarize-lp <study-dir>`: tables of the large-prime campaigns
//! (PROTOCOL.md v1 and PROTOCOL_v2.md) from their recorded JSON lines.
//! Paired ratios use the same (q, seed) instance. Duplicate (q, seed, tag)
//! records are an error, never silently overwritten.
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// One flat JSON object (the attack output plus `tag`, `order`): every
/// top-level value kept as its raw text.
pub struct Rec(BTreeMap<String, String>);

impl Rec {
    pub fn parse(line: &str) -> Result<Rec, String> {
        let b = line.trim().as_bytes();
        if b.first() != Some(&b'{') || b.last() != Some(&b'}') {
            return Err("not a JSON object".into());
        }
        let mut m = BTreeMap::new();
        let mut i = 1;
        let skip_ws = |i: &mut usize| {
            while *i < b.len() && (b[*i] as char).is_whitespace() {
                *i += 1;
            }
        };
        loop {
            skip_ws(&mut i);
            if b[i] == b'}' {
                break;
            }
            if b[i] != b'"' {
                return Err(format!("expected key at byte {i}"));
            }
            let ks = i + 1;
            i = ks;
            while b[i] != b'"' {
                i += 1;
            }
            let key = String::from_utf8_lossy(&b[ks..i]).into_owned();
            i += 1;
            skip_ws(&mut i);
            if b[i] != b':' {
                return Err(format!("expected ':' after {key}"));
            }
            i += 1;
            skip_ws(&mut i);
            let vs = i;
            let mut depth = 0i32;
            let mut in_str = false;
            while i < b.len() {
                let c = b[i];
                if in_str {
                    if c == b'"' {
                        in_str = false;
                    }
                } else if c == b'"' {
                    in_str = true;
                } else if c == b'[' || c == b'{' {
                    depth += 1;
                } else if c == b']' || (c == b'}' && depth > 0) {
                    depth -= 1;
                } else if depth == 0 && (c == b',' || c == b'}') {
                    break;
                }
                i += 1;
            }
            m.insert(key, String::from_utf8_lossy(&b[vs..i]).trim().to_string());
            if b[i] == b',' {
                i += 1;
            }
        }
        Ok(Rec(m))
    }
    pub fn s(&self, k: &str) -> String {
        self.0.get(k).map(|v| v.trim_matches('"').to_string()).unwrap_or_else(|| panic!("missing field {k}"))
    }
    pub fn f(&self, k: &str) -> f64 {
        self.s(k).parse().unwrap_or_else(|_| panic!("field {k} is not a number"))
    }
    pub fn u(&self, k: &str) -> u64 {
        self.f(k) as u64
    }
    pub fn b(&self, k: &str) -> bool {
        self.s(k) == "true"
    }
    fn gates(&self) -> bool {
        ["solved", "pushforward_8P", "T_P_nonzero", "iota_pairs_constant", "x_inf_eq_4c"].iter().all(|k| self.b(k))
    }
    /// expected-steps rho time of this run: sqrt(pi r / 2) x measured s/step
    fn rho_exp(&self) -> f64 {
        self.f("rho_expected_steps") * self.f("rho_seconds") / self.f("rho_steps")
    }
}

fn med(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}
fn lo(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::INFINITY, f64::min)
}
fn hi(v: &[f64]) -> f64 {
    v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}

type Inst = BTreeMap<(u64, u64), BTreeMap<String, Rec>>;

fn load(path: &std::path::Path) -> Result<Inst, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut inst: Inst = BTreeMap::new();
    for (n, line) in text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
        let r = Rec::parse(line).map_err(|e| format!("{}:{}: {e}", path.display(), n + 1))?;
        let key = (r.u("p"), r.u("seed"));
        let tag = r.s("tag");
        let slot = inst.entry(key).or_default();
        if slot.contains_key(&tag) {
            return Err(format!("{}:{}: duplicate record for q = {}, seed = {}, tag = {tag}", path.display(), n + 1, key.0, key.1));
        }
        slot.insert(tag, r);
    }
    Ok(inst)
}

fn complete<'a>(inst: &'a Inst, tags: &[&str]) -> Vec<((u64, u64), &'a BTreeMap<String, Rec>)> {
    inst.iter().filter(|(_, a)| tags.iter().all(|t| a.contains_key(*t))).map(|(k, a)| (*k, a)).collect()
}

fn write(dir: &std::path::Path, name: &str, body: &str) -> Result<(), String> {
    std::fs::write(dir.join(name), body).map_err(|e| format!("{name}: {e}"))
}

pub fn run(study: &str) -> Result<(), String> {
    let dir = std::path::Path::new(study);
    let res = dir.join("results");
    let mut printed = String::new();
    if res.join("campaign.jsonl").exists() {
        printed += &v1_section(&res)?;
    }
    if res.join("v2.jsonl").exists() {
        printed += &v2_section(&res)?;
    }
    if printed.is_empty() {
        return Err(format!("no campaign.jsonl or v2.jsonl in {}", res.display()));
    }
    print!("{printed}");
    Ok(())
}

fn v1_section(res: &std::path::Path) -> Result<String, String> {
    let v1 = load(&res.join("campaign.jsonl"))?;
    let tags1 = ["baseline", "aa", "lp50", "lp35"];
    let c1 = complete(&v1, &tags1);
    let n1: usize = v1.values().map(|a| a.len()).sum();
    let g1 = v1.values().flat_map(|a| a.values()).all(|r| r.gates());
    let mut md = format!("runs: {n1}; complete instances: {}; all correctness gates pass: {g1}\n\n", c1.len());
    md += "| q | seeds | t_ic baseline s | A/A ratio median [min, max] | lp:0.5 ratio median [min, max] | lp:0.5 faster | lp:0.35 ratio median [min, max] | lp:0.35 faster | IC/rho-exp base | IC/rho-exp lp:0.5 |\n";
    md += "|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    let mut csv1 = String::from("q,aa_med,aa_lo,aa_hi,lp50_med,lp50_lo,lp50_hi,lp50_faster,lp35_med,lp35_lo,lp35_hi,lp35_faster,n,gap_base,gap_lp50\n");
    let mut inst1 = String::from("q,seed,baseline_rounds,aa,lp50,lp35\n");
    let qs1: Vec<u64> = {
        let mut q: Vec<u64> = c1.iter().map(|(k, _)| k.0).collect();
        q.dedup();
        q
    };
    for &q in &qs1 {
        let rows: Vec<_> = c1.iter().filter(|(k, _)| k.0 == q).collect();
        let ratio = |t: &str| -> Vec<f64> { rows.iter().map(|(_, a)| a["baseline"].f("t_ic_total") / a[t].f("t_ic_total")).collect() };
        let (aa, l5, l3) = (ratio("aa"), ratio("lp50"), ratio("lp35"));
        let gap = |t: &str| -> Vec<f64> { rows.iter().map(|(_, a)| a[t].f("t_ic_total") / a["baseline"].rho_exp()).collect() };
        let (gb, gl) = (gap("baseline"), gap("lp50"));
        let tb: Vec<f64> = rows.iter().map(|(_, a)| a["baseline"].f("t_ic_total")).collect();
        let n = rows.len();
        let f5 = l5.iter().filter(|x| **x > 1.0).count();
        let f3 = l3.iter().filter(|x| **x > 1.0).count();
        let _ = writeln!(md, "| {q} | {n} | {:.2} | {:.3} [{:.3}, {:.3}] | {:.2} [{:.2}, {:.2}] | {f5}/{n} | {:.2} [{:.2}, {:.2}] | {f3}/{n} | {:.2} | {:.2} |",
            med(&tb), med(&aa), lo(&aa), hi(&aa), med(&l5), lo(&l5), hi(&l5), med(&l3), lo(&l3), hi(&l3), med(&gb), med(&gl));
        let _ = writeln!(csv1, "{q},{:.3},{:.3},{:.3},{:.2},{:.2},{:.2},{f5},{:.2},{:.2},{:.2},{f3},{n},{:.2},{:.2}",
            med(&aa), lo(&aa), hi(&aa), med(&l5), lo(&l5), hi(&l5), med(&l3), lo(&l3), hi(&l3), med(&gb), med(&gl));
        for (i, ((_, s), a)) in rows.iter().enumerate() {
            let _ = writeln!(inst1, "{q},{s},{},{:.4},{:.4},{:.4}", a["baseline"].u("la_rounds"), aa[i], l5[i], l3[i]);
        }
    }
    write(res, "summary.md", &md)?;
    write(res, "summary_v1.csv", &csv1)?;
    write(res, "instances_v1.csv", &inst1)?;
    Ok(md + "\n")
}

fn v2_section(res: &std::path::Path) -> Result<String, String> {
    let v2 = load(&res.join("v2.jsonl"))?;
    let tags2 = ["v1", "pre", "pre_aa", "lp50pre"];
    let c2 = complete(&v2, &tags2);
    let n2: usize = v2.values().map(|a| a.len()).sum();
    let g2 = v2.values().flat_map(|a| a.values()).all(|r| r.gates());
    let mut md2 = format!("runs: {n2}; complete instances: {}; all correctness gates pass: {g2}\n\n", c2.len());
    md2 += "| q | seed | v1 t_ic (rounds) | pre t_ic (rounds, ext.) | M1 v1/pre | A/A pre/pre_aa | lp50pre t_ic (rounds) | M2 pre/lp50pre |\n|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    let mut inst2 = String::from("q,seed,v1_rounds,m1,aa,m2,gap_v1,gap_pre,gap_lp50pre\n");
    let mut csv2 = String::from("q,m1_med,m1_lo,m1_hi,multi_faster,multi_n,aa_med,aa_lo,aa_hi,m2_med,m2_lo,m2_hi,lp_faster,n,gap_v1,gap_pre,gap_lp\n");
    let mut perq: BTreeMap<u64, Vec<(f64, f64, f64, u64, f64, f64, f64)>> = BTreeMap::new();
    for ((q, s), a) in &c2 {
        let t = |k: &str| a[k].f("t_ic_total");
        let (m1, aa, m2) = (t("v1") / t("pre"), t("pre") / t("pre_aa"), t("pre") / t("lp50pre"));
        let re = a["v1"].rho_exp();
        let rounds = a["v1"].u("la_rounds");
        let _ = writeln!(md2, "| {q} | {s} | {:.2} ({rounds}) | {:.2} ({}, {}) | {m1:.2} | {aa:.3} | {:.2} ({}) | {m2:.2} |",
            t("v1"), t("pre"), a["pre"].u("la_rounds"), a["pre"].u("precheck_extensions"), t("lp50pre"), a["lp50pre"].u("la_rounds"));
        let _ = writeln!(inst2, "{q},{s},{rounds},{m1:.4},{aa:.4},{m2:.4},{:.4},{:.4},{:.4}", t("v1") / re, t("pre") / re, t("lp50pre") / re);
        perq.entry(*q).or_default().push((m1, aa, m2, rounds, t("v1") / re, t("pre") / re, t("lp50pre") / re));
    }
    md2 += "\n| q | M1 median [min, max] | multi-round v1 instances: pre faster | A/A median [min, max] | M2 median [min, max] | lp50pre faster | IC/rho-exp v1 | pre | lp50pre |\n|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n";
    for (q, v) in &perq {
        let col = |f: fn(&(f64, f64, f64, u64, f64, f64, f64)) -> f64| -> Vec<f64> { v.iter().map(f).collect() };
        let (m1, aa, m2) = (col(|x| x.0), col(|x| x.1), col(|x| x.2));
        let multi: Vec<_> = v.iter().filter(|x| x.3 > 1).collect();
        let mf = multi.iter().filter(|x| x.0 > 1.0).count();
        let lf = m2.iter().filter(|x| **x > 1.0).count();
        let (g1, gp, gl) = (med(&col(|x| x.4)), med(&col(|x| x.5)), med(&col(|x| x.6)));
        let n = v.len();
        let _ = writeln!(md2, "| {q} | {:.2} [{:.2}, {:.2}] | {mf}/{} | {:.3} [{:.3}, {:.3}] | {:.2} [{:.2}, {:.2}] | {lf}/{n} | {g1:.2} | {gp:.2} | {gl:.2} |",
            med(&m1), lo(&m1), hi(&m1), multi.len(), med(&aa), lo(&aa), hi(&aa), med(&m2), lo(&m2), hi(&m2));
        let _ = writeln!(csv2, "{q},{:.2},{:.2},{:.2},{mf},{},{:.3},{:.3},{:.3},{:.2},{:.2},{:.2},{lf},{n},{g1:.2},{gp:.2},{gl:.2}",
            med(&m1), lo(&m1), hi(&m1), multi.len(), med(&aa), lo(&aa), hi(&aa), med(&m2), lo(&m2), hi(&m2));
    }
    write(res, "summary_v2.md", &md2)?;
    write(res, "summary_v2.csv", &csv2)?;
    write(res, "instances_v2.csv", &inst2)?;
    Ok(md2)
}
