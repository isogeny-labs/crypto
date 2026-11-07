//! Assemble validator-report.json and replay-certificate.json (port of
//! build_report.py) from the validator's own outputs in OUT and the replay
//! receipts in REPLAY (written by `ecbench verify --replay-all`, replay.sh).
//!
//! Claimed values are transcribed from cards/VERDICT-P32.md (sha256
//! 415a89d7...9ea3) and the handoff's claim_under_review, with the precision
//! they are printed at; each headline's rel_tol is half a unit in the last
//! printed digit divided by the claimed value (1e-9 where the claim is printed
//! to 15 digits or exact).

use crate::py::{self, float_repr, fval, num_repr, truthy, Num, Obj};
use crate::v;
use serde_json::Value;
use std::path::Path;

struct Heads(Vec<Value>);

/// rel_tol for a value printed as the string `printed`.
fn tol(printed: &str) -> f64 {
    let s = printed.trim_start_matches('-');
    let d = s.split_once('.').map(|(_, f)| f.chars().count()).unwrap_or(0);
    let val = printed.parse::<f64>().unwrap().abs();
    if d >= 12 {
        return 1e-9;
    }
    // 0.5 * 10 ** (-d): int ** negative int is float pow(); 10 ** 0 is int 1
    let p = if d == 0 { 1.0 } else { 10f64.powf(-(d as f64)) };
    if val != 0.0 {
        (0.5 * p) / val
    } else {
        1e-9
    }
}

/// Python `max(a, b, c)` on numbers: the first maximal argument.
fn pymax(xs: &[Num]) -> Num {
    let mut m = xs[0];
    for x in &xs[1..] {
        if x.cmp(m) == std::cmp::Ordering::Greater {
            m = *x;
        }
    }
    m
}

impl Heads {
    fn h(&mut self, name: &str, printed: &str, recomputed: Value, source: &str, exact: bool) {
        let claimed = printed.parse::<f64>().unwrap();
        let exact = exact || printed == "1.0" || printed == "0.0";
        let t = if exact { 1e-12 } else { tol(printed) };
        let rc = Num::of(&recomputed);
        let lhs = Num::Float(claimed).sub(rc).abs();
        let rhs = Num::Float(t).mul(pymax(&[Num::Float(claimed.abs()), rc.abs(), Num::Float(1e-300)]));
        let reproduced = lhs.cmp(rhs) != std::cmp::Ordering::Greater;
        let mut o = Obj::new();
        o.set("name", name)
            .set("claimed", fval(claimed))
            .set("claimed_as_printed", printed)
            .set("recomputed", recomputed)
            .set("rel_tol", fval(t))
            .set("source", source)
            .set("reproduced", reproduced);
        self.0.push(o.val());
    }
    fn reproduced_where(&self, f: impl Fn(&str) -> bool) -> bool {
        self.0.iter().filter(|h| f(h["name"].as_str().unwrap())).all(|h| h["reproduced"] == Value::Bool(true))
    }
}

fn f(x: f64) -> Value {
    fval(x)
}

fn n(v: &Value) -> f64 {
    Num::of(v).f()
}

fn cond(name: &str, ok: bool, value: Value) -> Value {
    serde_json::json!({"condition": name, "reproduced": ok, "recomputed": value})
}

fn len(v: &Value) -> u64 {
    v.as_array().unwrap().len() as u64
}

fn empty(v: &Value) -> bool {
    !truthy(v)
}

fn eqi(v: &Value, i: i128) -> bool {
    Num::of(v).eq(Num::Int(i))
}

