# Resume: crypto @ claude/leads-20261113 (PR #4065)

## Goal
Owner (2026-11-15): "Pursue the leads" on the genus-5 twisted-Humbert cover. The double-large-prime
campaign (dlp_20261113) showed full elimination doubles LA weight; this run tests filtered matrices
(large primes kept as columns, singleton pruning, cost-guided merges) on the same relation set.

## Spec
research/humbert_genus5_cover_20261021/dlp_filter_20261115/PROTOCOL.md, frozen before data.
Binary: dlp_filter_20261115/bin/humbert-v4 (sha256 in BINARY.sha256; rebuild with
`cargo build --release` at this commit). Runner: dlp_filter_20261115/run_v4.sh <new-dir>.

## State
2026-11-15: code (src/filter.rs, dlpm/dlpf arms) and protocol committed; campaign launching into
dlp_filter_20261115/campaign/ under bench record benchmarks/records/<stamp>-humbert-genus5-dlp-filter.json.
S3 checkpoint unavailable (no AWS identity); the branch push is the durable copy.
PR #4065: merges check fails on merge 5598316 (Binary-Regenerated trailer missing); waiting on the
owner's choice (force-with-lease or a replacement PR). crypto-autoresearcher #3282 needs vault KEY.

## Next
Summarize (C0, F1–F5), figures, REPORT.typ/pdf, benchmarks copy, benchrec close, session log, push.
