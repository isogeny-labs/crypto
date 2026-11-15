# Resume: crypto @ perf/solve-profile-20261106

## Goal
Lane 2 of the 2026-11-06 parallel workflow: profile where a complete index-calculus solve spends its time (prime-field ecbench ic.pipeline mitm-prime-batch:m=2 at 28 and 32 bits with targets walk and walk-prime-batch-4/8/16; Koblitz compact-orbit examples/koblitz_orbit_dlp_fast_online at n=41). Phases: relation collection, table build, linear algebra, descent. Output: study dir research/ic_solve_profile_20261106 with hotspot table, phase bars, ranked Amdahl list, PDF; PR ready for review, no auto-merge, no merge.

## Lead / hypothesis
Explains cost only, claims no speedup. Hypothesis: relation collection (target walk + guard/probe) dominates the prime pipeline; the "137 ns floor" (crypto#2844 / PR #2806 diagnosis) is inflated by the diagnostic's own clock reads. Kill: if timer-off perf attribution of the probe/guard matches the timer counters within error, the bias is small.

## Spec
Inputs: ecbench ProfileInput JSON built from research/ecbench_ic_asm_euclid_20261103/data/spec-p32-batch-arms.json pattern; 28-bit curve prime_search seed 59297, base 512; 32-bit seed 20261018202 base 1024. Host has no hardware PMU (perf cpu-clock sampling only): cycles/IPC are refused, not estimated.

## State
Sparse worktree /home/user/wt/solve-profile; release build (line-tables-only debug) on /dev/shm/target-solve under flock; Cargo.toml locally stripped of bin/example sections (do not commit it).

## Next steps
1. Wait for build; run ecbench exec children under the CPU lock, interleaved, repeated.
2. tools/profile.sh cpu per target; analyse; write study dir, session log, PR.

## Runs in flight
None yet.

## Stop and success conditions
Stop if checkpoint push fails or the CPU lock cannot be obtained; report that plainly.

## Open questions
walk-prime-batch-32 is not on main (4/8/16 only).

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
