# Amendment 4: q=16 split-versus-nonsplit falsifier

14 November 2026, before execution. The first three field sizes suggest
that the q=8 incidence pattern differs when `3 | ell`, equivalently
`7 | (q-1)`. The hypothesis is that the earlier complement expression
`(q-1)(q^3-1)` may hold only in the nonsplit case `3 ∤ ell`. This is a
pattern from two nonsplit points, not a theorem. At `q=16`, `ell=4`, it
predicts complement `15*4095=61425` and W1 union
`16^6-1-61425=16715790` inside the nonzero trace-zero space.

Enumerate the exact published product locus for both cubic factors at
`q=16`, `K=GF(2^28)`, using projective first factors and every
`w in V*`, `c in k*`. This generates each product at least once; the
bitset deduplicates it. Choose the first ascending monic irreducible binary
degree-28 polynomial with constant term one. Record each cubic's unique
count, their union and overlap, the complement in `q^6-1`, and whether the
prediction holds. This arm does not run the quotient membership test,
construct a Hess curve, or benchmark an ECDLP. The source is native Rust;
retain source and binary hashes, stdout, host, and `/usr/bin/time -l`.
Resource limit: ten minutes wall and 256 MiB bitset memory, stop at any
assertion failure or resource breach. Gate through S3 checkpoint first.
