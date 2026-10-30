//! **Can a learned component speed up a SAT-based PDP oracle at all?**
//!
//! Ceiling test for ML-aided SAT on the symmetrised binary Semaev `S₄`
//! point-decomposition problem (`m = 3`).  Protocol, predictions and
//! decision rules: `research/pdp_ml_sat_ceiling_20261027/PROTOCOL.md`.
//!
//! Three levers a model could occupy, each measured against its exact
//! ceiling with simple learners standing in for the model:
//!
//! * **L1 triage** — score the target (from `x_r` bits, or from solver
//!   telemetry after `k` conflicts) and only finish the solve if the
//!   score clears a threshold fitted on the training split.
//! * **L2 cutoff** — stop every solve at a fixed conflict budget.
//! * **L3 branching order** — static branch priorities, one of them
//!   learned from VSIDS activity on training refutations.
//!
//! The cost unit is solver conflicts; every label is cross-checked by
//! exhaustive sorted-triple search.
//!
//! ```bash
//! cargo run --release --example pdp_ml_sat_ceiling -- OUT_DIR
//! ```

use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::binary_semaev_s4::{elementary_symmetric_3, symmetrised_s4_eval};
use crypto_lib::cryptanalysis::sat::{SolveResult, SolverStats};
use crypto_lib::cryptanalysis::semaev_corpus::CORPUS;
use crypto_lib::cryptanalysis::semaev_sat::{encode_semaev_s4_with, S4Options, S4SatEncoding};
use serde_json::json;
use std::time::Instant;

const SEED: u64 = 20261027;
const TEL_K: [u64; 2] = [64, 512];
const N_TEL: usize = 11;

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[derive(Clone)]
struct Family {
    name: &'static str,
    id: u64,
    n: u32,
    l: u32,
    irr: IrreduciblePoly,
    b: F2mElement,
    count: usize,
    l3_targets: usize,
}

fn families() -> Vec<Family> {
    [("n15l5", 1u64, 3000usize, 60usize), ("n17l6", 2, 1200, 30)]
        .into_iter()
        .map(|(name, id, count, l3)| {
            let inst = CORPUS.iter().find(|c| c.name.starts_with(name)).unwrap();
            Family {
                name,
                id,
                n: inst.n,
                l: inst.l,
                irr: inst.irr(),
                b: inst.b(),
                count,
                l3_targets: l3,
            }
        })
        .collect()
}

fn elt(v: u64, n: u32) -> F2mElement {
    let bits: Vec<u32> = (0..n).filter(|j| (v >> j) & 1 == 1).collect();
    F2mElement::from_bit_positions(&bits, n)
}

fn encode(f: &Family, x_r: u64) -> S4SatEncoding {
    encode_semaev_s4_with(f.n, f.l, &f.irr, &f.b, &elt(x_r, f.n), S4Options::default())
}

/// Exhaustive sorted-triple decision, independent of the SAT path.
fn brute_force(f: &Family, base: &[F2mElement], x_r: u64) -> bool {
    let xr = elt(x_r, f.n);
    let s = base.len();
    for a in 0..s {
        for b in a..s {
            for c in b..s {
                let (e1, e2, e3) = elementary_symmetric_3(&base[a], &base[b], &base[c], &f.irr);
                if symmetrised_s4_eval(&e1, &e2, &e3, &xr, &f.irr).is_zero() {
                    return true;
                }
            }
        }
    }
    false
}

fn telemetry(s: &SolverStats, conflicts: u64, root_fixed: usize, decided: bool) -> [f64; N_TEL] {
    let c = conflicts.max(1) as f64;
    let learnt = s.learnt_clauses.max(1) as f64;
    [
        s.decisions as f64 / c,
        s.propagations as f64 / c,
        s.xor_propagations as f64 / c,
        s.xor_conflicts as f64 / c,
        s.restarts as f64,
        s.learnt_clauses as f64 / c,
        s.learnt_lits_kept as f64 / learnt,
        s.conflict_level_sum as f64 / c,
        s.max_level as f64,
        root_fixed as f64,
        decided as u8 as f64,
    ]
}

