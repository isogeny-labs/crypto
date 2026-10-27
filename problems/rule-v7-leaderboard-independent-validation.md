# Rule-v7 Table A lacks independent leaderboard validation

- **Status:** `investigating` (independent evidence in [PR #2512](https://github.com/aburan28/crypto/pull/2512), hosted CI pending)
- **Kind:** validation evidence gap
- **Affected work:** [rule-v7 analysis](../research/ic_tool_program/rule/v7/README.md), [IC leaderboard](../docs/ic/LEADERBOARD.md), and [registry/browser PR #2452](https://github.com/aburan28/crypto/pull/2452)
- **Next action owner:** reviewer of the independent validation PR
- **Last updated:** 2026-10-24

## Problem

The separate [validation PR #2512](https://github.com/aburan28/crypto/pull/2512)
now contains a source-bound independent replay and metric check for all six v7
sizes. It passes the local leaderboard and validation gates on its exact head;
hosted CI and merge remain pending. This registry branch is still based on
`5cb5538`, so the baseline failure below remains relevant until that evidence
lands and this branch is synchronized. The computational replay does not prove
timings in Lean.

The six Koblitz one-target Table A rows at baseline v7 cannot pass the owner’s
leaderboard validation gate because their evidence directory has no independent
`VALIDATION.json` bundle and the rows are absent from the pre-gate freeze.
The published analysis covers n = 41, 47, 53, 57, 59, and 61, with 64 unseen
targets and four A/A repeats per size. Its cold IC/rho ratios and online
speedups remain observations until replay and metric recomputation are bound
to the gate; this record changes no result or scientific status.

## Evidence and uncertainty

- **Observed:** On exact crypto `main` `5cb553810e5bbfbd8691ec614ffcaacefc625757`, `python3 scripts/build_ic_leaderboard.py --check` exits 1 and names all six rule-v7 rows as lacking `VALIDATION.json` and a pre-gate digest. The same source and gate blobs occur in the local merge for PR #2452. The checked-in rows say `unvalidated (pre-gate)`, but the owner rule does not permit newly added rows to acquire that status without its frozen digest. [PR comment](https://github.com/aburan28/crypto/pull/2452#issuecomment-6389061911).
- **Observed:** The rule-v7 tree contains 408 producer-checked claim rows, replay files, raw price records, reference runs, and its analysis. These files are inputs for an independent review, not a completed gate bundle.
- **Derived or proved:** The validation gate failure and missing manifest are established for this exact tree. The rule-v7 performance metrics have not been independently rederived in this record.
- **Suspected explanation:** The dashboard was refreshed to v7 before the separate independent gate bundle was filed; the timing of that change is not itself proof of the metrics’ correctness or error.
- **Missing capability on this baseline:** A merged, exact-head validated source-bound scalar replay, metric validator, and matched strongest-rho reference report for all six rows. PR #2512 supplies those artifacts locally and awaits hosted checks.
- **What this evidence does not establish:** The failure does not refute the reported experiment, prove a speedup, or justify marking any row validated. The existing checked claims are not a substitute for independent validation.

## What would unblock it

- [ ] Independently replay the 408 frozen rows, including `[d]G = Q` for each retained answer and the retry exclusion rule; reject a deliberately wrong scalar.
- [ ] Recompute the six cold IC/rho ratios, online speedups, uncertainty intervals, and source digests from the frozen raw records, with mismatches and accounting scope explicit.
- [ ] Check that the rho arm is the protocol’s strongest available signed-Frobenius reference on the same target, process, and units; record the native-work band and any unpriced operations.
- [ ] File SHA-bound replay and validator reports plus a compliant `VALIDATION.json`, then pass `scripts/build_ic_leaderboard.py --check` and exact-head CI without `--allow-unvalidated` or a new pre-gate freeze entry.

Those conditions would validate the stated Table A rows under the repository
gate. They would not turn measured timings into Lean theorems or establish an
IC advantage over rho; the reported cold ratios remain above one.

## Next discriminating step

- **Action or proof obligation:** Build an independent read-only validator for the sealed v7 run, claim, and reference files, starting with one of the six sizes and extending to all 408 retained rows only after a negative scalar control passes.
- **Inputs and controls:** Exact rule-v7 `PROTOCOL.md`, `analysis.json`, `claims/`, `runs/`, and the existing Table A reference rule. Keep contended retries excluded and retain each source hash.
- **Execution limits and stop condition:** Replay only the frozen 408 rows and their declared reference controls; do not launch a new metered campaign under this record. Stop and report a missing source, mismatched scalar, changed digest, or unpriced native operation instead of filling the gap by assertion.
- **Outcomes and next actions:** A full match supports filing the gate bundle for independent review; a mismatch or missing input stays open with the exact row and cause; a code or infrastructure failure requires a fixed verifier and rerun on the same frozen inputs.
- **Recheck trigger:** A new independent validator receipt or source-bound correction for the v7 row set.

## Resolution and history

Unresolved on this branch. This record documents the 2026-10-24 exact-main gate
failure; PR #2512 provides the separate candidate resolution. Merging this
record alone would not validate the six rows.
