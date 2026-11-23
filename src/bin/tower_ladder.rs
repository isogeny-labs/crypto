//! `tower_ladder`: one elliptic discrete log solved at genus 1, 2, 3 and 4 with
//! the same counted index-calculus instrument.
//!
//! ```text
//! tower_ladder [--json out.json] [--svg out.svg] [--md tables.md]
//! ```
//!
//! Rungs 1-3 are `E` itself and the two degree-2 covers of `chained_transfer::tower`.
//! Rung 4 is a degree-3 cover built here: with `c(s) = s³ + a2 s² + a4 s + a6`
//! and `h(x) = x³ + h1 x + h0`, the curve `C₄ : y² = c(h(x))` (degree 9, genus 4)
//! maps to `E : Y² = c(s)` by `(x, y) ↦ (h(x), y)`.  The pull-back of an affine
//! `Q = (s, Y)` is the Mumford pair `u = h(x) − s`, `v = Y`; the pull-back of
//! `O` is the identity.  The homomorphism is checked on every multiple `k < r`.
//! Every solve is checked against the planted log.  All figures are
//! `[pcc:unverified]`.

use std::process::ExitCode;

use crypto_lib::cryptanalysis::chained_transfer::ec::{Curve, Pt};
use crypto_lib::cryptanalysis::chained_transfer::field::Fp;
use crypto_lib::cryptanalysis::chained_transfer::measure::EcGroup;
use crypto_lib::cryptanalysis::chained_transfer::tower::{build, TowerInstance};
use crypto_lib::cryptanalysis::ecbench::hyper_ic::{solve, HyperIcParams};
use crypto_lib::cryptanalysis::hyperelliptic_word::{HCurve, HDiv};
use crypto_lib::cryptanalysis::ic_boundary::rho_reference;

const PRIMES: [u64; 8] = [53, 79, 101, 131, 151, 181, 211, 251];
const SEEDS: u32 = 8;

#[derive(serde::Serialize, Clone)]
struct Row {
    p: u64,
    r: u64,
    n_e: u64,
    jac_order: [u64; 4],
    h1: u64,
    h0: u64,
    h_attempts: u32,
    hom_checks: [u32; 4],
    seeds: u32,
    rho_e: f64,
    /// Group operations (adds + doubles) of `hyper_ic`, genus 1..4.
    ic: [f64; 4],
    /// Unpriced `linalg_mulmods*` counters of `hyper_ic`, genus 1..4.
    la: [f64; 4],
    all_recovered: bool,
}

fn params() -> HyperIcParams {
    HyperIcParams {
        fb_size: usize::MAX,
        large_primes: false,
        extra_relations: 8,
        branches: 64,
        d_step_in: 4,
        max_trials: 5_000_000,
    }
}

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn pmul(k: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = k.add(c[i + j], k.mul(x, y));
        }
    }
    c
}

/// `f = c(h(x))` lowest degree first (length 10).
fn cover4_f(k: &Fp, e: &Curve, h1: u64, h0: u64) -> Vec<u64> {
    let h = [h0, h1, 0, 1];
    let h2 = pmul(k, &h, &h);
    let h3 = pmul(k, &h2, &h);
    let mut f = vec![0u64; 10];
    for (i, &v) in h3.iter().enumerate() {
        f[i] = k.add(f[i], v);
    }
    for (i, &v) in h2.iter().enumerate() {
        f[i] = k.add(f[i], k.mul(e.a2, v));
    }
    for (i, &v) in h.iter().enumerate() {
        f[i] = k.add(f[i], k.mul(e.a4, v));
    }
    f[0] = k.add(f[0], e.a6);
    f
}

fn pull4(k: &Fp, h1: u64, h0: u64, pt: &Pt) -> HDiv {
    match pt {
        None => HDiv::IDENTITY,
        Some((s, y)) => {
            let mut d = HDiv {
                deg: 3,
                ..HDiv::IDENTITY
            };
            d.u[0] = k.sub(h0, s.a) as u8;
            d.u[1] = h1 as u8;
            d.v[0] = y.a as u8;
            d
        }
    }
}

