//! Blind-spot item 16: analysis of the size-gated equivariant oracle.
//!
//! Reads the JSONL rows written by `blindspots_16_size_gated_oracle --mode
//! oracle` (std only, no dependencies) and applies the frozen rule of
//! `research/blindspots_20261018/16-size-gated-equivariant/PROTOCOL.md`:
//!
//! 1. From the **training** rows, tune the gate threshold `N0` (the barrel
//!    unknown count `sym_vars` from which the equivariant arm is used).
//! 2. On the **held-out** rows, evaluate the gated arm (c) against the
//!    incumbent (a): per size and pooled, geometric means of `a/c` with 95 %
//!    percentile bootstrap CIs (10,000 resamples, targets resampled within
//!    size, fixed seed), and the verdict.
//!
//! Arm costs per target (thread CPU seconds, system build included in all):
//! a = `i_cpu`, b = `c_cpu` (equivariant always), c = `c_cpu` when
//! `sym_vars >= N0` and otherwise the incumbent's own measurement `i_cpu`
//! (below the threshold the dispatcher runs the incumbent's code, so a/c is 1
//! by construction and the one-integer comparison is charged 0); the A/A
//! noise floor a/a2 (`a2 = i2_cpu`) is reported next to it.
//! The counted unit is F2 word XORs; GF(2^d) products of the equivariant
//! pass are priced at 0, 0.1 and 1 word-XOR equivalents.
//!
//! ```bash
//! blindspots_16_gate --train raw/train-a.jsonl,raw/train-b.jsonl --held raw/held-a.jsonl
//! ```

use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct Row {
    n: u32,
    t: u64,
    sym_vars: u32,
    gate: bool,
    f_gate: bool,
    decomposes: bool,
    i_cpu: f64,
    i2_cpu: f64,
    c_cpu: f64,
    f_cpu: f64,
    i_ops: f64,
    c_ops: f64,
    c_triv: f64,
    c_spec: f64,
    c_gf: f64,
    i_splits: f64,
    c_splits: f64,
    maxrss_kb: f64,
}

fn field<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let at = line.find(&pat)? + pat.len();
    let rest = &line[at..];
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    Some(rest[..end].trim().trim_matches('"'))
}
fn num(line: &str, key: &str) -> f64 {
    field(line, key)
        .unwrap_or_else(|| panic!("missing {key} in {line}"))
        .parse()
        .unwrap_or_else(|_| panic!("bad number for {key}"))
}
fn flag(line: &str, key: &str) -> bool {
    field(line, key).unwrap_or_else(|| panic!("missing {key}")) == "true"
}

fn load(files: &str) -> Vec<Row> {
    let mut rows = Vec::new();
    for f in files.split(',').filter(|s| !s.is_empty()) {
        let text = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("{f}: {e}"));
        for line in text.lines().filter(|l| l.starts_with('{')) {
            if field(line, "mode") != Some("oracle") {
                continue;
            }
            rows.push(Row {
                n: num(line, "n") as u32,
                t: num(line, "target") as u64,
                sym_vars: num(line, "sym_vars") as u32,
                gate: flag(line, "gate"),
                f_gate: flag(line, "f_gate"),
                decomposes: flag(line, "decomposes"),
                i_cpu: num(line, "i_cpu"),
                i2_cpu: num(line, "i2_cpu"),
                c_cpu: num(line, "c_cpu"),
                f_cpu: num(line, "f_cpu"),
                i_ops: num(line, "i_word_ops"),
                c_ops: num(line, "c_word_ops"),
                c_triv: num(line, "c_triv_word_ops"),
                c_spec: num(line, "c_spec_word_ops"),
                c_gf: num(line, "c_gf_muls"),
                i_splits: num(line, "i_splits"),
                c_splits: num(line, "c_splits"),
                maxrss_kb: num(line, "maxrss_kb"),
            });
        }
    }
    rows
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

const RESAMPLES: usize = 10_000;

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

