# Lead disposition: independent review result (2026-11-01)

Supersedes `PROPOSED-lead-disposition.md` (do not apply that text literally). Full review: `validator_report.md`.

## Review outcome (validator-breakthrough, blind stage then lifted)
* Verified by the validator's own code: sizing and gamma_design re-derive exactly; all 134,340 null and 20,151 structured counts recounted with zero
  mismatches; order certification passes on all 6232 curves; slopes, classes (22), C3-C6 and the reading O-SUB-BE reproduce
  (CI endpoints within 0.0008; max upper95 delta 0.0141-0.0145; upper95 kappa(32) 1.11).
* Evidence adequate, but **not admitted as a receipt**: records were written outside the handoff's write scope, manifests record commit null, there are no supervisor
  receipts, and no Coordinator snapshot of these bytes. Formal verdict: incomplete. This is paperwork, not a defect in the counts.
* Verdict: **CONCUR-WITH-CONDITIONS**; DISSENT if applied literally to the lead's own `disposition` field.

## Conditions before anything is applied
1. Admit the records: a supervised run of EXP-PFDR-ada71b whose digests match main-jobs.jsonl `fe4b23ab...` and pilot `73612c5d...`, or a Coordinator archive of these exact bytes.
2. Record the result as a new scoped row; the parent lead `LEAD-ICP-coincidence-harvest-structured-base` stays `open_next_step` with the FAM-1 ladder as its next step.
3. A committed Coordinator decision cites this review.

## Wording the validator requires (replaces the earlier proposal)
```
disposition: exhausted_at_scale
obstruction:
  kind: measured
  statement: >-
    x-coordinate window and rank-2/3 label-GAP point bases (theta 0.30/0.35, b 28-2352), prime-order E/F_p, 16-32 bits, h=2:
    no detectable growth of kappa (max upper95 delta 0.014 vs 1/3) and no detectable excess (kappa 0.916-1.058; upper95 <= 1.14 per rung,
    1.11 at 32 bits). Bounds elasticity only; kappa-1 below ~0.1 not excluded.
  measured_by: [admitted EXP-PFDR-ada71b RUN ids]
  revisit_when: rungs above 32 bits; other theta; h >= 3; a label base with a named mechanism
not_covered: [FAM-1, log-GAP growth, bbc53e, target preselection, j=0/1728]
forecloses: [pair-sum excess of x-window/label-GAP point bases at h=2, 16-32 bits]
```
Do not write "no structural relation source on F_p"; say these six lines show no detectable pair-sum excess. Quote 0.014 and 1.11, not four digits.

## Status
Lead YAML unchanged. Nothing was applied: conditions 1 and 3 cannot be met from a session without push access to crypto-autoresearcher.
