# Quartic matched-rho may be overpriced ~1.73x on existing IC rows

Status: open (review requested from the owners of the plane-quartic IC rows).
Raised by lead L10 of the genus>=2 pass, 2026-11-16. All numbers `[pcc:unverified]`.

## What it blocks

Trusting the IC/rho gap printed on the existing plane-quartic index-calculus
rows. If the correction below is right, those rows overstate how far index
calculus already sits below rho on quartic targets by a factor of about 1.73,
so any ranking or leaderboard statement that uses them is off by that factor.
It does not create or remove a below-rho result; IC stays below rho on the
affected rows either way.

## The gap

L10's accounting arm (H1) found that the matched rho those rows compare
against is a negation walk priced at `(A + N) / (sqrt(2) * A)` times the plain
step cost, i.e. about 1.73x too expensive, where `A` is the plain step
multiplication count and `N` the extra work the negation map adds per step
(re-canonicalisation). Measured `F_Q = 1.7338, 1.7269, 1.7322` at
`p = 4099, 8209, 16411`, per-curve spread 1.714 to 1.734 over 9 curves;
`A + N` reconciled with the lab's executed `Q_rho_neg` rows to within 0.4%, so
H1 reprices what the lab actually ran, not a model of it. Evidence:
`research/g2plus_leads_20261116/L10_c34_picard_matched_rho/` (RESULTS.md H1
table, ASSESSMENT.md sections 1-2, raw/rhocost.jsonl).

Effect on the rows L10 names (IC/rho, before -> after the 1.73x correction):
- `genus_ladder_bound_r` 2LP/rho: ~0.0089, 0.0086, 0.013 -> ~0.0154, 0.0149, 0.0225.
- `genus_ladder` at p = 1031: ~0.027 -> ~0.047.

Separately, L10 found that for Picard targets the correctly matched rho is the
`<rho>` class walk (argmin S3), about 0.447 of the plain negation walk the rows
use; that is a second, larger mismatch for the Picard subfamily specifically.

## What is NOT established

- H1 and H2 are model values: ideal step counts times measured per-op costs.
  No plain-walk or `<rho>`-walk rho was executed end to end, and fruitless
  cycles and walk overhead are not modelled. A cheaper negation formula, if one
  exists, would shrink `F_Q` and this correction.
- L10 edits no row and files no leaderboard change; this record is the hand-off,
  per that lead's PROTOCOL section 7.

## What would unblock it

An executed negation-walk and `<rho>`-class-walk rho on the same quartic
Jacobians, charged in the same unit as the IC rows, under the validation gate
with a replay certificate; then the row owners correct (or refute) the matched
rho on each affected row. Note the lab-source caveat: L10 ran against
`plane_quartic_pic.rs` at hash 3bf45c3a, and the main branch lost most of src/
in a bad merge on 2026-11-16, so the current file must be recovered before a
re-run (a built instrument from commit a552dc2ee8 was used by the follow-up
pass).
