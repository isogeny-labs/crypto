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
First attempt (local) lost with its container at ~16:36 UTC; no rows survive.
Protocol amendment 1 moves the run to Iso Lab EKS (rule/v9/launch/). Binaries
rebuilt (hashes in rule/v9/binaries.sha256; both pins held locally). The
isolab daily budget read 202.84/200 at 17:16 UTC 2026-11-06: no submission
until it resets.

## Next steps
1. Publish the image: `research/ic_tool_program/rule/v9/launch/build-layer.sh /tmp/claude-0/rulev9 /tmp/claude-0/rulev9/layer rule-v9-20261106-<sha12>`;
   commit launch/image.json; `make-request.sh 1`; commit; push; check-experiment-git.
2. After 00:00 UTC: re-read DescribeBudgets (us-east-1); submit with
   remote/launch-remote-command request-a1.json only if the day has room for USD 12.
3. On a Spot loss: `make-request.sh 2 s3://…/runs/<job>/<pod-uid>/`.
4. After the Job: download, check SHA256SUMS, claims/analyse/matched locally,
   report/PDF, validator, VALIDATION.json, benchmarks row, leaderboard, PR.

## Runs in flight
Iso Lab Job rule-v9-20261106-a1 (submitted 2026-11-10 22:02 UTC on owner authorization over the
monthly budget; launch/README.md). Job UID 5a545fed-4434-4e26-bda4-ce55bee21c5e, pod
33053ca8-de9a-4639-bea4-a129ce6d5461, node ip-10-84-68-133 (AMD EPYC 9R45), deadline 43200 s.
Rows: s3://isolab-results-104186811093-us-east-2/runs/rule-v9-20261106-a1/33053ca8-de9a-4639-bea4-a129ce6d5461/
Both pins held in the pod. On a Spot loss: launch/make-request.sh 2 <that prefix>.

## Stop and success conditions
Rule v8's stop rule (failed row check, logarithm differing from §23, failed
pin, rho slower a step than the strong walk). A measurement; no acceptance decision.

## Open questions
Leaderboard Table A reads rule v7 while rule v8 (matched rho) exists; v9 would supersede both.