struct Genus4 {
    curve: HCurve,
    h1: u64,
    h0: u64,
    jac_order: u64,
    attempts: u32,
    hom_checks: u32,
    d: HDiv,
    t: HDiv,
}

/// Search `h` until `r` divides `#J(C₄)` exactly once, then check the pull-back
/// homomorphism on every `k` in `1..r`.
fn build_genus4(t: &TowerInstance, seed: u64) -> Result<Genus4, String> {
    let k = t.e.k;
    for attempt in 0..300u32 {
        let z = mix(seed.wrapping_mul(7919).wrapping_add(u64::from(attempt)));
        let (h1, h0) = (z % t.p, (z >> 24) % t.p);
        let f = cover4_f(&k, &t.e, h1, h0);
        let Ok(curve) = HCurve::new(t.p, 4, &f) else {
            continue;
        };
        let Ok(jo) = curve.jacobian_order(t.r, seed ^ u64::from(attempt)) else {
            continue;
        };
        if !jo.is_multiple_of(t.r) || (jo / t.r).is_multiple_of(t.r) {
            continue;
        }
        let d = pull4(&k, h1, h0, &t.gen);
        if !curve.is_valid(&d) || d.is_identity() || !curve.mul_uncounted(d, t.r).is_identity() {
            return Err("pull-back of the generator is invalid or not of order r".into());
        }
        let mut checks = 0;
        for kk in 1..t.r {
            let q = t.e.mul(&t.gen, u128::from(kk));
            let dq = pull4(&k, h1, h0, &q);
            if !curve.is_valid(&dq) {
                return Err(format!("pull-back of [{kk}]G is not a valid divisor"));
            }
            if dq != curve.mul_uncounted(d, kk) {
                return Err(format!("homomorphism fails at k = {kk}"));
            }
            checks += 1;
        }
        let tt = pull4(&k, h1, h0, &t.target);
        if tt != curve.mul_uncounted(d, t.x) {
            return Err("target pull-back is not [x]D".into());
        }
        return Ok(Genus4 {
            curve,
            h1,
            h0,
            jac_order: jo,
            attempts: attempt + 1,
            hom_checks: checks,
            d,
            t: tt,
        });
    }
    Err("no h with r || #J4 in 300 attempts".into())
}

fn ic_cost(
    cur: &HCurve,
    d: HDiv,
    tt: HDiv,
    t: &TowerInstance,
    order: u64,
    seed: u64,
    ok: &mut bool,
) -> (f64, f64) {
    match solve(cur, d, tt, t.r, order, seed, &params()) {
        Ok(out) => {
            *ok &= out.recovered == Some(t.x);
            let ops: u64 = out
                .phases
                .iter()
                .map(|(_, g, _, _)| g.adds + g.doubles)
                .sum();
            let la: u64 = out
                .phases
                .iter()
                .flat_map(|(_, _, c, _)| c.iter())
                .filter(|(k, _)| k.starts_with("linalg_mulmods"))
                .map(|(_, v)| *v)
                .sum();
            (ops as f64, la as f64)
        }
        Err(e) => {
            eprintln!("hyper_ic p={} genus={}: {e}", t.p, cur.genus());
            *ok = false;
            (0.0, 0.0)
        }
    }
}

