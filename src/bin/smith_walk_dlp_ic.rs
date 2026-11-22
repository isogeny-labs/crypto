//! `smith_walk_dlp_ic`: round 2 of the Smith route H -> plane quartic. Local development checks;
//! all figures `[pcc:unverified]`. Study: research/isogeny-walk-quartic_20261121/ (PLAN.md frozen first).
//!
//! ```text
//! smith_walk_dlp_ic run --p P --seed S --curves N [--z3 1] [--rho-runs R] [--c-list 200,400,600]   # JSON lines
//! ```
//! Per curve: transfer (canonical quartic from 20 points, push-forward of 8 points), group order
//! (zeta counts, route Z3; N1, N2 + BSGS window, route Z2B), Diem line relations with sparse
//! linear algebra on the quartic (qds_ext), pullback and verification on the ORIGINAL H, and a
//! measured negation-map rho on J_H with the same Cantor arithmetic.

#[allow(dead_code)]
#[path = "../../research/isogeny-walk-quartic_20261121/src/st.rs"]
mod st;

#[allow(dead_code)]
#[path = "../../research/isogeny-walk-quartic_20261121/src/planefit.rs"]
mod planefit;

#[allow(dead_code, clippy::wrong_self_convention)]
#[path = "../cryptanalysis/diem_quartic.rs"]
mod dq;

#[path = "../../research/isogeny-walk-quartic_20261121/src/qds_ext.rs"]
mod qds_ext;

#[path = "../../research/isogeny-walk-quartic_20261121/src/hcount.rs"]
mod hcount;

#[path = "../../research/isogeny-walk-quartic_20261121/src/hgroup.rs"]
mod hgroup;

use crypto_lib::cryptanalysis::smith_prop24::{analyze_curve, peval, SplitMix64};
use crypto_lib::prime_hyperelliptic::{FpPoly, HyperellipticCurveP, MumfordDivisorP};
use num_bigint::BigUint;
use qds_ext::{IcIn, PhiIn};
use serde_json::{json, Value};
use st::{Fp2, Trigonal, F2};
use std::collections::HashSet;
use std::time::Instant;

fn arg(a: &[String], k: &str) -> Option<String> {
    a.iter()
        .position(|x| x == k)
        .and_then(|i| a.get(i + 1))
        .cloned()
}

// ----------------------------------------------------------------- canonical map (copied from
// smith_walk_dlp.rs of round 1; the field type is the counted copy `st`)

type Cub = [Fp2; 3];

fn mulx(k: &F2, a: Cub, g: [u64; 3]) -> Cub {
    let top = a[2];
    [
        k.neg(k.scale(top, g[0])),
        k.sub(a[0], k.scale(top, g[1])),
        k.sub(a[1], k.scale(top, g[2])),
    ]
}

fn mulmod(k: &F2, a: Cub, b: Cub, g: [u64; 3]) -> Cub {
    let mut r = [Fp2::default(); 3];
    let mut xb = b;
    for ai in a {
        for (rj, bj) in r.iter_mut().zip(xb.iter()) {
            *rj = k.add(*rj, k.mul(ai, *bj));
        }
        xb = mulx(k, xb, g);
    }
    r
}

fn solve3(k: &F2, mut m: [[Fp2; 4]; 3]) -> Option<[Fp2; 3]> {
    for c in 0..3 {
        let piv = (c..3).find(|&i| !k.is_zero(m[i][c]))?;
        m.swap(c, piv);
        let iv = k.inv(m[c][c]);
        for j in 0..4 {
            m[c][j] = k.mul(m[c][j], iv);
        }
        for i in 0..3 {
            if i != c && !k.is_zero(m[i][c]) {
                let f = m[i][c];
                for j in 0..4 {
                    m[i][j] = k.sub(m[i][j], k.mul(f, m[c][j]));
                }
            }
        }
    }
    Some([m[0][3], m[1][3], m[2][3]])
}

