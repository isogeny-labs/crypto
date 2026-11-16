# Pre-run closure of the last source/carrier orbit pair

The 24 source-mask-38/80 cells completed with zero exact identities. A
read-only PGL2(F3) coordinate check partitioned the twelve carrier masks
into two six-model orbits, represented by 9735 and 17371. The prior
source-mask-11 degree-five screen was only on carrier 9735. Since source
mask 11 has its own five-element Frobenius orbit, a degree-five screen on
`(bmask,fmask)=(11,17371)` and exact coordinate transformations will
decide the remaining 30 equation pairs in the recorded slice.

Before execution, freeze `screen_11.sage` for that exact pair and
`carrier_orbits.sage` for all 24 projective linear coordinate classes.
The latter computes `(c x+d)^12 f((a x+b)/(c x+d))` exactly and retains
every monic odd-degree model reached from each representative. An exact
zero-hit cell plus two six-model orbit receipts would extend the
degree-five obstruction to all 240 recorded carrier/source equations.
