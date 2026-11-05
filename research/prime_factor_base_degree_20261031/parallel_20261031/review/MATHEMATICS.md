# Independent secant/graph mathematics review

2026-10-31. Reviewer: separate `secant_independent_review` agent. This review
uses source inspection and written algebra; it launches no scientific panel.
The source baseline is local commit
`f43c623ba17f78963f45e3df5027aaf491ce767d`.

## Finding

No mathematical defect was found in the stated complete finite-target secant
equivalence or in the **specified homogeneous top-ideal degree** comparison.
The claim remains presentation-specific. It does not establish observed F4
solving degree, total relation-finding work, literature-wide novelty, or a
P-256 / 17-term result.

The universal polynomial/group bridge and the quadratic degree argument are
written proofs. The Lean target checks the monomial-coverage argument only;
this review does not promote it to a full polynomial-ring formalization.

## Sources and scope

Read the primary group-law derivation in
[MIT 18.783, Lecture 2](https://math.mit.edu/classes/18.783/2023/LectureNotes2.pdf),
including the secant, inverse-pair and tangent cases. Read the factor-base
framework and scope discussion in
[Petit–Kosters–Messeng, PKC 2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf).
These support the classical antecedents and the need to measure actual solver
behavior; neither is treated as a certificate of the new comparison's novelty.
Repository sources inspected: `SECANT_GRAPH.md`, `THEORY.md`, the secant
exporter, the native upper/lower certificate checker and `SecantGraphTop.lean`.
This is not a comprehensive prior-art review.

| Source | SHA-256 at review |
| --- | --- |
| `SECANT_GRAPH.md` | `dd126ec78a1ed642eb3bebf069aac370e570326d5a1689355d920568d09108c7` |
| `native/secant_graph.rs` | `1650a796cd00a7e63737fa69b056a839ff308a18f24b4b32c0d38b379506f1c2` |
| `native/secant_top.rs` | `1d3b34b41540131e608995f2ef73d60df17adafeb8568f7d4c41e7c99cbd39d1` |
| `proof/SecantGraphTop.lean` | `838e0642b98a6236ae494a6d584b4fa614bd5e83cd94bac5f0f42f7489e335f1` |

## Complete-system equivalence

Write `L(T)=m(T-r)-s`, `c=s²-r³-Ar-B`,
`g1=m²-x-z-r` and `g2=xz-r²-rm²-A-2ms`.
Independently expanding the difference between `f(T)-L(T)²` and
`(T-r)(T-x)(T-z)` gives coefficients

- of T²: `-g1`;
- of T: `r*g1-g2`;
- constant: `r*g2-c`.

This yields exactly the identity recorded in the theorem. On `g1=g2=c=0`,
the intersection cubic is the monic product with roots r,x,z including their
multiplicities. The ordinate constraints put P,Q and -R on the same line.
The complete-splitting hypothesis then forces x,z into the exact rational
base and supplies both curve membership and rational lifts.

Repeated abscissae do not introduce spurious collinearity solutions:
`x=z` is a double root of the intersection cubic and enforces tangency.
If the repeated point had ordinate zero, a finite-slope double root would
force `f'(x)=0`; together with `f(x)=0` this contradicts nonsingularity.
Thus the invalid vertical tangent is excluded algebraically. The cases
`x=r` or `z=r` are also represented with the required intersection
multiplicities. If all three abscissae coincide, the cubic triple-root case
is the usual flex/three-torsion situation, not an extra branch to discard.

Conversely any valid P+Q=R with finite R has a finite secant/tangent:
the vertical inverse pair sums to O instead. Comparing the cubic coefficients
recovers g1,g2, and one of the four sign branches supplies the ordinates.
The target O is correctly outside the finite-target model and must retain its
separate exact inverse-pair references.

All common roots are rational under the complete-splitting hypothesis.
After rational x,z and ordinates have been recovered, the slope is rational
by the secant or nonvertical tangent formula. Even the all-abscissae-equal
case has a rational tangent slope. Field equations therefore need not be
added to this particular complete model. A candidate without the splitting
hypothesis would require a new membership/lift argument.

## Homogeneous degree and control

For d>=3, the top forms span exactly
`(m²,xz,x^d,z^d)`: top(g2)=xz-rm², and the remaining leading coefficients
are nonzero. A standard monomial has at most one positive x/z exponent,
that exponent below d, and m exponent at most one. Its maximum degree is d,
attained by `x^(d-1)m`; hence the first zero component is d+1.

For d=2, the ideal is
`(m²,xz,x²-alpha*mx,z²-beta*mz)` with nonzero alpha,beta.
All cubic monomials vanish by direct substitution. The four homogeneous
quadratic generators are linearly independent in the six-dimensional degree-
two space, leaving a nonzero quotient of dimension two; the degree is three.
The native certificate checker verifies actual polynomial identities for all
ten cubic monomials and a degree-two separating functional in each branch.
Its generator list is bound to the expected fixed F17 top ideal, and its
canonical exponent/coefficient checks prevent overflowing malformed recipes.

The dense top ideal `(x^(2d),z^(2d),x²z²)` has first zero component 2d+1,
as shown in THEORY.md. These are generator-list invariants under the explicit
definition; changing coordinates, adding consequences or decomposing into
branches can change the value. Four secant branches and one extra variable
must be retained in every cost comparison.

For the fixed nongraph control, the proposed cubic interpolant matches the
four supplied ordinates. Exact membership polynomials remain necessary.
Their degree-four tops are redundant in
`(m²,xz,x³,z³)`, giving degree four. No quadratic graph can interpolate any
choice of signs for those abscissae: their underlying multiples are
G,2G,3G,7G. Every signed integer sum is odd and lies between -13 and 13,
so none vanishes modulo 19. The usual divisor/group correspondence supplies
the required zero-sum condition for four points on a quadratic graph.
This verifies the stated control argument, not optimality over all encodings.

## Independent all-target invariants for the pending audit

For either eight-point signed base on the prime order-19 group:

- There are 64 ordered point pairs, eight of which sum to O.
- Across all 18 finite targets there must be 56 ordered relations/model
  roots, including exactly eight tangent/double roots.
- The finite unordered-pair total is 32: eight doubles plus 48/2 distinct-
  point pairs. There are four unordered inverse pairs at O.
- Counts for R and -R agree. Therefore the number of satisfiable finite
  targets is even.

These are derived checks, not experiment outputs. More generally a signed
base of K points without rational two-torsion has K²-K ordered relations
across all finite targets. Consequently equal-cardinality bases have the same
mean ordered all-target yield on a fixed group; their SAT-target coverage,
distribution, duplicates and useful row structure can differ.

There are also fixed-fixture coverage expectations derivable in the order-19
cyclic group. The candidate is `±{G,2G,3G,6G}`. Its pair sums include every
`±kG` for k=1,...,9: representatives are `2−1`, `1+1`, `1+2`, `1+3`,
`2+3`, `3+3`, `6+1`, `6+2`, `6+3`. Thus all 18 finite targets are covered.
The control `±{G,2G,3G,7G}` covers all those classes except `±7G`, since
no element in its signed coefficient set has a partner summing to either
7 or -7 modulo 19; its finite coverage is 16 targets. These are theorem-
derived **in-sample calibration expectations**, not observations from a new
panel or held-out evidence. The missing control targets have coordinates
`(0,6)` and `(0,11)` under the supplied G convention.

## Instrument review obligations

The following were communicated to the responsible agents before a panel run:

1. Audit operation counts must explicitly exclude or charge polynomial input
   construction. The initial arithmetic Counter did not cover it. Payload
   bytes are not operating-system disk traffic, and manifest/readback exclusions
   must be named. Full finite-panel execution must not be relabeled as a test.
2. Dense F4 input must instantiate two variables, stripping the padded zero
   m exponent in the shared schema. Leaving a free third variable changes
   the ideal/dimension and no longer matches the stated dense invariant.
3. Record maximum input degree, maximum selected-pair degree and maximum
   exported-basis degree separately. An explicitly defined maximum of input
   and selected-pair degrees may be reported as observed F4 degree. A stat
   omitting imported generators must not silently be called the full degree.
4. Retain all four branch costs, SAT/UNSAT classes, failures and raw outputs.
   L0 clock readings stay diagnostic. Stock solver counts must be reported
   with their actual meaning; missing field-operation counters remain null.
5. The ladder must retain all prospective curve/degree cells and construction
   failures, use target-independent selection, and preserve duplicate control
   sets. The K²-K and paired-target invariants provide independent checks.

At this review version, the instruments are under development. Their final
source and any admitted outputs require a separate review addendum before
claiming that the three requested experiment tracks completed.

## Coordination and limits

`conductor check` for this review path failed with command-not-found (exit127);
the repository documents missing-installation fail-open behavior. Only this
owned review directory was written. No experiment, CAS panel, paid resource,
repository commit or publication was initiated by this reviewer.
