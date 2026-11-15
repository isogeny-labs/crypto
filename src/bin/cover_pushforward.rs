//! `cover_pushforward`: push the genus-2 index-calculus relations of the lab's
//! bielliptic covers forward through both elliptic quotients, and compare
//! index calculus on `J(C)` with index calculus on `E1` over the target-side
//! factor base `B = φ1(degree-1 places)` and with matched rho on `E1`.
//!
//! ```text
//! cover_pushforward run [--primes 211,419,...] [--seeds 3] [--json F] [--svg F]
//! ```
//!
//! Same curves, instances and seeds as `chained_transfer_bench` (the curve of a
//! prime is the first candidate of the same search loop that passes the same
//! admissibility checks; seed 0 of each cell reproduces its `results_v3.json`
//! row). Protocol: `research/cover_pushforward_20261112/PROTOCOL.md`.
//! The transfer (cover, `φ^*`) is a one-time cost reported separately.
//! All figures are `[pcc:unverified]`.

use std::collections::BTreeMap;
use std::process::ExitCode;
use std::time::Instant;

use crypto_lib::cryptanalysis::chained_transfer::amortized::certify_dlog;
use crypto_lib::cryptanalysis::chained_transfer::bielliptic::{Bielliptic, Quotient};
use crypto_lib::cryptanalysis::chained_transfer::chain::{Chain, Elt, Hop};
use crypto_lib::cryptanalysis::chained_transfer::ec::{Curve, Pt, VeluIsogeny};
use crypto_lib::cryptanalysis::chained_transfer::measure::{genus2_index_calculus, rho_on_ec};
use crypto_lib::cryptanalysis::chained_transfer::pushforward::{
    b_in_subgroup, b_structure, certify_ec, collect_jc, e1_birthday_first_collision, e1_fibre_ic,
    e1_two_term_ic, jc_first_collision, push_summary, solve_rows, subgroup_points, BStructure,
    Collision, E1Run, PushSummary,
};
use crypto_lib::cryptanalysis::genus2_fp::Divisor;
use serde_json::{json, Value};

// ── curve selection: the same loop and checks as chained_transfer_bench ──

fn largest_cyclic_prime(mut n: u64) -> u64 {
    let mut best = 1u64;
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            let mut e = 0;
            while n.is_multiple_of(d) {
                n /= d;
                e += 1;
            }
            if e == 1 {
                best = best.max(d);
            }
        }
        d += 1;
    }
    if n > 1 {
        best = best.max(n);
    }
    best
}

fn ec_gen(e: &Curve, r: u64) -> Option<Pt> {
    let order = e.order();
    if !order.is_multiple_of(r) {
        return None;
    }
    let cof = order / r;
    for s in 1..5000u64 {
        let g = e.mul(&e.point_from_seed(s.wrapping_mul(0x9E37_79B9)), cof as u128);
        if g.is_some() && e.mul(&g, r as u128).is_none() {
            return Some(g);
        }
    }
    None
}

fn find_isogeny(e: &Curve, r: u64) -> Option<(VeluIsogeny, Pt)> {
    let order = e.order();
    for ell in [2u64, 3, 5, 7] {
        if !order.is_multiple_of(ell) || r == ell {
            continue;
        }
        let cof = order / ell;
        for s in 1..2000u64 {
            let g = e.mul(
                &e.point_from_seed(s.wrapping_mul(2_654_435_761)),
                cof as u128,
            );
            if g.is_none() {
                continue;
            }
            if let Some(iso) = VeluIsogeny::from_generator(e, &g) {
                return Some((iso, g));
            }
        }
    }
    None
}

struct Curveinfo {
    b: Bielliptic,
    candidate: u64,
    coeffs: [u64; 4],
    seed0: u64,
    r: u64,
    gen: Pt,
    skipped: BTreeMap<&'static str, u64>,
    transfer_ms: f64,
}

