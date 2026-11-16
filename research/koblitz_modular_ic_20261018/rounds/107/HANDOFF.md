# 2026-11-05 R107 handoff

## What was done

Evaluated full R51 G51 over both R106 base completions modulo s^32,
translated lambda to F4, and found exact Newton orders
`[28,23,23,0,0]` and `[27,23,23,0,0]` at u exponents 0,1,4,8,16.
Fresh replay, changed-hash control, independent Hasse composition and
Lean hull/degree arithmetic passed. The Newton criterion gives four
normalized x=1 closed places with residue F4 and ramification
indices 3,21,4,28.

## Decisions made

Use the degree-1 and degree-7 F4 branches separately when evaluating
N51. The four-place result resolves R101's two nonreduced source-order
points at the field-completion level.

## Open threads

Construct explicit lambda branch series and N51 valuations, then the
degree-7/11/22 and x=0/infinity supports. The shared dated log remains
reserved by unrelated T-1590.

## Errors

No R107 producer or replay failure. PR #3370's `migrate` CI check still
fails before project tests because trusted checkout lacks
`.github/scripts/artifact_prefetch.py`; `review` passes. The PR API head
lagged behind the pushed Git ref during R106 publication; reconcile exact
head checks again after R107 is pushed.
