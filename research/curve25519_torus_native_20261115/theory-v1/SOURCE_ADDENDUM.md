# Source and proof-scope addendum, 15 November 2026

This supplements the frozen `energy-lemma.md`; it does not upgrade that
draft to a verified theorem. [pcc:unverified]

## Degree inequality: precise dependency

J. S. Milne, [Abelian Varieties, section 5, pp. 21–23](https://www.jmilne.org/math/CourseNotes/AV.pdf),
Corollary 5.3 gives the pullback line-bundle identity for three regular maps
from a variety into an abelian variety. Apply it to a smooth projective curve
C, target E, the symmetric degree-one bundle O_E(O), and maps f,g,-g.
Taking degrees gives

    d(f+g) + d(f-g) = 2 d(f) + 2 d(g),

where d(f)=deg(f*O_E(O)), also for constant maps (degree zero). Corollary 5.4
gives d([n]f)=n² d(f). These are total degrees, so inseparability is included.
The cube identity polarizes this nonnegative quadratic form on maps modulo
constants. Nonnegativity of d(af+bg) for integers a,b, then rational density,
gives Cauchy–Schwarz for its polarization and hence

    d(sum a_i f_i) <= (sum |a_i| sqrt(d(f_i)))².

This explains the imported inequality; it does not construct the smooth
projective multiquadratic cover or verify its elliptic morphism degrees.
Milne's notes explicitly defer the full proof of the theorem of the cube.
They are a source for the stated corollaries, not evidence that their full
foundational proof was checked here. Independent review remains required.
[pcc:unverified]

## Related distribution work and limits of transfer

Farashahi, Fouque, Shparlinski, Tibouchi and Voloch,
[Indifferentiable Deterministic Hashing to Elliptic and Hyperelliptic Curves](https://www.normalesup.org/~tibouchi/papers/welldistributed.pdf),
Theorems 1–3, analyze sums of encodings through character bounds and
geometric covers. Theorem 2 bounds distance from uniform for sums of a
well-distributed encoding; Theorem 3 has an explicit condition excluding
factorization through a nontrivial unramified cover. Those hypotheses must
be checked before importing a character bound. The paper's full-domain
encoding statements do not, by themselves, prove the present bound for a
small torus-coset set under a shared random multiplier. [pcc:unverified]

This is related primary literature, not a novelty clearance. The present
cover-and-degree draft could be an application of existing techniques.
No claim of an original theorem follows from the searches made here.

## Added native identities and partial Lean endpoint

Both elliptic-map polynomial identities now have maintained native identity
certificates in `../four-point-v1/map-{plus,minus}-identity.json`, plus exact
integer coefficient expansion checks in `tools/torus-s4-tree/tests/identity.rs`.
These establish neither extension to projective morphisms nor their degrees.
[pcc:unverified]

The separate Core-only Lean file `../lean/Arithmetic.lean` checks M|(p+1),
28M<p+1, K*W0<p+1-28M, and 2^52<(M-1)/2. The geometric energy bound,
its application to the actual factor base, solver cost and any exponent
change remain outside these arithmetic theorem statements. [pcc:unverified]