struct Row {
    x_r: u64,
    sat: bool,
    conflicts: u64,
    word_ops: u64,
    secs: f64,
    tel: Vec<[f64; N_TEL]>,
}

fn run_target(f: &Family, base: &[F2mElement], x_r: u64) -> Row {
    let mut enc = encode(f, x_r);
    let t = Instant::now();
    let mut tel = Vec::new();
    let mut result = SolveResult::Unknown;
    for &k in &TEL_K {
        if result == SolveResult::Unknown {
            enc.solver.conflict_budget = k;
            result = if enc.trivially_unsat {
                SolveResult::Unsat
            } else {
                enc.solver.solve()
            };
        }
        let decided = result != SolveResult::Unknown;
        tel.push(telemetry(
            &enc.solver.stats,
            enc.solver.conflicts(),
            enc.solver.root_fixed_count(),
            decided,
        ));
    }
    if result == SolveResult::Unknown {
        enc.solver.conflict_budget = u64::MAX;
        result = enc.solver.solve();
    }
    let secs = t.elapsed().as_secs_f64();
    let sat = match result {
        SolveResult::Sat => {
            let xs = enc.decode();
            let (e1, e2, e3) = elementary_symmetric_3(&xs[0], &xs[1], &xs[2], &f.irr);
            assert!(
                symmetrised_s4_eval(&e1, &e2, &e3, &elt(x_r, f.n), &f.irr).is_zero(),
                "{} x_r={x_r:#x}: SAT model is not a decomposition",
                f.name
            );
            true
        }
        SolveResult::Unsat => false,
        SolveResult::Unknown => unreachable!("unbounded solve returned Unknown"),
    };
    assert_eq!(
        sat,
        brute_force(f, base, x_r),
        "{} x_r={x_r:#x}: SAT label disagrees with exhaustive search",
        f.name
    );
    Row {
        x_r,
        sat,
        conflicts: enc.solver.conflicts(),
        word_ops: enc.solver.stats.word_ops,
        secs,
        tel,
    }
}

fn par_map<T: Sync, R: Send>(items: Vec<T>, f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    use rayon::prelude::*;
    items.par_iter().map(&f).collect()
}

// ── learner: ridge logistic regression by IRLS ──────────────────────

struct Model {
    mean: Vec<f64>,
    sd: Vec<f64>,
    w: Vec<f64>,
}

fn solve_linear(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let d = b.len();
    for i in 0..d {
        let p = (i..d)
            .max_by(|&r, &s| a[r][i].abs().total_cmp(&a[s][i].abs()))
            .unwrap();
        a.swap(i, p);
        b.swap(i, p);
        for r in i + 1..d {
            let m = a[r][i] / a[i][i];
            if m != 0.0 {
                for c in i..d {
                    a[r][c] -= m * a[i][c];
                }
                b[r] -= m * b[i];
            }
        }
    }
    let mut x = vec![0.0; d];
    for i in (0..d).rev() {
        let s: f64 = (i + 1..d).map(|c| a[i][c] * x[c]).sum();
        x[i] = (b[i] - s) / a[i][i];
    }
    x
}

