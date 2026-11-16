# Resume: crypto @ claude/leads-20261113 (PR #4065)

## Goal
Owner (2026-11-16): "Yes do all the above optimizations in workflow in parallel" — the leads of
la_solver_20261116: faster relation collection, threaded LA, fix the dlpx clique/precheck loop
and retest excess 2–5 %. (The L2 rerun needs isolab-eks; no AWS identity in this session.)

## Spec
research/humbert_genus5_cover_20261021/levers_20261116/PROTOCOL.md, frozen before data.
Binary levers_20261116/bin/humbert-v6 sha256 85be709b… (rebuild: cargo build --release at the
protocol commit). Runner: BIN=levers_20261116/bin/humbert-v6 BIN_SHA256_PREFIX=85be709b
PRIMES=<q> levers_20261116/run_v6.sh levers_20261116/campaign/q<q>; resume with SEEDS=.

## State
Levers built/verified by workflow wf_ff8cffdd-dad (scratchpad levers/{rel,la,clique}), merged
into src/ (relfast.rs new; la.rs threaded engine; filter.rs safe clique rule), 12 tests pass.
Smoke q16001 s1: counted fields equal v5; dlpf t_ic 24.2 s -> 4.7 s with REL_FAST+LA_THREADS=4.
Campaign under bench record benchmarks/records/<stamp>-humbert-genus5-levers.json.
S3 checkpoint unavailable (no AWS identity); the branch push is the durable copy.

## Next
After each q commit rows. Then summarize (python, levers_20261116/analyze_v6.py), figures,
REPORT.typ/pdf, benchmarks/20261116-humbert-genus5-levers, benchrec close, session log, push,
update PR #4065 body.
