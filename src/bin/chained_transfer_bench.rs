//! `chained_transfer_bench`: build verified chained ECDLP transfers over a
//! prime field and measure the complete cost of solving the transferred
//! instance against a matched Pollard-rho reference.
//!
//! ```text
//! chained_transfer_bench run                 # table + JSON to stdout
//! chained_transfer_bench run --json out.json # also write JSON
//! ```
//!
//! For each bielliptic curve `C/F_p : y² = F(x²)` with elliptic quotients
//! `E1, E2` it reports, on the order-`r` subgroup of `E1`:
//! - `rho_src`: counted rho on `E1` (the direct cost);
//! - `rho_isog`: counted rho after an isogeny hop `E1 → E1'` (verified);
//! - `rho_jac`: counted rho on the order-`r` subgroup of `J(C)` after the
//!   verified pull-back `φ^* : E1 → J(C)`;
//! - `ic_jac`: counted genus-2 index calculus on the same `J(C)` instance.
//!
//! Every chain is verified to preserve the discrete logarithm before it is
//! measured. Version 3 differs from version 2 in three ways:
//! - the genus-2 instance solved is the real pulled-back pair
//!   `(φ^*(P), φ^*([x]P))`, not a surrogate from `jac_instance` (F5);
//! - the index-calculus elimination is oracle-free: it returns whatever it
//!   finds and the claim is judged afterwards (F4);
//! - every solve claim, for all four arms, is written as a
//!   `chained.solve.certificate/v1` record (`--certs FILE`, JSON lines) and
//!   re-checked in process; `verify_certs` re-checks the file on its own (F14).
//!
//! The transfer (building the cover or isogeny and evaluating `φ^*`) is a
//! one-time cost and is not part of any per-instance figure here.
//! No result here is a verified claim until CI's pcc gate signs it; the
//! numbers below are `[pcc:unverified]` assertions.

use std::process::ExitCode;

use crypto_lib::cryptanalysis::chained_transfer::bielliptic::{Bielliptic, Quotient};
use crypto_lib::cryptanalysis::chained_transfer::certify::{
    self, div_elem, ec_group, jac_group, pt_elem, Cert, Transfer,
};
use crypto_lib::cryptanalysis::chained_transfer::chain::{Chain, Elt, Hop};
use crypto_lib::cryptanalysis::chained_transfer::ec::{Curve, Pt, VeluIsogeny};
use crypto_lib::cryptanalysis::chained_transfer::measure::{
    genus2_index_calculus, rho_on_ec, rho_on_jac,
};
use crypto_lib::cryptanalysis::genus2_fp::Genus2Curve;
use std::collections::BTreeMap;

/// Largest prime `r` with `r² ∤ n` (a usable cyclic subgroup).
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

/// A generator of the order-`r` subgroup of `E` and `[x]gen`.
fn ec_instance(e: &Curve, r: u64, x: u64) -> Option<(Pt, Pt)> {
    let order = e.order();
    if !order.is_multiple_of(r) {
        return None;
    }
    let cof = order / r;
    for s in 1..5000u64 {
        let g = e.mul(&e.point_from_seed(s.wrapping_mul(0x9E37_79B9)), cof as u128);
        if g.is_some() && e.mul(&g, r as u128).is_none() {
            return Some((g, e.mul(&g, x as u128)));
        }
    }
    None
}

/// Find a prime-degree isogeny `E → E'` from a rational `ell`-torsion point.
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

#[derive(serde::Serialize)]
struct Row {
    p: u64,
    r: u64,
    e1_order: u64,
    e2_order: u64,
    jac_order: u64,
    x: u64,
    rho_src_ops: u64,
    rho_isog_ops: u64,
    isog_degree: u64,
    rho_jac_ops: u64,
    ic_jac_group_ops: u64,
    ic_jac_trials: u64,
    ic_jac_relations: u64,
    ic_jac_fb: usize,
    ic_recovered_ok: bool,
    /// Every rho arm recovered the planted log.
    rho_recovered_ok: bool,
    /// Z/r multiplications in the IC's linear algebra (unpriced work).
    ic_jac_la_mulmods: u64,
    chain_verified: bool,
    /// Which instance the genus-2 arms solved: `pullback` is
    /// `(φ^*(P), φ^*([x]P))` itself.
    instance_type: &'static str,
    /// The log the index calculus returned, right or wrong.
    ic_claimed_log: Option<u64>,
    /// Certificates issued for this row (one per arm) and how many passed the
    /// in-process re-check.
    certs_issued: u64,
    certs_verified: u64,
    /// Index of the candidate curve (the search counter) this row came from.
    candidate: u64,
}

