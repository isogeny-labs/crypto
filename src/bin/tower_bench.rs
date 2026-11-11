//! `tower_bench`: carry one elliptic discrete log up the cover tower
//! (genus 1 → genus 2 → genus 3) and measure the complete solve at each rung.
//!
//! ```text
//! tower_bench run [--json out.json] [--svg out.svg]
//! ```
//!
//! For each prime `p < 256` it builds a target `E/F_p` with an order-`r`
//! instance, its genus-2 and genus-3 covers ([`tower`]), verifies the
//! pull-back on every multiple whose preimages are rational, and reports
//! counted group operations, averaged over seeds, for:
//! rho on `E`; rho on the order-`r` subgroup of `J(C₂)` and `J(C₃)`; and the
//! repository's counted genus-g index calculus (`ecbench::hyper_ic`, full
//! degree-1 factor base) on `J(C₂)` and `J(C₃)`.  Every solve is checked
//! against the planted log.  All figures are `[pcc:unverified]`.

use std::process::ExitCode;

use crypto_lib::cryptanalysis::chained_transfer::measure::EcGroup;
use crypto_lib::cryptanalysis::chained_transfer::tower::{build, TowerInstance};
use crypto_lib::cryptanalysis::ecbench::hyper_ic::{solve, HyperIcParams};
use crypto_lib::cryptanalysis::ic_boundary::rho_reference;

