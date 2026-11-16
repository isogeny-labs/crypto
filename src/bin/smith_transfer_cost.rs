//! `smith_transfer_cost run --primes P,... --out F.jsonl [options]`: the
//! counted cost of Smith's reduction on genus-3 hyperelliptic DLP instances
//! where it applies, against index calculus and rho on the curve itself.
//!
//! For every prime: curves `y² = f(x)` (`genus_ladder::random_f`, `deg f = 7`)
//! whose Jacobian order is `h·r` (`h ≤ --max-h`, `r` prime) and that are
//! Smith-positive with a normal-form trigonal map
//! (`smith_canonical::find_trigonal`), for which the canonical quartic `C` is
//! smooth and `#C(F_p) = #H(F_p)`.  Per instance and seed, four arms on the
//! same generator `G` and target `T = [t]G`:
//!
//! * `H_C_double_lp+pre`: `genus_ladder::run_ic`, double large prime, prefilter;
//! * `H_rho_neg`: `genus_ladder::run_rho`;
//! * `C_transfer_diem_2lp`: `Ψ(G), Ψ(T)` by `smith_canonical::CountedTransfer`
//!   (its `F_p` work counted), then `quartic_ladder::run_diem` (double large
//!   prime) on `C`.  The transfer's multiplications are folded into `rel_mul`
//!   and its inversions into `cantor_inv`, and both are also reported apart
//!   (`transfer_mul`, `transfer_inv`, `transfer_parts`);
//! * `C_rho_neg`: `quartic_ladder::run_rho` on `C` with the transferred
//!   classes (reference on `C`; the transfer is not charged to it).
//!
//! Records are JSON lines in the layout of `genus_ladder run` (family
//! `smith_transfer`, `g = 3`), readable by `genus_ladder_analyze`.
//!
//! Not counted (stated in the study's RESULTS.md): finding Smith's trigonal
//! map (`smith_prop24::analyze_curve`), the fixed-degree polynomial set-up of
//! `Trigonal::new`, and the smoothness certificate inside
//! `PlaneQuartic::new`.  Instance search, orders, the zeta check and the
//! checks of the transferred classes are instance set-up or verification.
//!
//! Protocol: `research/smith_transfer_cost_20261115/PROTOCOL.md`.

use std::io::Write;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crypto_lib::cryptanalysis::chained_transfer::ic_control::split_order;
use crypto_lib::cryptanalysis::genus_ladder as gl;
use crypto_lib::cryptanalysis::plane_quartic_pic::PicClass;
use crypto_lib::cryptanalysis::quartic_ladder as ql;
use crypto_lib::cryptanalysis::smith_canonical as sc;
use serde_json::{json, Value};

struct Cfg {
    primes: Vec<u64>,
    curves: usize,
    seeds: u64,
    cc: f64,
    qc: f64,
    max_h: u64,
    threads: usize,
    rho_cap: f64,
    seed_search: u64,
    max_trials: u64,
    out: String,
}

fn parse() -> Result<Cfg, String> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.first().map(String::as_str) != Some("run") {
        return Err("usage: smith_transfer_cost run --primes P,... --out F [options]".into());
    }
    let mut c = Cfg {
        primes: vec![],
        curves: 3,
        seeds: 2,
        cc: 2.0,
        qc: 3.0,
        max_h: 8,
        threads: 4,
        rho_cap: 1.2e8,
        seed_search: 4000,
        max_trials: 50_000_000,
        out: String::new(),
    };
    let mut i = 1;
    while i < a.len() {
        let flag = a[i].as_str();
        let v = a
            .get(i + 1)
            .cloned()
            .ok_or(format!("missing value for {flag}"))?;
        let num = |v: &str| v.parse::<f64>().map_err(|e| format!("{flag}: {e}"));
        match flag {
            "--primes" => {
                c.primes = v
                    .split(',')
                    .map(|s| s.parse().map_err(|e| format!("{e}")))
                    .collect::<Result<_, _>>()?
            }
            "--curves" => c.curves = num(&v)? as usize,
            "--seeds" => c.seeds = num(&v)? as u64,
            "--cc" => c.cc = num(&v)?,
            "--qc" => c.qc = num(&v)?,
            "--max-h" => c.max_h = num(&v)? as u64,
            "--threads" => c.threads = num(&v)? as usize,
            "--rho-cap" => c.rho_cap = num(&v)?,
            "--seed-search" => c.seed_search = num(&v)? as u64,
            "--max-trials" => c.max_trials = num(&v)? as u64,
            "--out" => c.out = v,
            _ => return Err(format!("unknown flag {flag}")),
        }
        i += 2;
    }
    if c.primes.is_empty() || c.out.is_empty() {
        return Err("--primes and --out are required".into());
    }
    Ok(c)
}