/// Why a candidate curve produced no row.
type Skip = &'static str;

fn cert(
    arm: &str,
    instance_type: &str,
    p: u64,
    group: certify::GroupData,
    r: u64,
    (g, t): (certify::Elem, certify::Elem),
    claimed_k: Option<u64>,
    transfer: Option<Transfer>,
    seed: u64,
    solver_ops: u64,
) -> Cert {
    Cert {
        schema: certify::SCHEMA.into(),
        arm: arm.into(),
        instance_type: instance_type.into(),
        p,
        group,
        r,
        g,
        t,
        claimed_k,
        transfer,
        seed,
        solver_ops,
    }
}

/// Measure one candidate curve. `Err` names why it was skipped before any
/// solver ran; once the structural checks pass the row is reported whatever
/// the solvers return (no selection on solver success).
fn measure_curve(p: u64, c: [u64; 4], seed: u64) -> Result<(Row, Vec<Cert>), Skip> {
    let b = Bielliptic::new(p, c).map_err(|e| match e {
        crypto_lib::cryptanalysis::chained_transfer::bielliptic::BiellipticError::Genus2(_) => {
            "bielliptic_rejected_genus2_model"
        }
        _ => "bielliptic_rejected_degenerate_or_no_rational_weierstrass_point",
    })?;
    let e1 = *b.curve(Quotient::E1);
    let e2 = *b.curve(Quotient::E2);
    let jac: Genus2Curve = b.jac.clone();
    let e1_order = e1.order();
    let e2_order = e2.order();
    // #J(C) = #E1 · #E2 via the (2,2)-isogeny (no genus-2 point counting, so p
    // is not capped by the O(p²) counter).
    let jac_order = e1_order * e2_order;
    let r = largest_cyclic_prime(e1_order);
    if r < 50 {
        return Err("r_below_50");
    }
    // The order-r part of J(C) is cyclic iff r² ∤ #J(C) = #E1·#E2. r² ∤ #E1
    // by the choice of r, so the condition is r ∤ #E2.
    if e2_order.is_multiple_of(r) {
        return Err("r_divides_e2_order_so_r_part_of_J_not_cyclic");
    }
    if jac_order.is_multiple_of(r * r) {
        return Err("r_squared_divides_jac_order");
    }
    let x = (seed.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
    let (gen, target) = ec_instance(&e1, r, x).ok_or("no_order_r_point_on_E1")?;
    let src_g = pt_elem(&gen).map_err(|_| "non_rational_point")?;
    let src_t = pt_elem(&target).map_err(|_| "non_rational_point")?;
    let mut certs: Vec<Cert> = vec![];

    // --- direct cost: rho on the source curve ---
    let rho_src = rho_on_ec(&e1, gen, target, r, seed);
    let mut rho_ok = rho_src.recovered == Some(x);
    let rho_src_ops = rho_src.group_ops.adds + rho_src.group_ops.doubles;
    certs.push(cert(
        "rho_e1",
        "direct",
        p,
        ec_group(&e1),
        r,
        (src_g.clone(), src_t.clone()),
        rho_src.recovered,
        None,
        seed,
        rho_src_ops,
    ));

    // --- chain 1: isogeny hop E1 → E1', then rho ---
    let (iso, kernel) = find_isogeny(&e1, r).ok_or("no_rational_small_isogeny")?;
    let e_prime = iso.codomain;
    let isog_degree = iso.degree() as u64;
    let hop = Hop::isogeny(iso, &gen, &target, r).map_err(|_| "isogeny_hop_failed")?;
    let ch = Chain::new(vec![hop]);
    if !ch.verify(&Elt::E(gen), r, x as u128) {
        return Err("isogeny_chain_unverified");
    }
    let (gp, tp) = match (ch.transport(&Elt::E(gen)), ch.transport(&Elt::E(target))) {
        (Elt::E(a), Elt::E(b)) => (a, b),
        _ => (None, None),
    };
    let rho_i = rho_on_ec(&e_prime, gp, tp, r, seed ^ 0xA5);
    rho_ok &= rho_i.recovered == Some(x);
    let rho_isog_ops = rho_i.group_ops.adds + rho_i.group_ops.doubles;
    certs.push(cert(
        "rho_e1prime",
        "isogeny",
        p,
        ec_group(&e_prime),
        r,
        (
            pt_elem(&gp).map_err(|_| "non_rational_point")?,
            pt_elem(&tp).map_err(|_| "non_rational_point")?,
        ),
        rho_i.recovered,
        Some(Transfer::Isogeny {
            src: ec_group(&e1),
            kernel: pt_elem(&kernel).map_err(|_| "non_rational_point")?,
            src_g: src_g.clone(),
            src_t: src_t.clone(),
        }),
        seed ^ 0xA5,
        rho_isog_ops,
    ));

    // --- chain 2: pull-back E1 → J(C); solve the pulled-back pair itself ---
    let hop = Hop::pullback(&b, Quotient::E1, &gen, r).map_err(|_| "pullback_hop_failed")?;
    let ch = Chain::new(vec![hop]);
    let pull_ok = ch.verify(&Elt::E(gen), r, x as u128);
    if !pull_ok {
        return Err("pullback_chain_unverified");
    }
    let (dg, dt) = match (ch.transport(&Elt::E(gen)), ch.transport(&Elt::E(target))) {
        (Elt::J(a), Elt::J(b)) => (a, b),
        _ => return Err("pullback_not_a_divisor"),
    };
    let transfer = Transfer::Pullback {
        c: b.c,
        src_g: src_g.clone(),
        src_t: src_t.clone(),
    };
    let rho = rho_on_jac(&jac, dg, dt, r, seed ^ 0x5A);
    rho_ok &= rho.recovered == Some(x);
    let rho_jac_ops = rho.group_ops.adds + rho.group_ops.doubles;
    certs.push(cert(
        "rho_jc",
        "pullback",
        p,
        jac_group(&jac),
        r,
        (div_elem(&dg), div_elem(&dt)),
        rho.recovered,
        Some(transfer.clone()),
        seed ^ 0x5A,
        rho_jac_ops,
    ));
    // Full degree-1 factor base (≈ p/2 places): genus-2 IC uses every
    // rational place, so smoothness ≈ 1/2.
    let ic = genus2_index_calculus(&jac, &dg, &dt, r, p as usize, 50_000_000, seed);
    certs.push(cert(
        "ic_jc",
        "pullback",
        p,
        jac_group(&jac),
        r,
        (div_elem(&dg), div_elem(&dt)),
        ic.recovered,
        Some(transfer),
        seed,
        ic.group_ops,
    ));
    let certs_verified = certs.iter().filter(|c| certify::verify(c).ok).count() as u64;
    let row = Row {
        p,
        r,
        e1_order,
        e2_order,
        jac_order,
        x,
        rho_src_ops,
        rho_isog_ops,
        isog_degree,
        rho_jac_ops,
        ic_jac_group_ops: ic.group_ops,
        ic_jac_trials: ic.trials,
        ic_jac_relations: ic.relations,
        ic_jac_fb: ic.fb_size,
        ic_recovered_ok: ic.recovered == Some(x),
        rho_recovered_ok: rho_ok,
        ic_jac_la_mulmods: ic.la_field_ops,
        chain_verified: true,
        instance_type: "pullback",
        ic_claimed_log: ic.recovered,
        certs_issued: certs.len() as u64,
        certs_verified,
        candidate: 0,
    };
    Ok((row, certs))
}

/// Log–log figure of counted group operations against `p`, every coordinate
/// computed from the measured rows.
fn render_svg(rows: &[Row]) -> String {
    let (x0, x1) = (200f64.ln(), 2000f64.ln());
    let (y0, y1) = (200f64.ln(), 50000f64.ln());
    let xp = |p: f64| 80.0 + (p.ln() - x0) / (x1 - x0) * 520.0;
    let yp = |v: f64| 360.0 - (v.ln() - y0) / (y1 - y0) * 300.0;
    let mut s = String::new();
    s.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"640\" height=\"420\" viewBox=\"0 0 640 420\" font-family=\"Helvetica,Arial,sans-serif\" font-size=\"12\">\n<rect width=\"640\" height=\"420\" fill=\"#ffffff\"/>\n");
    s.push_str("<text x=\"320\" y=\"24\" text-anchor=\"middle\" font-size=\"15\" font-weight=\"bold\">Chained transfer E1 → E1′ → J(C): counted group operations</text>\n");
    s.push_str("<text x=\"320\" y=\"42\" text-anchor=\"middle\" fill=\"#555\">log–log axes; one seed per cell; values [pcc:unverified]</text>\n");
    for t in [300.0, 1000.0, 3000.0, 10000.0, 30000.0] {
        let y = yp(t);
        s.push_str(&format!("<line x1=\"80\" y1=\"{y:.1}\" x2=\"600\" y2=\"{y:.1}\" stroke=\"#e5e5e5\"/><text x=\"74\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#444\">{t}</text>\n", y + 4.0));
    }
    for t in [250.0, 500.0, 1000.0, 2000.0] {
        let x = xp(t);
        s.push_str(&format!("<line x1=\"{x:.1}\" y1=\"60\" x2=\"{x:.1}\" y2=\"360\" stroke=\"#e5e5e5\"/><text x=\"{x:.1}\" y=\"376\" text-anchor=\"middle\" fill=\"#444\">{t}</text>\n"));
    }
    s.push_str("<line x1=\"80\" y1=\"360\" x2=\"600\" y2=\"360\" stroke=\"#999\"/><line x1=\"80\" y1=\"60\" x2=\"80\" y2=\"360\" stroke=\"#999\"/>\n");
    s.push_str("<text x=\"340\" y=\"400\" text-anchor=\"middle\">field size p</text><text x=\"22\" y=\"210\" text-anchor=\"middle\" transform=\"rotate(-90 22 210)\">group operations</text>\n");
    let series: [(&str, &str, &str, fn(&Row) -> u64); 4] = [
        (
            "#c0392b",
            "none",
            "genus-2 IC on J(C), full degree-1 base",
            |r| r.ic_jac_group_ops,
        ),
        (
            "#2c3e50",
            "none",
            "rho on source E1 (matched reference)",
            |r| r.rho_src_ops,
        ),
        (
            "#27ae60",
            "5,3",
            "rho on J(C) order-r subgroup after φ*",
            |r| r.rho_jac_ops,
        ),
        ("#8e44ad", "2,3", "rho on isogenous E1′", |r| {
            r.rho_isog_ops
        }),
    ];
    s.push_str(
        "<rect x=\"96\" y=\"66\" width=\"262\" height=\"80\" fill=\"#fff\" stroke=\"#ccc\"/>\n",
    );
    for (i, (color, dash, label, f)) in series.iter().enumerate() {
        let pts: Vec<String> = rows
            .iter()
            .map(|r| format!("{:.1},{:.1}", xp(r.p as f64), yp(f(r) as f64)))
            .collect();
        s.push_str(&format!("<polyline fill=\"none\" stroke=\"{color}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\" points=\"{}\"/>\n", pts.join(" ")));
        for p in &pts {
            let (x, y) = p.split_once(',').unwrap();
            s.push_str(&format!(
                "<circle cx=\"{x}\" cy=\"{y}\" r=\"3.5\" fill=\"{color}\"/>"
            ));
        }
        let ly = 80 + 18 * i;
        s.push_str(&format!("\n<line x1=\"104\" y1=\"{ly}\" x2=\"128\" y2=\"{ly}\" stroke=\"{color}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\"/><text x=\"134\" y=\"{}\">{label}</text>\n", ly + 4));
    }
    s.push_str("</svg>\n");
    s
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let json_path = args
        .iter()
        .position(|a| a == "--json")
        .and_then(|i| args.get(i + 1));

    // A spread of prime sizes (dense Z/r linear algebra over ≈ p/2 columns
    // keeps the top size modest). Coefficients searched deterministically.
    let primes = [211u64, 419, 821, 1619];
    let mut rows = vec![];
    let mut all_certs: Vec<Cert> = vec![];
    // Per p: candidates tried and, per reason, how many were skipped.
    let mut skipped: BTreeMap<u64, BTreeMap<&'static str, u64>> = BTreeMap::new();
    let mut tried: BTreeMap<u64, u64> = BTreeMap::new();
    for (i, &p) in primes.iter().enumerate() {
        let mut found = false;
        for s in 0..4000u64 {
            let c = [
                1 + (s.wrapping_mul(2_654_435_761) >> 3) % (p - 1),
                (s.wrapping_mul(40_503) >> 5) % p,
                (s.wrapping_mul(2_246_822_519) >> 7) % p,
                1 + (s.wrapping_mul(3_266_489_917) >> 9) % (p - 1),
            ];
            *tried.entry(p).or_default() += 1;
            match measure_curve(p, c, 0xC0FFEE + i as u64 * 101 + s) {
                Ok((mut row, certs)) => {
                    row.candidate = s;
                    all_certs.extend(certs);
                    rows.push(row);
                    found = true;
                    break;
                }
                Err(reason) => {
                    *skipped.entry(p).or_default().entry(reason).or_default() += 1;
                }
            }
        }
        let n_tried = tried.get(&p).copied().unwrap_or(0);
        eprintln!(
            "p = {p}: {n_tried} candidate(s) tried, skipped by reason: {:?}",
            skipped.get(&p).cloned().unwrap_or_default()
        );
        if !found {
            eprintln!("no suitable bielliptic curve found at p = {p}");
        }
    }

    println!(
        "{:>7} {:>8} {:>10} {:>12} {:>12} {:>7} {:>10} {:>12} {:>12}  chain",
        "p",
        "r",
        "rho_src",
        "rho_isog(d)",
        "rho_jac",
        "ic_fb",
        "ic_grpops",
        "ic_trials",
        "ic/rho_src"
    );
    for row in &rows {
        let ratio = row.ic_jac_group_ops as f64 / row.rho_src_ops.max(1) as f64;
        let tail = format!(
            "{} ic_log={} certs={}/{}",
            if row.chain_verified { "ok" } else { "FAIL" },
            if row.ic_recovered_ok {
                "right"
            } else {
                "WRONG"
            },
            row.certs_verified,
            row.certs_issued
        );
        println!(
            "{:>7} {:>8} {:>10} {:>8}({}) {:>12} {:>7} {:>10} {:>12} {:>10.1}x  {}",
            row.p,
            row.r,
            row.rho_src_ops,
            row.rho_isog_ops,
            row.isog_degree,
            row.rho_jac_ops,
            row.ic_jac_fb,
            row.ic_jac_group_ops,
            row.ic_jac_trials,
            ratio,
            tail,
        );
    }
    println!(
        "\nrho_src  = direct Pollard rho on E1 (group ops)\n\
         rho_isog = rho after a verified isogeny hop E1→E1' (same order r)\n\
         rho_jac  = rho on the order-r subgroup of J(C) after verified φ^*\n\
         ic_jac   = genus-2 index calculus on J(C), counted group ops\n\
         All figures [pcc:unverified]; isogeny/pull-back DLP-preservation is\n\
         checked in-process before each measurement."
    );

    if let Some(path) = args
        .iter()
        .position(|a| a == "--svg")
        .and_then(|i| args.get(i + 1))
    {
        std::fs::write(path, render_svg(&rows)).expect("write svg");
        eprintln!("wrote {path}");
    }

    if let Some(path) = args
        .iter()
        .position(|a| a == "--certs")
        .and_then(|i| args.get(i + 1))
    {
        let mut out = String::new();
        for c in &all_certs {
            out.push_str(&serde_json::to_string(c).unwrap());
            out.push('\n');
        }
        std::fs::write(path, out).expect("write certs");
        eprintln!("wrote {path} ({} certificates)", all_certs.len());
    }

    if let Some(path) = json_path {
        let j = serde_json::json!({
            "schema": "chained.transfer.bench/v3",
            "instance_type": "pullback",
            "oracle_free_elimination": true,
            "transfer_cost": "one-time, excluded from every per-instance figure",
            "candidates_tried": tried,
            "candidates_skipped_by_reason": skipped,
            "certificates_issued": all_certs.len(),
            "certificates_verified_in_process": rows.iter().map(|r| r.certs_verified).sum::<u64>(),
            "rows": rows,
        });
        std::fs::write(path, serde_json::to_string_pretty(&j).unwrap()).expect("write json");
        eprintln!("wrote {path}");
    }
    ExitCode::SUCCESS
}
