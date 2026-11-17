# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (round 5: GPU m = 3 with incremental-gauss-echelon)

## Goal
User (2026-11-15), "Keep working on open leads". Lead from research/ic_la_m3_20261115 (PR #4310):
incremental-gauss-echelon matches incremental-gauss (trials, relations, logarithm, S) with 0.10-0.15x the
row_ops. Measure the whole-solve effect on the round-4 GPU arm (probe_build=pairs, daemon, cuda).

## Lead / hypothesis
PROTOCOL.md: identity across IC arms; at |F_x| = 4096 the GPU arm's median solve with echelon <= 0.5x
with gauss; row_ops ratio 0.10-0.20.

## Spec
specs/smoke.json, specs/rf3la.json (38/40-bit prime_explicit curves, |F_x| 2048/4096, 3 targets, 2
rounds; CPU and GPU arms x gauss/echelon; GPU echelon A/A at 4096; rho and rho A/A). job.sh on one A100,
launched by run_pod.sh under benchrec run.

## State
2026-11-15 19:39Z: done; pod deleted. 168/168 verified+replayed, 84 identity checks equal. GPU m=3 solve
with echelon 0.18-0.20x gauss at |F_x|=4096 (0.28-0.31 s), 0.41-0.58x at 2048; 29-90x one CPU thread.
REPORT.md; records r2 (censored, launch failures) and r3 (measured).

## Runs in flight
None.

## Stop and success conditions
Any S/relation/logarithm mismatch or failed replay voids that comparison. Wall L0, A/A floor. Cap
~1.5 GPU-hours, $4. S3: same waiver as earlier rounds (session AWS keys rejected).

## Next steps
Validator check; set-up at 4096 and the relation phase are the next bounds.
