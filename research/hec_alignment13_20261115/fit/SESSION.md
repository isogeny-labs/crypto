# Source projective calibration handoff

## What was done
Pinned/copied prior source evaluator, kernel/alpha inputs and centered target
coordinate worker. Built offline locked Cargo release, not executed. Protocol
registers12fit+4scalar holdouts+4mixed holdouts, known image certificates only.

## Decisions made
Keep setup/calibration explicit, no cold cost assertion. Raw source Bdegree2
unsupported, fixedtranslateB+D1needed before later inversion. No targetscalar
input. Prior target centering gate stays unchanged.

## Open threads
Freeze/checkpoint, execute once; actual inverse lift afterward if alignment holds.

## Errors
No build failure. Sourcepathlocalized to copied arith.rs; original unchanged.
