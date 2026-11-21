use std::path::PathBuf;

use prime_aux_family_ablation_20261121::run_ablation;

fn main() {
    let out = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("RESULTS.json"));
    let summary = run_ablation();
    println!("\n=== Ranking (score=2A+3B+C) ===");
    for (i, line) in summary.ranking.iter().enumerate() {
        println!("  {}. {line}", i + 1);
    }
    for r in &summary.rows {
        println!(
            "  [{}] lift={:.3} deg={} cols={} ops_x={:.2} wall_x={:.2} planted={} → {}",
            r.family,
            r.lift_rate,
            r.solving_degree_max,
            r.max_cols,
            r.ops_speedup_x,
            r.wall_speedup_x,
            r.planted_hit_verified,
            r.verdict
        );
    }
    let json = serde_json::to_string_pretty(&summary).expect("json");
    std::fs::write(&out, json).expect("write RESULTS");
    println!("wrote {}", out.display());
}
