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

---

# Resume: crypto @ claude/hky-progress-growth-law-20261115 (growth-law cells rerun)

## Goal
Owner (2026-11-15): "Yes go ahead with the engine fix and rerun"; budget "All three cells, 24h each, $80".
Decide d_F at the three open Conjecture G cells of the PKM/Kummer tower (prime_ic_leads_20261110/growth_law).

## Lead / hypothesis
Conjecture G: d_F(W,t) = ffd(W) + max(0, floor((t - s_W)/4)). Registered predictions:
(a) W=1,t=12: d_F=6; (b) W=3,t=13: d_F=7; (c) W=2,t=14: d_F=7.

## Spec
research/prime_ic_leads_20261110/growth_law/rerun_20261115/PLAN.md (frozen before data) and cells_job.sh.
Binary pkm_tower_pilot from commit in source_commit.txt, sha256 in binary.sha256 (checked on pod).

## State
Engine fix committed (fall_degrees_with + --hky-progress). Pod about to launch.

## Runs in flight
RunPod pod bdr-growthlaw (cpu5m, 16 vCPU), job /work/in/cells_job.sh, outputs /work/out.
Kit: research/runpod_kit_20261112/rp.sh (get/exec/delete). Cleanup: rp.sh delete bdr-growthlaw.

## Stop and success conditions
Pod deleted at ALL_DONE, on BINARY HASH MISMATCH, or before spend exceeds USD 80 (~72 h at USD 1.06/h).
Reading rules in PLAN.md (holds / fails / censored). Partial: decided degrees from out/prog_*.jsonl.

## Next steps
Poll at most every 4 h: rp.sh get out/ into rerun_20261115/out/. At ALL_DONE: copy back, delete pod,
close benchrec record, write REPORT + figure + PDF, update PR.
