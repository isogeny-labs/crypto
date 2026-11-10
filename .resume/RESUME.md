# Resume: crypto @ claude/exponent-study-20261108

## Goal
Exponent study (owner preregistration 2026-11-08): fit the exponent of the cost T(E,m,L) of ONE
decomposition system against |V| for multiplicative-subgroup bases on the Curve25519-family
miniatures, m = 2, 3, 4, symmetrised system in the elementary symmetric polynomials. Success rule:
every verified measured improvement counts; report distance to rho.

## Lead / hypothesis
Lever iff m = 4 exponent < 0.5 (upper 95% bound too) or m = 3 exponent < 0. Prediction: m = 2 about 1.5,
m = 3 about 3.5, m = 4 >= 4.5 or capped; no lever.

## Spec
research/curve25519_exponent_study_20261108/PROTOCOL.md (frozen, data/frozen.sha256). Code:
src/cryptanalysis/ic_framework/subgroup_semaev.rs (6 unit tests pass), `curve25519_family f4sub` and
`subcensus`. Binary /home/user/ctarget-f1/release/curve25519_family. Run: python3 scripts/run_grid.py
(4 workers, cells/*.json, resumable: finished cells skipped). Analysis: scripts/analyse.py.

## State
Local worktree /home/user/crypto, branch claude/exponent-study-20261108 (not pushed by instruction).
Conductor, ecdb and research-kb unreachable. Disk about 2.6 GB free: no second target dir.

## Next steps
1. Run the grid; census (subcensus); calibration (bits base vs tower f4cost G=0).
2. analyse.py; README.md with [pcc:unverified] result sentences; figure; REPORT.pdf; pcc lint-prose.
3. Session log docs/session-logs/2026-11-08.md; local commits with the two attribution lines.

## Stop and success conditions
Stop when every grid cell is complete or censored, the analysis, README, figure, PDF and session log
are written and committed locally. Success: fitted exponents with intervals per m (or a stated
censoring), verdicts on the preregistered thresholds, failed predictions kept as failed.

## Runs in flight
Grid launched by scripts/run_grid.py in the background on this machine; cells land in
research/curve25519_exponent_study_20261108/cells/. Relaunch the same command to resume.
