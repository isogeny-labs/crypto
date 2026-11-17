#[path = "../../../research/hec_anchor_relations_20261114/anchor.rs"]
mod anchor;
#[path = "../src/table_factor.rs"]
mod table_factor;
fn main() {
    let gate = std::env::var("HEC_GATE").is_ok();
    let seeds: u64 = std::env::var("HEC_SEEDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    for (g, p) in [
        (2, 101),
        (2, 251),
        (2, 503),
        (2, 1009),
        (2, 2003),
        (2, 4001),
        (3, 61),
        (3, 211),
        (3, 401),
        (4, 31),
        (4, 61),
    ] {
        let i = hec_native_ic::instance::pick_instance(p, g).unwrap();
        let k = i.ell / 3 + 7;
        let target = i.curve.mul(&i.d1, k);
        for seed in 0..seeds {
            let r = anchor::solve(&i.curve, &i.d1, &target, i.ell, 20260916 + seed, gate);
            println!("ANCHOR {{\"g\":{g},\"p\":{p},\"seed\":{seed},\"gate\":{gate},\"m\":{},\"anchors\":{},\"draws\":{},\"unique_tuples\":{},\"splits\":{},\"admitted\":{},\"duplicate_relations\":{},\"pure_rows\":{},\"unions\":{},\"pins\":{},\"solves\":{},\"descent_steps\":{},\"raw_checks\":{},\"pure_checks\":{},\"table_bytes\":{},\"setup_ns\":{},\"relation_ns\":{},\"forest_ns\":{},\"solve_ns\":{},\"total_ns\":{},\"verified\":{},\"capped\":{}}}",r.m,r.anchors,r.draws,r.unique_tuples,r.splits,r.admitted,r.duplicate_relations,r.pure_rows,r.unions,r.pins,r.solves,r.descent_steps,r.raw_checks,r.pure_checks,r.memory_bytes,r.setup_ns,r.relation_ns,r.forest_ns,r.solve_ns,r.total_ns,r.k==Some(k),r.k.is_none());
        }
    }
}
