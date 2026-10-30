# Canonical export for a phase that never started

- **Status:** `open`
- **Kind:** metadata-contract gap for an honestly stopped incomplete experiment
- **Affected work:** [NIST actual attempt 001](../research/nist_skip_criterion_20261025/generator_b_execution_20261029/reports/attempt-001/REPORT.md)
- **Next action owner:** canonical experiment-protocol maintainers, unassigned
- **Last updated:** 2026-10-29

## Problem

The canonical checker refuses a truthful two-row export when a frozen
three-phase protocol stops after production times out and verification never
starts. The completed version2 comparison is admitted separately; this gap
concerns the preserved failed version 1 only.

## Evidence and uncertainty

- **Observed:** protocol 1 plans positive, production and verification. Positive
  exits 0, production exits 124 at 300 seconds, and the stop-on-failure launcher
  never invokes verification. There are exactly two real phase status files.
- **Observed:** canonical tool revision 8c5db250d4fff65eb40fffede9c8391de403897f
  reports four findings: missing verification arm,2 versus 3 measured rows,
  verification instance shortfall and unequal instance identities. The
  [actual refusal](../research/nist_skip_criterion_20261025/generator_b_execution_20261029/evidence/export-v1-refusal.txt)
  is preserved. The partial export's normalized row-flow says planned 2;
  the actual frozen plan remains3. That export is not admitted.
- **Missing capability:** a supported representation/admission rule for a
  protocol phase never attempted because an earlier required phase failed.
  We have not changed the shared schema or independently established that
  every possible existing representation fails.
- **What this does not establish:** no complete point comparison or verified
  census can be inferred from the timed-out attempt. This is not a reason to
  weaken the all-row gate or fabricate a cancelled verification execution.

## What would unblock it

- [ ] Canonical maintainers document or implement a supported stop-on-failure
      representation retaining planned 3, attempted 2, completed 1 and
      not-attempted 1, without counting a nonexistent verifier run.
- [ ] Add positive and malformed controls in the canonical tool repository;
      retain its refused historic export and link an additive corrected one.

## Next discriminating step

Review the canonical contract for explicit unattempted stages and dependency
stops before proposing a shared change. Do not patch the scientific study's
status files or rerun an old failed phase solely to make metadata green.
No canonical-tool patch or background task is scheduled by this record.

## Resolution and history

2026-10-29: recorded the actual checker refusal, preserving the absent phase.
The separate complete version2 export passes file verification with zero
findings. Unresolved for version 1 metadata; the actual failed attempt remains
fully reported and archived.