/// `h_i = [x^2] (x^i D b^-1 mod G)`: the canonical map of Recillas' curve X (round 1, amended A1).
fn canon_map(t: &Trigonal, k: &F2, tt: u64, b: [Fp2; 3]) -> Option<[Fp2; 3]> {
    let p = t.p;
    st::count_m(3 * (t.g[0].len() + t.g[1].len() + t.g[2].len()) as u64 / 3);
    let g = [
        peval(&t.g[0], tt, p),
        peval(&t.g[1], tt, p),
        peval(&t.g[2], tt, p),
    ];
    let d = [k.c(t.d0), k.c(t.d1), k.c(1)];
    let mut cols = [[Fp2::default(); 3]; 3];
    let mut xj = [k.c(1), Fp2::default(), Fp2::default()];
    for col in cols.iter_mut() {
        *col = mulmod(k, b, xj, g);
        xj = mulx(k, xj, g);
    }
    let mut m = [[Fp2::default(); 4]; 3];
    for j in 0..3 {
        m[j][..3].copy_from_slice(&cols[j]);
        m[j][3] = if j == 2 { k.c(1) } else { Fp2::default() };
    }
    let r = solve3(k, m)?;
    let mut v = d;
    let mut h = [Fp2::default(); 3];
    for hi in h.iter_mut() {
        let mut s = Fp2::default();
        for (rj, vj) in r.iter().zip(v.iter()) {
            s = k.add(s, k.mul(*rj, *vj));
        }
        *hi = s;
        v = mulx(k, v, g);
    }
    Some(h)
}

type Chart = (Fp2, Fp2);

fn canon_chart(t: &Trigonal, k: &F2, q: &st::XPoint) -> Option<Chart> {
    let h = canon_map(t, k, q.t, q.mumford)?;
    if k.is_zero(h[2]) {
        return None;
    }
    let i2 = k.inv(h[2]);
    Some((k.mul(h[0], i2), k.mul(h[1], i2)))
}

// ------------------------------------------------------------------------------ generator

struct Gen {
    p: u64,
    rng: SplitMix64,
}

impl Gen {
    fn new(p: u64, seed: u64) -> Gen {
        Gen {
            p,
            rng: SplitMix64(seed),
        }
    }

    /// Next monic degree-7 Smith-positive curve and the counted cost of building its `Trigonal`.
    fn next_curve(&mut self) -> Option<(Vec<u64>, Trigonal, (u64, u64))> {
        let p = self.p;
        for _ in 0..50_000 {
            let f: Vec<u64> = (0..7)
                .map(|_| self.rng.below(p))
                .chain(std::iter::once(1))
                .collect();
            let Some(co) = analyze_curve(&f, p, &mut self.rng) else {
                continue;
            };
            let found = co
                .subgroups
                .iter()
                .filter(|s| s.trig_rational() && s.iso_rational(false))
                .flat_map(|s| s.maps.iter())
                .filter(|m| m.valid() && m.alpha_chi == 1)
                .find_map(|m| m.map.smith_coeffs());
            if let Some((n1, n0, d1, d0)) = found {
                let s0 = st::snap();
                if let Ok(t) = Trigonal::new(&f, p, n1, n0, d1, d0) {
                    let s1 = st::snap();
                    return Some((f, t, (s1.0 - s0.0, s1.1 - s0.1)));
                }
            }
        }
        None
    }
}

fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        let mut e = 0;
        while n.is_multiple_of(d) {
            n /= d;
            e += 1;
        }
        if e > 0 {
            out.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

/// Largest prime `ell` with `ell || N` and `N / ell <= max_cof`.
fn pick_ell(n: u64, max_cof: u64) -> Option<u64> {
    factor(n)
        .into_iter()
        .filter(|&(l, e)| e == 1 && n / l <= max_cof && l > 2)
        .map(|(l, _)| l)
        .max()
}

struct Cfg {
    z3: bool,
    rho_runs: u64,
    c_list: Vec<u64>,
    seed: u64,
}

fn hpoint(hc: &HyperellipticCurveP, x: u64, y: u64) -> Option<MumfordDivisorP> {
    MumfordDivisorP::from_point(hc, &BigUint::from(x), &BigUint::from(y))
}

fn hsmul(d: &MumfordDivisorP, k: u64, hc: &HyperellipticCurveP) -> MumfordDivisorP {
    d.scalar_mul_fast(&BigUint::from(k), hc)
}

fn cost_json(m: u64, i: u64, nat: u64) -> Value {
    json!({"M": m, "I": i, "NAT": nat})
}

/// One curve end to end. `Err` = the candidate is rejected before the measurement (not charged).
fn process(
    p: u64,
    idx: u64,
    f: &[u64],
    t: &Trigonal,
    build: (u64, u64),
    cfg: &Cfg,
) -> Result<(Value, Vec<Value>), String> {
    let k = F2::new(p);
    let pb = BigUint::from(p);
    // ---------------------------------------------------------------- transfer
    let s0 = st::snap();
    let t_tr = Instant::now();
    let pts_h = t.points_over_u();
    let mut canon_pts: Vec<Chart> = Vec::new();
    let mut seen: HashSet<Chart> = HashSet::new();
    let mut cands: Vec<(u64, u64, PhiIn)> = Vec::new();
    let mut used_x: HashSet<u64> = HashSet::new();
    let mut skipped = 0u64;
    for (x, y) in pts_h {
        if canon_pts.len() >= 20 && cands.len() >= 14 {
            break;
        }
        if used_x.contains(&x) {
            continue;
        }
        let Ok(ip) = t.image_points(x, y) else {
            skipped += 1;
            continue;
        };
        let charts: Vec<Option<Chart>> = ip.points.iter().map(|q| canon_chart(t, &k, q)).collect();
        if charts.iter().any(Option::is_none) {
            skipped += 1;
            continue;
        }
        let ch: Vec<Chart> = charts.into_iter().flatten().collect();
        let rational_ok = ch.iter().all(|c| c.0.b == 0 && c.1.b == 0);
        if ip.rational && !rational_ok {
            skipped += 1;
            continue;
        }
        used_x.insert(x);
        for c in &ch {
            if seen.insert(*c) {
                canon_pts.push(*c);
            }
        }
        let arr = |c: &Chart| [c.0.a, c.0.b, c.1.a, c.1.b];
        let phi = if ip.rational {
            PhiIn {
                rational: true,
                pts: vec![arr(&ch[0]), arr(&ch[1])],
            }
        } else {
            PhiIn {
                rational: false,
                pts: vec![arr(&ch[0])],
            }
        };
        cands.push((x, y, phi));
    }
    if canon_pts.len() < 17 || cands.len() < 10 {
        return Err(format!(
            "too few canonical points ({}) or candidates ({})",
            canon_pts.len(),
            cands.len()
        ));
    }
    let cub_dim = planefit::kernel_dim(&canon_pts, 3, p);
    let basis = planefit::kernel_basis(&canon_pts, 4, p);
    if cub_dim != 0 || basis.len() != 1 {
        return Err(format!(
            "no unique quartic (cubic kernel {cub_dim}, quartic kernel {})",
            basis.len()
        ));
    }
    let mons = planefit::monomials(4);
    let mut fc = [[0u64; 5]; 5];
    for (pos, &(i, j)) in mons.iter().enumerate() {
        fc[i][j] = basis[0][pos];
    }
    if fc[0][4] == 0 {
        return Err("y^4 coefficient zero (the driver needs it non-zero)".into());
    }
    let s1 = st::snap();
    let transfer = (s1.0 - s0.0 + build.0, s1.1 - s0.1 + build.1);
    let wall_transfer = t_tr.elapsed().as_secs_f64();
    // ---------------------------------------------------------------- the original curve H
    let hf = FpPoly::from_coeffs(f.iter().map(|&x| BigUint::from(x)).collect(), pb.clone());
    let hc = HyperellipticCurveP::new(pb.clone(), hf, 3);
    let hdiv: Vec<Option<MumfordDivisorP>> = cands.iter().map(|c| hpoint(&hc, c.0, c.1)).collect();
    if hdiv.iter().any(Option::is_none) {
        return Err("a candidate point is not on H".into());
    }
    let hdiv: Vec<MumfordDivisorP> = hdiv.into_iter().flatten().collect();
    let hdiff = |a: usize, b: usize| hdiv[a].add_fast(&hdiv[b].neg_fast(&hc), &hc);
    // ---------------------------------------------------------------- group order, route Z2B
    let t_z2 = Instant::now();
    let c1 = hcount::count_fp(f, p);
    let c2 = hcount::count_fp2(f, p);
    let (base, lo, hi) = hcount::window_from_n1n2(p, c1.n, c2.n);
    let nlo = (base - hi) as u64;
    let width = (hi - lo) as u64;
    let g0 = hdiff(0, 1);
    let (hits, wcost, wnat) = hgroup::window_search(&hc, &g0, nlo, width);
    let mut ncand: Vec<u64> = hits.iter().map(|j| nlo + j).collect();
    if ncand.len() > 1 {
        let g1 = hdiff(2, 3);
        ncand.retain(|&n| hsmul(&g1, n, &hc).is_identity());
    }
    if ncand.len() != 1 {
        return Err(format!(
            "order window ambiguous ({} candidates)",
            ncand.len()
        ));
    }
    let n_z2b = ncand[0];
    let wall_z2 = t_z2.elapsed().as_secs_f64();
    let z2b = json!({
        "N1": c1.n, "N2": c2.n, "window_width": width, "candidates_before_filter": hits.len(),
        "count_M": c1.m + c2.m, "count_NAT": c1.nat + c2.nat,
        "bsgs_M": wcost.m, "bsgs_I": wcost.i, "bsgs_NAT": wnat,
        "M": c1.m + c2.m + wcost.m, "I": wcost.i, "NAT": c1.nat + c2.nat + wnat, "wall_s": wall_z2,
    });
    let Some(ell) = pick_ell(n_z2b, 64) else {
        return Err(format!(
            "N = {n_z2b} has no prime ell || N with N/ell <= 64"
        ));
    };
    // ---------------------------------------------------------------- group order, route Z3
    let z3 = if cfg.z3 {
        let t_z3 = Instant::now();
        let c3 = hcount::count_fp3(f, p);
        let (_, _, _, n) = hcount::lpoly_from_counts(p, c1.n, c2.n, c3.n);
        if n as u64 != n_z2b {
            return Err(format!("Z3 order {n} differs from Z2B order {n_z2b}"));
        }
        Some(json!({
            "N1": c1.n, "N2": c2.n, "N3": c3.n, "N": n as u64,
            "M": c1.m + c2.m + c3.m, "I": 0, "NAT": c1.nat + c2.nat + c3.nat,
            "M_fp3_part": c3.m, "NAT_fp3_part": c3.nat, "wall_s": t_z3.elapsed().as_secs_f64(),
        }))
    } else {
        None
    };
    // ---------------------------------------------------------------- Diem arm on C (twice: A/A)
    let n = n_z2b;
    let cof = n / ell;
    let phis: Vec<PhiIn> = cands.iter().map(|c| c.2.clone()).collect();
    let mk = |ic_seed: u64, c100: u64| IcIn {
        q: p,
        nr: k.nr,
        fc,
        n_group: n,
        ell,
        cands: phis.clone(),
        seed: cfg.seed ^ idx,
        ic_seed,
        c100,
    };
    // factor-base size c (x100) escalates only after a failed attempt; every attempt is charged
    let run_ic = |ic_seed: u64| -> (qds_ext::IcOut, Vec<qds_ext::IcOut>) {
        let mut failed = Vec::new();
        let mut last = qds_ext::IcOut::default();
        for &c in &cfg.c_list {
            last = qds_ext::smith_ic(&mk(ic_seed, c));
            if last.status == "ok" || last.chosen.len() < 8 {
                break;
            }
            failed.push(last.clone());
        }
        (last, failed)
    };
    let (out, failed_main) = run_ic(cfg.seed ^ idx ^ 0x1C);
    if out.chosen.len() < 8 || out.tgts.is_empty() {
        return Err(format!("diem driver: {}", out.status));
    }
    let (out_aa, failed_aa) = run_ic(cfg.seed ^ idx ^ 0x1D);
    // ---------------------------------------------------------------- pullback on the original H
    let ch = &out.chosen;
    let gh = hdiff(ch[0], ch[1]);
    let gp = hsmul(&gh, cof, &hc);
    let h_order_ok = hsmul(&gp, ell, &hc).is_identity() && !gp.is_identity();
    let hq: Vec<MumfordDivisorP> = (0..3)
        .map(|j| hsmul(&hdiff(ch[2 + 2 * j], ch[3 + 2 * j]), cof, &hc))
        .collect();
    let mut pull = Vec::new();
    let mut tgt_rows = Vec::new();
    for (j, tg) in out.tgts.iter().enumerate() {
        let c0 = hgroup::snap();
        let (verified_h, kfound) = match tg.0 {
            Some(kk) => (hsmul(&gp, kk, &hc) == hq[j], Some(kk)),
            None => (false, None),
        };
        let dc = hgroup::since(hgroup::snap(), c0);
        pull.push((dc, verified_h));
        tgt_rows.push(json!({
            "j": j, "q_trivial": tg.4, "k": kfound, "verified_on_C": tg.3, "verified_on_H": verified_h,
            "descent": cost_json(tg.1[0], tg.1[1], tg.1[2]), "descent_adds": tg.2,
            "pullback": {"M": dc.m, "I": dc.i, "adds_field": dc.add},
        }));
    }
    let (gm, gi, ga) = {
        let c0 = hgroup::snap();
        let mut x = hsmul(&gp, 12345 % ell.max(2), &hc);
        let a = hsmul(&gp, 777 % ell.max(2) + 1, &hc);
        let c1_ = hgroup::snap();
        let reps = 300u64;
        for _ in 0..reps {
            x = x.add_fast(&a, &hc);
        }
        std::hint::black_box(&x);
        let d = hgroup::since(hgroup::snap(), c1_);
        let _ = c0;
        (
            d.m as f64 / reps as f64,
            d.i as f64 / reps as f64,
            d.add as f64 / reps as f64,
        )
    };
    // ---------------------------------------------------------------- rho on J_H
    let mut rho_rows = Vec::new();
    for (j, qh) in hq.iter().enumerate() {
        if qh.is_identity() {
            continue;
        }
        for run in 0..cfg.rho_runs {
            let seed = cfg.seed ^ (idx << 40) ^ ((j as u64) << 32) ^ (run << 8) ^ 0x3E;
            let tr = Instant::now();
            let r = hgroup::rho(&hc, &gp, qh, ell, true, seed);
            rho_rows.push(json!({
                "kind": "rho_neg", "p": p, "curve": idx, "target": j, "run": run, "aa_set": run % 2,
                "seed": seed, "ell": ell, "found": r.found, "verified": r.verified, "steps": r.steps,
                "M": r.cost.m, "I": r.cost.i, "adds_field": r.cost.add, "NAT": r.nat,
                "restarts": r.restarts, "cycle_escapes": r.cycle_escapes,
                "setup_M": r.setup.m, "setup_I": r.setup.i, "setup_adds_field": r.setup.add, "wall_s": tr.elapsed().as_secs_f64(),
            }));
        }
    }
    let curve = json!({
        "kind": "curve", "p": p, "curve": idx, "f": f, "seed": cfg.seed, "status": out.status,
        "N": n, "ell": ell, "cof": cof, "h_order_ok": h_order_ok, "c_order_ok": out.order_check_c,
        "transfer": {"M": transfer.0, "I": transfer.1, "NAT": 0, "wall_s": wall_transfer, "skipped_points": skipped,
                     "canonical_points": canon_pts.len(), "build_trigonal_M": build.0},
        "order_z2b": z2b, "order_z3": z3,
        "diem": {
            "main": diem_json(&out), "aa": diem_json(&out_aa),
            "failed_attempts_main": failed_main.iter().map(diem_json).collect::<Vec<_>>(),
            "failed_attempts_aa": failed_aa.iter().map(diem_json).collect::<Vec<_>>(),
        },
        "targets": tgt_rows,
        "gae_h": {"M": gm, "I": gi, "adds_field": ga},
        "chosen": out.chosen,
        "wall_note": "wall times are diagnostic only",
    });
    let _ = pull;
    Ok((curve, rho_rows))
}

fn diem_json(o: &qds_ext::IcOut) -> Value {
    json!({
        "status": o.status, "b": o.b, "pairs": o.pairs, "rounds": o.rounds, "rows_fb": o.rows_fb,
        "w_fb_mean": o.w_fb_mean, "known_frac": o.known_frac, "n1_C": o.n1_c,
        "c_setup": cost_json(o.c_setup[0], o.c_setup[1], o.c_setup[2]),
        "phi_build": cost_json(o.phi_build[0], o.phi_build[1], o.phi_build[2]),
        "fb_setup": cost_json(o.fb_setup[0], o.fb_setup[1], o.fb_setup[2]),
        "relations": cost_json(o.rel[0], o.rel[1], o.rel[2]),
        "elim_ops": o.elim_ops, "elim_entries": o.elim_entries, "wied_ops": o.wied_ops,
        "bm_ops": o.bm_ops, "matvecs": o.matvecs,
        "gae_c": {"M": o.gae_c.0, "I": o.gae_c.1},
        "wall_rel": o.wall_rel, "wall_elim": o.wall_elim, "wall_wied": o.wall_wied,
        "targets": o.tgts.iter().map(|t| json!({"k": t.0, "M": t.1[0], "I": t.1[1], "NAT": t.1[2], "adds": t.2, "verified_C": t.3})).collect::<Vec<_>>(),
    })
}

fn run(a: &[String]) {
    let p: u64 = arg(a, "--p").unwrap().parse().unwrap();
    let seed: u64 = arg(a, "--seed").unwrap().parse().unwrap();
    let want: u64 = arg(a, "--curves").unwrap().parse().unwrap();
    let cfg = Cfg {
        z3: arg(a, "--z3").is_some_and(|v| v == "1"),
        rho_runs: arg(a, "--rho-runs").map_or(6, |v| v.parse().unwrap()),
        c_list: arg(a, "--c-list")
            .unwrap_or_else(|| "200,400,600".into())
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect(),
        seed,
    };
    let max_cand: u64 = arg(a, "--max-cand").map_or(400, |v| v.parse().unwrap());
    let mut gen = Gen::new(p, seed);
    let (mut accepted, mut tried) = (0u64, 0u64);
    while accepted < want && tried < max_cand {
        let Some((f, t, build)) = gen.next_curve() else {
            break;
        };
        tried += 1;
        match process(p, tried, &f, &t, build, &cfg) {
            Ok((curve, rho_rows)) => {
                accepted += 1;
                println!("{curve}");
                for r in rho_rows {
                    println!("{r}");
                }
            }
            Err(e) => println!(
                "{}",
                json!({"kind": "reject", "p": p, "candidate": tried, "f": f, "reason": e})
            ),
        }
    }
    println!(
        "{}",
        json!({"kind": "summary", "p": p, "seed": seed, "accepted": accepted, "candidates": tried,
               "git": std::env::var("SWI_GIT").unwrap_or_else(|_| "unknown".into())})
    );
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("run") => run(&a),
        _ => {
            eprintln!("usage: smith_walk_dlp_ic run --p P --seed S --curves N [--z3 1] [--rho-runs R] [--c-list 200,400,600] [--max-cand M]");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_ell_needs_a_prime_with_small_cofactor() {
        assert_eq!(pick_ell(2 * 3 * 1_000_003, 64), Some(1_000_003));
        assert_eq!(pick_ell(3 * 3 * 1_000_003, 64), Some(1_000_003));
        assert_eq!(pick_ell(1_000_003 * 101, 64), None);
        assert_eq!(factor(360), vec![(2, 3), (3, 2), (5, 1)]);
    }

    #[test]
    fn neg_rho_and_order_window_on_h() {
        // small H over F_101 (the first Smith-positive monic curve of the generator)
        let p = 101u64;
        let mut gen = Gen::new(p, 31);
        let (f, _t, _) = gen.next_curve().unwrap();
        let pb = BigUint::from(p);
        let hf = FpPoly::from_coeffs(f.iter().map(|&x| BigUint::from(x)).collect(), pb.clone());
        let hc = HyperellipticCurveP::new(pb, hf, 3);
        let mut pts = Vec::new();
        for x in 0..p {
            for y in 0..p {
                if let Some(d) = hpoint(&hc, x, y) {
                    pts.push(d);
                    break;
                }
            }
            if pts.len() >= 4 {
                break;
            }
        }
        let g = pts[0].add_fast(&pts[1].neg_fast(&hc), &hc);
        let c1 = hcount::count_fp(&f, p);
        let c2 = hcount::count_fp2(&f, p);
        let c3 = hcount::count_fp3(&f, p);
        let (_, _, e3, n) = hcount::lpoly_from_counts(p, c1.n, c2.n, c3.n);
        let (base, lo, hi) = hcount::window_from_n1n2(p, c1.n, c2.n);
        assert!(lo <= e3 && e3 <= hi);
        assert_eq!(n, base - e3);
        let (hits, _, _) = hgroup::window_search(&hc, &g, (base - hi) as u64, (hi - lo) as u64);
        assert!(hits.contains(&((n - (base - hi)) as u64)));
        assert!(hsmul(&g, n as u64, &hc).is_identity());
        // rho in the prime-order part
        let nn = n as u64;
        let Some(ell) = (3..nn)
            .rev()
            .find(|&l| nn % l == 0 && factor(l).len() == 1 && factor(l)[0].0 == l)
        else {
            return;
        };
        let cof = nn / ell;
        let gp = hsmul(&g, cof, &hc);
        if gp.is_identity() || ell < 50 {
            return;
        }
        let kk = 7 % ell;
        let qp = hsmul(&gp, kk, &hc);
        let r = hgroup::rho(&hc, &gp, &qp, ell, true, 5);
        assert!(r.found && r.verified, "{r:?}");
        assert_eq!(hsmul(&gp, r.k, &hc), qp);
    }

    /// End to end at p = 101: a curve passes the filters, the Diem arm finds the logs on the
    /// quartic and every found log verifies on the original H (also exercises `div_conj`).
    #[test]
    fn smith_route_verifies_on_h_at_p101() {
        let p = 101u64;
        let cfg = Cfg {
            z3: true,
            rho_runs: 1,
            c_list: vec![200, 400, 600],
            seed: 31,
        };
        let mut gen = Gen::new(p, 31);
        for idx in 1..=200u64 {
            let (f, t, build) = gen.next_curve().unwrap();
            if let Ok((curve, rho_rows)) = process(p, idx, &f, &t, build, &cfg) {
                let tg = curve["targets"].as_array().unwrap();
                let found: Vec<_> = tg.iter().filter(|t| !t["k"].is_null()).collect();
                assert!(!found.is_empty());
                for t in &found {
                    assert_eq!(t["verified_on_H"], json!(true), "{curve}");
                }
                assert_eq!(curve["order_z3"]["N"], curve["N"]);
                assert!(rho_rows.iter().all(|r| r["verified"] == json!(true)));
                return;
            }
        }
        panic!("no curve accepted in 200 candidates");
    }
}
