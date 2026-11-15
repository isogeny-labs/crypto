# Resume: crypto @ claude/hky-progress-growth-law-20261115 (growth-law cells rerun)

## Goal
Owner (2026-11-15): "Attack the algebraic gap and 1" — single-target IC toward rho parity: (1) the C = 1 arm toward its 1.5x generic floor; (A) the algebraic gap.

## Spec
(1) Rerun the frozen C = 1 confirmation spec research/curve25519_autolab_20261104/c1/spec-confirm-k32.json unchanged (copied to research/ic_c1_headroom_20261115/spec-k32.json) with `ecbench run --cpus none --allow-busy` (L0), to get per-phase GAE locally (the original records are in the artifact store, unreachable without AWS). Then split C = 1 S into setup vs walk vs floor 1.329 and compare with rho-neg's split.
(A) Finding so far (no run): the algebraic screen's best row (m = 2 root-find per base point) is the generic rootfind-subtract sweep; on the real algebraic base (L13) even a free oracle is ~17x behind rho at w = 0 (coverage-limited).

## State
Complete: rerun reproduced C = 1 S = 1.545; phase split and algebraic analysis written in research/ic_c1_headroom_20261115/README.md; record closed.

## Next
Phase split, README + graph, close record, PR; report to owner.

---

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
