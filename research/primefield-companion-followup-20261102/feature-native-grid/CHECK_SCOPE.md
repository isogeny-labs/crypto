# Native grid software checks

Three fixed local unit methods: exact saved labels/projective coordinates/
affine coordinates/inverse-Z for the three original public setup grids, plus
independent affine known-multiplier checks; identity, inverse, doubling,
equivalent-scaled coordinates, singleton batch and zero-y controls; invalid
curve/point/partition and empty/infinite batch rejection. Test affine reference
uses Fermat inversion independently of production extended Euclid.

One standalone rustc build capped120seconds and one invocation capped30seconds,
source and fixture locks committed first. No fresh sign-source call, unknown
scalar target, recovery cohort, benchmark or cloud job. This ports public known
multipliers, not a collision/search procedure. Caller certifies prime p and
subgroup order r; fixed-width backend p<=u64::MAX/2. Unit timing is diagnostic.
