# Resume: crypto @ claude/leads-20261113 (PR #4065)

## Goal
Owner (2026-11-16): "keep working on the next novel leads" on the genus-5 twisted-Humbert cover.
dlp_filter_20261115 left the linear algebra as the bottleneck (dlpf LA 1.6x lp's; 63 of 96 s at
q = 24001). This run tests three LA levers in isolated arms: one Wiedemann solve with an end-to-end
x P = Q check (w1), scalar Lanczos (l1), and 10 % excess + clique removal (dlpx).

## Spec
research/humbert_genus5_cover_20261021/la_solver_20261116/PROTOCOL.md, frozen before data.
Binary: la_solver_20261116/bin/humbert-v5 (sha256 in BINARY.sha256; rebuild with
`cargo build --release` at the protocol commit). Runner: `PRIMES=<q> la_solver_20261116/run_v5.sh <dir>/q<q>`,
one launch per q in {16001, 24001, 32003} (each under the 2 h background limit).

## State
2026-11-16: code (la.rs kernel_lanczos, HUMBERT_LA_SOLVER, e2e_ok, filter.rs build_x clique removal,
dlpx arm, solversummary.rs) and protocol committed; campaign launching into la_solver_20261116/campaign/q*/
under bench record benchmarks/records/<stamp>-humbert-genus5-la-solver.json.
S3 checkpoint unavailable (no AWS identity); the branch push is the durable copy.
PR #4065: merges check fails on merge 5598316 (Binary-Regenerated trailer missing); waiting on the
owner's choice. crypto-autoresearcher #3282 needs vault KEY.

## Next
After each q: commit rows. After all three: `humbert summarize-solver la_solver_20261116/results
la_solver_20261116/campaign/q*/results/v5.jsonl dlp_filter_20261115/campaign/results/v4.jsonl`;
figures, REPORT.typ/pdf, benchmarks copy, benchrec close, session log, push.