/// The bench's `measure_curve` up to (and including) the verified pull-back,
/// without the solvers. `Err` names why the candidate is skipped.
fn admissible(p: u64, c: [u64; 4], seed: u64) -> Result<(Bielliptic, u64, Pt, f64), &'static str> {
    let t0 = Instant::now();
    let b = Bielliptic::new(p, c).map_err(|e| match e {
        crypto_lib::cryptanalysis::chained_transfer::bielliptic::BiellipticError::Genus2(_) => {
            "bielliptic_rejected_genus2_model"
        }
        _ => "bielliptic_rejected_degenerate_or_no_rational_weierstrass_point",
    })?;
    let e1 = b.e1;
    let e2 = b.e2;
    let (n1, n2) = (e1.order(), e2.order());
    let r = largest_cyclic_prime(n1);
    if r < 50 {
        return Err("r_below_50");
    }
    if n2.is_multiple_of(r) {
        return Err("r_divides_e2_order_so_r_part_of_J_not_cyclic");
    }
    if (n1 * n2).is_multiple_of(r * r) {
        return Err("r_squared_divides_jac_order");
    }
    let x = (seed.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
    let gen = ec_gen(&e1, r).ok_or("no_order_r_point_on_E1")?;
    let target = e1.mul(&gen, x as u128);
    let (iso, _k) = find_isogeny(&e1, r).ok_or("no_rational_small_isogeny")?;
    let hop = Hop::isogeny(iso, &gen, &target, r).map_err(|_| "isogeny_hop_failed")?;
    if !Chain::new(vec![hop]).verify(&Elt::E(gen), r, x as u128) {
        return Err("isogeny_chain_unverified");
    }
    let hop = Hop::pullback(&b, Quotient::E1, &gen, r).map_err(|_| "pullback_hop_failed")?;
    let ch = Chain::new(vec![hop]);
    if !ch.verify(&Elt::E(gen), r, x as u128) {
        return Err("pullback_chain_unverified");
    }
    Ok((b, r, gen, t0.elapsed().as_secs_f64() * 1e3))
}

fn select_curve(p: u64, i: usize) -> Option<Curveinfo> {
    let mut skipped: BTreeMap<&'static str, u64> = BTreeMap::new();
    for s in 0..4000u64 {
        let c = [
            1 + (s.wrapping_mul(2_654_435_761) >> 3) % (p - 1),
            (s.wrapping_mul(40_503) >> 5) % p,
            (s.wrapping_mul(2_246_822_519) >> 7) % p,
            1 + (s.wrapping_mul(3_266_489_917) >> 9) % (p - 1),
        ];
        let seed0 = 0xC0FFEE + i as u64 * 101 + s;
        match admissible(p, c, seed0) {
            Ok((b, r, gen, ms)) => {
                return Some(Curveinfo {
                    b,
                    candidate: s,
                    coeffs: c,
                    seed0,
                    r,
                    gen,
                    skipped,
                    transfer_ms: ms,
                })
            }
            Err(reason) => *skipped.entry(reason).or_default() += 1,
        }
    }
    None
}

// ── measurement ─────────────────────────────────────────────────────────

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

fn run_json(r: &E1Run) -> Value {
    serde_json::to_value(r).unwrap()
}

