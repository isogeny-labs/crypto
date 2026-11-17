# Degree-six syzygy exclusion: development contract

2026-10-30. Production is not frozen or launched. The full objective remains
n83 Koblitz, general binary and prime m2–9 with actual fixed factor bases from
2^27 candidate x-coordinates. This stage targets all42 retained higher-m prime
sources, without changing their points, targets or equations. Existing
source-bound FFD bounds6–17 remain authoritative until new certificates pass.

## Literature and mathematical extension

Read Caminata–Gorla, https://arxiv.org/html/2112.05579, Definition1.3 and
Definitions1.4–1.5, before this extension. The highest-form syzygy quotient,
graded regularity and filtered LFD are distinct. PRIME_LINEAR_DESIGN.md and
its completed dataset provide the linear-multiplier covering argument; the
native source validator is reused unchanged. The following degree-six
extension is a study-specific argument, not a claim from the cited paper.

Work modulo the regular sequence of all selector squares after invertible
normalization. A degree-six syzygy has quadratic coordinates on the quartic
chain generators. Its remaining coordinates belong to selector squares.
Reduce each quadratic coordinate modulo those squares. If the complete map
(Q_j) -> sum Q_j F_j is injective in this quotient, all reduced Q_j vanish.
Subtract mixed square/quartic Koszul syzygies to remove the unreduced square
terms in Q_j. The remaining square-only syzygy is Koszul. Quartic/quartic
Koszul syzygies start at degree8; characteristic-p truncation is irrelevant
in degree6 at the actual83-bit prime. Multiplication by a free intermediate U
also excludes lower-degree nontrivial reduced relations. Successful injection
therefore implies FFD>=7, not equality and not an LFD result.

Project each27-selector block onto seven selectors, retaining all intermediate
variables. Every nonzero normalized projected system is the same template
after renaming. To infer original injectivity, projections MUST cover every
pair of local selector indices, including pairs in distinct blocks. Simple
coverage of individual indices is insufficient for quadratic coefficients.
For each pair i<=j in0..26, retain i,j and add the smallest missing indices
until seven are retained; deduplicate subsets. Use the same subset in every
block. Every surviving quadratic monomial uses at most two local selector
indices, so at least one projection preserves its coefficient. Unrestricted U
powers are retained in every projection. Native pair-coverage checks verify
all27x27 pairs explicitly. A deficient projected map is inconclusive about
the original system; no projection kernel is lifted without verification.

## Complete domain and resource plan

For m, the template has n=8m-2 variables,7m of which are selectors. Its
degree-two normal-form basis consists of all pairs a<=b except a=b<7m.
Columns are generator-major, then lexicographic multiplier pairs. Count:
(m-1)*(n*(n+1)/2-7m), giving464,1311,2824,5195,8616,13279,19376
columns for m3..9. Rows are all nonzero degree-six coefficient monomials.
Selector squares reduce to zero; intermediate squares must remain.

The first implementation reuses the exact sparse coefficient map and modular
elimination from prime_linear.rs, retaining original pivot rows and their
nonzero pivots. Full reconstruction/replay checks the whole map, not only a
reported rank. Production memory/time limits and whether row streaming is
needed must be decided from retained development controls before freeze.
No production rank or bounds may be reported from small-prime controls.

## Controls and publication gate

Compare small-prime sparse ranks with an independent dense implementation;
retain known three- and five-selector annihilator controls. Verify all
quadratic multiplier columns, selector-square exclusion, intermediate-square
retention, pair coverage and independent six-term quartic expansion. Keep
all predecessor donor controls. All failures and versions are retained.

Before n83 production: freeze source/binary, all42 input identities, limits,
failure accounting and experiment.protocol/v1; source S3 restoration and
checkpoint; then full producer/fresh/restored replay and export/report.
The production entry point is explicitly gated during development. This
development checkpoint does not tighten the existing6–17 interval.
