# Resume: crypto @ claude/ic-coverage-20261108

## Goal
Coverage report of index-calculus work on the Curve25519-family miniatures
(research/curve25519_ic_coverage_20261108/REPORT.md) and the top untried directions run as
single-target ecbench studies under frozen protocols (research/curve25519_tw_jumps_20261108,
research/curve25519_tw_scaling_20261108). Success rule: every verified measured improvement over
the current best arm (twcj-x15, 1.40x rho-neg at k = 20, w = 0) counts; distance to rho reported.

## Lead / hypothesis
Study A: the target-in-walk arm draws jump indices independently of the point, so its collision
rate does not depend on the jump count for jumps >= 4; cutting jumps 16 -> 4 saves the set-up
(0.116 sqrt(r) at k = 20) with no walk penalty; jumps = 2 fails. Study B: twcj's distance to
rho at k = 28, 32 stays in [1.30, 1.55] as rho's set-up overhead vanishes.

## Spec
Protocols: research/curve25519_tw_jumps_20261108/PROTOCOL.md (frozen, data/frozen.sha256),
research/curve25519_tw_scaling_20261108/PROTOCOL.md. Specs in each study's specs/. Binary:
/home/user/ctarget-f1/release/ecbench (no code change). Launch: python3 tools/benchrec.py run
<record> -- research/<study>/scripts/run_sessions.sh <k>.

## State
Worktree /home/user/crypto-ic (sparse checkout; disk nearly full, do not build). Conductor not
installed. research-kb and ecdb unreachable from this machine. Sessions are local L0 counted
units only. Records: benchmarks/records/*curve25519-tw-*.json.

## Next steps
1. Run study A at k = 20, 24; analyse.py; README, figure, PDF; close records.
2. Run study B at k = 28, 32; same.
3. Coverage REPORT.md + figure + PDF; session log entry docs/session-logs/2026-11-08.md; commit.

## Runs in flight
None.

## Stop and success conditions
About 90 minutes of runs in total; failed predictions stay failed; no push, no PR.

## Open questions
Band pricing of table inserts (PR #3700 opt-in): both readings are reported.