fn run_prime(p: u64, i: usize) -> Result<Row, String> {
    let t = build(p, 11 + i as u64).ok_or("tower::build returned None")?;
    let g4 = build_genus4(&t, 1000 + i as u64)?;
    // Rung 1: E itself as an HCurve.
    let k = t.e.k;
    let c1 = HCurve::new(p, 1, &[t.e.a6, t.e.a4, t.e.a2, 1])?;
    let (gx, gy) = t.gen.ok_or("generator is O")?;
    let (tx, ty) = t.target.ok_or("target is O")?;
    let d1 = c1.point(gx.a, gy.a).ok_or("generator not on HCurve")?;
    let t1 = c1.point(tx.a, ty.a).ok_or("target not on HCurve")?;
    if t1 != c1.mul_uncounted(d1, t.x) {
        return Err("genus-1 target != [x]D".into());
    }
    let _ = k;
    let n_e = t.e.order();
    let eg = EcGroup(t.e);
    let rungs: [(&HCurve, HDiv, HDiv, u64); 4] = [
        (&c1, d1, t1, n_e),
        (
            &t.rungs[0].cover.curve,
            t.rungs[0].d,
            t.rungs[0].t,
            t.rungs[0].jac_order,
        ),
        (
            &t.rungs[1].cover.curve,
            t.rungs[1].d,
            t.rungs[1].t,
            t.rungs[1].jac_order,
        ),
        (&g4.curve, g4.d, g4.t, g4.jac_order),
    ];
    let mut ok = true;
    let (mut rho, mut ic, mut la) = (0f64, [0f64; 4], [0f64; 4]);
    for s in 0..u64::from(SEEDS) {
        let r = rho_reference(&eg, t.gen, t.target, t.r, 100 + s, 1 << 24);
        ok &= r.recovered == Some(t.x);
        rho += (r.group_ops.adds + r.group_ops.doubles) as f64;
        for (g, (cur, d, tt, order)) in rungs.iter().enumerate() {
            let (o, l) = ic_cost(cur, *d, *tt, &t, *order, 300 + s, &mut ok);
            ic[g] += o;
            la[g] += l;
        }
    }
    let n = f64::from(SEEDS);
    Ok(Row {
        p,
        r: t.r,
        n_e,
        jac_order: [n_e, rungs[1].3, rungs[2].3, g4.jac_order],
        h1: g4.h1,
        h0: g4.h0,
        h_attempts: g4.attempts,
        hom_checks: [
            t.r as u32 - 1,
            t.rungs[0].hom_checks,
            t.rungs[1].hom_checks,
            g4.hom_checks,
        ],
        seeds: SEEDS,
        rho_e: rho / n,
        ic: ic.map(|v| v / n),
        la: la.map(|v| v / n),
        all_recovered: ok,
    })
}

/// Least-squares slope of `ln y` on `ln p`.
fn slope(rows: &[Row], f: impl Fn(&Row) -> f64) -> f64 {
    let n = rows.len() as f64;
    let xs: Vec<f64> = rows.iter().map(|r| (r.p as f64).ln()).collect();
    let ys: Vec<f64> = rows.iter().map(|r| f(r).max(1e-9).ln()).collect();
    let (mx, my) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
    let sxy: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    sxy / sxx
}

