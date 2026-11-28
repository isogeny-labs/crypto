//! Run the bounded APS point-batch queue on a public known-answer curve.
//! Usage: cargo run --example aps_queued_workers -- --workers 4

use crypto_lib::cryptanalysis::{
    aps_single_relation::{solve_dlp_queued, Encoding, Labels},
    prime_fast::FastCurve,
};
use std::time::Duration;

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    let workers = match args.as_slice() {
        [_] => 1,
        [_, flag, value] if flag == "--workers" => value
            .parse::<usize>()
            .map_err(|_| "--workers requires a positive integer")?,
        _ => return Err("usage: aps_queued_workers [--workers N]".into()),
    };
    let curve = FastCurve::new("aps-public-fixture", 101, 59, 6, 107);
    let g = curve.point(78, 4);
    let q = curve.mul(Some(g), 37).ok_or("fixture target is infinity")?;
    let attempts = solve_dlp_queued(
        &curve,
        g,
        q,
        Encoding::Partitioned3,
        Labels::Guaranteed,
        3,
        20261026,
        32,
        Duration::from_secs(10),
        workers,
        workers.saturating_mul(2).min(128),
    )?;
    let recovered = attempts
        .iter()
        .find_map(|(_, attempt)| attempt.recovery.as_ref());
    let logarithm = recovered
        .ok_or("bounded search found no verified relation")?
        .logarithm;
    if logarithm != 37 || curve.mul(Some(g), logarithm) != Some(q) {
        return Err("fixture scalar replay failed".into());
    }
    let output = serde_json::json!({
        "schema": "aps.queued-workers.correctness/v1",
        "workers": workers,
        "queue_capacity": workers.saturating_mul(2).min(128),
        "seed": 20261026u64,
        "attempt_cap": 32,
        "attempts_completed": attempts.len(),
        "attempts": attempts,
        "verified_logarithm": logarithm,
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&output).map_err(|e| e.to_string())?
    );
    Ok(())
}