#[allow(clippy::too_many_arguments)]
fn measure_seed(
    ci: &Curveinfo,
    seed: u64,
    ge: &Pt,
    cross_check_library: bool,
) -> (Value, Vec<(&'static str, f64, f64)>) {
    let b = &ci.b;
    let p = b.k.p;
    let r = ci.r;
    let e1 = b.e1;
    let x = (seed.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
    let te = e1.mul(ge, x as u128);
    let dg: Divisor = b.pull(Quotient::E1, ge);
    let dt: Divisor = b.pull(Quotient::E1, &te);
    let transfer_ok =
        b.jac.mul(&dg, r).is_identity() && !dg.is_identity() && b.jac.mul(&dg, x) == dt;

    // A0 rho on E1
    let rho = rho_on_ec(&e1, *ge, te, r, seed);
    let rho_ops = rho.group_ops.adds + rho.group_ops.doubles;
    let rho_ok =
        rho.recovered == Some(x) && rho.recovered.is_some_and(|k| certify_ec(&e1, ge, &te, k));

    // A1 IC on J(C): copy of the library collector
    let t = Instant::now();
    let jc = collect_jc(&b.jac, &dg, &dt, r, p as usize, 50_000_000, seed);
    let (jc_rec, jc_la) = solve_rows(&jc.rels, jc.nfb, r);
    let jc_secs = t.elapsed().as_secs_f64();
    let jc_cert = jc_rec.is_some_and(|k| certify_dlog(&b.jac, &dg, &dt, k).0);
    let mut lib_match = Value::Null;
    if cross_check_library {
        let lib = genus2_index_calculus(&b.jac, &dg, &dt, r, p as usize, 50_000_000, seed);
        lib_match = json!({
            "ops": lib.group_ops == jc.ops,
            "trials": lib.trials == jc.trials,
            "relations": lib.relations == jc.rels.len() as u64,
            "recovered": lib.recovered == jc_rec,
            "la": lib.la_field_ops == jc_la,
        });
    }

    // A2 pushed relations
    let ps: PushSummary = push_summary(b, &jc.rels, (ge, &te), (&dg, &dt));
    let jc_coll: Option<Collision> = jc_first_collision(b, &jc.rels, r);

    // A4 native 1-term IC on E1, same stream and stop rule
    let (a4, a4_trials, a4_coll) = e1_fibre_ic(b, ge, &te, r, jc.rels.len(), 50_000_000, seed);
    let jc_trials: Vec<u64> = jc.rels.iter().map(|q| q.trial).collect();
    let same_accepted = a4_trials == jc_trials;

    // control (amendment 1): generic birthday on E1, no cover, no factor base
    let bday = e1_birthday_first_collision(b, ge, &te, r, 50_000_000, seed);

    // A5 generic two-term IC on E1 over B_r
    let a5 = e1_two_term_ic(b, ge, &te, r, 50_000_000, seed);

    let coll_json = |c: &Option<Collision>| -> Value {
        match c {
            Some(c) => {
                let ok = c.log.is_some_and(|k| certify_ec(&e1, ge, &te, k));
                json!({"relations": c.relations, "trials": c.trials, "ops": c.ops, "log": c.log, "certified": ok})
            }
            None => Value::Null,
        }
    };

    // totals at the three section 5a prices
    let price = |ops: u64, la: u64, w: f64| ops as f64 + w * la as f64;
    let mut ratios: Vec<(&'static str, f64, f64)> = vec![];
    let mut arms = serde_json::Map::new();
    let rho_f = rho_ops as f64;
    let mut add_arm = |name: &'static str, ops: u64, la: u64, ok: bool| {
        let mut totals = serde_json::Map::new();
        for w in [0.0f64, 0.1, 1.0] {
            let t = price(ops, la, w);
            totals.insert(format!("w{w}"), json!(t));
            totals.insert(format!("ratio_w{w}"), json!(t / rho_f));
        }
        arms.insert(
            name.to_string(),
            json!({"group_ops": ops, "la_mulmods": la, "certified": ok, "totals": totals}),
        );
        ratios.push((
            name,
            price(ops, la, 0.0) / rho_f,
            price(ops, la, 1.0) / rho_f,
        ));
    };
    add_arm("rho_e1", rho_ops, 0, rho_ok);
    add_arm("ic_jc", jc.ops, jc_la, jc_cert && jc_rec == Some(x));
    add_arm(
        "ic_e1_fibre_full",
        a4.group_ops,
        a4.la_mulmods,
        a4.certified && a4.recovered == Some(x),
    );
    if let Some(c) = &a4_coll {
        let ok = c.log.is_some_and(|k| certify_ec(&e1, ge, &te, k));
        add_arm("ic_e1_fibre_first_collision", c.ops, 0, ok);
    }
    if let Some(c) = &jc_coll {
        let ok = c.log.is_some_and(|k| certify_dlog(&b.jac, &dg, &dt, k).0);
        add_arm("ic_jc_first_collision", c.ops, 0, ok);
    }
    if let Some(c) = &bday {
        let ok = c.log.is_some_and(|k| certify_ec(&e1, ge, &te, k));
        add_arm("birthday_e1_first_collision", c.ops, 0, ok);
    }
    add_arm(
        "ic_e1_two_term",
        a5.group_ops,
        a5.la_mulmods,
        a5.certified && a5.recovered == Some(x),
    );

    let distinct_cols = {
        let mut s = std::collections::HashSet::new();
        for q in &jc.rels {
            for (i, &e) in q.row.iter().enumerate() {
                if e != 0 {
                    s.insert(i);
                }
            }
        }
        s.len()
    };
    let v = json!({
        "seed": seed,
        "x": x,
        "transfer_ok": transfer_ok,
        "rho": {"ops": rho_ops, "recovered_ok": rho_ok},
        "ic_jc": {
            "recovered": jc_rec, "certified": jc_cert, "group_ops": jc.ops, "trials": jc.trials,
            "relations": jc.rels.len(), "fb": jc.nfb, "la_mulmods": jc_la, "secs": jc_secs,
            "trials_per_relation": jc.trials as f64 / jc.rels.len().max(1) as f64,
            "library_match": lib_match,
            "first_collision": coll_json(&jc_coll),
            "distinct_columns_used": distinct_cols,
        },
        "pushed": serde_json::to_value(&ps).unwrap(),
        "ic_e1_fibre": {
            "run": run_json(&a4),
            "accepted_trials_equal_jc": same_accepted,
            "trials_per_relation": a4.trials as f64 / a4.relations.max(1) as f64,
            "first_collision": coll_json(&a4_coll),
        },
        "birthday_e1": coll_json(&bday),
        "ic_e1_two_term": {
            "run": run_json(&a5),
            "trials_per_relation": a5.trials as f64 / a5.relations.max(1) as f64,
            "search_ops_per_trial": a5.search_ops as f64 / a5.trials.max(1) as f64,
        },
        "arms": arms,
    });
    (v, ratios)
}

/// Local wall-clock ratio of a Cantor addition to an elliptic addition
/// (unisolated host, no timing claim).
fn op_cost_ratio(ci: &Curveinfo, ge: &Pt) -> (f64, f64) {
    let b = &ci.b;
    let e = b.e1;
    let n = 4000usize;
    let pts: Vec<Pt> = (1..=64u128).map(|m| e.mul(ge, m * 7)).collect();
    let t = Instant::now();
    let mut acc: Pt = None;
    for i in 0..n {
        acc = e.add(&acc, &pts[i % 64]);
    }
    let ec_ns = t.elapsed().as_nanos() as f64 / n as f64;
    std::hint::black_box(&acc);
    let ds: Vec<Divisor> = (1..=64u128)
        .map(|m| b.pull(Quotient::E1, &e.mul(ge, m * 7)))
        .collect();
    let t = Instant::now();
    let mut dacc = Divisor::IDENTITY;
    for i in 0..n {
        dacc = b.jac.add(&dacc, &ds[i % 64]);
    }
    let jac_ns = t.elapsed().as_nanos() as f64 / n as f64;
    std::hint::black_box(&dacc);
    (ec_ns, jac_ns)
}

fn render_svg(rows: &[Value]) -> String {
    let ps: Vec<f64> = rows.iter().map(|r| r["p"].as_f64().unwrap()).collect();
    let (x0, x1) = (150f64.ln(), 20000f64.ln());
    let (y0, y1) = (100f64.ln(), 2.0e6f64.ln());
    let xp = |p: f64| 80.0 + (p.ln() - x0) / (x1 - x0) * 560.0;
    let yp = |v: f64| 380.0 - (v.ln() - y0) / (y1 - y0) * 300.0;
    let mut s = String::new();
    s.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"700\" height=\"600\" viewBox=\"0 0 700 600\" font-family=\"Helvetica,Arial,sans-serif\" font-size=\"12\">\n<rect width=\"700\" height=\"600\" fill=\"#ffffff\"/>\n");
    s.push_str("<text x=\"350\" y=\"24\" text-anchor=\"middle\" font-size=\"15\" font-weight=\"bold\">Cover push-forward: counted group operations to a certified log</text>\n");
    s.push_str("<text x=\"350\" y=\"42\" text-anchor=\"middle\" fill=\"#555\">log-log; median over seeds; LA unpriced (w = 0); values [pcc:unverified]</text>\n");
    for t in [100.0, 1000.0, 10000.0, 100000.0, 1000000.0] {
        let y = yp(t);
        s.push_str(&format!("<line x1=\"80\" y1=\"{y:.1}\" x2=\"640\" y2=\"{y:.1}\" stroke=\"#e5e5e5\"/><text x=\"74\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#444\">{t}</text>\n", y + 4.0));
    }
    for t in [211.0, 419.0, 821.0, 1619.0, 3203.0, 6421.0, 12841.0] {
        let x = xp(t);
        s.push_str(&format!("<line x1=\"{x:.1}\" y1=\"60\" x2=\"{x:.1}\" y2=\"380\" stroke=\"#eee\"/><text x=\"{x:.1}\" y=\"396\" text-anchor=\"middle\" fill=\"#444\">{t}</text>\n"));
    }
    s.push_str("<line x1=\"80\" y1=\"380\" x2=\"640\" y2=\"380\" stroke=\"#999\"/><line x1=\"80\" y1=\"60\" x2=\"80\" y2=\"380\" stroke=\"#999\"/>\n");
    s.push_str("<text x=\"360\" y=\"416\" text-anchor=\"middle\">field size p</text><text x=\"22\" y=\"220\" text-anchor=\"middle\" transform=\"rotate(-90 22 220)\">group operations</text>\n");
    let series: [(&str, &str, &str, &str); 6] = [
        ("rho_e1", "#2c3e50", "none", "matched rho on E1"),
        ("ic_jc", "#c0392b", "none", "IC on J(C), full relation set"),
        (
            "ic_e1_fibre_full",
            "#e67e22",
            "5,3",
            "IC on E1 over B, 1-term (fibre), same stop rule",
        ),
        (
            "ic_e1_fibre_first_collision",
            "#27ae60",
            "2,3",
            "same, stopped at the first collision",
        ),
        (
            "ic_e1_two_term",
            "#8e44ad",
            "none",
            "IC on E1 over B_r, generic 2-term",
        ),
        (
            "birthday_e1_first_collision",
            "#7f8c8d",
            "1,2",
            "control: birthday on E1, no cover",
        ),
    ];
    s.push_str(
        "<rect x=\"96\" y=\"64\" width=\"330\" height=\"112\" fill=\"#fff\" stroke=\"#ccc\"/>\n",
    );
    for (i, (key, color, dash, label)) in series.iter().enumerate() {
        let mut pts = vec![];
        for (j, r) in rows.iter().enumerate() {
            if let Some(v) = r["median_ops"][key].as_f64() {
                pts.push((ps[j], v));
            }
        }
        let path: Vec<String> = pts
            .iter()
            .map(|(p, v)| format!("{:.1},{:.1}", xp(*p), yp(*v)))
            .collect();
        s.push_str(&format!("<polyline fill=\"none\" stroke=\"{color}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\" points=\"{}\"/>\n", path.join(" ")));
        for q in &path {
            let (x, y) = q.split_once(',').unwrap();
            s.push_str(&format!(
                "<circle cx=\"{x}\" cy=\"{y}\" r=\"3.5\" fill=\"{color}\"/>"
            ));
        }
        let ly = 80 + 17 * i;
        s.push_str(&format!("\n<line x1=\"104\" y1=\"{ly}\" x2=\"128\" y2=\"{ly}\" stroke=\"{color}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\"/><text x=\"134\" y=\"{}\">{label}</text>\n", ly + 4));
    }
    s.push_str("<text x=\"80\" y=\"446\" font-weight=\"bold\">Target-side factor base B = phi1(degree-1 places of C)</text>\n");
    s.push_str("<text x=\"80\" y=\"464\" fill=\"#333\">p</text><text x=\"150\" y=\"464\" fill=\"#333\">|B|/#E1</text><text x=\"240\" y=\"464\" fill=\"#333\">|B cap subgroup|/r</text><text x=\"380\" y=\"464\" fill=\"#333\">places per image (2 : 1)</text><text x=\"540\" y=\"464\" fill=\"#333\">pushed rows valid E1 / E2</text>\n");
    for (j, r) in rows.iter().enumerate() {
        let y = 482 + 16 * j;
        if y > 590 {
            break;
        }
        s.push_str(&format!(
            "<text x=\"80\" y=\"{y}\">{}</text><text x=\"150\" y=\"{y}\">{:.4}</text><text x=\"240\" y=\"{y}\">{:.4}</text><text x=\"380\" y=\"{y}\">{} : {}</text><text x=\"540\" y=\"{y}\">{} / {}</text>\n",
            r["p"], r["b_over_e1"].as_f64().unwrap_or(0.0), r["b_sub_over_r"].as_f64().unwrap_or(0.0),
            r["fibre_hist"]["2"], r["fibre_hist"]["1"], r["pushed_e1_valid_frac"].as_str().unwrap_or(""), r["pushed_e2_valid_frac"].as_str().unwrap_or("")
        ));
    }
    s.push_str("</svg>\n");
    s
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let arg = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let primes: Vec<u64> = arg("--primes")
        .map(|s| s.split(',').filter_map(|t| t.trim().parse().ok()).collect())
        .unwrap_or_else(|| vec![211, 419, 821, 1619, 3203, 6421, 12841]);
    let nseeds: usize = arg("--seeds").and_then(|s| s.parse().ok()).unwrap_or(3);

    let mut rows: Vec<Value> = vec![];
    println!(
        "{:>6} {:>5} {:>7} {:>7} | {:>6} {:>7} | {:>6} {:>6} | {:>8} {:>9} {:>9} {:>9} {:>9} {:>9}",
        "p",
        "r",
        "#E1",
        "|B|",
        "B/E1",
        "Bsub/r",
        "E1ok",
        "E2ok",
        "rho",
        "ic_jc",
        "e1_fibre",
        "e1_1stcol",
        "e1_2term",
        "jc/rho"
    );
    for (i, &p) in primes.iter().enumerate() {
        let Some(ci) = select_curve(p, i) else {
            eprintln!("no admissible curve at p = {p}");
            continue;
        };
        let ge = ci.gen;
        let bs: BStructure = b_structure(&ci.b);
        let (sub, _build) = subgroup_points(&ci.b.e1, &ge, ci.r);
        let b_sub = b_in_subgroup(&ci.b, &sub);
        drop(sub);
        let (ec_ns, jac_ns) = op_cost_ratio(&ci, &ge);
        let mut seeds_json = vec![];
        let mut med: BTreeMap<&'static str, Vec<f64>> = BTreeMap::new();
        let mut med_w1: BTreeMap<&'static str, Vec<f64>> = BTreeMap::new();
        let mut med_abs: BTreeMap<&'static str, Vec<f64>> = BTreeMap::new();
        let mut agg = PushAgg::default();
        for k in 0..nseeds {
            let seed = ci.seed0 ^ (k as u64 * 0x1000);
            let (v, ratios) = measure_seed(&ci, seed, &ge, k == 0 && p <= 6421);
            agg.add(&v);
            for (name, r0, r1) in &ratios {
                med.entry(name).or_default().push(*r0);
                med_w1.entry(name).or_default().push(*r1);
                let ops = v["arms"][name]["group_ops"].as_f64().unwrap();
                med_abs.entry(name).or_default().push(ops);
            }
            seeds_json.push(v);
        }
        let mut median_ratio = serde_json::Map::new();
        let mut median_ratio_w1 = serde_json::Map::new();
        let mut median_ops = serde_json::Map::new();
        for (name, v) in med.iter_mut() {
            median_ratio.insert(name.to_string(), json!(median(v)));
        }
        for (name, v) in med_w1.iter_mut() {
            median_ratio_w1.insert(name.to_string(), json!(median(v)));
        }
        for (name, v) in med_abs.iter_mut() {
            median_ops.insert(name.to_string(), json!(median(v)));
        }
        let s0 = &seeds_json[0];
        let row = json!({
            "p": p,
            "candidate": ci.candidate,
            "coeffs": ci.coeffs,
            "seed0": ci.seed0,
            "r": ci.r,
            "e1_order": bs.e1_order,
            "e2_order": ci.b.e2.order(),
            "candidates_skipped_by_reason": ci.skipped,
            "transfer_one_time_ms": ci.transfer_ms,
            "b": serde_json::to_value(&bs).unwrap(),
            "b_over_e1": bs.b_size as f64 / bs.e1_order as f64,
            "b_over_r": bs.b_size as f64 / ci.r as f64,
            "b_in_subgroup": b_sub,
            "b_sub_over_r": b_sub as f64 / ci.r as f64,
            "fibre_hist": bs.fibre_hist.iter().map(|(k, v)| (k.to_string(), *v)).collect::<BTreeMap<_, _>>(),
            "ec_add_ns_unisolated": ec_ns,
            "cantor_add_ns_unisolated": jac_ns,
            "cantor_over_ec_wall_ratio_unisolated": jac_ns / ec_ns,
            "pushed_totals": serde_json::to_value(&agg).unwrap(),
            "pushed_e1_valid_frac": format!("{}/{}", agg.e1_valid, agg.relations),
            "pushed_e2_valid_frac": format!("{}/{}", agg.e2_valid, agg.relations),
            "median_ratio_to_rho_w0": median_ratio,
            "median_ratio_to_rho_w1": median_ratio_w1,
            "median_ops": median_ops,
            "seeds": seeds_json,
        });
        let g = |k: &str| row["median_ops"][k].as_f64().unwrap_or(f64::NAN);
        println!(
            "{:>6} {:>5} {:>7} {:>7} | {:>6.4} {:>7.4} | {:>6} {:>6} | {:>8.0} {:>9.0} {:>9.0} {:>9.0} {:>9.0} {:>8.1}x",
            p, ci.r, bs.e1_order, bs.b_size,
            bs.b_size as f64 / bs.e1_order as f64, b_sub as f64 / ci.r as f64,
            agg.e1_valid, agg.e2_valid,
            g("rho_e1"), g("ic_jc"), g("ic_e1_fibre_full"),
            g("ic_e1_fibre_first_collision"), g("ic_e1_two_term"),
            g("ic_jc") / g("rho_e1")
        );
        let _ = s0;
        rows.push(row);
    }

    if let Some(path) = arg("--svg") {
        std::fs::write(&path, render_svg(&rows)).expect("write svg");
        eprintln!("wrote {path}");
    }
    if let Some(path) = arg("--json") {
        let j = json!({
            "schema": "cover.pushforward/v1",
            "protocol": "research/cover_pushforward_20261112/PROTOCOL.md",
            "unit": "counted group operations (1 GAE each); LA multiplications priced at w in {0, 0.1, 1}",
            "transfer_cost": "one-time, excluded from every per-instance figure",
            "rows": rows,
        });
        std::fs::write(&path, serde_json::to_string_pretty(&j).unwrap()).expect("write json");
        eprintln!("wrote {path}");
    }
    ExitCode::SUCCESS
}

#[derive(Default, serde::Serialize)]
struct PushAgg {
    relations: u64,
    e1_valid: u64,
    e1_invalid: u64,
    e2_valid: u64,
    e2_invalid: u64,
    e2_rows_carrying_target: u64,
    e1_rows_carrying_target: u64,
    fibre_type_e1: u64,
    fibre_type_e2: u64,
    push_mismatch: u64,
}

impl PushAgg {
    fn add(&mut self, v: &Value) {
        let g = |k: &str| v["pushed"][k].as_u64().unwrap_or(0);
        self.relations += g("relations");
        self.e1_valid += g("e1_valid");
        self.e1_invalid += g("e1_invalid");
        self.e2_valid += g("e2_valid");
        self.e2_invalid += g("e2_invalid");
        self.e2_rows_carrying_target += g("e2_rows_carrying_target");
        self.e1_rows_carrying_target += g("e1_rows_carrying_target");
        self.fibre_type_e1 += g("fibre_type_e1");
        self.fibre_type_e2 += g("fibre_type_e2");
        self.push_mismatch += g("e1_push_mismatch") + g("e2_push_mismatch");
    }
}
