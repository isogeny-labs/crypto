# Resume: crypto @ codex/retained-chain-ic-20261109

## Goal
Retain Kummer power and elliptic-addition chains, compare orders through m=4,
then collect independent relations and solve complete public toy pipelines.
PR destination: isogeny-labs/crypto.

## Lead / hypothesis
Variable layout/addition tree may change F4 support/degree/elimination work.
No scaling benefit is assumed. All projective intermediate charts are covered.

## Spec
research/retained_chain_ic_20261109/PROTOCOL.md; native ecbench command and
pipeline-m3.json/pipeline-m4.json. EKS Spot, 4 CPUs/4GiB, one concurrent job,
1800-second deadline, zero retries, USD5 aggregate build/trial ceiling.

## State
Rust core and command type-check. Final default-feature focused checks: 15 pass,
1 existing ignored. Command/spec-plan unit test passes. Full library attempts
were interrupted after unrelated archived-fixture/proc-namespace failures.
PR: https://github.com/isogeny-labs/crypto/pull/3765 (regular; auto-merge requested).
Implementation source: 1e6184a7935f016fa1fa28cba5864ef964ada387.
No paid build or cloud trial.
Checkpoint gate failed: upload route unavailable. Live budget unknown; GitHub
workflow metadata readable but variables endpoint HTTP403. See admission.json.

## Next steps
Publish the final validation/handoff update and follow PR #3765 under the
repository check-in cadence; no subscription/scheduling tool is available. Before cloud build/trial, establish
configured checkpoint upload plus live balances/reservations/capacity. Preserve
and hash all failures and producer observations; do not substitute local trials.

## Runs in flight
None. Scientific phases were never submitted; development checks have stopped.

## Stop and success conditions
Do not launch while checkpoint gate fails or shared budget is unknown.
Study ceiling USD5, original deadline1800s, no retries. Success requires actual
source-bound EKS receipts, complete panel/failure accounting, rank/scalar replay,
and independent timing/PCC admission where claimed.

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
