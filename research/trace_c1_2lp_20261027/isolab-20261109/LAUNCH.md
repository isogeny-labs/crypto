# Iso Lab launch: trace-line exact MITM reference panel (2026-11-09)

Frozen before dispatch. **This is the exact MITM correctness-reference design
only (`reference-d*.json`), not the algebraic C1 experiment of
[PROTOCOL.md](../PROTOCOL.md), which stays blocked on the C1 oracle.** Its
results are a stage diagnostic of relation yield and collector cost, not a
priced end-to-end IC result.

| field | value |
|---|---|
| route | GitHub `isolab-experiment.yml`, `workflow_dispatch` on `main` (AWS bridge unavailable: this session's AWS keys fail STS with `InvalidClientTokenId`) |
| source_revision | `f07b6d3e4eb7501ed51271dcd69e06cb543af96d` (main HEAD; the four specs are unchanged there) |
| specs | `research/trace_c1_2lp_20261027/reference-d2.json`, `-d3`, `-d4`, `-d5` (N = 8, 12, 16, 20) |
| per spec | 2 curves (seeds 20261027, 20261127), 8 public targets, arms standard / selected / standard-aa / rho-neg, 1 warmup + 6 rounds, L2 required, 120 s per execution |
| instance / deadline | `c8a.2xlarge`, `seconds = 1800` (includes provisioning) |
| dispatches | 4, one per spec; no automatic resubmission |
| dollar ceiling | USD 10 for all four, failures included |
| local feasibility | development probe of d5 (1 target, 1 round, L0, not a measurement): 8 executions, all verified, 3.2 s wall total |
| benchmark record | `benchmarks/records/20261109T1453Z-trace-line-reference-isolab.json` |
| resume checkpoint | S3 upload impossible (no token or profile); this pushed commit is the durable checkpoint |

## Dispatch log

Filled in after each dispatch: UTC time, workflow run ID, outcome.