fn fit(xs: &[Vec<f64>], ys: &[bool], lambda: f64) -> Model {
    let d = xs[0].len();
    let n = xs.len() as f64;
    let mut mean = vec![0.0; d];
    let mut sd = vec![0.0; d];
    for x in xs {
        for j in 0..d {
            mean[j] += x[j] / n;
        }
    }
    for x in xs {
        for j in 0..d {
            sd[j] += (x[j] - mean[j]).powi(2) / n;
        }
    }
    sd.iter_mut()
        .for_each(|s| *s = if *s > 1e-12 { s.sqrt() } else { 1.0 });
    let z: Vec<Vec<f64>> = xs
        .iter()
        .map(|x| {
            let mut v = vec![1.0];
            v.extend((0..d).map(|j| (x[j] - mean[j]) / sd[j]));
            v
        })
        .collect();
    let dd = d + 1;
    let mut w = vec![0.0; dd];
    for _ in 0..25 {
        let mut h = vec![vec![0.0; dd]; dd];
        let mut g = vec![0.0; dd];
        for (zi, &yi) in z.iter().zip(ys) {
            let eta: f64 = zi.iter().zip(&w).map(|(a, b)| a * b).sum();
            let p = 1.0 / (1.0 + (-eta).exp());
            let r = (yi as u8 as f64) - p;
            let s = (p * (1.0 - p)).max(1e-9);
            for a in 0..dd {
                g[a] += r * zi[a];
                for b in a..dd {
                    h[a][b] += s * zi[a] * zi[b];
                }
            }
        }
        for a in 0..dd {
            for b in 0..a {
                h[a][b] = h[b][a];
            }
            if a > 0 {
                h[a][a] += lambda;
                g[a] -= lambda * w[a];
            }
        }
        let step = solve_linear(h, g);
        let norm: f64 = step.iter().map(|s| s * s).sum::<f64>().sqrt();
        w.iter_mut().zip(&step).for_each(|(wi, si)| *wi += si);
        if norm < 1e-8 {
            break;
        }
    }
    Model { mean, sd, w }
}

impl Model {
    fn score(&self, x: &[f64]) -> f64 {
        let mut eta = self.w[0];
        for j in 0..x.len() {
            eta += self.w[j + 1] * (x[j] - self.mean[j]) / self.sd[j];
        }
        eta
    }
}

fn auc(scores: &[f64], ys: &[bool]) -> f64 {
    let mut idx: Vec<usize> = (0..scores.len()).collect();
    idx.sort_by(|&a, &b| scores[a].total_cmp(&scores[b]));
    // Average ranks over ties.
    let mut rank = vec![0.0; scores.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && scores[idx[j + 1]] == scores[idx[i]] {
            j += 1;
        }
        let r = (i + j) as f64 / 2.0 + 1.0;
        for t in i..=j {
            rank[idx[t]] = r;
        }
        i = j + 1;
    }
    let pos = ys.iter().filter(|&&y| y).count() as f64;
    let neg = ys.len() as f64 - pos;
    if pos == 0.0 || neg == 0.0 {
        return f64::NAN;
    }
    let sum: f64 = ys
        .iter()
        .zip(&rank)
        .filter(|(y, _)| **y)
        .map(|(_, r)| r)
        .sum();
    (sum - pos * (pos + 1.0) / 2.0) / (pos * neg)
}

fn bootstrap<F: Fn(&[usize]) -> f64>(n: usize, seed: u64, stat: F) -> (f64, f64) {
    let mut rng = SplitMix(seed);
    let mut v: Vec<f64> = (0..1000)
        .map(|_| {
            let sample: Vec<usize> = (0..n).map(|_| (rng.next() % n as u64) as usize).collect();
            stat(&sample)
        })
        .filter(|x| x.is_finite())
        .collect();
    v.sort_by(f64::total_cmp);
    (v[v.len() * 25 / 1000], v[v.len() * 975 / 1000])
}

/// Relations per conflict of a triage policy, relative to solving
/// everything to completion.  `k` telemetry conflicts are charged on
/// every target; targets decided within `k` cost what they cost.
fn triage_gain(rows: &[&Row], keep: &[bool], k: u64, idx: &[usize]) -> f64 {
    let (mut y, mut c, mut y0, mut c0) = (0.0, 0.0, 0.0, 0.0);
    for &i in idx {
        let r = rows[i];
        y0 += r.sat as u8 as f64;
        c0 += r.conflicts as f64;
        let early = r.conflicts <= k;
        if early || keep[i] {
            y += r.sat as u8 as f64;
            c += r.conflicts as f64;
        } else {
            c += k as f64;
        }
    }
    (y / c.max(1.0)) / (y0 / c0.max(1.0))
}

