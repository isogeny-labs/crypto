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

## First fit gate failed and retained
Source6d012eba2099/checkpoint202911 preceded one exit101execution.
Rank63/invertibleM,12training/4scalarholdouts andfirst3mixedinputs pass.
Fourth B+2D1target chart unsupported. No fit-input replacement or rerun.
Next diagnostic on exactly this target class, before corrective amendment.

## Same-class diagnostic completed
Sourced3cb538e6232/checkpoint203329 preceded one diagnostic exit0.
The fourth class has no original affine graph. Its annihilator contains a
functional supported at infinity[1:13:0] (row2-39row1), explaining chartfailure.
Next projective X/Zswap, transforming curve AND cubic numerators, with crosschart
checks and identical calibration/mixedholdouts. No fitrerun yet.
