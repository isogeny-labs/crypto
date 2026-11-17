# Homogeneous normalization seeds through the source

All identities are in the supported odd prime field. Every finite normalization
seed is nonzero. Represent c_i=N_i/D_i with N_i,D_i nonzero canonical residues.
For Delta=x(P)-x(T), the preserved recurrence is

    c_i = Delta*c_P^2*c_(i-1)^2/c_(i-2).

With S=c_P^2 prepared once, choose

    N_i = Delta*S*N_(i-1)^2*D_(i-2),
    D_i = D_(i-1)^2*N_(i-2).

Their quotient is the original update. Independent nonzero rescaling of either
input pair multiplies both output coordinates by the same factor. The helper
checks its nonzero/canonical domain and never divides. It uses six field products
(including squares); the prior scalar expression used four products and one inverse.
This local algebra trades that inverse for two products. It does not price the
homogeneous consumer's added work.

Anchors and direct phase seeds enter as [c,1]. The public identity point is [0,1]
with sign0; existing anchor precedence prevents it from entering a division step.
Prepared windows expose PairSeedWindow.seed_pairs, not falsely normalized scalars.
The recovery API only consumes signs and logical method counts, so its public input
and receipt remain unchanged. Pair storage uses two field residues per seed instead
of one. The old scalar API remains in its original module.

## Consuming pairs without normalizing them

For each needed n, the normalized division value is c^(n^2)*psi_n(Q). The consumer
uses the pair

    [N^e*psi_n(Q), D^e],  e=n^2 mod(p-1).

Reducing e is valid because N,D are nonzero. This is a representation of the same
normalized value, not a different seed. Existing value-division arithmetic is kept.

For recurrence pairs v=[vN,vD], previous=[aN,aD] and square=[sN,sD], the update
becomes

    [Delta*vN^2*sN*aD, vD^2*sD*aN].

The old scalar square is the case sD=1. The initial +/-u values negate only the
numerator. Pair products accumulate numerator and denominator independently.
The resultant contributes in the numerator as before. A partial-tail factor
c^(2s) now contributes cN^(2s) above and cD^(2s) below. A singleton -u contributes
-uN above and uD below. Omitting either denominator would change the source value.
The final normalized sign is decided by the already checked cross-multiplication
predicate, without converting the seed or source ratio to an affine scalar.

The scalar raw-parts function is preserved byte-for-byte and remains the direct
reference path. Geometry, polynomial products and scalar division-value recurrence
are unchanged. The homogeneous consumer adds denominator powers, squares and
products. Other inversions remain in direct phase computation, setup, geometry
and the value-division engine. No whole-source inversion-free claim is made.

## Checks and next comparison

20736 cases cover every nonzero previous/older value and independent input scale
over F13 (Delta=3,S=4), against Fermat-inverse reference updates. Nine source cases
cover three saved curves and scales1,2,p-1. Existing prepared-window tests compare
every pair against independently computed direct phases by cross multiplication,
including identity crossings and small partial/singleton branches. All148 native
methods pass, including8 complete recovery receipts against ratio-sign baseline.

This is a correctness-checked candidate. Next instrument all added denominator
operations and compare complete source costs; local removal of inversions does not
by itself determine the full tradeoff. Source partition and decoder sizes, original
4563-case cohort, and existing comparison identities remain unchanged.

Prior context: previous shared-seed and ratio-sign derivations; standard homogeneous
representations and scaling at https://www.hyperelliptic.org/EFD/g1p/index.html.
No novelty is claimed for rational-pair arithmetic itself.
