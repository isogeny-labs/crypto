# Resume: crypto @ claude/fall-degree-e14-20261109

## Goal
E14 of research/fall_degree_bounds_20261008 (Amendment 19): the E13 parts that ran out of 13 GiB locally,
on one RunPod CPU pod (cpu3g, 64 vCPU, On-Demand), hard cap USD 100.

## Lead / hypothesis
E14b: 6:10:3 controls d1,d3,d4 refute at degree 4 under the pe engine (completes E13b's independent check).
E14a: exact d_F at (3,21,7) via the degree-5 pe closure. E14c: (4,28,7) with hky_window w1x (H_ell at m=4).

## Spec
PREREGISTRATION.md Amendment 19 (commit a430db11e3). Pod script research/fall_degree_bounds_20261008/e14/run_pod.sh.
Source: frozen archive fd-src-a2be6609.tgz (sha256 18b14930...) in s3://ephemeral-runs-104186811093-us-east-1/jobs/inputs/.
Outputs: s3://ephemeral-runs-104186811093-us-east-1/jobs/e13-runpod-20261107/out.tgz (presigned PUT, reused key; valid to 2026-11-14).

## State
Amendment and script pushed on branch claude/fall-degree-e14-20261109. Pod being created via RunPod REST (rest.runpod.io/v1/pods).
Conductor not installed in this container.

## Next steps
1. Watch the pod; download out.tgz; verify hashes; unpack into e14/pod/.
2. Score E14a/b/c per Amendment 19 (adapt e13/score_e13.py); E14a deg-4 entries must equal e13/local/B_3-21-7_d*.jsonl.
3. RESULTS section, report/graph/PDF (reports/run-2026110x-e14), session log, PR ready + auto-merge.

## Runs in flight
RunPod pod "fd-e14-20261109" (id recorded in session log once created). Delete it if still running after its deadline.

## Stop and success conditions
Pod self-deletes at finish or at min(36 h, floor(100/costPerHr) h). Nothing retried. Failed predictions stay failed.

## Open questions
None.
