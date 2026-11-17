# Actual small-degree source denominator poles

Frozen before data,2026-11-17. Native development qualifier,600seconds/1GiB,
same fixed source/quartic/global inputs and unsigned standing. No performance
or paid run. Reuse source snapshots from closed-transpose-v1.

Take the radical of the exact common U,V denominator. Extract every
irreducible factor of degrees1,2,3 by distinct-degree Frobenius gcds and the
existing bounded deterministic equal-degree splitter. Replay factors and check
irreducibility. Recheck the remaining factor has no degree1/2/3 factor;
do not claim its complete factorization.

For each extracted factor f, work in Fp[X]/f. Test whether h has a square
root (including zero); use exact finite-field exponentiation and Tonelli
iteration when needed, retaining any failure. A root v in this field gives
an actual rational reduced Mumford class (f,v); include both signs when
distinct. Restore the original extension context before source arithmetic.

These classes contain actual denominator-pole support. Require automatic
forward movement, exact principal moving identity, denominator-coprime moved
support, and a complete finite target cycle on the exact quartic. Retain
all factors, roots, skipped nonsquare cases and image witnesses. A nonsquare
over degree e gives a point of degree2e; for e=1 this is not found by merely
using u=f and must be classified separately. The report must not call this
an exhaustive list of all reduced-class exceptional supports.