fn cutoff_gain(rows: &[&Row], cut: u64, idx: &[usize]) -> f64 {
    let (mut y, mut c, mut y0, mut c0) = (0.0, 0.0, 0.0, 0.0);
    for &i in idx {
        let r = rows[i];
        y0 += r.sat as u8 as f64;
        c0 += r.conflicts as f64;
        c += r.conflicts.min(cut) as f64;
        if r.sat && r.conflicts <= cut {
            y += 1.0;
        }
    }
    (y / c.max(1.0)) / (y0 / c0.max(1.0))
}

fn features(group: &str, r: &Row, n: u32) -> Vec<f64> {
    let bits: Vec<f64> = (0..n).map(|j| ((r.x_r >> j) & 1) as f64).collect();
    match group {
        "F-x" => bits,
        "F-xx" => {
            let mut v = bits.clone();
            for a in 0..n as usize {
                for b in a + 1..n as usize {
                    v.push(bits[a] * bits[b]);
                }
            }
            v
        }
        "F-tel64" => r.tel[0].to_vec(),
        "F-tel512" => r.tel[1].to_vec(),
        _ => unreachable!(),
    }
}

fn group_k(group: &str) -> u64 {
    match group {
        "F-tel64" => 64,
        "F-tel512" => 512,
        _ => 0,
    }
}

/// Fit on train, choose τ on train (≥ 10 train relations kept), report
/// test AUC and test gain with bootstrap CIs.
fn evaluate(
    rows: &[&Row],
    xs: &[Vec<f64>],
    ys: &[bool],
    train: &[usize],
    test: &[usize],
    k: u64,
    seed: u64,
) -> serde_json::Value {
    let tx: Vec<Vec<f64>> = train.iter().map(|&i| xs[i].clone()).collect();
    let ty: Vec<bool> = train.iter().map(|&i| ys[i]).collect();
    let model = fit(&tx, &ty, 1.0);
    let scores: Vec<f64> = xs.iter().map(|x| model.score(x)).collect();
    let test_scores: Vec<f64> = test.iter().map(|&i| scores[i]).collect();
    let test_y: Vec<bool> = test.iter().map(|&i| ys[i]).collect();
    let a = auc(&test_scores, &test_y);
    let auc_ci = bootstrap(test.len(), seed, |s| {
        let sc: Vec<f64> = s.iter().map(|&j| test_scores[j]).collect();
        let yy: Vec<bool> = s.iter().map(|&j| test_y[j]).collect();
        auc(&sc, &yy)
    });
    let mut train_sorted: Vec<f64> = train.iter().map(|&i| scores[i]).collect();
    train_sorted.sort_by(f64::total_cmp);
    let (mut best_tau, mut best_gain) = (f64::NEG_INFINITY, 1.0);
    for q in 1..20 {
        let tau = train_sorted[train_sorted.len() * q / 20];
        let keep: Vec<bool> = scores.iter().map(|&s| s >= tau).collect();
        let kept_sat = train
            .iter()
            .filter(|&&i| ys[i] && (keep[i] || rows[i].conflicts <= k))
            .count();
        if kept_sat < 10 {
            continue;
        }
        let g = triage_gain(rows, &keep, k, train);
        if g > best_gain {
            best_gain = g;
            best_tau = tau;
        }
    }
    let keep: Vec<bool> = scores.iter().map(|&s| s >= best_tau).collect();
    let g = triage_gain(rows, &keep, k, test);
    let g_ci = bootstrap(test.len(), seed ^ 1, |s| {
        let ix: Vec<usize> = s.iter().map(|&j| test[j]).collect();
        triage_gain(rows, &keep, k, &ix)
    });
    json!({
        "test_auc": a, "test_auc_ci95": [auc_ci.0, auc_ci.1],
        "train_gain": best_gain, "tau": if best_tau.is_finite() { json!(best_tau) } else { json!(null) },
        "test_gain": g, "test_gain_ci95": [g_ci.0, g_ci.1],
    })
}

