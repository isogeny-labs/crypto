# Target centering gate handoff

## What was done
Read Tian4.2.3 and pinned target/good section code. Derived group-reference
translation beta, implemented ideal-to-graph extraction and nonsplit residue
classification. Worker built/formatchecked, not executed. Protocol/fingerprints
ready for source freeze.

## Decisions made
Test fixed32public target-class controls with exclusions retained, minimum16
centered negation pairs, actual irreducible curve divisor and wrongcenter
negative control. No input isogeny or timing claim.

## Open threads
Commit/checkpoint then one run; preserve outcome. Source alignment and general
lifting/fullcost/genus2/4 remain open.

## Errors
No build error. Sharedresume untouched.

## Completed gate
Sourcec99ae1c7dc89/checkpoint202001 preceded one exit0execution.
29centered pairs pass, exclusionsn3/16/21retained,22irreducible/50nonsplit
actual curve divisors,25wrongcenter disagreements. No scientific rerun,
sourcealignment or timing result. Next source quotient calibration/holdout.
