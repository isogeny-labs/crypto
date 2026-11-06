# Resume: crypto @ rule-v9 (local) — rule v9 re-pricing, 2026-11-06

## Goal
Re-price the Koblitz one-target rule at today's main (rule v9), with rule v8's
binary as a same-session control (v9c), so the leaderboard's Table A n53 row
(still rule v7, 7.03× against an unmatched rho) can be read from a current,
matched, validated run. The aimed-subset gather (#2448) is among the changes.

## Lead / hypothesis
`research/ic_tool_program/rule/v9/PROTOCOL.md`: IC cold v9/v9c at n53 in
[0.85, 1.00]; same walks and logarithms in both runs.

## Spec
- Protocol: research/ic_tool_program/rule/v9/PROTOCOL.md (+ ../v9c/PROTOCOL.md), run.sh.
- Binaries: /tmp/claude-0/rulev9/bin, hashes in rule/v9/binaries.sha256.
  ic-v9 from main f465f2fb1b03; ic-v9c from 370e19193f6f; icprog from local
  commit 9d76e4de0918 (branch rule-v9, not pushed: the designated branch
  aburan28/laughing-darwin-3qdnk2 carries open PR isogeny-labs/crypto#3516).
  Rebuild: CARGO_TARGET_DIR=/dev/shm/tgt cargo build --release --bin ic
  (and --bin isolated_bench --example koblitz_rho_fixture at f465f2fb; --bin icprog at rule-v9).
- Bench record: benchmarks/records/20261106T0330Z-ic-rule-v9.json (open).

## State
Protocol committed (65a61d758c); no row run yet.

## Next steps
1. `bash research/ic_tool_program/rule/v9/run.sh /tmp/claude-0/rulev9 benchmarks/records/20261106T0330Z-ic-rule-v9.json`
   (resumable; log /tmp/claude-0/rulev9/rule9-run.log).
2. Report/graph/PDF; validator (replay certificate + report); VALIDATION.json;
   benchmarks entry + row; leaderboard Table A Koblitz rows from v9 if
   validated; benchrec metric + close; pcc claims.
3. After #3516 merges: push to aburan28/laughing-darwin-3qdnk2, open PR (ready, auto-merge).

## Runs in flight
none (until step 1 starts)

## Stop and success conditions
Rule v8's stop rule (failed row check, logarithm differing from §23, failed
pin, rho slower a step than the strong walk). A measurement; no acceptance decision.

## Open questions
Leaderboard Table A reads rule v7 while rule v8 (matched rho) exists; v9 would supersede both.