/// Geometric mean of exp(logs) with a percentile bootstrap CI; `strata` are
/// disjoint groups of log-ratios resampled within themselves and pooled with
/// equal weight per target.
fn gm_ci(strata: &[Vec<f64>], seed: u64) -> (f64, f64, f64) {
    let all: Vec<f64> = strata.iter().flatten().copied().collect();
    let point = mean(&all).exp();
    let mut rng = Rng(seed);
    let mut est = Vec::with_capacity(RESAMPLES);
    for _ in 0..RESAMPLES {
        let (mut s, mut c) = (0.0, 0usize);
        for st in strata {
            for _ in 0..st.len() {
                s += st[rng.below(st.len())];
                c += 1;
            }
        }
        est.push((s / c as f64).exp());
    }
    est.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (
        point,
        est[(RESAMPLES as f64 * 0.025) as usize],
        est[(RESAMPLES as f64 * 0.975) as usize - 1],
    )
}

/// Ratio of sums (cost-weighted) with a stratified bootstrap CI.
fn sum_ratio_ci(strata: &[Vec<(f64, f64)>], seed: u64) -> (f64, f64, f64) {
    let ratio = |pairs: &mut dyn Iterator<Item = (f64, f64)>| {
        let (mut a, mut b) = (0.0, 0.0);
        for (x, y) in pairs {
            a += x;
            b += y;
        }
        a / b
    };
    let point = ratio(&mut strata.iter().flatten().copied());
    let mut rng = Rng(seed);
    let mut est = Vec::with_capacity(RESAMPLES);
    for _ in 0..RESAMPLES {
        let mut draw = Vec::new();
        for st in strata {
            for _ in 0..st.len() {
                draw.push(st[rng.below(st.len())]);
            }
        }
        est.push(ratio(&mut draw.into_iter()));
    }
    est.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (
        point,
        est[(RESAMPLES as f64 * 0.025) as usize],
        est[(RESAMPLES as f64 * 0.975) as usize - 1],
    )
}

fn by_size(rows: &[Row]) -> BTreeMap<u32, Vec<&Row>> {
    let mut m: BTreeMap<u32, Vec<&Row>> = BTreeMap::new();
    for r in rows {
        m.entry(r.n).or_default().push(r);
    }
    m
}

fn logs(rows: &[&Row], f: impl Fn(&Row) -> f64) -> Vec<f64> {
    rows.iter().map(|r| f(r).ln()).collect()
}