fn l1_l2(f: &Family, rows: &[Row]) -> serde_json::Value {
    let refs: Vec<&Row> = rows.iter().collect();
    let ys: Vec<bool> = rows.iter().map(|r| r.sat).collect();
    let splits: [(&str, Vec<usize>, Vec<usize>); 2] = [
        (
            "even_odd",
            (0..rows.len()).filter(|i| i % 2 == 0).collect(),
            (0..rows.len()).filter(|i| i % 2 == 1).collect(),
        ),
        (
            "aa_mod4",
            (0..rows.len())
                .filter(|i| i % 4 == 0 || i % 4 == 3)
                .collect(),
            (0..rows.len())
                .filter(|i| i % 4 == 1 || i % 4 == 2)
                .collect(),
        ),
    ];
    let mut out = serde_json::Map::new();
    for (split, train, test) in &splits {
        let mut groups = serde_json::Map::new();
        for (gi, group) in ["F-x", "F-xx", "F-tel64", "F-tel512"]
            .into_iter()
            .enumerate()
        {
            let xs: Vec<Vec<f64>> = rows.iter().map(|r| features(group, r, f.n)).collect();
            let k = group_k(group);
            let seed = SEED ^ (f.id << 8) ^ gi as u64;
            let real = evaluate(&refs, &xs, &ys, train, test, k, seed);
            // Negative control: permuted training labels.
            let mut rng = SplitMix(seed ^ 0xabcd);
            let mut perm = ys.clone();
            for i in (1..perm.len()).rev() {
                perm.swap(i, (rng.next() % (i as u64 + 1)) as usize);
            }
            let mut ys_neg = ys.clone();
            for &i in train {
                ys_neg[i] = perm[i];
            }
            let neg = {
                let tx: Vec<Vec<f64>> = train.iter().map(|&i| xs[i].clone()).collect();
                let ty: Vec<bool> = train.iter().map(|&i| ys_neg[i]).collect();
                let m = fit(&tx, &ty, 1.0);
                let sc: Vec<f64> = test.iter().map(|&i| m.score(&xs[i])).collect();
                let yy: Vec<bool> = test.iter().map(|&i| ys[i]).collect();
                auc(&sc, &yy)
            };
            // Ceiling: exact labels after k conflicts.
            let keep: Vec<bool> = ys.clone();
            let ceiling = triage_gain(&refs, &keep, k, test);
            groups.insert(
                group.into(),
                json!({ "k": k, "dims": xs[0].len(), "model": real, "negative_control_auc": neg, "ceiling_gain_exact_labels": ceiling }),
            );
        }
        // Positive control: label flipped w.p. 0.25 appended to F-x.
        let mut rng = SplitMix(SEED ^ (f.id << 8) ^ 0x5050);
        let xs: Vec<Vec<f64>> = rows
            .iter()
            .map(|r| {
                let mut v = features("F-x", r, f.n);
                let noisy = r.sat ^ (rng.unit() < 0.25);
                v.push(noisy as u8 as f64);
                v
            })
            .collect();
        let pos = evaluate(&refs, &xs, &ys, train, test, 0, SEED ^ 0x77);
        groups.insert("positive_control".into(), pos);
        // L2: train-optimal fixed cutoff.
        let mut cuts: Vec<u64> = train.iter().map(|&i| rows[i].conflicts).collect();
        cuts.sort();
        cuts.dedup();
        let (mut best_c, mut best_g) = (u64::MAX, 1.0);
        for &c in &cuts {
            let kept = train
                .iter()
                .filter(|&&i| rows[i].sat && rows[i].conflicts <= c)
                .count();
            if kept < 10 {
                continue;
            }
            let g = cutoff_gain(&refs, c, train);
            if g > best_g {
                best_g = g;
                best_c = c;
            }
        }
        let g = cutoff_gain(&refs, best_c, test);
        let ci = bootstrap(test.len(), SEED ^ 0x99, |s| {
            let ix: Vec<usize> = s.iter().map(|&j| test[j]).collect();
            cutoff_gain(&refs, best_c, &ix)
        });
        groups.insert(
            "L2_cutoff".into(),
            json!({ "cutoff": if best_c == u64::MAX { json!(null) } else { json!(best_c) }, "train_gain": best_g, "test_gain": g, "test_gain_ci95": [ci.0, ci.1] }),
        );
        out.insert((*split).into(), serde_json::Value::Object(groups));
    }
    serde_json::Value::Object(out)
}