fn tables(rows: &[Row]) -> String {
    let mut s = String::new();
    s.push_str("Counted group operations (mean of 8 seeds; IC excludes linear algebra):\n\n");
    s.push_str(
        "| p | r | rho_E | IC1 | IC2 | IC3 | IC4 | ok |\n|--:|--:|--:|--:|--:|--:|--:|:-:|\n",
    );
    for r in rows {
        s.push_str(&format!(
            "| {} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {} |\n",
            r.p, r.r, r.rho_e, r.ic[0], r.ic[1], r.ic[2], r.ic[3], r.all_recovered
        ));
    }
    s.push_str(
        "\nUnpriced linear-algebra multiplications `linalg_mulmods*` (mean of 8 seeds):\n\n",
    );
    s.push_str("| p | LA1 | LA2 | LA3 | LA4 |\n|--:|--:|--:|--:|--:|\n");
    for r in rows {
        s.push_str(&format!(
            "| {} | {:.0} | {:.0} | {:.0} | {:.0} |\n",
            r.p, r.la[0], r.la[1], r.la[2], r.la[3]
        ));
    }
    s.push_str(
        "\nPer-rung ratios of group operations (smoothness-probability prediction about g):\n\n",
    );
    s.push_str("| p | IC2/IC1 | IC3/IC2 | IC4/IC3 | IC4/IC1 |\n|--:|--:|--:|--:|--:|\n");
    for r in rows {
        s.push_str(&format!(
            "| {} | {:.2} | {:.2} | {:.2} | {:.2} |\n",
            r.p,
            r.ic[1] / r.ic[0],
            r.ic[2] / r.ic[1],
            r.ic[3] / r.ic[2],
            r.ic[3] / r.ic[0]
        ));
    }
    let n = rows.len() as f64;
    let mean = |f: &dyn Fn(&Row) -> f64| rows.iter().map(f).sum::<f64>() / n;
    s.push_str(&format!(
        "| mean | {:.2} | {:.2} | {:.2} | {:.2} |\n",
        mean(&|r| r.ic[1] / r.ic[0]),
        mean(&|r| r.ic[2] / r.ic[1]),
        mean(&|r| r.ic[3] / r.ic[2]),
        mean(&|r| r.ic[3] / r.ic[0])
    ));
    s.push_str("\nLeast-squares log-log slope against p (8 points, no interval):\n\n| series | slope |\n|--|--:|\n");
    s.push_str(&format!("| rho on E | {:.2} |\n", slope(rows, |r| r.rho_e)));
    for g in 0..4 {
        s.push_str(&format!(
            "| IC{} group ops | {:.2} |\n",
            g + 1,
            slope(rows, |r| r.ic[g])
        ));
    }
    for g in 0..4 {
        s.push_str(&format!(
            "| IC{} linalg mulmods | {:.2} |\n",
            g + 1,
            slope(rows, |r| r.la[g])
        ));
    }
    for g in 0..4 {
        s.push_str(&format!(
            "| IC{} ops + 1.0 x linalg | {:.2} |\n",
            g + 1,
            slope(rows, |r| r.ic[g] + r.la[g])
        ));
    }
    s.push_str("\nIC_g / rho_E with each linear-algebra multiplication priced at w GAE (w = 0, 0.1, 1):\n\n");
    s.push_str("| p | g=1 @0 | @0.1 | @1 | g=2 @0 | @0.1 | @1 | g=3 @0 | @0.1 | @1 | g=4 @0 | @0.1 | @1 |\n");
    s.push_str("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|\n");
    for r in rows {
        s.push_str(&format!("| {} ", r.p));
        for g in 0..4 {
            for w in [0.0, 0.1, 1.0] {
                s.push_str(&format!("| {:.2} ", (r.ic[g] + w * r.la[g]) / r.rho_e));
            }
        }
        s.push_str("|\n");
    }
    s.push_str("\nGenus-4 cover data:\n\n| p | #E | #J4 | h1 | h0 | h attempts | hom checks g1/g2/g3/g4 |\n|--:|--:|--:|--:|--:|--:|--|\n");
    for r in rows {
        s.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {}/{}/{}/{} |\n",
            r.p,
            r.n_e,
            r.jac_order[3],
            r.h1,
            r.h0,
            r.h_attempts,
            r.hom_checks[0],
            r.hom_checks[1],
            r.hom_checks[2],
            r.hom_checks[3]
        ));
    }
    s
}

