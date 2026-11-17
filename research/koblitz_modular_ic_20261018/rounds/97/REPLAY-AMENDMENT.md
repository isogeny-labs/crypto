# R97 fresh-replay repair, frozen before corrected replay

The first fresh readback replay using the committed `REPLAY97.sing`
is **rejected**. It declared `donorF97r` as a string and then attempted
to declare a polynomial with the same name. Singular printed `?`
errors at the declaration and source comparison, yet later emitted
`REPLAY97_ALL_CHECKS_PASS`. The failed log is retained as
`replay-fresh97.log`; neither marker nor exit zero is an acceptance.

`REPLAY97_CORRECTED.sing` changes only that identifier collision:
the donor source string is `donorText97r`, and its parsed polynomial is
`donorF97r`. It uses the same fresh S3 readback, source polynomial,
derivative/resultant recomputation, specializations, full factor product,
degree weights and corrupted control. Accept it only with native exit
zero, exact `REPLAY97_CORRECTED_PASS`, and **no** `?` lines. Preserve
both replay logs. The producer is not repeated.