/// Train the telemetry model on one family, score another.
fn transfer(src: &[Row], dst: &[Row], group: &str, nsrc: u32, ndst: u32) -> serde_json::Value {
    let xs: Vec<Vec<f64>> = src.iter().map(|r| features(group, r, nsrc)).collect();
    let ys: Vec<bool> = src.iter().map(|r| r.sat).collect();
    let m = fit(&xs, &ys, 1.0);
    let sc: Vec<f64> = dst
        .iter()
        .map(|r| m.score(&features(group, r, ndst)))
        .collect();
    let yy: Vec<bool> = dst.iter().map(|r| r.sat).collect();
    let a = auc(&sc, &yy);
    let ci = bootstrap(dst.len(), SEED ^ 0x31, |s| {
        let a: Vec<f64> = s.iter().map(|&j| sc[j]).collect();
        let b: Vec<bool> = s.iter().map(|&j| yy[j]).collect();
        auc(&a, &b)
    });
    json!({ "group": group, "test_auc": a, "test_auc_ci95": [ci.0, ci.1] })
}

// ── L3: branching order ─────────────────────────────────────────────

/// Per-solve conflict cap for the L3 arms: `L3_CAP_MULT` × the family's
/// median default-order refutation (PROTOCOL.md amendment 1).
const L3_CAP_MULT: u64 = 10;

fn l3(f: &Family, rows: &[Row]) -> serde_json::Value {
    let l = f.l;
    let mut unsat_c: Vec<u64> = rows
        .iter()
        .filter(|r| !r.sat)
        .map(|r| r.conflicts)
        .collect();
    unsat_c.sort();
    let cap = L3_CAP_MULT * unsat_c[unsat_c.len() / 2];
    let n_x = 3 * l;
    let n_e = 6 * l - 3;
    // Learned order: mean normalised final VSIDS activity over training refutations.
    let train_unsat: Vec<u64> = rows
        .iter()
        .enumerate()
        .filter(|(i, r)| i % 2 == 0 && !r.sat)
        .map(|(_, r)| r.x_r)
        .take(4 * f.l3_targets)
        .collect();
    let acts: Vec<Vec<f64>> = par_map(train_unsat, |&x| {
        let mut enc = encode(f, x);
        enc.solver.solve();
        let a = &enc.solver.activities()[..(n_x + n_e) as usize];
        let mx = a.iter().cloned().fold(0.0, f64::max).max(1e-300);
        a.iter().map(|v| v / mx).collect()
    });
    let mut mean = vec![0.0; (n_x + n_e) as usize];
    for a in &acts {
        for (m, v) in mean.iter_mut().zip(a) {
            *m += v / acts.len() as f64;
        }
    }
    let mut order: Vec<u32> = (1..=n_x + n_e).collect();
    order.sort_by(|&a, &b| mean[(b - 1) as usize].total_cmp(&mean[(a - 1) as usize]));
    let learned: Vec<u32> = order[..(2 * l) as usize].to_vec();

    let arms: Vec<(&str, Vec<u32>)> = vec![
        ("a0_default", vec![]),
        ("a1_x_all", (1..=n_x).collect()),
        ("a2_x1x2", (1..=2 * l).collect()),
        ("a3_e_vars", (n_x + 1..=n_x + n_e).collect()),
        ("a4_learned_activity", learned.clone()),
    ];
    let test_unsat: Vec<u64> = rows
        .iter()
        .enumerate()
        .filter(|(i, r)| i % 2 == 1 && !r.sat)
        .map(|(_, r)| r.x_r)
        .take(f.l3_targets)
        .collect();
    let span = 1u64 << l;
    let triples = (span * (span + 1) * (span + 2) / 6) as f64;
    let pairs = (span * span) as f64;
    let mut out = serde_json::Map::new();
    let mut a0_sum = 0.0;
    for (name, prio) in &arms {
        let res: Vec<(u64, u64, bool)> = par_map(test_unsat.clone(), |&x| {
            let mut enc = encode(f, x);
            if !prio.is_empty() {
                enc.solver.set_branch_priority(prio);
            }
            enc.solver.conflict_budget = cap;
            let r = enc.solver.solve();
            assert_ne!(
                r,
                SolveResult::Sat,
                "{} x_r={x:#x}: arm {name} found SAT on an UNSAT target",
                f.name
            );
            (
                enc.solver.conflicts(),
                enc.solver.stats.word_ops,
                r == SolveResult::Unknown,
            )
        });
        let capped = res.iter().filter(|r| r.2).count();
        eprintln!(
            "{} L3 {name}: {} targets, {capped} capped",
            f.name,
            res.len()
        );
        let mut c: Vec<u64> = res.iter().map(|r| r.0).collect();
        c.sort();
        let sum: f64 = c.iter().map(|&v| v as f64).sum();
        let wsum: f64 = res.iter().map(|r| r.1 as f64).sum();
        if *name == "a0_default" {
            a0_sum = sum;
        }
        let nn = c.len() as f64;
        out.insert(
            (*name).into(),
            json!({
                "targets": c.len(), "capped_at": cap, "capped": capped,
                "sums_are_lower_bounds": capped > 0,
                "median_conflicts": c[c.len() / 2], "sum_conflicts": sum,
                "sum_word_ops": wsum, "ratio_to_a0": sum / a0_sum,
                "conflicts_per_sorted_triple": sum / nn / triples, "conflicts_per_pair": sum / nn / pairs,
            }),
        );
    }
    out.insert("learned_priority_vars".into(), json!(learned));
    out.insert("x_vars".into(), json!([1, n_x]));
    out.insert("e_vars".into(), json!([n_x + 1, n_x + n_e]));
    serde_json::Value::Object(out)
}

