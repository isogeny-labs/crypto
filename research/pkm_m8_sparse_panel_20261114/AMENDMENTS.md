# Post-freeze amendments and clarifications, protocol PKM-M8-PANELA-20261114 v2.2  [pcc:unverified]

Freeze: commit `97def9de33ea3b234d8bdcebb5e2732c5d2f889f` (`FREEZE.json`). Every entry states the UTC time and what
had been seen when it was written. No data-seed row had been run at any entry below unless the entry says so.

## A1 — 2026-11-14T23:50Z, analysis clarifications (seen: pilot seed 777, mini dry run on m = 4 rows; no data row)

Written while `analysis/build_results.py` was built, before launch. They resolve wording, not thresholds:

1. **Reading B (setup share)** cannot be evaluated in CPU seconds: `cell.json` records no base-construction CPU
   time and the target mask is recorded as counted operations (`verify.json` `work.target_setup`). Reading B is
   therefore reported in counted operations beside reading A and does not enter the verdict; NOT KILLED is decided
   on reading A alone. (Both NOT KILLED and its failure are reported as INCONCLUSIVE with direction, so the verdict
   set is unchanged.)
2. A planned natural row with no record counts as "did not complete": at t = 2 it makes `γ_lo` undefined (as
   frozen); at t = 3 it also blocks KILL (a minimum over fewer rows is anti-conservative).
3. `γ_hi` is computed over completed t = 3 rows only; any censored t = 3 row blocks NOT KILLED (censoring reading);
   the lower bound including censored rows is printed beside it.
4. Required gates for KILL and NOT KILLED: exporter gate (c), second instrument (m = 4, 5, 6 pairs; the m = 8 pair
   when it completes), t = 2 planted (positive), negative, A/A, canary. Gates (a) and (b) are pre-run (tests and
   `PILOT.md`) and are reported, not re-evaluated. A required gate that is not evaluable blocks both verdicts.
5. `ρ_AA` is the ratio of the two `aa` rows; the natural t2r9 SAT 1 row (same target) is reported as an extra.
6. Canary drift: flagged when the ratio to the first canary exceeds 1.5; a ratio below 1/1.5 is reported, not
   flagged; a canary whose `up_to_degree_8` round set differs from the first is not evaluable. Every B ≥ 8 row
   (probe included) is attached to the nearest preceding canary.
7. "Algebraic solutions found" is reported as msolve's distinct-solution count (`check` variety gate) and the
   F_p-rational solution count from `verify.json`, both shown.
8. Not computed by the pipeline: `E_all(γ)` for C3, the gap to the matched rho, any rho ranking (§5a). Rho rows are
   listed as references only.
9. "Verified natural positive at t = 3" = complete, label `sat`, consistent with the classifier, every real
   decomposition recovered, at least one real decomposition.

## A2 — 2026-11-15T00:00Z, orchestration hardening (seen: smoke pods on 2-row/61-row m = 4 mini plans only; no data row)

Measurement commands, caps, plan and cells are unchanged (`pod/plan.json`, `pod/bundle.tgz` and `cells/` keep their
freeze hashes). Orchestration files changed so a 72 h run survives the loss of the controlling session and cannot
bill past its deadline (`pod/HARDENING.md` has the smoke-pod evidence):

- `pod/job.sh` (+17): (i) msolve identity gate before row 1: a missing or wrong `/usr/bin/msolve` writes
  `done.json` with `"aborted":"msolve_identity"` and exits 5 (before: an install failure was only recorded and the
  plan ran on whatever msolve was present); (ii) a `STOP` file created by the pod-side guard at the deadline makes
  the remaining rows `not_run` (reason `deadline`) in `runs.jsonl`. New sha256: dd243da0abb0723e964b94d5d7329c570412002bb9b9e9f77a2ec100719b7bd4.
- `pod/guard.sh` (new, pod side): at the deadline creates `STOP`; after `done.json` + `fetched.flag`, or at the
  deadline + 6 h grace, stops the pod's compute with `runpodctl stop pod` (a bare `kill 1` only restarts the
  container and keeps billing, measured). sha256: 667ea094e069bb1ecc392a05a9477c7c83c197d718c4aded1db64dcf965809e1.
- `pod/launch.sh` (+123/−22): starts the guard; writes `launch.json` (pod id, $/h, created and deadline epochs);
  sets `fetched.flag` after a sha-verified or partial fetch; `--attach NAME` mode for a resumed session; copies the
  pod state to `.resume/pod_state/` and a `## Pod in flight` line into `.resume/RESUME.md`; 3× retry on `put`;
  pod-status check in the poll loop. sha256: 88de4f76925c7df5a012a413c30f10843aeea677a1983b9ba42eb4b6e9a5afd5.
- `pod/podserver.py` (+8/−1): SIGTERM handler; re-stop if the container restarts after a guard stop.