#[derive(Clone)]
struct Spec {
    p: u64,
    curve_seed: u64,
    order: u64,
    h: u64,
    r: u64,
    n_points: u64,
}

fn pool<T: Sync, F: Fn(&T) + Sync>(items: &[T], threads: usize, f: F) {
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= items.len() {
                    break;
                }
                f(&items[i]);
            });
        }
    });
}

fn parts(c: &sc::TransferCost) -> Value {
    json!({"map_mul": c.map_mul, "fit_mul": c.fit_mul, "fit_inv": c.fit_inv,
        "setup_mul": c.setup_mul, "setup_inv": c.setup_inv,
        "image_mul": c.image_mul, "image_inv": c.image_inv,
        "pic_mul": c.pic_mul, "pic_inv": c.pic_inv, "pic_ops": c.pic_ops,
        "cantor_mul": c.cantor_mul, "cantor_inv": c.cantor_inv, "cantor_ops": c.cantor_ops,
        "root_mul": c.root_mul, "tries": c.tries})
}

fn main() -> ExitCode {
    let cfg = match parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };
    let file = std::fs::File::create(&cfg.out).expect("create --out");
    let out = Mutex::new(std::io::BufWriter::new(file));
    let emit = |v: Value| {
        let mut w = out.lock().unwrap();
        writeln!(w, "{v}").unwrap();
        w.flush().unwrap();
    };
    let t0 = Instant::now();
    emit(
        json!({"kind": "config", "primes": cfg.primes, "genera": [3], "curves": cfg.curves,
        "seeds": cfg.seeds, "cc": cfg.cc, "qc": cfg.qc, "max_h": cfg.max_h,
        "rho_cap": cfg.rho_cap, "seed_search": cfg.seed_search, "max_trials": cfg.max_trials,
        "threads": cfg.threads, "restart": gl::RESTART, "fit_points": sc::CountedTransfer::FIT_POINTS}),
    );

    // ── instance search ─────────────────────────────────────────────────
    let specs: Mutex<Vec<Spec>> = Mutex::new(Vec::new());
    pool(&cfg.primes, cfg.threads, |&p| {
        let ts = Instant::now();
        let (mut tried, mut admitted, mut positive, mut accepted) = (0u64, 0u64, 0u64, 0usize);
        let mut rejects: std::collections::BTreeMap<String, u64> = Default::default();
        let mut bump = |k: &str| *rejects.entry(k.to_string()).or_default() += 1;
        for cs in 1..=cfg.seed_search {
            if accepted >= cfg.curves {
                break;
            }
            tried += 1;
            let f = gl::random_f(p, 3, cs);
            let curve = match gl::JCurve::new(p, 3, &f) {
                Ok(c) => c,
                Err(_) => {
                    bump("not_squarefree");
                    continue;
                }
            };
            let order = match curve.jacobian_order(cs) {
                Ok(o) => o,
                Err(_) => {
                    bump("order_failed");
                    continue;
                }
            };
            let Some((h, r)) = split_order(order, cfg.max_h) else {
                bump("order_not_near_prime");
                continue;
            };
            admitted += 1;
            let tr = match sc::find_trigonal(&f, p, cs) {
                Ok(tr) => tr,
                Err(e) => {
                    if e.starts_with("not smith-positive") {
                        bump("not_smith_positive");
                    } else if e.starts_with("no normal-form") {
                        positive += 1;
                        bump("positive_no_normal_form_map");
                    } else {
                        positive += 1;
                        bump("positive_trigonal_failed");
                    }
                    continue;
                }
            };
            positive += 1;
            let ctf = match sc::CountedTransfer::new(tr.clone()) {
                Ok(c) => c,
                Err(e) => {
                    bump(&format!("construction: {e}"));
                    continue;
                }
            };
            let n_h = sc::count_h_points(&f, p, 1);
            let n_c = sc::count_quartic_points(p, &ctf.quartic, 1);
            if n_h != n_c {
                bump("zeta_mismatch");
                emit(
                    json!({"kind": "zeta_mismatch", "p": p, "curve_seed": cs, "n_h": n_h, "n_c": n_c}),
                );
                continue;
            }
            accepted += 1;
            emit(
                json!({"kind": "instance", "family": "smith_transfer", "p": p, "g": 3,
                "curve_seed": cs, "f": f, "order": order, "h": h, "r": r,
                "N": [tr.n1, tr.n0], "D": [tr.d1, tr.d0], "C": ctf.quartic.to_vec(),
                "P0": ctf.p0.to_vec(), "n_points": n_c, "smooth": true}),
            );
            specs.lock().unwrap().push(Spec {
                p,
                curve_seed: cs,
                order,
                h,
                r,
                n_points: n_c,
            });
        }
        emit(
            json!({"kind": "cell", "family": "smith_transfer", "p": p, "g": 3, "tried": tried,
            "admitted": admitted, "smith_positive": positive, "accepted": accepted,
            "rejects": rejects, "search_wall_s": ts.elapsed().as_secs_f64()}),
        );
    });

    // ── runs ────────────────────────────────────────────────────────────
    let mut sp = specs.into_inner().unwrap();
    sp.sort_by_key(|a| (a.p, a.curve_seed));
    let mut tasks: Vec<(Spec, u64)> = Vec::new();
    for s in &sp {
        for seed in 1..=cfg.seeds {
            tasks.push((s.clone(), seed));
        }
    }
    pool(&tasks, cfg.threads, |(spec, seed)| {
        let (p, cs) = (spec.p, spec.curve_seed);
        let f = gl::random_f(p, 3, cs);
        let curve = gl::JCurve::new(p, 3, &f).expect("admitted curve");
        let inst =
            gl::instance_from(curve, spec.order, spec.h, spec.r, cs).expect("admitted instance");
        let head = json!({"family": "smith_transfer", "p": p, "g": 3, "curve_seed": cs,
            "seed": seed, "r": spec.r, "h": spec.h});

        // (a) index calculus on H.
        let o = gl::run_ic(
            &inst,
            gl::IcParams {
                arm: gl::Arm::DoubleLp,
                nb: gl::fb_size_2lp(p, 3, cfg.cc),
                seed: *seed,
                max_trials: cfg.max_trials,
                restart: gl::RESTART,
                prefilter: true,
                verify_relations: false,
                sge: None,
            },
        );
        let mut rec = head.clone();
        rec["kind"] = json!("run");
        rec["arm"] = json!("H_C_double_lp+pre");
        rec["certified"] = json!(o.certified);
        rec["exhausted"] = json!(o.exhausted);
        rec["ok"] = json!(o.recovered == Some(inst.t_secret));
        rec["group_ops"] = json!(o.group_ops());
        rec["setup_ops"] = json!(o.setup_ops);
        rec["walk_ops"] = json!(o.walk_ops);
        rec["cantor_mul"] = json!(o.cantor_mul);
        rec["cantor_inv"] = json!(o.cantor_inv);
        rec["rel_mul"] = json!(o.rel.mul);
        rec["rel_add"] = json!(o.rel.add);
        rec["la_mul"] = json!(o.la.mul);
        rec["la_add"] = json!(o.la.add);
        rec["fb_cols"] = json!(o.fb_cols);
        rec["trials"] = json!(o.trials);
        rec["candidates"] = json!(o.candidates);
        rec["relations"] = json!(o.relations);
        rec["rels_by_lp"] = json!(o.rels_by_lp);
        rec["core_rows"] = json!(o.core_rows);
        rec["core_cols"] = json!(o.core_cols);
        rec["attempts"] = json!(o.attempts);
        rec["wall_ns"] = json!(o.wall_ns);
        emit(rec);

        // (b) rho on H.
        let o = gl::run_rho(&inst, *seed);
        let mut rec = head.clone();
        rec["kind"] = json!("run");
        rec["arm"] = json!("H_rho_neg");
        rec["certified"] = json!(o.certified);
        rec["ok"] = json!(o.recovered == Some(inst.t_secret));
        rec["group_ops"] = json!(o.group_ops());
        rec["walk_ops"] = json!(o.walk_ops);
        rec["cantor_mul"] = json!(o.cantor_mul);
        rec["cantor_inv"] = json!(o.cantor_inv);
        rec["native"] = json!(o.native);
        rec["steps"] = json!(o.steps);
        rec["wall_ns"] = json!(o.wall_ns);
        emit(rec);

        // (c) the transfer, then Diem on C.
        let tw = Instant::now();
        let transferred = (|| -> Result<(ql::QInstance, sc::TransferCost, u64), String> {
            let tr = sc::find_trigonal(&f, p, cs)?;
            let ctf = sc::CountedTransfer::new(tr)?;
            // A fresh Jacobian so that the Cantor counters hold the transfer only.
            let jc = gl::JCurve::new(p, 3, &f)?;
            let img = ctf.transfer(&jc, &[inst.g, inst.target], *seed)?;
            let cost = ctf.cost();
            // ord G / ord Ψ(G): r is an odd prime, so 1 unless Ψ(G) = 0.
            let zero: PicClass = ctf.pq.zero();
            let ratio = if img[0] == zero { spec.r } else { 1 };
            let qi = ql::make_instance_from_classes(
                p as u32,
                ctf.quartic,
                ctf.p0,
                &img[0],
                &img[1],
                spec.order,
                spec.h,
                spec.r,
                spec.n_points,
                Some(inst.t_secret),
            )?;
            Ok((qi, cost, ratio))
        })();
        let transfer_wall = tw.elapsed().as_nanos() as u64;
        let (qi, cost, ratio) = match transferred {
            Ok(x) => x,
            Err(e) => {
                let mut rec = head.clone();
                rec["kind"] = json!("transfer_failed");
                rec["arm"] = json!("C_transfer_diem_2lp");
                rec["error"] = json!(e);
                emit(rec);
                return;
            }
        };
        let o = ql::run_diem(
            &qi,
            ql::QIcParams {
                arm: ql::QArm::DoubleLp,
                nb: (cfg.qc * (p as f64).sqrt()).ceil() as usize,
                seed: *seed,
                max_lines: cfg.max_trials,
                verify_relations: false,
            },
        );
        let mut rec = head.clone();
        rec["kind"] = json!("run");
        rec["arm"] = json!("C_transfer_diem_2lp");
        rec["certified"] = json!(o.certified);
        rec["exhausted"] = json!(o.exhausted);
        rec["ok"] = json!(o.recovered == Some(inst.t_secret));
        rec["ord_ratio"] = json!(ratio);
        rec["group_ops"] = json!(o.group_ops);
        rec["cantor_mul"] = json!(o.pic_mul);
        rec["cantor_inv"] = json!(o.pic_inv + cost.inv());
        rec["rel_mul"] = json!(o.rel.mul + cost.mul());
        rec["rel_add"] = json!(o.rel.add);
        rec["la_mul"] = json!(o.la.mul);
        rec["la_add"] = json!(o.la.add);
        rec["diem_pic_inv"] = json!(o.pic_inv);
        rec["diem_rel_mul"] = json!(o.rel.mul);
        rec["transfer_mul"] = json!(cost.mul());
        rec["transfer_inv"] = json!(cost.inv());
        rec["transfer_parts"] = parts(&cost);
        rec["transfer_wall_ns"] = json!(transfer_wall);
        rec["fb_cols"] = json!(o.fb_cols);
        rec["trials"] = json!(o.lines_tried);
        rec["relations"] = json!(o.relations);
        rec["rels_by_lp"] = json!(o.rels_by_lp);
        rec["decomp_trials"] = json!(o.decomp_trials);
        rec["core_rows"] = json!(o.core_rows);
        rec["core_cols"] = json!(o.core_cols);
        rec["attempts"] = json!(o.attempts);
        rec["wall_ns"] = json!(o.wall_ns + transfer_wall);
        emit(rec);

        // (d) rho on C with the transferred classes.
        let expect = (std::f64::consts::PI * spec.r as f64 / 4.0).sqrt();
        let mut rec = head.clone();
        rec["arm"] = json!("C_rho_neg");
        rec["ord_ratio"] = json!(ratio);
        if expect > cfg.rho_cap / 100.0 {
            rec["kind"] = json!("skip");
            rec["reason"] = json!("rho_cap");
            rec["expected_steps"] = json!(expect);
        } else {
            let o = ql::run_rho(&qi, *seed);
            rec["kind"] = json!("run");
            rec["certified"] = json!(o.certified);
            rec["ok"] = json!(o.recovered == Some(inst.t_secret));
            rec["group_ops"] = json!(o.group_ops);
            rec["cantor_mul"] = json!(o.pic_mul);
            rec["cantor_inv"] = json!(o.pic_inv);
            rec["steps"] = json!(o.steps);
            rec["wall_ns"] = json!(o.wall_ns);
        }
        emit(rec);
    });
    emit(json!({"kind": "done", "wall_s": t0.elapsed().as_secs_f64()}));
    ExitCode::SUCCESS
}