fn label_family(f: &Family) -> Vec<Row> {
    let mut rng = SplitMix(SEED ^ f.id);
    let mask = (1u64 << f.n) - 1;
    let targets: Vec<u64> = (0..f.count)
        .map(|_| loop {
            let v = rng.next() & mask;
            if v != 0 {
                break v;
            }
        })
        .collect();
    let base: Vec<F2mElement> = (0..1u64 << f.l).map(|v| elt(v, f.n)).collect();
    par_map(targets, |&x| run_target(f, &base, x))
}

/// Rows written by an earlier labelling run, whose labels were already
/// cross-checked; the target sequence is regenerated and must match.
fn load_rows(f: &Family, dir: &std::path::Path) -> Vec<Row> {
    let text = std::fs::read_to_string(dir.join(format!("rows_{}.jsonl", f.name))).unwrap();
    let mut rng = SplitMix(SEED ^ f.id);
    let mask = (1u64 << f.n) - 1;
    let rows: Vec<Row> = text
        .lines()
        .map(|line| {
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            let tel = |k: &str| -> [f64; N_TEL] {
                let a = v[k].as_array().unwrap();
                std::array::from_fn(|i| a[i].as_f64().unwrap())
            };
            let x_r = v["x_r"].as_u64().unwrap();
            let expect = loop {
                let t = rng.next() & mask;
                if t != 0 {
                    break t;
                }
            };
            assert_eq!(
                x_r, expect,
                "{}: saved rows do not match the seeded target sequence",
                f.name
            );
            Row {
                x_r,
                sat: v["sat"].as_bool().unwrap(),
                conflicts: v["conflicts"].as_u64().unwrap(),
                word_ops: v["word_ops"].as_u64().unwrap(),
                secs: v["secs"].as_f64().unwrap(),
                tel: vec![tel("tel64"), tel("tel512")],
            }
        })
        .collect();
    assert_eq!(rows.len(), f.count, "{}: row count", f.name);
    rows
}

