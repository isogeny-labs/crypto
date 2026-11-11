# Resume: crypto @ aburan28/eloquent-bell-2jerrf

## Goal
Owner (2026-11-11): "we do number one, make row equal solid" — turn the multi-target IC4 result of PR #3624
(research/mt_fieldops_20261107) into a validated, admitted row: pre-registered stronger rho arms (rhoseq, rho2x),
fresh curves, independent validator, VALIDATION.json, pcc claim, bench record, benchmarks row, leaderboard.

## Lead / hypothesis
IC4 (x-only pair-sum table, shared inversions) cheaper per target than the strongest of four rho arms at
T >= 4096 for I/M in {3,10,30}, across h in {0,0.1,1}; dearer than paired BSGS. All [pcc:unverified].

## Spec
research/mt_fieldops_v2_20261111/PROTOCOL.md; runner run.py holdout; analysis analyze.py; 2112 runs, local L0.
Route: local (no AWS identity in this container; S3 checkpoint push impossible, Git push of this branch is the checkpoint).

## Next steps
1. run.py holdout under benchrec; analyze; figure/PDF.
2. Independent validator -> replay-certificate.json, validator-report.json; VALIDATION.json.
3. pcc claim; benchmarks/20261111-mt-fieldops-ic4/ + README row; leaderboard regen; PR.
