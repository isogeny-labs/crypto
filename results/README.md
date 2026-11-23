# Hourly relation-yield monitor

This branch uses the existing frozen Stage-21 `examples/koblitz_relation_yield_bridge.rs` harness, **not** an ECC2K-130 implementation. The registered production fixture is Koblitz `GF(2^23)`, `a=0`, `m=2`, with 256 natural targets, 64 planted-SAT controls and 64 exact-pair-table-proven UNSAT controls. The 16-target `--smoke` mode is deliberately rejected by the recorder.

## Recording and metric definitions

`scripts/hourly_relation_finding.py` builds the Rust example with the repository's pinned Cargo lock and runs it without `--smoke`. Every attempt appends a row to `results/hourly_log.csv`; a failure has status `failed` and intentionally blank metrics. Among verified successful runs:

- `natural_relations`: natural-arm exact pair-table hits with independently checked point witnesses.
- `success_rate`: natural relations / 256 natural targets. Planted controls must all hit and UNSAT controls must all miss; they are never counted as natural relations.
- `wall_seconds`: subprocess elapsed monotonic wall time for the producer only, excluding Cargo compilation.
- `relations_per_sec`: natural relations / producer wall seconds, including factor-base construction, exact pair enumeration and natural target queries.

`results/relation_baseline.json` is established only by the first successful measured run; no missing baseline is synthesized. Subsequent runs compare success rate and throughput to that committed baseline using a **strictly greater than 20% two-sided relative change**. Curve/factor-base identity changes also alert. The production seed and target mix are fixed, so success rate is mostly a correctness drift signal; throughput can vary with runner CPU, cache, and noisy neighbors. The CSV logs CPU identity for interpretation.

Failed runs and alerts write `results/hourly_alert.md` locally and the GitHub Actions workflow opens an issue. Historical raw producer JSON is retained as a short-lived workflow artifact. The outcome is not an ECC2K-130 security or index-calculus scaling claim.

## Start another measurement

The existing scheduled automation can create a unique file in `results/hourly_triggers/` on branch `hourly-runs`. The push activates `.github/workflows/hourly-relation-finding.yml`; writing results back to the branch does not re-trigger itself. An example trigger path is `results/hourly_triggers/20261117T190000Z.txt`. Do not reuse an existing name; use the UTC run time to avoid duplicate commits. A GitHub workflow manual dispatch also works for this branch.

GitHub Actions runs from this branch: https://github.com/isogeny-labs/crypto/actions/workflows/hourly-relation-finding.yml?query=branch%3Ahourly-runs

**Limit:** Running the same fixed n=23 fixture every hour measures performance/reproducibility; a meaningful ECC2K-130 relation-finding experiment requires a separately validated curve-specific producer, factor base, target distribution, completeness checks and cost accounting before the runner can be switched.