fn ci_json(label: &str, n: u32, k: usize, c: (f64, f64, f64)) -> String {
    format!(
        r#"{{"kind":"ratio","label":"{label}","n":{n},"targets":{k},"gm":{:.4},"lo":{:.4},"hi":{:.4}}}"#,
        c.0, c.1, c.2
    )
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let (mut train_files, mut held_files) = (String::new(), String::new());
    let mut forced_n0: Option<u32> = None;
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--train" => {
                i += 1;
                train_files = argv[i].clone();
            }
            "--held" => {
                i += 1;
                held_files = argv[i].clone();
            }
            // Only for the instrument's self-check: never used for a verdict.
            "--force-n0" => {
                i += 1;
                forced_n0 = Some(argv[i].parse().unwrap());
            }
            other => panic!("unknown argument {other}"),
        }
        i += 1;
    }

    // ── 1. Training: tune N0 ───────────────────────────────────────────
    let train = load(&train_files);
    let tr_gate_fail = train.iter().filter(|r| !r.gate).count();
    let tr_by = by_size(&train);
    let mut sizes: Vec<(u32, u32)> = tr_by.iter().map(|(n, v)| (*n, v[0].sym_vars)).collect();
    sizes.sort_by_key(|x| x.1);
    println!(
        r#"{{"kind":"training","rows":{},"gate_failures":{tr_gate_fail}}}"#,
        train.len()
    );
    let mut gain: BTreeMap<u32, f64> = BTreeMap::new(); // sym_vars -> mean ln(I/C)
    let mut tr_lo_hi: BTreeMap<u32, (f64, f64)> = BTreeMap::new();
    for (n, rows) in &tr_by {
        let good: Vec<&Row> = rows.iter().copied().filter(|r| r.gate).collect();
        let l = logs(&good, |r| r.i_cpu / r.c_cpu);
        let c = gm_ci(&[l.clone()], 0xB16_0001 + *n as u64);
        let aa = gm_ci(&[logs(&good, |r| r.i_cpu / r.i2_cpu)], 0xB16_0002 + *n as u64);
        let fc = gm_ci(&[logs(&good, |r| r.f_cpu / r.c_cpu)], 0xB16_0003 + *n as u64);
        println!("{}", ci_json("train I/C", *n, good.len(), c));
        println!("{}", ci_json("train I/I2 (A/A)", *n, good.len(), aa));
        println!("{}", ci_json("train F/C", *n, good.len(), fc));
        gain.insert(rows[0].sym_vars, mean(&l));
        tr_lo_hi.insert(*n, (c.1, c.2));
    }
    // Candidates: each observed sym_vars, and infinity (never use C).
    let mut best: (Option<u32>, f64) = (None, 0.0);
    let mut cand: Vec<Option<u32>> = gain.keys().map(|k| Some(*k)).collect();
    cand.push(None);
    for t in &cand {
        let j: f64 = gain
            .iter()
            .filter(|(sv, _)| t.map_or(false, |t| **sv >= t))
            .map(|(_, g)| *g)
            .sum();
        println!(
            r#"{{"kind":"tuning","threshold":{},"objective_sum_mean_ln_I_over_C":{j:.4}}}"#,
            t.map_or("null".to_string(), |t| t.to_string())
        );
        // Strictly greater, scanning thresholds in increasing order: ties go
        // to the smallest threshold; "never" (None) is last and wins only if
        // it is strictly better, which it never is at objective 0 vs >= 0.
        if j > best.1 + 1e-12 {
            best = (*t, j);
        }
    }
    let n0 = forced_n0.or(best.0);
    println!(
        r#"{{"kind":"n0","n0":{},"forced":{}}}"#,
        n0.map_or("null".to_string(), |v| v.to_string()),
        forced_n0.is_some()
    );
    // Training positive controls: the smallest size shows the known loss
    // (I/C CI upper < 1) and the largest the known win (CI lower > 1).
    let smallest = tr_by.keys().next().copied();
    let largest = tr_by.keys().next_back().copied();
    let ctl_loss = smallest.map_or(false, |n| tr_lo_hi[&n].1 < 1.0);
    let ctl_win = largest.map_or(false, |n| tr_lo_hi[&n].0 > 1.0);
    println!(r#"{{"kind":"training_controls","loss_seen_at_smallest":{ctl_loss},"win_seen_at_largest":{ctl_win}}}"#);

    if held_files.is_empty() {
        return;
    }

    // ── 2. Held-out evaluation ─────────────────────────────────────────
    let held = load(&held_files);
    let h_gate_fail = held.iter().filter(|r| !r.gate).count();
    let h_f_fail = held.iter().filter(|r| !r.f_gate).count();
    let max_rss = held.iter().map(|r| r.maxrss_kb).fold(0.0, f64::max);
    println!(
        r#"{{"kind":"heldout","rows":{},"gate_failures":{h_gate_fail},"f_gate_failures":{h_f_fail},"max_rss_kb":{max_rss:.0}}}"#,
        held.len()
    );
    let h_by = by_size(&held);
    let on = |r: &Row| n0.map_or(false, |t| r.sym_vars >= t);
    let c_cpu = |r: &Row| if on(r) { r.c_cpu } else { r.i_cpu };
    let c_ops = |r: &Row, w: f64| {
        if on(r) {
            r.c_ops + r.c_triv + r.c_spec + w * r.c_gf
        } else {
            r.i_ops
        }
    };
    let mut strata_ac: Vec<Vec<f64>> = Vec::new();
    let mut strata_sum: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut per_size_regress = false;
    let mut per_size_all_below = true;
    let mut per_size_any_above = false;
    for (n, rows) in &h_by {
        let good: Vec<&Row> = rows.iter().copied().filter(|r| r.gate).collect();
        let used = if on(rows[0]) { "C" } else { "I" };
        println!(
            r#"{{"kind":"size","n":{n},"sym_vars":{},"targets":{},"decomposing":{},"arm_used":"{used}","median_i_splits":{:.0},"median_c_splits":{:.0}}}"#,
            rows[0].sym_vars,
            good.len(),
            good.iter().filter(|r| r.decomposes).count(),
            median(good.iter().map(|r| r.i_splits).collect()),
            median(good.iter().map(|r| r.c_splits).collect()),
        );
        let l_ac = logs(&good, |r| r.i_cpu / c_cpu(r));
        let ac = gm_ci(&[l_ac.clone()], 0xB16_1000 + *n as u64);
        println!("{}", ci_json("held a/c (gated vs incumbent)", *n, good.len(), ac));
        let ab = gm_ci(&[logs(&good, |r| r.i_cpu / r.c_cpu)], 0xB16_1100 + *n as u64);
        println!("{}", ci_json("held a/b (always-equivariant vs incumbent)", *n, good.len(), ab));
        let aa = gm_ci(&[logs(&good, |r| r.i_cpu / r.i2_cpu)], 0xB16_1200 + *n as u64);
        println!("{}", ci_json("held a/a2 (A/A)", *n, good.len(), aa));
        let fc = gm_ci(&[logs(&good, |r| r.f_cpu / r.c_cpu)], 0xB16_1300 + *n as u64);
        println!("{}", ci_json("held F/C (attribution, not in the rule)", *n, good.len(), fc));
        for w in [0.0, 0.1, 1.0] {
            let o = gm_ci(
                &[logs(&good, |r| r.i_ops / c_ops(r, w))],
                0xB16_1400 + *n as u64,
            );
            println!("{}", ci_json(&format!("held counted a/c, GF product = {w} XOR"), *n, good.len(), o));
        }
        if ac.2 < 1.0 {
            per_size_regress = true;
        }
        if ac.1 > 1.0 {
            per_size_any_above = true;
            per_size_all_below = false;
        } else if ac.2 >= 1.0 {
            per_size_all_below = false;
        }
        strata_ac.push(l_ac);
        strata_sum.push(good.iter().map(|r| (r.i_cpu, c_cpu(r))).collect());
    }
    let n_total: usize = strata_ac.iter().map(|s| s.len()).sum();
    let pooled = gm_ci(&strata_ac, 0xB16_2000);
    println!("{}", ci_json("held a/c POOLED", 0, n_total, pooled));
    let pooled_sum = sum_ratio_ci(&strata_sum, 0xB16_2100);
    println!("{}", ci_json("held a/c POOLED, ratio of summed CPU", 0, n_total, pooled_sum));
    let mut band_lo = Vec::new();
    for w in [0.0, 0.1, 1.0] {
        let st: Vec<Vec<f64>> = h_by
            .values()
            .map(|rows| {
                let good: Vec<&Row> = rows.iter().copied().filter(|r| r.gate).collect();
                logs(&good, |r| r.i_ops / c_ops(r, w))
            })
            .collect();
        let o = gm_ci(&st, 0xB16_2200);
        band_lo.push(o.1);
        println!("{}", ci_json(&format!("held counted a/c POOLED, GF product = {w} XOR"), 0, n_total, o));
    }

    // ── Verdict by the frozen rule ─────────────────────────────────────
    let gates_ok = h_gate_fail == 0 && tr_gate_fail == 0;
    let controls_ok = ctl_loss && ctl_win;
    let band_robust = band_lo.iter().all(|lo| *lo > 1.0);
    let verdict = if !gates_ok {
        "INCONCLUSIVE (a correctness gate failed)"
    } else if !controls_ok {
        "INCONCLUSIVE (a training positive control failed)"
    } else if pooled.1 > 1.0 && !per_size_regress && !band_robust {
        "INCONCLUSIVE (CPU time and the counted unit disagree)"
    } else if pooled.1 > 1.0 && !per_size_regress {
        "ADVANCES"
    } else if pooled.2 < 1.0 && per_size_all_below {
        "REGRESSES"
    } else if pooled.1 <= 1.0 && pooled.2 >= 1.0 && !per_size_regress && !per_size_any_above {
        "MATCHES"
    } else {
        "TRADES"
    };
    println!(
        r#"{{"kind":"verdict","level":"constant","verdict":"{verdict}","scope":"barrel gauge-fixed oracle, n in {{5,7,11,13}}; not the production oracle at n in {{41,47,53,61}} (BLOCKED)","n0":{},"pooled_gm":{:.4},"pooled_lo":{:.4},"pooled_hi":{:.4},"any_size_regresses":{per_size_regress},"counted_unit_ci_lower_above_1_at_all_prices":{band_robust}}}"#,
        n0.map_or("null".to_string(), |v| v.to_string()),
        pooled.0,
        pooled.1,
        pooled.2,
    );
}

fn median(mut v: Vec<f64>) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = v.len() / 2;
    if v.len() % 2 == 1 {
        v[m]
    } else {
        0.5 * (v[m - 1] + v[m])
    }
}
