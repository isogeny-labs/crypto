# Candidate route beyond cubic matrices — unvalidated, 2026-10-30

This is a proposed proof strategy, not a new certificate, protocol or bound.
The recorded prime intervals remain7–17. Check every obligation below before
using this argument to alter any scientific result. Keep the cubic native
maps as an independent low-degree reference, not an abandoned dataset.

## Primary source checked

Lundqvist–Nicklasson, *On the structure of monomial complete intersections in
positive characteristic*, https://arxiv.org/html/1604.06820, Theorem2.7 and
the first paragraph of Section3: when the characteristic exceeds the socle
degree, the sum of variables is a strong Lefschetz element. For a block of27
square-zero variables the socle degree is27, below the actual prime. This
source establishes the block fact, not the chain argument proposed below.
Caminata–Gorla https://arxiv.org/html/2112.05579 remains the FFD definition.

## Proposed reduction and obligations

1. Normalize the nonzero selector weights using the existing exact source
   checker. Put L_i equal to the sum in selector block i. The squarefree
   block should admit graded strings generated in degree k, of length
   ell-2k+1, with multiplicity C(ell,k)-C(ell,k-1). Prove the decomposition
   from strong Lefschetz and validate explicit small-block bases. Tensor
   these decompositions across blocks and retain free U variables.

2. In a tensor sector of primitive total degree K, a degree-d syzygy has
   coefficient degree r=d-4-K. Every chain quartic has degree at most two
   in each L_i. Thus its product with a coefficient has L_i exponent at
   most r+2=d-2-K. The string in block i permits exponents through
   ell-2k_i. Since k_i<=K<=d-4, the sufficient no-truncation inequality is
   2d-6<=ell. For ell27 this covers d<=16. Check the boundary, all sector
   shifts, and that this argument covers the whole kernel rather than
   selected coefficients.

3. Before string truncation, work in the polynomial ring k[L_0,...,L_(m-1),
   U_0,...,U_(m-3)]. Give every L weight m and give U_j weight m-2-j.
   The proposed leading monomials of the chain quartics are
   L_0^2 L_1^2, U_0^2 L_2^2, ..., U_(m-3)^2 L_(m-1)^2.
   Their supports are pairwise disjoint. Verify every actual six-term
   quartic, positivity of weights, strict leading weights and terminal
   specialization for every m. Prove these leading monomials imply the
   quartics form a regular sequence, hence the first syzygies are Koszul.

4. Transport each sector's free-ring Koszul representation back to the
   squarefree quotient. Prove this transport is valid even at the string
   boundaries and preserves homogeneous degrees. Then lift to the original
   ring by mixed selector-square/quartic Koszul syzygies; remaining
   selector-square syzygies are Koszul. Check the precise FFD quotient,
   including characteristic truncation, against the source definition.

If all four steps survive validation, they may exclude non-Koszul syzygies
through degree16 for the full27-selector systems, making the existing
source-bound upper17 certificates decisive. This implication is presently
unvalidated; do not report exactFFD17 for higher m from this note.

## Required evidence before any promotion

- Explicit small-block Lefschetz strings checked independently against the
  original squarefree multiplication matrices, including bad-characteristic
  negative controls and exact string multiplicities.
- Sector-map kernel dimensions compared against the retained direct
  linear/quadratic/cubic maps, including deficient projections.
- Machine-checked leading weights, disjointness and truncation inequalities
  for every required degree and m; counterexample controls for each gate.
- All42 actual source identities, prime/weight validation and upper-bound
  joins under a separately frozen protocol; fresh and S3-restored replay.
- Independent scientific review/formalization. No LFD, solving-degree or
  elite-base claim follows automatically from any highest-form result.

This is potentially more useful than deliberately running large cubic
matrices into predetermined caps. It must be tested, not assumed.
