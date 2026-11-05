#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Reusable recursive companion inverse
2026-11-01 · Derived construction; syntax checked; zero numerical trials

*Construction.* For displacement Z_a A − A Z_b = G Kᵗ, split a leading
n-by-n block E from dimension N. Two combined boundary queries produce
width-(s+2) generators, combining the historical four correction columns.
For zero-based full-space coordinate vectors:

```text
q = (A (e_n - d e_0))[:n]
t = (A^t (c e_(n-1) - a e_(N-1)))[:n]
Z_c E - E Z_d = G[:n] K[:n]^t + u_0 t^t + q u_(n-1)^t
```

This uses one forward and one transpose source query plus two input
negations. It is an uncached derived query schedule, not measured timing.

#image("pipeline.svg",width:100%)

*Explicit inversion.* Prepare inverses recursively for E and its Schur
complement S. An internal block solve uses two A queries, two E-inverse
queries and one S-inverse query. Apply these solves to G and transposed
solves to K, returning U=−A⁻¹G and V=A⁻ᵗK. The inverse displacement has
SWAPPED wraps b,a and width s. Subsequent solves use that generated operator;
they do not recursively replay the factorization. Scalar leaves use one
source query and invert a nonzero scalar. No dense inverse is supplied.

*Derived nonleaf schedule.* Outside recursive child setup: (6s+8) source
queries, (6s+4) leading-inverse queries and 2s trailing-inverse queries,
equally split by direction. Extra field work is m products and
2+m(2s+5)+3Ns additions/subtractions, where m=N−n. Child widths are s+2,
except scalar inverse outputs of width one. Count each actual fused ledger;
field, native, bit and byte units stay separate. Audit histories also consume
memory; generator storage alone is not total storage.

*Domain and next check.* Conditional on the fixed pivot tree succeeding.
A zero leaf reports its E/S path and leaves root rank undetermined. Retain
invertible matrices with singular leading blocks as failure controls. No
pivot retries, field extension or rank-profile success probability is assumed.
Freeze both-direction inverse identities, every principal cut, odd sizes,
scalar leaves, rank obstructions, mutation controls and exact node schedules
before numerical execution. Existing full kernel and integration prerequisites
remain in force. Rank-profile and singular-witness handling are next extensions.

*Provenance.* principal.py, recursive.py and DERIVATION.md. Prior art:
Bostan et al., SIAM J. Matrix Anal. Appl. 38(3), 2017, Section 6.2,
DOI 10.1137/16M1062855. Its conventions and randomized regularization
assumptions are not imported. This is an implementation derivation, not a
novelty or measured speed claim.
