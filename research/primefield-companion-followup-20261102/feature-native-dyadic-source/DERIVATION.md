# A public sign source for arbitrary dyadic projection order

2026-11-07 · T-1823 · Derived mathematics and exact native producer checks. [pcc:unverified]

## Domain and normalization

Let p>3 and r>=7 be caller-certified primes, r!=p and gcd(r,p-1)=1, within the existing u64/2 arithmetic bounds. Let P have order r and Q belong to its rational subgroup. Write p-1=q*u with q=2^v2(p-1), u odd; pi(z)=z^u maps nonzero F_p values to its q-th roots of unity. The previous standalone projected source supported q=2,4. Here q has no extra bound beyond the field-word bound.

For nonzero r-torsion T, write W_j(T)=psi_j(T) and

    beta(T) = W_(r+1)(T)^2 W_2(T) / W_(r+2)(T).

These factors are nonzero because r>=7 is prime. Ward quasi-periodicity and the division-polynomial chain rule give

    beta([k]T) W_k(T)^(r^2) = beta(T)^(k^2).       (1)

The predecessor DERIVATION.md expands the cancellation directly using Ward constants; the same identity holds without a restriction on q. Let e be the unique inverse of r^2 modulo q, which exists because r is odd. Define

    C(T) = pi(beta(T))^(-e).

Projecting (1) and raising to -e yields

    C([k]T) = pi(W_k(T)) C(T)^(k^2).              (2)

Only the exponent on W_k is reduced modulo q: e*r^2=1 there. No scalar-field/F_p identification is used. beta(-T)=-beta(T) by parity of the division polynomials. Since u and e are odd, C(-T)=-C(T). Thus C is a public, nonzero, odd point function on the subgroup.

For h=(r-1)/2 define F(T)=product_(j=1)^h C([j]T). Multiplication by k permutes the half-set modulo signs, so F([k]P)/F(P) is the sign of that half-set permutation. Multiplying its indices modulo r shows that sign is k^h modulo r, hence Legendre_r(k). The resulting abstract sign is represented as +/-1 in F_p. Identity is assigned0 separately. This proves the source sign law for every allowed q.

## Native construction

Use seed=beta(T)^(-e) in F_p, computed from value-only division queries and extended-Euclidean inversions. Its projection is C(T). The scaled values V_j=seed^(j^2) W_j need not form the intended periodic sequence as full F_p values; their projections do, by (2). This distinction is essential.

The existing block-pairing geometry computes the half-product with multiplicative rational recurrences and a resultant in public x-coordinates. Its initialization uses V_(N-1)=-V_N, N=(r+1)/2. This identity holds after projection because [N-1]T=-[N]T and C is odd. Every subsequent seed-dependent operation is multiplication, division or integer powering, which commutes with pi. Additive x-coordinate differences and resultant polynomials do not depend on the choice of seed. Consequently projecting the final full-field block result gives F(T), even though its intermediate unprojected representatives can differ from the old derivative-normalized ones.

lib.rs is generated from the existing full block source with three changes: remove derivative jets, calculate beta and e through the shared value-only cache after checking W_r=0, and project the final rational block value. The constructor adds the inherited coprime/word-domain checks and a geometry-lane cap before allocation. Calibration stores F(P)^-1; sign compares the calibrated value with +/-1. The existing prepared-window geometry is retained and also uses this projected raw result.

The r-torsion check implies membership in the intended rational prime subgroup under gcd(r,p-1)=1: rank-two rational r-torsion would force r|(p-1) by the Weil pairing. This does not remove the coprime condition or cover r=p. No arbitrary radical, extension-field transfer, discrete logarithm extractor, scalar label or root search occurs in runtime.

## Costs and relation to previous sources

The exponent inversion e is in Z/qZ via extended Euclid; field inversions and modular powers are charged. The three beta values share a ValueDivision cache with the torsion check and block division values. Geometry, full-field multiplicative pair recurrence and resultant remain charged, with O(sqrt r) materialized geometry lanes and the existing polynomial backend. This broadens the value-only projected source domain; it does not claim a lower source-cost exponent. Unlike the specialized q=2,4 residual backend, this route retains full-field pair values until final projection. The older general derivative-normalized source remains a control and is not claimed to be newly discovered.

## Exact source checks

The deterministic native fixture finder searches a,b in0..8 and all affine points at the prescribed small primes17,41,97,193,257, selecting the first nonsingular curve with prime group order r>=7 distinct from p. This bounded reference-only enumeration chooses fixtures; it is absent from Source APIs. The original p5,p7,p509 source fixtures are retained. The resulting new fixtures are:

| p | a | b | r | P | q | e |
|---|---|---|---|---|---|---|
|17|2|2|19|(0,6)|16|9|
|41|1|5|47|(0,13)|8|1|
|97|0|5|79|(1,43)|32|1|
|193|2|7|191|(0,59)|64|1|
|257|1|7|281|(1,3)|256|145|

validation-01 checks all1,108 nonzero points across the eight groups: scalar Euler signs, direct projected half-products and covariance(2) all agree. Twenty-four windows cross identity or begin at base/identity and match expected signs. On q<=4 the prior standalone source also agrees. The deliberately wrong exponent e=1 differs at266 point values in fixtures requiring correction; this control is on C, not a claim that every final half-product would fail. Identity, lane-cap and noncanonical-point controls are retained. [pcc:unverified]

The separate recovery harness copies the workspace-adaptive decoder, changing only its source import. Its old controls/63public fixtures remain. An additional complete-group panel covers all targets on the five new curves, using m=2,2,3,2,5 respectively; source signs are point-only and scalar labels verify outputs. Original4,563-case cohort unchanged. This is deterministic algebra/software validation, not a timing or cloud experiment. [pcc:unverified]

## Sources, scope and next work

Read [Lauter-Stange2008](https://arxiv.org/pdf/0803.0728), Theorem3 division-chain transformation and Theorem4 Ward quasi-periodicity, and the local standalone derivation. These are established identities; no novelty claim for Ward normalization. KB unavailable. The report distinguishes this general dyadic route from the faster specialized projected-residual implementation and from older general full-value sources.

Next extend quotient-class resultant/recurrence arithmetic beyond q=4 so the broader domain can also discard unneeded full-value information. Keep the current exact source as its reference. No independent signed verdict or full formal proof is supplied.

## Recovery outcome and dependency check

recovery-01 passes all617 new complete-group targets with5,929 source calls and zero unresolved outputs. The original63public equivalences also pass with511calls. New-source ordinary-envelope and workspace-adaptive routes agree in scalar, route and query fields. The normal metadata build succeeds with four dependency inputs and contains no derivative-jets, old full-source or prior standalone-source dependency. This is dependency evidence, not a performance measurement. All commands/outputs are retained; no native assertion or build failure occurred. [pcc:unverified]
