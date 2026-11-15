# Projective atlas calibration handoff

## What was done
Added homogeneous X/Zchart with original-ideal reconstruction, transformed
F/N_i and crosschart comparisons. Samecalibration/mixedcohort and exactoldM
regression required. Built locked release, not executed.

## Decisions made
Retain failedfit and diagnostic, no input or gate weakening. Freeze/checkpoint
before run. Other unsupported charts/full inverse/fullcost remain open.

## Open threads
One atlasgate, then actual source input lifting if it passes.

## Errors
Pre-freeze offline lock regeneration after rename; duplicated helper removed
before successful build. No scientific execution or rerun during those fixes.
