# Resume: crypto @ claude/humbert-next-leads-20261120 (PR #4848, draft)

## Goal
Owner (2026-11-20): "Do all the next leads in parallel" — after levers_20261116: factor-base
rebalance, faster setup, matched multi-thread rho; L2 rerun blocked (no AWS identity).

## Spec
research/humbert_genus5_cover_20261021/next_leads_20261120/PROTOCOL.md (frozen before data).
Binary next_leads_20261120/bin/humbert-v7 sha256 a4b55b71… (rebuild: cargo build --release at the
protocol commit). Runner: BIN=… BIN_SHA256_PREFIX=a4b55b71 PRIMES=<q> next_leads_20261120/run_v7.sh
next_leads_20261120/campaign/q<q>; resume with SEEDS=.

## State
Workflow wf_51f8d293-995 verified all three leads (scratchpad levers2/{fb,setup,rho}); setup and rho
patches merged (setupfast.rs, rhopar.rs), 19 tests pass. Smoke q16001 s1: comb7 2.79 s vs matched
rho expected ~1.3 s (22M steps/s). Bench record benchmarks/records/20261120T2130Z-humbert-genus5-next-leads.json.
S3 checkpoint unavailable (no AWS identity); the branch push is the durable copy.

## Next
Run q16001, 24001, 32003 serially; analyze (next_leads_20261120/analyze_v7.py), report/PDF,
benchmarks folder, close record, session log, ACTIVE tick, PR ready.