pub fn run(out_dir: &Path, replay_dir: &Path, vdir: &Path) {
    let j = |name: &str| v::jload(&out_dir.join(name));
    let j1_32 = j("j1-P32.json");
    let j1_24 = j("j1-P24.json");
    let j1 = |b: u32| if b == 32 { &j1_32 } else { &j1_24 };
    let (j2, j3, j4) = (j("j2.json"), j("j3.json"), j("j4.json"));
    let (sc, xc, st) = (j("scalars.json"), j("j1-inpod-crosscheck.json"), j("selftest.json"));
    let (s3, s3r, _c2_file) = (j("s3check.json"), j("s3requests.json"), j("c2_identity.json"));
    let vv = "cards/VERDICT-P32.md";
    let mut hs = Heads(Vec::new());
    let mut h = |name: &str, printed: &str, rec: Value, src: String| hs.h(name, printed, rec, &src, false);
    let d = &j1(32)["decision"];
    let (r, a, c) = (&d["dgb->dgbf"], &d["aa dgb->dgb-aa"], &d["corrected"]);
    let fu = format!("{vv} Facts used");
    h("P32 ns_per_gae dgb", "84.5526", r["ns_per_gae_a"].clone(), format!("{vv} Facts used, decision block"));
    h("P32 ns_per_gae dgbf", "81.0446", r["ns_per_gae_b"].clone(), format!("{vv} Facts used, decision block"));
    h("P32 ratio_b_over_a dgbf/dgb", "0.958512", r["ratio"].clone(), fu.clone());
    h("P32 ratio ci95 lower", "0.956084", r["ci95_log"][0].clone(), fu.clone());
    h("P32 ratio ci95 upper", "0.960945", r["ci95_log"][1].clone(), fu.clone());
    h("P32 aa_ratio dgb-aa/dgb", "0.998337", a["ratio"].clone(), fu.clone());
    h("P32 aa ci95 lower", "0.996146", a["ci95_log"][0].clone(), fu.clone());
    h("P32 aa ci95 upper", "1.000534", a["ci95_log"][1].clone(), fu.clone());
    let acr = format!("{vv} (A); claim_under_review");
    h("P32 corrected_ratio dgbf/dgb", "0.960107806", c["ratio"].clone(), acr.clone());
    h("P32 corrected ci95 lower", "0.956898", c["ci95_log"][0].clone(), acr.clone());
    h("P32 corrected ci95 upper", "0.963328", c["ci95_log"][1].clone(), acr.clone());
    h("P32 corrected_dof", "562.6", c["dof"].clone(), fu.clone());
    let c9 = &j1(32)["cond9"];
    h("P32 condition 9: ratio dgbf/dgb-aa (w=0 file)", "0.960107806240821", c9["dgb-aa->dgbf"]["ratio"].clone(), format!("{vv} Facts used, condition 9"));
    drop(h);
    hs.h("P32 condition 9: relative difference to corrected_ratio", "0.0", c9["rel_diff_vs_corrected"].clone(), &format!("{vv} Facts used, condition 9"), true);
    let mut h = |name: &str, printed: &str, rec: Value, src: String| hs.h(name, printed, rec, &src, false);
    let pc = &j1(32)["dgb->dgbf-edge"];
    h("P32 position control dgbf-edge/dgb", "0.959282", pc["ratio"].clone(), format!("{vv} (A) position control"));
    h("P32 position control ci95 lower", "0.956531", pc["ci95_log"][0].clone(), format!("{vv} (A)"));
    h("P32 position control ci95 upper", "0.962040", pc["ci95_log"][1].clone(), format!("{vv} (A)"));
    let fe = &j1(32)["dgbf-edge->dgbf"];
    h("P32 dgbf/dgbf-edge", "0.999197", fe["ratio"].clone(), format!("{vv} Facts used, A/A controls"));
    h("P32 dgbf/dgbf-edge ci95 lower", "0.997648", fe["ci95_log"][0].clone(), fu.clone());
    h("P32 dgbf/dgbf-edge ci95 upper", "1.000749", fe["ci95_log"][1].clone(), fu.clone());
    let a_ = format!("{vv} (A)");
    h("P32 wall per charged GAE reduction, percent", "3.99", f(100.0 * (1.0 - n(&c["ratio"]))), a_.clone());
    h("P32 reduction interval low, percent", "3.67", f(100.0 * (1.0 - n(&c["ci95_log"][1]))), a_.clone());
    h("P32 reduction interval high, percent", "4.31", f(100.0 * (1.0 - n(&c["ci95_log"][0]))), a_.clone());
    let b8 = &j3["P32-wall-cond8"]["bitops"];
    h("P32 bitop/v1 dgbf/dgb (measured records)", "1.0030127", b8["measured_ratio_dgbf_over_dgb"].clone(), fu.clone());
    h("P32 bitop/v1 rise, percent", "0.301", f(100.0 * (n(&b8["measured_ratio_dgbf_over_dgb"]) - 1.0)), a_.clone());
    drop(h);
    hs.h("P32 bitop difference dgbf - dgb (measured)", "115931990016", b8["measured_difference"].clone(), &fu, true);
    let tv = &j3["P32-wall-cond8"]["tfbs"]["values_per_curve"];
    hs.h("P32 table_filter_bit_sets icv1-fp32-t89141-95879fe3", "1048276", tv["icv1-fp32-t89141-95879fe3"][0].clone(), &fu, true);
    hs.h("P32 table_filter_bit_sets icv1-fp32-tm68303-f902d2f0", "1048295", tv["icv1-fp32-tm68303-f902d2f0"][0].clone(), &fu, true);
    let mut h = |name: &str, printed: &str, rec: Value, src: String| hs.h(name, printed, rec, &src, false);
    let ph = &j1(32)["phase_sums_ns"];
    let ms = |arm: &str, k: &str| n(&ph[arm][k]) / 1e6;
    let fps = format!("{vv} Facts used, phase sums");
    h("P32 phase sum solve_wall dgb-aa, ms", "48192.2", f(ms("dgb-aa", "solve_wall_ns")), fps);
    h("P32 phase sum solve_wall dgbf, ms", "46269.7", f(ms("dgbf", "solve_wall_ns")), fu.clone());
    h("P32 phase sum target_PDP dgb-aa, ms", "14047.8", f(ms("dgb-aa", "online.target_PDP")), fu.clone());
    h("P32 phase sum target_PDP dgbf, ms", "10413.1", f(ms("dgbf", "online.target_PDP")), fu.clone());
    h("P32 phase sum oracle_setup dgb-aa, ms", "6396.3", f(ms("dgb-aa", "oracle_setup")), fu.clone());
    h("P32 phase sum oracle_setup dgbf, ms", "8167.2", f(ms("dgbf", "oracle_setup")), fu.clone());
    h("P32 phase sum target_query dgb-aa, ms", "26776.7", f(ms("dgb-aa", "online.target_query")), fu.clone());
    h("P32 phase sum target_query dgbf, ms", "26765.8", f(ms("dgbf", "online.target_query")), fu.clone());
    h("P32 solve_wall change dgbf vs dgb-aa, percent", "-3.99", f(100.0 * (ms("dgbf", "solve_wall_ns") / ms("dgb-aa", "solve_wall_ns") - 1.0)), fu.clone());
    h("P32 target_PDP change, percent", "-25.9", f(100.0 * (ms("dgbf", "online.target_PDP") / ms("dgb-aa", "online.target_PDP") - 1.0)), format!("{vv} (A), (D) 5"));
    h("P32 oracle_setup change, percent", "27.7", f(100.0 * (ms("dgbf", "oracle_setup") / ms("dgb-aa", "oracle_setup") - 1.0)), format!("{vv} (D) 5"));
    h("P32 oracle_setup rise, ms", "1770.9", f(ms("dgbf", "oracle_setup") - ms("dgb-aa", "oracle_setup")), a_.clone());
    h("P32 oracle_setup rise as share of dgb-aa solve wall, percent", "3.67", f(100.0 * (ms("dgbf", "oracle_setup") - ms("dgb-aa", "oracle_setup")) / ms("dgb-aa", "solve_wall_ns")), a_.clone());
    h("P32 target_PDP share of solve wall, percent", "29", f(100.0 * ms("dgb-aa", "online.target_PDP") / ms("dgb-aa", "solve_wall_ns")), format!("{vv} (A) prediction miss"));
    h("P32 target_query share of solve wall, percent", "56", f(100.0 * ms("dgb-aa", "online.target_query") / ms("dgb-aa", "solve_wall_ns")), format!("{vv} (A) prediction miss"));
    let (sw, sc_) = (&j4["P32-wall"]["dgbf/dgb"], &j4["P32-counted"]);
    let fcr = format!("{vv} Facts used; claim_under_review");
    h("P32 S' dgbf/dgb w=0 (wall)", "1.0", sw["0.0"]["ratio_b_over_a"].clone(), fcr.clone());
    h("P32 S' dgbf/dgb w=0.1 (wall)", "1.050488", sw["0.1"]["ratio_b_over_a"].clone(), fcr.clone());
    h("P32 S' dgbf/dgb w=1 (wall)", "1.358633", sw["1.0"]["ratio_b_over_a"].clone(), fcr.clone());
    h("P32 S' dgbf/dgb w=1 per curve t89141 (wall)", "1.3547", sw["1.0"]["per_curve"]["icv1-fp32-t89141-95879fe3"].clone(), fu.clone());
    h("P32 S' dgbf/dgb w=1 per curve tm68303 (wall)", "1.3626", sw["1.0"]["per_curve"]["icv1-fp32-tm68303-f902d2f0"].clone(), fu.clone());
    h("P32 S' dgbf/dgb w=0.1 per curve tm68303 (wall)", "1.0509", sw["0.1"]["per_curve"]["icv1-fp32-tm68303-f902d2f0"].clone(), fu.clone());
    h("P32 S' without bit-set term dgbf/dgb w=1", "1.0", sw["1.0"]["ratio_without_bitset_term"].clone(), fu.clone());
    h("P32 S' filter cost at w=0.1, percent", "5.05", f(100.0 * (n(&sw["0.1"]["ratio_b_over_a"]) - 1.0)), format!("{vv} (A) band"));
    h("P32 S' filter cost at w=1, percent", "35.86", f(100.0 * (n(&sw["1.0"]["ratio_b_over_a"]) - 1.0)), format!("{vv} (A) band"));
    let ftc = format!("{vv} Facts used table; claim_under_review");
    let ft = format!("{vv} Facts used table");
    for (w, p1, p2, p3) in [("0.0", "31.4719", "31.4719", "1.0"), ("0.1", "29.9858", "31.4997", "1.0505"), ("1.0", "23.2775", "31.6255", "1.3586")] {
        h(&format!("P32 counted S' dgb/rho-neg w={w}"), p1, sc_["dgb/rho-neg"][w]["ratio_b_over_a"].clone(), ftc.clone());
        h(&format!("P32 counted S' dgbf/rho-neg w={w}"), p2, sc_["dgbf/rho-neg"][w]["ratio_b_over_a"].clone(), ftc.clone());
        h(&format!("P32 counted S' dgbf/dgb w={w}"), p3, sc_["dgbf/dgb"][w]["ratio_b_over_a"].clone(), ft.clone());
    }
    // build_report.py names this loop's third variable `c2`, which shadows the
    // c2_identity.json it loaded earlier; see `c2_identity_facts` below.
    let mut c2_leftover = "";
    for (pair, c1, c2) in [("dgb/rho-neg", "21.81", "24.97"), ("dgbf/rho-neg", "29.55", "34.02"), ("dgbf/dgb", "1.3547", "1.3626")] {
        h(&format!("P32 counted S' {pair} w=1 per curve t89141"), c1, sc_[pair]["1.0"]["per_curve"]["icv1-fp32-t89141-95879fe3"].clone(), ft.clone());
        h(&format!("P32 counted S' {pair} w=1 per curve tm68303"), c2, sc_[pair]["1.0"]["per_curve"]["icv1-fp32-tm68303-f902d2f0"].clone(), ft.clone());
        c2_leftover = c2;
    }
    let mut allpc: Vec<&Value> = Vec::new();
    for p in ["dgb/rho-neg", "dgbf/rho-neg"] {
        for w in ["0.0", "0.1", "1.0"] {
            for (_, x) in sc_[p][w]["per_curve"].as_object().unwrap() {
                allpc.push(x);
            }
        }
    }
    let pick = |xs: &[&Value], want: std::cmp::Ordering| -> Value {
        let mut m = xs[0];
        for x in &xs[1..] {
            if Num::of(x).cmp(Num::of(m)) == want {
                m = x;
            }
        }
        m.clone()
    };
    h("P32 smallest per-curve IC/rho-neg ratio in the band", "21.81", pick(&allpc, std::cmp::Ordering::Less), fu.clone());
    let mut agg: Vec<&Value> = Vec::new();
    for p in ["dgb/rho-neg", "dgbf/rho-neg"] {
        for w in ["0.0", "0.1", "1.0"] {
            agg.push(&sc_[p][w]["ratio_b_over_a"]);
        }
    }
    h("P32 rho-neg cheaper by factor, low", "23.3", pick(&agg, std::cmp::Ordering::Less), format!("{vv} (A) against matched rho"));
    h("P32 rho-neg cheaper by factor, high", "31.6", pick(&agg, std::cmp::Ordering::Greater), format!("{vv} (A) against matched rho"));
    for (w, p1, p2) in [("0.0", "31.47", "31.47"), ("0.1", "29.99", "31.50"), ("1.0", "23.28", "31.63")] {
        h(&format!("P32 (D)4 dgb/rho-neg w={w}"), p1, sc_["dgb/rho-neg"][w]["ratio_b_over_a"].clone(), format!("{vv} (D) 4"));
        h(&format!("P32 (D)4 dgbf/rho-neg w={w}"), p2, sc_["dgbf/rho-neg"][w]["ratio_b_over_a"].clone(), format!("{vv} (D) 4"));
    }
    drop(h);
    let g32 = &j2["P32-counted"]["gate"]["below_l2_from_gate"];
    hs.h("P32 counted below_l2 entries", "14", g32["count"].clone(), &fu, true);
    hs.h("P32 counted below_l2 on rho-neg", "7", g32["per_arm"]["rho-neg"].clone(), &fu, true);
    hs.h("P32 counted below_l2 on rho-neg-aa", "7", g32["per_arm"]["rho-neg-aa"].clone(), &fu, true);
    for k in ["P32-wall", "P32-counted"] {
        hs.h(&format!("{k} in-pod audit replays identical"), "1152", j2[k]["audit"]["replays_identical"].clone(), &fu, true);
    }
    let c10 = &j3["P32-wall-cond10"];
    hs.h("P32 condition 10 p max", "3965125361", c10["p_max"].clone(), &fu, true);
    hs.h("P32 condition 10 signed points max", "2048", c10["signed_points_max"].clone(), &fu, true);
    // P24
    let mut h = |name: &str, printed: &str, rec: Value, src: String| hs.h(name, printed, rec, &src, false);
    let d = &j1(24)["decision"];
    let (a, c) = (&d["aa dgb->dgb-aa"], &d["corrected"]);
    let v24 = format!("{vv} Facts used, P24");
    h("P24 corrected_ratio dgbf/dgb", "1.003200", c["ratio"].clone(), v24.clone());
    h("P24 corrected ci95 lower", "1.000564", c["ci95_log"][0].clone(), v24.clone());
    h("P24 corrected ci95 upper", "1.005842", c["ci95_log"][1].clone(), v24.clone());
    h("P24 aa_ratio", "1.000506", a["ratio"].clone(), v24.clone());
    h("P24 aa ci95 lower", "0.999012", a["ci95_log"][0].clone(), v24.clone());
    h("P24 aa ci95 upper", "1.002003", a["ci95_log"][1].clone(), v24.clone());
    let pc = &j1(24)["dgb->dgbf-edge"];
    h("P24 position control dgbf-edge/dgb", "1.005304", pc["ratio"].clone(), v24.clone());
    h("P24 position control ci95 lower", "1.002677", pc["ci95_log"][0].clone(), v24.clone());
    h("P24 position control ci95 upper", "1.007938", pc["ci95_log"][1].clone(), v24.clone());
    let d2 = format!("{vv} (D) 2");
    h("P24 (D)2 corrected", "1.0032", c["ratio"].clone(), d2.clone());
    h("P24 (D)2 corrected ci95 lower", "1.0006", c["ci95_log"][0].clone(), d2.clone());
    h("P24 (D)2 corrected ci95 upper", "1.0058", c["ci95_log"][1].clone(), d2.clone());
    let s24 = &j4["P24-counted"];
    for (w, p1, p2, p3) in [("0.0", "12.1159", "12.1159", "1.0"), ("0.1", "11.6498", "12.2388", "1.0506"), ("1.0", "9.4528", "12.8184", "1.3560")] {
        h(&format!("P24 counted S' dgb/rho-neg w={w}"), p1, s24["dgb/rho-neg"][w]["ratio_b_over_a"].clone(), v24.clone());
        h(&format!("P24 counted S' dgbf/rho-neg w={w}"), p2, s24["dgbf/rho-neg"][w]["ratio_b_over_a"].clone(), v24.clone());
        h(&format!("P24 counted S' dgbf/dgb w={w}"), p3, s24["dgbf/dgb"][w]["ratio_b_over_a"].clone(), v24.clone());
    }
    for (w, p1, p2) in [("0.0", "12.12", "12.12"), ("0.1", "11.65", "12.24"), ("1.0", "9.45", "12.82")] {
        h(&format!("P24 (D)4 dgb/rho-neg w={w}"), p1, s24["dgb/rho-neg"][w]["ratio_b_over_a"].clone(), format!("{vv} (D) 4"));
        h(&format!("P24 (D)4 dgbf/rho-neg w={w}"), p2, s24["dgbf/rho-neg"][w]["ratio_b_over_a"].clone(), format!("{vv} (D) 4"));
    }
    drop(h);
    // in-pod instrument cross-check (full precision; the native A/A admission file)
    for b in [32u32, 24] {
        let x = &xc[format!("P{b}")];
        for k in ["ratio_b_over_a", "aa_ratio", "corrected_ratio", "corrected_ci95_lo", "corrected_ci95_hi", "corrected_dof"] {
            let fv = &x["file"][k];
            let printed = match fv {
                Value::Number(nm) => num_repr(nm),
                _ => py::repr(fv),
            };
            let mut o = Obj::new();
            o.set("name", format!("P{b} control-comparison.json wall.{k} (in-pod instrument output)"))
                .set("claimed", fv.clone())
                .set("claimed_as_printed", printed)
                .set("recomputed", x["mine"][k].clone())
                .set("rel_tol", f(1e-9))
                .set("source", format!("P{b}/wall/eks-results/control-comparison.json"))
                .set("reproduced", n(&x["rel_diff"][k]) <= 1e-9);
            hs.0.push(o.val());
        }
    }
    let heads = &hs;

    let mut dd = Obj::new();
    dd.set("name", "P32 S' dgbf/dgb w=0.1 per curve icv1-fp32-t89141-95879fe3")
        .set("claimed_as_printed", "1.0500")
        .set("recomputed", sw["0.1"]["per_curve"]["icv1-fp32-t89141-95879fe3"].clone())
        .set("source", format!("{vv} Facts used, S' for dgb->dgbf (line 61); also the counted session gives the same value"))
        .set("reproduced", false)
        .set("reading", "1.0500605 rounds to 1.0501 at the printed precision; 1.0500 is a truncation. Descriptive per-curve figure; not in (A), (D) or the claim under review; the aggregate 1.050488 and every claim-under-review number reproduce. Not counted as a defect of the evidence; the Coordinator may rule otherwise.");
    let descriptive_discrepancies = vec![dd.val()];

    let mut six1 = Obj::new();
    for b in [32u32, 24] {
        let (w2, w1) = (&j2[format!("P{b}-wall")], j1(b));
        let (w3, w4) = (&j3[format!("P{b}-wall-cond8")], &j4[format!("P{b}-wall")]);
        let cc = &j2[format!("P{b}-counted")]["gate"];
        let ddc = &w1["decision"];
        let c10w = &j3[format!("P{b}-wall-cond10")];
        let ops: Vec<Value> = ["0.0", "0.1", "1.0"].iter().map(|w| w4["dgbf/dgb"][*w]["ratio_without_bitset_term"].clone()).collect();
        let c8 = empty(&w3["mismatches"])
            && truthy(&w3["tfbs"]["absent_on_dgb_and_dgb_aa"])
            && truthy(&w3["tfbs"]["present_once_in_oracle_setup_on_filtered"])
            && truthy(&w3["tfbs"]["one_value_per_curve"])
            && empty(&w3["bitops"]["identity_failures"])
            && empty(&w3["bitops"]["reprice_mismatches"])
            && ops.iter().all(|x| Num::of(x).eq(Num::Float(1.0)));
        let wall = vec![
            cond("1 exit 0 from a single pod", truthy(&w2["cond1_exit"]),
                 serde_json::json!({"exit_code": w2["job_exit"]["exit_code"].clone(), "pod_prefixes": s3["jobs"][format!("P{b}-wall")]["pod_prefixes"].clone()})),
            cond("2 audit ok, complete, 0 problems, every measured replay identical", truthy(&w2["cond2_audit"]), w2["audit"].clone()),
            cond("3 counted A/A ops exactly 1 at w=0,0.1,1 (dgb/dgb-aa, dgbf-edge/dgbf)", truthy(&w2["cond3"]), w2["cond3_counted_aa"].clone()),
            cond("4 input-spec JSON-equal frozen spec; build-info revision and ecbench sha256", truthy(&w2["cond4"]), w2["spec_build"].clone()),
            cond("5 l2-gate accepted, every execution L2/L3, invalid_outcomes empty", truthy(&w2["cond5"]), w2["l2_gate"].clone()),
            cond("6 native A/A admission (control-comparison.json)", truthy(&w2["cond6"]), w2["cond6_control_comparison"].clone()),
            cond("7 decision block facts (recomputed from records; the analysis file itself was not read)",
                 eqi(&ddc["dgb->dgbf"]["n"], 288) && eqi(&ddc["dgb->dgbf"]["pairs_below_level"], 0) && eqi(&ddc["aa dgb->dgb-aa"]["n"], 288),
                 serde_json::json!({"pairs": ddc["dgb->dgbf"]["n"].clone(), "pairs_below_level": ddc["dgb->dgbf"]["pairs_below_level"].clone(),
                  "aa_pairs": ddc["aa dgb->dgb-aa"]["n"].clone(), "aa_arms": ["dgb", "dgb-aa"], "corrected_ci95": ddc["corrected"]["ci95_log"].clone(),
                  "weight_dependence": "none: the wall block reads charged total_gae only"})),
            cond("8 identical charged work; table_filter_bit_sets; bitop identity; ops dgbf/dgb exactly 1", c8,
                 serde_json::json!({"matched": w3["matched"].clone(), "mismatches": len(&w3["mismatches"]), "tfbs": w3["tfbs"].clone(),
                  "bitop_identity_checked": w3["bitops"]["identity_checked"].clone(), "bitop_identity_failures": len(&w3["bitops"]["identity_failures"]),
                  "records_repriced": w3["bitops"]["records_repriced"].clone(), "reprice_mismatches": len(&w3["bitops"]["reprice_mismatches"]),
                  "ops_dgbf_over_dgb": ops})),
            cond("9 identity: dgbf/dgb-aa ratio equals corrected_ratio within 1e-9",
                 n(&w1["cond9"]["rel_diff_vs_corrected"]) <= 1e-9 && eqi(&w1["cond9"]["dgb-aa->dgbf"]["n"], 288),
                 serde_json::json!({"ratio": w1["cond9"]["dgb-aa->dgbf"]["ratio"].clone(), "corrected": ddc["corrected"]["ratio"].clone(),
                  "rel_diff": w1["cond9"]["rel_diff_vs_corrected"].clone(), "sum_wall_ratio": w1["cond9"]["sum_wall_ratio_dgbf_over_dgbaa"].clone(),
                  "gae_sums_equal": w1["cond9"]["gae_sums_equal"].clone()})),
            cond("10 packed layout: p <= 2^32-1 and signed points <= 65536 on every IC record", empty(&c10w["failures"]), c10w.clone()),
        ];
        let cew = &j3[format!("P{b}-counted-equals-wall")];
        let c10c = &j3[format!("P{b}-counted-cond10")];
        let jc = &j2[format!("P{b}-counted")];
        let counted = vec![
            cond("1 audit", truthy(&cc["cond1_audit"]), jc["audit"].clone()),
            cond("2 rho-neg-aa/rho-neg exactly 1 at w=0,0.1,1", truthy(&cc["cond2"]), cc["cond2_rho_aa"].clone()),
            cond("3 spec and build", truthy(&cc["cond3_spec_build"]), jc["spec_build"].clone()),
            cond("4 every ic.pipeline execution L2/L3", truthy(&cc["cond4_ic_all_L2_L3"]), cc["records_levels"].clone()),
            cond("5 rho executions L2/L3, or L1 with only too-short L2 blockers", truthy(&cc["cond5_rho"]), cc["below_l2_from_gate"].clone()),
            cond("6 invalid_outcomes empty", truthy(&cc["cond6_invalid_outcomes_empty"]), serde_json::json!([])),
            cond("exit-42 clause (from l2-gate.json and from records.jsonl)", truthy(&cc["EXIT42_CLAUSE"]),
                 serde_json::json!({"exit_code": jc["job_exit"]["exit_code"].clone(), "below_l2_from_records_equal_gate": cc["below_l2_from_records_equal_gate"].clone()})),
            cond("counted dgb and dgbf records equal wall records (condition 8 fields, table_filter_bit_sets included)", truthy(&cew["equal"]),
                 serde_json::json!({"dgb": cew["dgb"]["matched"].clone(), "dgbf": cew["dgbf"]["matched"].clone()})),
            cond("condition 10 on counted IC records", empty(&c10c["failures"]), c10c.clone()),
        ];
        six1.set(format!("P{b}"), serde_json::json!({"wall": wall, "counted": counted}));
    }
    let six1 = six1.val();
    let all_six1 = six1
        .as_object()
        .unwrap()
        .values()
        .flat_map(|s| s.as_object().unwrap().values())
        .flat_map(|part| part.as_array().unwrap())
        .all(|x| x["reproduced"] == Value::Bool(true));
    let p28 = &j2["P28-counted"]["gate"];
    let p28_ok = truthy(&p28["EXIT42_CLAUSE"]) && truthy(&j3["P28-counted-equals-wall"]["equal"]) && empty(&j3["P28-counted-cond10"]["failures"]);
    let between1 = |c: &Value| n(&c[0]) < 1.0 && 1.0 < n(&c[1]);
    let null_ok = j1(32)["label_swap_null_dgbaa_dgbf"]
        .as_object()
        .unwrap()
        .values()
        .all(|x| (n(&x["null_median"]) - 1.0).abs() < 1e-3 && n(&x["null_fraction_le_observed"]) == 0.0);
    let mut ptm = Obj::new();
    ptm.set("P32 A/A dgb-aa/dgb interval contains 1", between1(&j1(32)["decision"]["aa dgb->dgb-aa"]["ci95_log"]))
        .set("P32 A/A dgbf/dgbf-edge interval contains 1", between1(&j1(32)["dgbf-edge->dgbf"]["ci95_log"]))
        .set("P24 corrected lower bound > 1 (regression reproduced, no improvement found)", n(&j1(24)["decision"]["corrected"]["ci95_log"][0]) > 1.0)
        .set(
            "counted A/A exactly 1 (dgb/dgb-aa, dgbf-edge/dgbf, rho-neg-aa/rho-neg) at every weight",
            [&j2["P32-wall"]["cond3"], &j2["P24-wall"]["cond3"], &j2["P32-counted"]["gate"]["cond2"], &j2["P24-counted"]["gate"]["cond2"]]
                .iter()
                .all(|x| truthy(x)),
        )
        .set("label-swap null (dgb-aa, dgbf) centred on 1, observed outside it", null_ok);
    let ptm_all = ptm.0.values().all(|x| x == &Value::Bool(true));
    let prefixes = ["P32 ns_per", "P32 ratio", "P32 aa", "P32 corrected", "P32 condition 9", "P24 corrected", "P24 aa"];
    let j1_ok = heads.reproduced_where(|nm| prefixes.iter().any(|p| nm.starts_with(p)))
        && n(&j1(32)["decision"]["corrected"]["ci95_log"][1]) < 1.0;
    let pos_ok = |b: u32, a: &str| {
        let p = &j2[format!("P{b}-wall")]["positions"]["positions"][a];
        py::py_eq(p, &serde_json::json!({"1": 144, "2": 144}))
    };
    let j2_ok = [32u32, 24].iter().all(|b| {
        ["cond1_exit", "cond2_audit", "cond4", "cond5", "cond6"].iter().all(|k| truthy(&j2[format!("P{b}-wall")][*k]))
    }) && [32u32, 24].iter().all(|b| empty(&j2[format!("P{b}-wall")]["raw_regrade"]["disagreements"]))
        && [32u32, 24].iter().all(|b| ["dgbf", "dgb-aa"].iter().all(|a| pos_ok(*b, a)));
    let j3_ok = [32u32, 24].iter().all(|b| {
        six1[format!("P{b}")]["wall"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| {
                let c = x["condition"].as_str().unwrap();
                c.starts_with('8') || c.starts_with("10")
            })
            .all(|x| x["reproduced"] == Value::Bool(true))
    }) && [32u32, 24, 28].iter().all(|b| truthy(&j3[format!("P{b}-counted-equals-wall")]["equal"]));
    let j4_ok = heads.reproduced_where(|nm| nm.contains("S'") || nm.contains("rho-neg"))
        && [32u32, 24, 28].iter().all(|b| {
            ["dgb/rho-neg", "dgbf/rho-neg"].iter().all(|p| truthy(&j4[format!("P{b}-counted")][*p]["strict_same_order_across_band_every_curve"]))
        })
        && [32u32, 24, 28].iter().all(|b| truthy(&j2[format!("P{b}-counted")]["gate"]["EXIT42_CLAUSE"]));
    let tags = ["P32-wall", "P32-counted", "P24-wall", "P24-counted", "P28-counted", "P28-wall"];
    let reps: Vec<Value> = tags.iter().map(|t| v::jload(&replay_dir.join(format!("{t}.replay-receipt.json")))).collect();
    let rep = |t: &str| &reps[tags.iter().position(|x| *x == t).unwrap()];
    let identical = |r: &Value| {
        r["replays"].as_array().unwrap().iter().filter(|x| truthy(&x["reproduced"]) && x["detail"].as_str() == Some("identical")).count() as u64
    };
    let j5_ok = tags[..4].iter().all(|t| {
        let r = rep(t);
        truthy(&r["ok"]) && len(&r["problems"]) == 0 && identical(r) == 1152
    }) && tags[..4].iter().all(|t| truthy(&sc[*t]["all_ok"]));
    let hb = |x: bool| if x { "holds" } else { "breaks" };
    let mut joints = Obj::new();
    joints
        .set("J1 decision statistic", hb(j1_ok))
        .set("J2 instrument", hb(j2_ok))
        .set("J3 charged-work identity", hb(j3_ok))
        .set("J4 counted route and band", hb(j4_ok))
        .set("J5 independent replay", hb(j5_ok))
        .set("proves_too_much", hb(ptm_all));
    let joints_hold = joints.0.values().all(|x| x.as_str() == Some("holds"));
    let defects: Vec<&Value> = heads.0.iter().filter(|h| h["reproduced"] != Value::Bool(true)).collect();
    let status = if defects.is_empty() && joints_hold && all_six1 && truthy(&st["all_pass"]) { "pass" } else { "fail" };

    let rel_from_evidence = "../../research/ic_packed_filter_20261029/";
    let repo = v::repo();
    let mut raw_files = Vec::new();
    for (b, kind) in [(32, "wall"), (32, "counted"), (24, "wall"), (24, "counted"), (28, "counted")] {
        let mut names = vec!["session/records.jsonl", "job-exit.json", "audit.json", "l2-gate.json", "input-spec.json", "build-info.json", "SHA256SUMS"];
        if kind == "wall" {
            names.push("control-comparison.json");
        }
        for name in names {
            let repo_rel = format!("research/ic_packed_filter_20261029/P{b}/{kind}/eks-results/{name}");
            raw_files.push(serde_json::json!({"path": format!("{rel_from_evidence}P{b}/{kind}/eks-results/{name}"), "repo_path": repo_rel,
                "sha256": v::sha256_file(&repo.join(&repo_rel))}));
        }
    }
    for spec in ["spec-P32.json", "spec-P32-rho.json", "spec-P24.json", "spec-P24-rho.json", "spec-P28-rho.json"] {
        let repo_rel = format!("backlog/XP-20261029-packed-filter/{spec}");
        raw_files.push(serde_json::json!({"path": format!("../../{repo_rel}"), "repo_path": repo_rel, "sha256": v::sha256_file(&repo.join(&repo_rel))}));
    }

    let mut s3cc = Obj::new();
    for (k, x) in s3["jobs"].as_object().unwrap() {
        s3cc.set(
            k.clone(),
            serde_json::json!({"one_pod_prefix_equal_to_job_exit": x["one_pod_prefix_equal_to_job_exit"].clone(),
                "all_files_equal": x.get("all_files_equal").cloned().unwrap_or(Value::Null),
                "launcher_exit_codes": x["launcher_results"].as_array().unwrap().iter().map(|y| y["exit_code"].clone()).collect::<Vec<_>>()}),
        );
    }
    let sprime: serde_json::Map<String, Value> = ["dgb/rho-neg", "dgbf/rho-neg", "dgbf/dgb"]
        .iter()
        .map(|p| (p.to_string(), Value::Array(["0.0", "0.1", "1.0"].iter().map(|w| j4["P28-counted"][*p][*w]["ratio_b_over_a"].clone()).collect())))
        .collect();
    let mut p28o = Obj::new();
    p28o.set("job", "ic-dgbf-rho-p28-20261029-r2")
        .set("route_satisfied", p28["route_satisfied"].clone())
        .set("EXIT42_CLAUSE", p28["EXIT42_CLAUSE"].clone())
        .set("below_l2", p28["below_l2_from_gate"].clone())
        .set("counted_equals_wall", j3["P28-counted-equals-wall"]["equal"].clone())
        .set("cond10", j3["P28-counted-cond10"].clone())
        .set("original_prefix_objects_now", s3["original_p28_counted_prefix_objects"].clone())
        .set("sprime", Value::Object(sprime))
        .set("ok", p28_ok)
        .set("note", "P28 wall used for counts only (equality check); no P28 wall field read; P28 stays undecidable");
    let defects_v: Vec<Value> = defects
        .iter()
        .map(|h| serde_json::json!({"name": h["name"].clone(), "claimed": h["claimed_as_printed"].clone(), "recomputed": h["recomputed"].clone()}))
        .collect();
    let mut rep_o = Obj::new();
    rep_o
        .set("schema", "validator-report/v1 (docs/VALIDATION_GATE.md)")
        .set("id", Value::Null)
        .set("id_provenance", "not allocated by the validator: tools/allocate_id.py has no validation-report type and the validator mints no identifier; the archiving session names this report by path and sha256, or allocates an id")
        .set("task_id", "TASK-20261029-654f73")
        .set("decision_ref", "DEC-20261029-5eb8e6")
        .set("study", "XP-20261029-packed-filter")
        .set("status", status)
        .set("independent", true)
        .set("independence", "validator's own code (validation/code/), sharing nothing with cards/tools/; no analysis/ file read; replay on a host of another class; blind_from respected")
        .set("raw_data_paths_relative_to", "a depth-2 evidence directory (benchmarks/<dir>/ or research/ic_packed_filter_20261029/); repo_path gives the repository-root path")
        .set("raw_data", raw_files)
        .set("headline", heads.0.clone())
        .set("descriptive_discrepancies", descriptive_discrepancies)
        .set("section_6_1", six1.clone())
        .set("section_6_1_all_reproduced", all_six1)
        .set("p28_counted_session_of_record", p28o.val())
        .set("joints", joints.val())
        .set("proves_too_much", ptm.val())
        .set("selftest_negative_controls", st.clone())
        .set("s3_copy_check", s3cc.val())
        .set("launcher_requests", s3r.clone())
        // RECORDED DEFECT, reproduced on purpose (correction CORR-20261030-1153f8):
        // build_report.py wrote `"c2_identity_facts": c2`, but its loop
        // `for pair, c1, c2 in ...` above had rebound c2 to the last printed
        // per-curve claim, "1.3626", so the published report carries that string
        // instead of the c2_identity.json facts. The facts themselves are in
        // out/c2_identity.json. Kept for byte-equality with the pinned report.
        .set("c2_identity_facts", c2_leftover)
        .set("defects", defects_v)
        .set("outputs", "validation/code/out/");
    py::dump(&rep_o.val(), &vdir.join("validator-report.json"));

    let mut runs = Vec::new();
    for (t, job, req) in [
        ("P32-wall", "ic-dgbf-p32-20261029", true),
        ("P32-counted", "ic-dgbf-rho-p32-20261029", true),
        ("P24-wall", "ic-dgbf-p24-20261029", true),
        ("P24-counted", "ic-dgbf-rho-p24-20261029", true),
        ("P28-counted", "ic-dgbf-rho-p28-20261029-r2", false),
        ("P28-wall", "ic-dgbf-p28-20261029", false),
    ] {
        let b: u32 = t[1..3].parse().unwrap();
        let kind = t.split('-').nth(1).unwrap();
        let rr = rep(t);
        let rpath = replay_dir.join(format!("{t}.replay-receipt.json"));
        let run = v::jload(&replay_dir.join(format!("{t}.replay-run.json")));
        let sess = v::jload(&v::session_dir(b, kind).join("session").join("session.json"));
        let sct = &sc[if t != "P28-wall" { t } else { "P28-wall(descriptive)" }];
        let own: serde_json::Map<String, Value> = ["records", "warmups", "scalar_ok", "status_verified", "matches_target_true"]
            .iter()
            .map(|k| (k.to_string(), sct[*k].clone()))
            .collect();
        let mut o = Obj::new();
        o.set("run", t)
            .set("job", job)
            .set("session_id", sess["session_id"].clone())
            .set("required_by_handoff", req)
            .set("descriptive_only", t == "P28-wall")
            .set("records_sha256", sess["records_sha256"].clone())
            .set("replay_receipt", serde_json::json!({"path": format!("code/out/replay/{t}.replay-receipt.json"), "sha256": v::sha256_file(&rpath)}))
            .set("command", format!("ecbench verify --dir <copy of P{b}/{kind}/eks-results/session> --replay-all --out <receipt>"))
            .set("ecbench_sha256", run["ecbench_sha256"].clone())
            .set("started", run["started"].clone())
            .set("finished", run["finished"].clone())
            .set("exit", run["exit"].clone())
            .set("ok", rr["ok"].clone())
            .set("session_status", rr["session_status"].clone())
            .set("problems", len(&rr["problems"]))
            .set("replay_selection", rr["replay_selection"].clone())
            .set("replay_candidates", rr["replay_candidates"].clone())
            .set("replays", len(&rr["replays"]))
            .set("identical", identical(rr))
            .set("warmup_records_not_replayable_by_binary", run["warmup_records_not_replayed"].clone())
            .set("replay_env_class_id", rr["auditor_env_class_id"].clone())
            .set("replay_hardware_class_id", rr["auditor_hardware_class_id"].clone())
            .set("pod_env_class_id", sess["env_class_id"].clone())
            .set("own_scalar_check", Value::Object(own))
            .set("own_scalar_check_failures", len(&sct["failures"]))
            .set("curve_binding", sct["curves"].clone());
        runs.push(o.val());
    }
    let required: Vec<Value> = runs.iter().filter(|x| x["required_by_handoff"] == Value::Bool(true)).cloned().collect();
    let cert_ok = required.iter().all(|x| {
        truthy(&x["ok"])
            && eqi(&x["problems"], 0)
            && int_eq(&x["identical"], &x["replays"])
            && eqi(&x["replays"], 1152)
            && eqi(&x["own_scalar_check"]["scalar_ok"], 1344)
            && eqi(&x["own_scalar_check_failures"], 0)
    });
    let host = v::read_text(&replay_dir.join("replay-host.txt"));
    let mut cert = Obj::new();
    cert.set("schema", "replay-certificate/v1 (docs/VALIDATION_GATE.md)")
        .set("task_id", "TASK-20261029-654f73")
        .set("status", if cert_ok { "verified" } else { "failed" })
        .set("independent", true)
        .set("scalars_verified", cert_ok)
        .set("binary", serde_json::json!({"path": "/dev/shm/pf-bundle/ecbench (local copy of the image binary)", "sha256": "21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503",
            "image": "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b",
            "source_revision": "3271a25d02dcef6da148293fb8f9e2149ce59858"}))
        .set("replay_host", py::splitlines(host.trim()))
        .set("pod_host", "c8a.2xlarge, AMD EPYC 9R45 (session/host.json)")
        .set("method", "every measured deterministic run re-executed by `ecbench verify --replay-all` on a byte-identical copy of the session directory (copy checked against SHA256SUMS before and after); warm-ups are refused by the binary's replay modes and are covered by the validator's own scalar check, which verifies [recovered]G = Q on every record (warm-ups included) with its own short-Weierstrass arithmetic, the curve bound by its ICV1 j-invariant, p + 1 - t = r and [r]G = O")
        .set("runs", required)
        .set("additional_runs", runs.iter().filter(|x| x["required_by_handoff"] != Value::Bool(true) && x["descriptive_only"] != Value::Bool(true)).cloned().collect::<Vec<_>>())
        .set("descriptive_runs", runs.iter().filter(|x| x["descriptive_only"] == Value::Bool(true)).cloned().collect::<Vec<_>>())
        .set("descriptive_note", "P28 wall replay is descriptive only: it never stands in for the missing in-pod audit (RULING-P28 (a), 'No post-hoc cure'), and P28 stays undecidable")
        .set("first_attempt", "an earlier attempt that added --replay-seq for each warm-up seq was refused for those seqs ('not replayable (a warm-up, ...)'); its P24-wall summary is in code/out/replay/attempt1-with-warmup-seqs/; the 1152 measured replays of that attempt were identical");
    py::dump(&cert.val(), &vdir.join("replay-certificate.json"));
    println!("status {status} cert {} 6.1 {all_six1} defects {} headlines {}", if cert_ok { "verified" } else { "failed" }, defects.len(), heads.0.len());
    let _ = float_repr;
}

fn int_eq(a: &Value, b: &Value) -> bool {
    Num::of(a).eq(Num::of(b))
}
