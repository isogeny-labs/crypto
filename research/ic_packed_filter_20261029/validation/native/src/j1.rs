//! J1 (port of j1_decision.py and xcheck_inpod.py): the decision statistic of
//! PROTOCOL section 5 from the wall session's records.jsonl.
//!
//! Pairs are keyed by (curve slug, target index, round), measured rounds only;
//! a pair is admitted when both runs are at L2 or L3 and verified.
//! ratio = ns_per_gae(B) / ns_per_gae(A) with a delta-method SE of ln ratio,
//! t with n-1 dof; corrected = ratio(dgb->dgbf) / ratio(dgb->dgb-aa), SEs
//! combined as independent, Welch-Satterthwaite dof. Descriptive: per-curve
//! intervals, target-clustered bootstrap, per-round estimates, label-swap null.

use crate::py::{self, int, int_to_f64, Num, Obj, PyRandom};
use crate::v::{self, Arms, Corrected, Key, OMap, Pair, Ratio};
use serde_json::Value;
use std::path::Path;

fn admitted(r: &Value) -> bool {
    matches!(r["isolation"]["level"].as_str(), Some("L2") | Some("L3"))
        && r["outcome"]["status"].as_str() == Some("verified")
}

fn gae(r: &Value) -> f64 {
    match Num::of(&r["cost"]["total_gae"]) {
        Num::Float(x) => x,
        Num::Int(_) => v::stop("total_gae is an int; the float-sum model assumes floats"),
    }
}

fn wall(r: &Value) -> i128 {
    int(&r["time"]["solve_wall_ns"])
}

pub fn pairs_for(arms: &Arms, a: &str, b: &str, filt: Option<&dyn Fn(&Key) -> bool>) -> (Vec<Pair>, usize, Vec<Key>) {
    let (ma, mb) = (&arms[&a.to_string()], &arms[&b.to_string()]);
    if ma.key_set() != mb.key_set() {
        v::stop(format!("key sets differ between {a} and {b}"));
    }
    let (mut out, mut below, mut keys) = (Vec::new(), 0usize, Vec::new());
    for k in ma.key_set() {
        let (ra, rb) = (ma[&k], mb[&k]);
        if let Some(f) = filt {
            if !f(&k) {
                continue;
            }
        }
        if !(admitted(ra) && admitted(rb)) {
            below += 1;
            continue;
        }
        out.push((wall(ra), gae(ra), wall(rb), gae(rb)));
        keys.push(k);
    }
    (out, below, keys)
}

pub fn summarize(pairs: &[Pair], below: usize) -> Ratio {
    let mut r = v::ratio_of_sums(pairs);
    r.pairs_below_level = below;
    r.ci95_log = v::ci_log(r.ratio, r.se_log, r.dof);
    r.ci95_linear = v::ci_linear(r.ratio, r.se_log, r.dof);
    r
}

pub fn decision(arms: &Arms, filt: Option<&dyn Fn(&Key) -> bool>) -> (Ratio, Ratio, Corrected) {
    let (d, bd, _) = pairs_for(arms, "dgb", "dgbf", filt);
    let (a, ba, _) = pairs_for(arms, "dgb", "dgb-aa", filt);
    let (rd, ra) = (summarize(&d, bd), summarize(&a, ba));
    let c = v::corrected(&rd, &ra);
    (rd, ra, c)
}

type Cluster = [Num; 6];

fn col_sum(sample: &[&Cluster], i: usize) -> Num {
    let xs: Vec<Num> = sample.iter().map(|c| c[i]).collect();
    py::sum_py(&xs)
}

fn stat(sample: &[&Cluster]) -> f64 {
    let w: Vec<Num> = (0..6).map(|i| col_sum(sample, i)).collect();
    w[4].div(w[5]) / w[2].div(w[3]) // (dgbf ns/GAE) / (dgb-aa ns/GAE)
}

fn stat_raw(sample: &[&Cluster]) -> f64 {
    let w: Vec<Num> = (0..6).map(|i| col_sum(sample, i)).collect();
    w[4].div(w[5]) / w[0].div(w[1])
}

