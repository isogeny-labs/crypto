# Fully rational base: prospective coefficient-descent proof

2026-11-14. Continue the general prime-field non-generic expected
sub-exponential bit-cost objective. This component tests a field-of-definition
obligation, not a successful public finder or its runtime. No novelty claim.

Use a factor base made of actual rational points, rather than all abscissas
of an unfiltered Kummer/Dickson fiber. Public points may be generated as
known scalar multiples of the input base point in a fresh one-target
attempt. Their generation, membership polynomial and retry/coverage costs
must eventually be charged; no free reusable precomputation is assumed.

Question: for an actual bounded monic norm pair A,B over an extension field,
does divisibility of its full norm by the squarefree product of allowed
rational abscissas force its coefficients to descend to the prime field?
Prove this for every allowed successful pair, without supplied signs,
a branch finder, an oracle declaring a coset good, or a rational extraction
oracle [pcc:unverified].

Acceptance obligations, in order:

1. For a field endomorphism fixing a public square root y, the scalar norm
   equation a^2=y^2*b^2 forces a*phi(b)=phi(a)*b. Derive either sign; do not
   assume the successful sign is part of the input.
2. An actual norm divisor of a nodal polynomial on distinct public nodes
   splits with no repeated roots. Select its root set mathematically from
   the divisor; no efficiently supplied successful support is a premise.
3. Use the pole-degree window and root count to force the cross product of
   the pair and its mapped pair to vanish. From equal actual norms and
   monic upper normalization, prove both coefficient polynomials fixed.
   Handle B=0, and prove required monicity/nonzero/degree guards rather than
   silently removing exceptional cases.
4. Specialize phi to Frobenius in characteristic p and derive membership of
   every coefficient in the prime subfield using the existing exact
   fixed-point theorem. The public curve/base coordinates are explicit
   prime-subfield data, not arbitrary rational abscissas with twist branches.
5. Bind the full norm divisor premise to the actual monic support division
   and membership remainder, with the target distinct from the base.

This would close the rationality implication for this selected base model.
It does not prove coverage, finite expected retries, an efficient norm
finder, a growing-family closure bound, a useful nonlinear rank map or
complete expected bit cost. A separate coverage proof must retain actual
sampling, collision/opposite/target exclusions and all attempted work.
Do not infer any of these from a finite solution set or a good-base promise.

Sources checked (KB): unavailable as already documented, no loaded tools and
missing wrapper executable; no unchanged retry. Bounded fallback queries
"norm rational factor base coefficient descent Frobenius" and "projective
polar normalization target derivative" across crypto, cryptanalysis and
ml-cryptanalysis are token matching, not novelty/absence certification.
Read actual NormBranchRepair.crossZero, normSquareRelation and
squaresProportional; the independent rationality review at
crypto:f87e25bb0b50a5ce137eaa33fc733d317a2f0981,
research/prime_one_shot_20261029/continuation/reviews/rationality.md;
the earlier elliptic-code and non-Semaev primary reports; and Mathlib
Polynomial.Splits, Separable, Lagrange.nodal and
Subfield.mem_bot_iff_pow_eq_self at the pinned study dependency.
The Kummer review explicitly excludes transfer to Dickson/arbitrary primes
and warns that good-base probability does not supply efficient recognition
or a solver-cost bound. This component changes the base premise openly.

Conductor remains unavailable as recorded. Scope extends to this protocol,
new NormSplitBaseDescent Lean source/evidence/report, study manifest,
current journal and append-only PCC. Preserve all existing mathematical
source, raw evidence, claims and ledger prefix. Freeze source before fresh
replay and claim binding. Local universal proof construction only; no new
fixture/unknown-root/F4/msolve/paid/cloud/attack or timing experiment,
checkpoint bypass, vault, fabricated signature or force push.