fn write_json(path: std::path::PathBuf, v: &serde_json::Value) {
    std::fs::write(&path, serde_json::to_string_pretty(v).unwrap()).unwrap();
    eprintln!("wrote {}", path.display());
}

/// `pdp_ml_sat_ceiling OUT_DIR [--from-rows DIR]`.  Each stage's result is
/// written as soon as it exists: rows, then `summary_<family>.json` (L1/L2),
/// `l3_<family>.json`, and `transfer.json`.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_dir = std::path::PathBuf::from(
        args.get(1)
            .expect("usage: pdp_ml_sat_ceiling OUT_DIR [--from-rows DIR]"),
    );
    let from_rows = args
        .iter()
        .position(|a| a == "--from-rows")
        .map(|i| std::path::PathBuf::from(&args[i + 1]));
    assert!(
        !out_dir.exists(),
        "refusing to overwrite {}",
        out_dir.display()
    );
    std::fs::create_dir_all(&out_dir).unwrap();
    let mut all_rows = Vec::new();
    for f in families() {
        let t = Instant::now();
        let rows = match &from_rows {
            Some(dir) => load_rows(&f, dir),
            None => {
                let rows = label_family(&f);
                let mut jsonl = String::new();
                for r in &rows {
                    jsonl.push_str(&json!({"x_r": r.x_r, "sat": r.sat, "conflicts": r.conflicts, "word_ops": r.word_ops, "secs": r.secs, "tel64": r.tel[0].to_vec(), "tel512": r.tel[1].to_vec()}).to_string());
                    jsonl.push('\n');
                }
                std::fs::write(out_dir.join(format!("rows_{}.jsonl", f.name)), jsonl).unwrap();
                rows
            }
        };
        let label_secs = t.elapsed().as_secs_f64();
        let sats = rows.iter().filter(|r| r.sat).count();
        eprintln!(
            "{}: {} targets, {} SAT, {:.1}s",
            f.name,
            rows.len(),
            sats,
            label_secs
        );
        let sat_c: Vec<u64> = {
            let mut v: Vec<u64> = rows.iter().filter(|r| r.sat).map(|r| r.conflicts).collect();
            v.sort();
            v
        };
        let unsat_c: Vec<u64> = {
            let mut v: Vec<u64> = rows
                .iter()
                .filter(|r| !r.sat)
                .map(|r| r.conflicts)
                .collect();
            v.sort();
            v
        };
        let l12 = l1_l2(&f, &rows);
        write_json(
            out_dir.join(format!("summary_{}.json", f.name)),
            &json!({
                "n": f.n, "l": f.l, "targets": rows.len(), "sat": sats,
                "labels_cross_checked_by_exhaustive_search": rows.len(),
                "rows_source": from_rows.as_ref().map(|d| d.display().to_string()),
                "sat_conflicts_median": sat_c.get(sat_c.len() / 2), "unsat_conflicts_median": unsat_c.get(unsat_c.len() / 2),
                "sat_conflicts_quartiles": [sat_c.get(sat_c.len() / 4), sat_c.get(3 * sat_c.len() / 4)],
                "unsat_conflicts_quartiles": [unsat_c.get(unsat_c.len() / 4), unsat_c.get(3 * unsat_c.len() / 4)],
                "unsat_conflicts_min_max": [unsat_c.first(), unsat_c.last()],
                "label_phase_wall_secs": label_secs,
                "seed": SEED, "protocol": "research/pdp_ml_sat_ceiling_20261027/PROTOCOL.md",
                "L1_L2": l12,
            }),
        );
        all_rows.push((f, rows));
    }
    let (f0, r0) = &all_rows[0];
    let (f1, r1) = &all_rows[1];
    write_json(
        out_dir.join("transfer.json"),
        &json!({"size_transfer_n15_to_n17": [transfer(r0, r1, "F-tel64", f0.n, f1.n), transfer(r0, r1, "F-tel512", f0.n, f1.n)]}),
    );
    for (f, rows) in &all_rows {
        write_json(out_dir.join(format!("l3_{}.json", f.name)), &l3(f, rows));
    }
}