pub fn run(b: u32, out: &Path) {
    let recs = v::load_wall(b);
    let arms = v::by_arm(&recs, true);
    let mut res = Obj::new();
    res.set("size", b as u64);
    let mut am = Obj::new();
    for (k, x) in arms.iter() {
        am.set(k.clone(), x.len() as u64);
    }
    res.set("arms", am.val());
    let (rd, ra, c) = decision(&arms, None);
    let mut dec = Obj::new();
    dec.set("dgb->dgbf", rd.val(true)).set("aa dgb->dgb-aa", ra.val(true)).set("corrected", c.val());
    res.set("decision", dec.val());
    // condition 9: direct dgb-aa -> dgbf
    let (p9, b9, _) = pairs_for(&arms, "dgb-aa", "dgbf", None);
    let r9 = summarize(&p9, b9);
    let mut c9 = Obj::new();
    c9.set("dgb-aa->dgbf", r9.val(true))
        .set("rel_diff_vs_corrected", py::fval((r9.ratio - c.ratio).abs() / c.ratio))
        .set("sum_wall_ratio_dgbf_over_dgbaa", py::fval(py::idiv(r9.sum_wall_b, r9.sum_wall_a)))
        .set("gae_sums_equal", r9.sum_gae_a == r9.sum_gae_b);
    res.set("cond9", c9.val());
    for (a_, b_) in [("dgb", "dgbf-edge"), ("dgbf-edge", "dgbf"), ("dgbf-edge", "dgb-aa")] {
        let (pp, bb, _) = pairs_for(&arms, a_, b_, None);
        res.set(format!("{a_}->{b_}"), summarize(&pp, bb).val(true));
    }
    // per curve
    let dgb = &arms[&"dgb".to_string()];
    let curves: Vec<String> = dgb.keys().map(|k| k.0.clone()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
    let mut pc = Obj::new();
    for cv in &curves {
        let f = |k: &Key| &k.0 == cv;
        let (rdc, rac, cc) = decision(&arms, Some(&f));
        let (pe, be, _) = pairs_for(&arms, "dgb", "dgbf-edge", Some(&f));
        let mut o = Obj::new();
        o.set("ratio", py::fval(rdc.ratio))
            .set("ci95_log", v::pair_val(rdc.ci95_log))
            .set("aa_ratio", py::fval(rac.ratio))
            .set("aa_ci95_log", v::pair_val(rac.ci95_log))
            .set("corrected", py::fval(cc.ratio))
            .set("corrected_ci95_log", v::pair_val(cc.ci95_log))
            .set("corrected_dof", py::fval(cc.dof))
            .set("pairs", rdc.n as u64)
            .set("position_control", py::fval(summarize(&pe, be).ratio));
        pc.set(cv.clone(), o.val());
    }
    res.set("per_curve", pc.val());
    // per round
    let rounds: std::collections::BTreeSet<i64> = dgb.keys().map(|k| k.2).collect();
    let mut prc = Obj::new();
    for rd_ in rounds {
        let f = |k: &Key| k.2 == rd_;
        let (_, _, cr) = decision(&arms, Some(&f));
        prc.set(rd_.to_string(), py::fval(cr.ratio));
    }
    res.set("per_round_corrected", prc.val());
    // target-clustered bootstrap of the corrected ratio (cluster = curve, target)
    let mut clusters: OMap<(String, i64), Cluster> = OMap::new();
    for k in dgb.keys() {
        let x = clusters.entry_or((k.0.clone(), k.1), || [Num::Int(0); 6]);
        for (i, a) in ["dgb", "dgb-aa", "dgbf"].iter().enumerate() {
            let r = arms[&a.to_string()][k];
            x[2 * i] = x[2 * i].add(Num::of(&r["time"]["solve_wall_ns"]));
            x[2 * i + 1] = x[2 * i + 1].add(Num::of(&r["cost"]["total_gae"]));
        }
    }
    let cl: Vec<Cluster> = clusters.values().cloned().collect();
    let clr: Vec<&Cluster> = cl.iter().collect();
    let ci_c = v::cluster_bootstrap(&cl, stat, 4000, 654733);
    let ci_r = v::cluster_bootstrap(&cl, stat_raw, 4000, 654733);
    let mut tb = Obj::new();
    tb.set("clusters", cl.len() as u64)
        .set("B", 4000u64)
        .set("seed", 654733u64)
        .set("corrected_point", py::fval(stat(&clr)))
        .set("corrected_ci95_pct", v::pair_val(ci_c))
        .set("ratio_point", py::fval(stat_raw(&clr)))
        .set("ratio_ci95_pct", v::pair_val(ci_r));
    res.set("target_clustered_bootstrap", tb.val());
    let mut tbc = Obj::new();
    for cv in &curves {
        let sub: Vec<Cluster> = clusters.iter().filter(|(t, _)| &t.0 == cv).map(|(_, c)| *c).collect();
        let subr: Vec<&Cluster> = sub.iter().collect();
        let ci_cc = v::cluster_bootstrap(&sub, stat, 4000, 654733);
        let mut o = Obj::new();
        o.set("clusters", sub.len() as u64)
            .set("corrected_point", py::fval(stat(&subr)))
            .set("corrected_ci95_pct", v::pair_val(ci_cc));
        tbc.set(cv.clone(), o.val());
    }
    res.set("target_clustered_bootstrap_per_curve", tbc.val());
    // label-swap null on the matched-position pair (dgb-aa, dgbf)
    let mut rng = PyRandom::new(20261030);
    let mut null = Obj::new();
    for n_sub in [72usize, 144, 288] {
        let sub: &[Pair] = if n_sub < p9.len() { &p9[..n_sub] } else { &p9 };
        let mut vals = Vec::with_capacity(4000);
        for _ in 0..4000 {
            let (mut sa, mut sb, mut ga, mut gb) = (0i128, 0i128, 0.0f64, 0.0f64);
            for &(wa, g_a, wb, g_b) in sub {
                let (wa, g_a, wb, g_b) = if rng.random() < 0.5 { (wb, g_b, wa, g_a) } else { (wa, g_a, wb, g_b) };
                sa += wa;
                ga += g_a;
                sb += wb;
                gb += g_b;
            }
            vals.push((int_to_f64(sb) / gb) / (int_to_f64(sa) / ga));
        }
        let obs = v::ratio_of_sums(sub).ratio;
        let le = vals.iter().filter(|x| **x <= obs).count();
        let mut o = Obj::new();
        o.set("null_median", py::fval(v::percentile_f(&vals, 0.5)))
            .set("null_ci95", v::pair_val([v::percentile_f(&vals, 0.025), v::percentile_f(&vals, 0.975)]))
            .set("observed", py::fval(obs))
            .set("null_fraction_le_observed", py::fval(py::idiv(le as i128, vals.len() as i128)));
        null.set(n_sub.to_string(), o.val());
    }
    res.set("label_swap_null_dgbaa_dgbf", null.val());
    // phase sums and memory (descriptive; measured records)
    let mut ph = Obj::new();
    for (a, d_) in arms.iter() {
        let mut acc: OMap<String, i128> = OMap::new();
        acc.insert("solve_wall_ns".into(), 0);
        let mut rss = Vec::new();
        for r in d_.values() {
            *acc.get_mut(&"solve_wall_ns".to_string()).unwrap() += int(&r["time"]["solve_wall_ns"]);
            for p in r["phases"].as_array().unwrap() {
                *acc.entry_or(p["name"].as_str().unwrap().to_string(), || 0) += int(&p["wall_ns"]);
            }
            for (name, ns) in r["online"]["phases_ns"].as_object().unwrap() {
                *acc.entry_or(format!("online.{name}"), || 0) += int(ns);
            }
            rss.push(Num::of(&r["time"]["max_rss_kib"]));
        }
        let mut o = Obj::new();
        for (k, x) in acc.iter() {
            o.set(k.clone(), py::int_val(*x));
        }
        let mx = rss.iter().copied().fold(None::<Num>, |m, x| match m {
            Some(m) if x.cmp(m) != std::cmp::Ordering::Greater => Some(m),
            _ => Some(x),
        });
        o.set("max_rss_kib_max", mx.unwrap().val())
            .set("max_rss_kib_median", v::percentile(&rss, 0.5).val())
            .set("records", d_.len() as u64);
        ph.set(a.clone(), o.val());
    }
    res.set("phase_sums_ns", ph.val());
    py::dump(&res.val(), out);
    println!("P{b} pairs {} below {}", rd.n, rd.pairs_below_level);
    println!(" corr  {:.15} dof {:.2}", c.ratio, c.dof);
}

/// xcheck_inpod.py: recompute the in-pod native A/A admission comparison
/// (control-comparison.json, --a dgbf-edge --b dgb-aa, aa arms dgbf-edge/dgbf).
fn inpod_crosscheck(b: u32) -> Value {
    let recs = v::load_wall(b);
    let arms = v::by_arm(&recs, true);
    let (p, bl, _) = pairs_for(&arms, "dgbf-edge", "dgb-aa", None);
    let (pa, ba, _) = pairs_for(&arms, "dgbf-edge", "dgbf", None);
    let (r, a) = (summarize(&p, bl), summarize(&pa, ba));
    let c = v::corrected(&r, &a);
    let f = &v::jload(&v::session_dir(b, "wall").join("control-comparison.json"))["wall"];
    let mine: Vec<(&str, Num)> = vec![
        ("ratio_b_over_a", Num::Float(r.ratio)),
        ("ci95_lo", Num::Float(r.ci95_log[0])),
        ("ci95_hi", Num::Float(r.ci95_log[1])),
        ("aa_ratio", Num::Float(a.ratio)),
        ("aa_ci95_lo", Num::Float(a.ci95_log[0])),
        ("aa_ci95_hi", Num::Float(a.ci95_log[1])),
        ("corrected_ratio", Num::Float(c.ratio)),
        ("corrected_ci95_lo", Num::Float(c.ci95_log[0])),
        ("corrected_ci95_hi", Num::Float(c.ci95_log[1])),
        ("corrected_dof", Num::Float(c.dof)),
        ("ns_per_gae_a", Num::Float(r.ns_per_gae_a)),
        ("ns_per_gae_b", Num::Float(r.ns_per_gae_b)),
        ("dof", Num::Int(r.dof_int.unwrap())),
    ];
    let filev: Vec<(&str, Num)> = vec![
        ("ratio_b_over_a", Num::of(&f["ratio_b_over_a"])),
        ("ci95_lo", Num::of(&f["ci95"][0])),
        ("ci95_hi", Num::of(&f["ci95"][1])),
        ("aa_ratio", Num::of(&f["aa_ratio"])),
        ("aa_ci95_lo", Num::of(&f["aa_ci95"][0])),
        ("aa_ci95_hi", Num::of(&f["aa_ci95"][1])),
        ("corrected_ratio", Num::of(&f["corrected_ratio"])),
        ("corrected_ci95_lo", Num::of(&f["corrected_ci95"][0])),
        ("corrected_ci95_hi", Num::of(&f["corrected_ci95"][1])),
        ("corrected_dof", Num::of(&f["corrected_dof"])),
        ("ns_per_gae_a", Num::of(&f["ns_per_gae_a"])),
        ("ns_per_gae_b", Num::of(&f["ns_per_gae_b"])),
        ("dof", Num::of(&f["dof"])),
    ];
    let (mut mo, mut fo, mut ro) = (Obj::new(), Obj::new(), Obj::new());
    let mut mx: Option<f64> = None;
    for ((k, m), (_, fv)) in mine.iter().zip(filev.iter()) {
        mo.set(*k, m.val());
        fo.set(*k, fv.val());
        let rel = m.sub(*fv).abs().div(fv.abs());
        ro.set(*k, py::fval(rel));
        if mx.is_none_or(|x| rel > x) {
            mx = Some(rel);
        }
    }
    let mut o = Obj::new();
    o.set("mine", mo.val()).set("file", fo.val()).set("rel_diff", ro.val()).set("max_rel_diff", py::fval(mx.unwrap()));
    o.val()
}

pub fn run_xcheck(out: &Path) {
    let mut o = Obj::new();
    for b in [32u32, 24] {
        let r = inpod_crosscheck(b);
        println!("P{b} max rel diff {:.3e}", r["max_rel_diff"].as_f64().unwrap());
        o.set(format!("P{b}"), r);
    }
    py::dump(&o.val(), out);
}