#[derive(serde::Serialize, Clone)]
struct Row {
    p: u64,
    r: u64,
    seeds: u32,
    hom_checks_g2: u32,
    hom_checks_g3: u32,
    rho_e: f64,
    rho_j2: f64,
    rho_j3: f64,
    ic_j2: f64,
    ic_j3: f64,
    ic_j2_linalg_mulmods: f64,
    ic_j3_linalg_mulmods: f64,
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

fn measure(t: &TowerInstance, seeds: u32) -> Row {
    let eg = EcGroup(t.e);
    let mut sums = [0f64; 7];
    let mut ok = true;
    for s in 0..seeds as u64 {
        let rho = rho_reference(&eg, t.gen, t.target, t.r, 100 + s, 1 << 24);
        ok &= rho.recovered == Some(t.x);
        sums[0] += (rho.group_ops.adds + rho.group_ops.doubles) as f64;
        for (i, rg) in t.rungs.iter().enumerate() {
            let cur = &rg.cover.curve;
            let rj = rho_reference(cur, rg.d, rg.t, t.r, 200 + s, 1 << 24);
            ok &= rj.recovered == Some(t.x);
            sums[1 + i] += (rj.group_ops.adds + rj.group_ops.doubles) as f64;
            match solve(cur, rg.d, rg.t, t.r, rg.jac_order, 300 + s, &params()) {
                Ok(out) => {
                    ok &= out.recovered == Some(t.x);
                    let ops: u64 = out
                        .phases
                        .iter()
                        .map(|(_, g, _, _)| g.adds + g.doubles)
                        .sum();
                    sums[3 + i] += ops as f64;
                    let la: u64 = out
                        .phases
                        .iter()
                        .flat_map(|(_, _, c, _)| c.iter())
                        .filter(|(k, _)| k.starts_with("linalg_mulmods"))
                        .map(|(_, v)| *v)
                        .sum();
                    sums[5 + i] += la as f64;
                }
                Err(e) => {
                    eprintln!("hyper_ic p={} genus={}: {e}", t.p, rg.cover.genus);
                    ok = false;
                }
            }
        }
    }
    let n = seeds as f64;
    Row {
        p: t.p,
        r: t.r,
        seeds,
        hom_checks_g2: t.rungs[0].hom_checks,
        hom_checks_g3: t.rungs[1].hom_checks,
        rho_e: sums[0] / n,
        rho_j2: sums[1] / n,
        rho_j3: sums[2] / n,
        ic_j2: sums[3] / n,
        ic_j3: sums[4] / n,
        ic_j2_linalg_mulmods: sums[5] / n,
        ic_j3_linalg_mulmods: sums[6] / n,
        all_recovered: ok,
    }
}

fn render_svg(rows: &[Row]) -> String {
    let (x0, x1) = (40f64.ln(), 300f64.ln());
    let (y0, y1) = (3f64.ln(), 300_000f64.ln());
    let xp = |p: f64| 80.0 + (p.ln() - x0) / (x1 - x0) * 520.0;
    let yp = |v: f64| 360.0 - (v.max(3.0).ln() - y0) / (y1 - y0) * 300.0;
    let mut s = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"640\" height=\"420\" viewBox=\"0 0 640 420\" font-family=\"Helvetica,Arial,sans-serif\" font-size=\"12\">\n<rect width=\"640\" height=\"420\" fill=\"#fff\"/>\n");
    s.push_str("<text x=\"320\" y=\"24\" text-anchor=\"middle\" font-size=\"15\" font-weight=\"bold\">Tower up: one elliptic log solved at genus 1, 2, 3</text>\n");
    s.push_str("<text x=\"320\" y=\"42\" text-anchor=\"middle\" fill=\"#555\">group operations only (IC linear algebra excluded, priced in TOWER.md); mean of 8 seeds; [pcc:unverified]</text>\n");
    for t in [10.0, 100.0, 1000.0, 10000.0, 100000.0] {
        let y = yp(t);
        s.push_str(&format!("<line x1=\"80\" y1=\"{y:.1}\" x2=\"600\" y2=\"{y:.1}\" stroke=\"#e5e5e5\"/><text x=\"74\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#444\">{t}</text>\n", y + 4.0));
    }
    for t in [50.0, 100.0, 200.0] {
        let x = xp(t);
        s.push_str(&format!("<line x1=\"{x:.1}\" y1=\"60\" x2=\"{x:.1}\" y2=\"360\" stroke=\"#e5e5e5\"/><text x=\"{x:.1}\" y=\"376\" text-anchor=\"middle\" fill=\"#444\">{t}</text>\n"));
    }
    s.push_str("<text x=\"340\" y=\"400\" text-anchor=\"middle\">field size p</text><text x=\"22\" y=\"210\" text-anchor=\"middle\" transform=\"rotate(-90 22 210)\">group operations</text>\n");
    let series: [(&str, &str, &str, fn(&Row) -> f64); 5] = [
        ("#8e44ad", "none", "IC on J(C₃), genus 3", |r| r.ic_j3),
        ("#c0392b", "none", "IC on J(C₂), genus 2", |r| r.ic_j2),
        ("#2c3e50", "none", "rho on E (matched reference)", |r| {
            r.rho_e
        }),
        ("#27ae60", "5,3", "rho on J(C₂) r-subgroup", |r| r.rho_j2),
        ("#e67e22", "2,3", "rho on J(C₃) r-subgroup", |r| r.rho_j3),
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
    let seeds = 8;
    let mut rows = vec![];
    for (i, p) in [53u64, 79, 101, 131, 151, 181, 211, 251]
        .into_iter()
        .enumerate()
    {
        match build(p, 11 + i as u64) {
            Some(t) => rows.push(measure(&t, seeds)),
            None => eprintln!("no tower instance at p = {p}"),
        }
    }
    println!(
        "{:>5} {:>5} {:>6} {:>8} {:>8} {:>8} {:>10} {:>10} {:>8} {:>8}  ok",
        "p", "r", "hom2/3", "rho_E", "rho_J2", "rho_J3", "IC_J2", "IC_J3", "IC2/rhoE", "IC3/rhoE"
    );
    for r in &rows {
        println!(
            "{:>5} {:>5} {:>3}/{:<3} {:>8.1} {:>8.1} {:>8.1} {:>10.1} {:>10.1} {:>7.1}x {:>7.1}x  {}",
            r.p, r.r, r.hom_checks_g2, r.hom_checks_g3, r.rho_e, r.rho_j2, r.rho_j3, r.ic_j2, r.ic_j3,
            r.ic_j2 / r.rho_e, r.ic_j3 / r.rho_e, r.all_recovered
        );
    }
    if let Some(path) = arg("--svg") {
        std::fs::write(path, render_svg(&rows)).expect("write svg");
    }
    if let Some(path) = arg("--json") {
        std::fs::write(path, serde_json::to_string_pretty(&rows).unwrap()).expect("write json");
    }
    if rows.iter().all(|r| r.all_recovered) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
