# Shared seeds and retained preparation — T-1491

## What was done (commands, files, features)
Read the exact seed setup and Oracle recurrence in mixed-projective-strides-v2/run.py and the checked native source. Added immutable borrowed PreparedSource with retained geometry, stored base phase, four anchors and shared recurrence. Prior direct-phase window remains a reference API. Seeded raw entry stays private.

## Decisions made (and why)
Retain the historical anchor-reset algorithm. Source calibration supplies the base phase without recomputing it. Affine doubling for the 2P anchor is setup work. One inversion and four field products per nonanchor seed recurrence remain explicit; no timing claim. Source and original full cohort remain prime-field only.

## Open threads (unfinished, pick up next time)
Run fixed integration checks; bind the shared source to native decoder and add complete costs. Investigate removing recurrence inversions only with a compatible seed representation.

## Errors (what went wrong, how it was handled)
No cloud submission or infrastructure change. Prior source files remain immutable.
