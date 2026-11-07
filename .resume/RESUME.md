# Resume: crypto @ claude/curve25519-index-calculus-i674j4

## Goal
Subagent task (2026-11-07): implement and measure an F = 1 wwalk variant with the target inside the
walk (`wwalk:...,tw=1`, plus a separate chained-jump option `cj=1`), single-target ecbench sessions,
k = 20 and k = 24 Curve25519-family miniatures, vs the current F = 1 arm (x = 0.7) and rho-neg.

## Lead / hypothesis
Every collision of E_t = u P1 + z G + b Q is a row du*s1 + db*log Q = -dz; stopping at rank 2 puts the
search phase at the two-collision floor 1.329 sqrt(r), below the current table-then-probe split
(~1.6 sqrt(r)). Predictions P1-P7 are frozen in the protocol.

## Spec
research/curve25519_f1_targetwalk_20261108/PROTOCOL.md (frozen; hashes in data/frozen.sha256), specs in
specs/k{20,24}-s{0..7}.json, model scripts/model.py -> data/model.json.

## State
Code: TargetWalk in src/cryptanalysis/ic_boundary.rs; tw/cj options on WWalkOracle
(src/cryptanalysis/ic_framework/plugins.rs); plan checks and help text in
src/cryptanalysis/ecbench/methods.rs. Tests pass (data/cargo-test-wwalk.txt, 9 passed). Release binary
in /home/user/ctarget-f1/release/ecbench. Benchrec records opened:
benchmarks/records/20261107T2153Z-curve25519-f1-targetwalk-k{20,24}.json. Nothing committed (the
main session lands the work; this subagent must not commit, push or open PRs).

## Next steps
Done 2026-11-07 22:10Z: both sizes ran (16 sessions, all audits OK), analysis, figures, README.md,
REPORT.pdf, benchrec records closed (raw upload pending: no AWS credentials here). Remaining for the
main session: land the work (commit/PR), raw upload, validation gate if a row is ever wanted.

## Runs in flight
None.

## Stop and success conditions
About 1-2 minutes of sessions per size; cap 30 minutes in total. Any session that fails its audit is
excluded and reported, not rerun.

## Open questions
None blocking.