fn render_svg(rows: &[Row]) -> String {
    let (x0, x1) = (40f64.ln(), 300f64.ln());
    let (y0, y1) = (3f64.ln(), 3_000_000f64.ln());
    let xp = |p: f64| 80.0 + (p.ln() - x0) / (x1 - x0) * 520.0;
    let yp = |v: f64| 360.0 - (v.max(3.0).ln() - y0) / (y1 - y0) * 300.0;
    let mut s = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"640\" height=\"420\" viewBox=\"0 0 640 420\" font-family=\"Helvetica,Arial,sans-serif\" font-size=\"12\">\n<rect width=\"640\" height=\"420\" fill=\"#fff\"/>\n");
    s.push_str("<text x=\"320\" y=\"24\" text-anchor=\"middle\" font-size=\"15\" font-weight=\"bold\">Tower ladder: one elliptic log solved at genus 1, 2, 3, 4</text>\n");
    s.push_str("<text x=\"320\" y=\"42\" text-anchor=\"middle\" fill=\"#555\">IC group ops exclude linear algebra; mean of 8 seeds; [pcc:unverified]</text>\n");
    for t in [10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0] {
        let y = yp(t);
        s.push_str(&format!("<line x1=\"80\" y1=\"{y:.1}\" x2=\"600\" y2=\"{y:.1}\" stroke=\"#e5e5e5\"/><text x=\"74\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#444\">{t}</text>\n", y + 4.0));
    }
    for t in [50.0, 100.0, 200.0] {
        let x = xp(t);
        s.push_str(&format!("<line x1=\"{x:.1}\" y1=\"60\" x2=\"{x:.1}\" y2=\"360\" stroke=\"#e5e5e5\"/><text x=\"{x:.1}\" y=\"376\" text-anchor=\"middle\" fill=\"#444\">{t}</text>\n"));
    }
    s.push_str("<text x=\"340\" y=\"400\" text-anchor=\"middle\">field size p</text><text x=\"22\" y=\"210\" text-anchor=\"middle\" transform=\"rotate(-90 22 210)\">group operations</text>\n");
    let series: [(&str, &str, &str, fn(&Row) -> f64); 5] = [
        ("#8e44ad", "none", "IC genus 4", |r| r.ic[3]),
        ("#e67e22", "none", "IC genus 3", |r| r.ic[2]),
        ("#c0392b", "none", "IC genus 2", |r| r.ic[1]),
        ("#2980b9", "none", "IC genus 1 (E itself)", |r| r.ic[0]),
        ("#2c3e50", "5,3", "rho on E (matched reference)", |r| {
            r.rho_e
        }),
    ];
    s.push_str(
        "<rect x=\"96\" y=\"64\" width=\"250\" height=\"98\" fill=\"#fff\" stroke=\"#ccc\"/>\n",
    );
    for (i, (col, dash, label, f)) in series.iter().enumerate() {
        let pts: Vec<String> = rows
            .iter()
            .map(|r| format!("{:.1},{:.1}", xp(r.p as f64), yp(f(r))))
            .collect();
        s.push_str(&format!("<polyline fill=\"none\" stroke=\"{col}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\" points=\"{}\"/>\n", pts.join(" ")));
        for pt in &pts {
            let (x, y) = pt.split_once(',').unwrap();
            s.push_str(&format!(
                "<circle cx=\"{x}\" cy=\"{y}\" r=\"3.2\" fill=\"{col}\"/>"
            ));
        }
        let ly = 78 + 18 * i;
        s.push_str(&format!("\n<line x1=\"104\" y1=\"{ly}\" x2=\"128\" y2=\"{ly}\" stroke=\"{col}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\"/><text x=\"134\" y=\"{}\">{label}</text>\n", ly + 4));
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
    };
    let results: Vec<(u64, Result<Row, String>)> = std::thread::scope(|sc| {
        let hs: Vec<_> = PRIMES
            .iter()
            .enumerate()
            .map(|(i, &p)| (p, sc.spawn(move || run_prime(p, i))))
            .collect();
        hs.into_iter()
            .map(|(p, h)| (p, h.join().unwrap_or_else(|_| Err("panic".into()))))
            .collect()
    });
    let mut rows = vec![];
    for (p, r) in results {
        match r {
            Ok(row) => rows.push(row),
            Err(e) => eprintln!("DROPPED p = {p}: {e}"),
        }
    }
    let tab = tables(&rows);
    println!("{tab}");
    println!(
        "primes kept: {:?}",
        rows.iter().map(|r| r.p).collect::<Vec<_>>()
    );
    if let Some(path) = arg("--svg") {
        std::fs::write(path, render_svg(&rows)).expect("write svg");
    }
    if let Some(path) = arg("--json") {
        std::fs::write(path, serde_json::to_string_pretty(&rows).unwrap()).expect("write json");
    }
    if let Some(path) = arg("--md") {
        std::fs::write(path, &tab).expect("write md");
    }
    if rows.len() >= 2 && rows.iter().all(|r| r.all_recovered) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
