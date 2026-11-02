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
use crypto_lib::cryptanalysis::ic_boundary::GroupOps;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut p = 14_680_063u64;
    let mut ts = vec![8usize, 10, 12, 14];
    let mut seeds = 16usize;
    let mut family = AuxFamily::Any;
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
            "--out" => {
                i += 1;
                out = Some(args[i].clone());
            }
            _ => panic!("usage: pkm_aux_seed_panel --p P --t T,T,… --seeds N --family any|z2xz8 --out F"),
        }
        i += 1;
    }
    let mut rows: Vec<serde_json::Value> = Vec::new();
    for &t in &ts {
        let t = t as u32;
        for seed in 0..seeds as u64 {
            let mut ops = GroupOps::default();
            let started = std::time::Instant::now();
            let tower = isogeny_tower(p, t, seed, family, &mut ops)
                .unwrap_or_else(|e| panic!("t = {t}, seed = {seed}: {e}"));
            let c = &tower.cost;
            rows.push(
                serde_json::json!({
                    "family": family.name(),
                    "p": p,
                    "t": t,
                    "seed": seed,
                    "curves_tried": c.curves_tried,
                    "singular": c.singular,
                    "filtered_out": c.filtered_out,
                    "filter_false_positives": c.filter_false_positives,
                    "orders_undetermined": c.orders_undetermined,
                    "two_sylow_rejected": c.two_sylow_rejected,
                    "search_gae": c.search_gae,
                    "count_gae": c.count_gae,
                    "build_gae": c.build_gae,
                    "bsgs_table_entries": c.bsgs_table_entries,
                    "aux_order": tower.aux_order,
                    "wall_ms": started.elapsed().as_millis() as u64,
                })
            );
            eprintln!(
                "{} t = {t} seed = {seed:2} tried = {:5} search = {:8} build = {:8} wall = {} ms",
                family.name(),
                c.curves_tried,
                c.search_gae,
                c.build_gae,
                started.elapsed().as_millis()
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
