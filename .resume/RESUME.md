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
E14 ended "not completed": RunPod terminated pod oubpztzdbbbo4s about 10:20 UTC 2026-11-09 (account balance 0) during E14b d1.
Write-up committed (e14/score.json, RESULTS.md E14, reports/run-20261109-e14). PR open from this branch.

## Next steps
1. Drive the E14 write-up PR to merge.
2. Any rerun needs a new amendment: first measure pe's degree-4 matrix size on 6:10:3 d1 locally; consider E14a first; needs RunPod funds.

## Runs in flight
None.

## Stop and success conditions
Nothing retried. Failed predictions stay failed.

## Open questions
Whether to fund and register a rerun (user decision).
