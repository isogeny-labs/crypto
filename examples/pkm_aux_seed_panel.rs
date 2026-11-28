//! **Seed panels for the ordinary auxiliary-curve search.**
//!
//! `research/pkm_isogeny_tower_ecbench_20261015/` panel 3 pooled
//! `aux_curves_tried ≈ C·2^t` over 224 builds (C = 1.05) and recorded a
//! seed spread of two to three orders of magnitude at fixed `t`; this
//! example regenerates that panel for both draw families of
//! `prime_tower::isogeny_tower` — `any` (every curve) and `z2xz8`
//! (the `X₁(2, 8)` family) — without running any pipeline:
//!
//! ```bash
//! cargo run --release --example pkm_aux_seed_panel -- --p 14680063 \
//!     --t 8,10,12,14 --seeds 16 --out panel.jsonl
//! ```
//!
//! One JSON row per `(family, t, seed)`: the tried-curve count, the
//! search/count/build GAE split, and the BSGS table size.  Stage
//! diagnostic of the solver axis (`AGENTS.md` §2): prices no pipeline.

use crypto_lib::cryptanalysis::ic_framework::prime_tower::{
    isogeny_tower, AuxFamily,
};
use crypto_lib::cryptanalysis::ic_framework::prime_tower_wide::isogeny_tower_wide;
use crypto_lib::cryptanalysis::fp_backend::FpWide;
use crypto_lib::cryptanalysis::ic_boundary::GroupOps;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut p = 14_680_063u64;
    let mut ts = vec![8usize, 10, 12, 14];
    let mut seeds = 16usize;
    let mut family = AuxFamily::Any;
    let mut wide = false;
    let mut out = None::<String>;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--p" => {
                i += 1;
                p = args[i].parse().expect("--p");
            }
            "--t" => {
                i += 1;
                ts = args[i].split(',').map(|x| x.parse().expect("a t")).collect();
            }
            "--seeds" => {
                i += 1;
                seeds = args[i].parse().expect("--seeds");
            }
            "--family" => {
                i += 1;
                family = AuxFamily::parse(&args[i]).expect("--family");
            }
            "--wide" => {
                wide = true;
            }
            "--out" => {
                i += 1;
                out = Some(args[i].clone());
            }
            _ => panic!("usage: pkm_aux_seed_panel --p P --t T,T,… --seeds N --family any|z2xz8 [--wide] --out F"),
        }
        i += 1;
    }
    let mut rows: Vec<serde_json::Value> = Vec::new();
    for &t in &ts {
        let t = t as u32;
        for seed in 0..seeds as u64 {
            let mut ops = GroupOps::default();
            let started = std::time::Instant::now();
            let (curves_tried, search_gae, count_gae, build_gae, wall_ms, aux_order) =
                if wide {
                    // FpWide<1> holds every prime below 2^64; the wide
                    // search is the same charged construction, on the
                    // multi-limb backend (a statistical cross-check of
                    // the u64 panel, not a per-seed one: the rng streams
                    // differ).
                    let f = FpWide::<1>::new(&num_bigint::BigUint::from(p));
                    let tw = isogeny_tower_wide(&f, t, seed, family, &mut ops)
                        .unwrap_or_else(|e| panic!("t = {t}, seed = {seed}: {e}"));
                    (
                        tw.cost.curves_tried,
                        tw.cost.search_gae,
                        tw.cost.count_gae,
                        tw.cost.build_gae,
                        started.elapsed().as_millis() as u64,
                        tw.aux_order.to_string(),
                    )
                } else {
                    let tower = isogeny_tower(p, t, seed, family, &mut ops)
                        .unwrap_or_else(|e| panic!("t = {t}, seed = {seed}: {e}"));
                    (
                        tower.cost.curves_tried,
                        tower.cost.search_gae,
                        tower.cost.count_gae,
                        tower.cost.build_gae,
                        started.elapsed().as_millis() as u64,
                        tower.aux_order.to_string(),
                    )
                };
            rows.push(
                serde_json::json!({
                    "family": family.name(),
                    "backend": if wide { "wide1" } else { "u64" },
                    "p": p,
                    "t": t,
                    "seed": seed,
                    "curves_tried": curves_tried,
                    "search_gae": search_gae,
                    "count_gae": count_gae,
                    "build_gae": build_gae,
                    "aux_order": aux_order,
                    "wall_ms": wall_ms,
                })
            );
            eprintln!(
                "{} t = {t} seed = {seed:2} tried = {curves_tried:5} search = {search_gae:8} build = {build_gae:8} wall = {wall_ms} ms",
                family.name(),
            );
        }
    }
    if let Some(path) = out {
        let mut f = File::create(&path).expect("out");
        for r in &rows {
            serde_json::to_writer(&mut f, r).unwrap();
            writeln!(f).unwrap();
        }
        eprintln!("wrote {} rows to {path}", rows.len());
    }
}
