# Earlier-cutoff refinement of the regular RR5 chart

2026-11-24. Development protocol and mathematical argument. [pcc:unverified]

## Question and fixed comparison

Can the degree-32 distinct-image oracle complete its existing five public-target
controls by replacing its last coefficient-map step with a union of smaller
systems? The reference is the existing `Distinct` implementation, source
`c301874200e5f55ad848d8d9ab1bd9c9225250cd`. The baseline executable is preserved
before modification. The original three-second msolve limit is tightened to one
three-second deadline per entire query in both new arms. It includes chart
enumeration, system construction, serialization, process setup, elimination,
root extraction, signed replay and union construction. Partial unions remain
undetermined. No timeout is discarded and no branch receives a fresh budget.

The fixed development panels are the existing four F127 degree-16 targets,
four F8191 degree-16 targets, and five F8191 degree-32 targets including the
existing planted positive. A later mask-recovery check retains seed 781815 and
30 draws. Exactness requires the entire returned signed set to equal the
pair-index reference restricted to the same regular distinct-image chart.
The reference is a checker only; the algebraic extractor cannot consult it.
Builds and correctness checks run locally under AGENTS.md section 9a. They do
not supply admitted timing or an end-to-end speed claim. No EKS spend is
authorized by this protocol; the standalone workload still lacks its registered
image/build route.

## Finite partition

Write `f(X)=X²−2`, `T_j=f^j`, and take a regular split fiber `T_d(X)=c`,
with `d>=3`. The current chart contains four distinct intermediate images
`T_(d-2)(x_i)`, one at each of the four roots of `T_2(X)-c`.
For each such coarse image `a`, its two fine images satisfy `f(b)=a`.
Choose one fine image over each coarse image, and form

    P_b(Z) = product over the four chosen b of (Z-b).

There are at most `2^4=16` choices. The refined chart replaces the last
coefficient step by the requirement that pushing the monic RR5 quartic through
`T_(d-3)` produces exactly `P_b`. The four chosen fine roots are distinct,
since their coarse images are distinct.

Every rational point tuple in the current chart determines exactly one choice
of fine roots. Conversely, a tuple in one refined chart pushes to the four
coarse roots and belongs to the original chart. Thus the refined charts form a
disjoint partition of its rational relation tuples, before and after sign
replay. This is a point-set assertion on the regular chart, not an assertion
about equality of unsaturated ideals or multiplicities. Fine images with no
curve-liftable factor-base abscissa can be removed using the factor base alone.
Their removal is target independent and is included in query accounting here.

At depth five the original system has eleven variables; each refined system
has seven. The number of systems and their sum of work must still be charged.
The construction preserves the original chart's coverage loss relative to
all four-summand relations. It does not restore the omitted collision charts.

## Sources checked

- Petit, Kosters and Messeng, [prime-field ECDLP, PKC 2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
  sections 3.2 and 3.4: composed rational maps and isogeny fibers, with
  unresolved elimination complexity. The factor-base construction is prior art.
- Isogeny Labs, [Dickson first-collision S3 study](https://github.com/isogeny-labs/crypto/blob/main/research/prime31_ic_20261115/native/DICKSON_FIRST_COLLISION_S3_20261115.md):
  exact finite partitions can reduce individual systems while increasing total
  elimination work. This motivates a common deadline and complete-union rule.
- The lane's [rank argument](REPORT.md) and [native controls](NATIVE.md):
  keep the regular cutoff and preserve all previous outcomes.
- Sources checked (KB): unavailable. No KB MCP tool is loaded. Both wrapper
  searches, `Dickson distinct image refinement factor base` and
  `successive resultants degree growth cutoff`, failed because the KB executable
  is absent. The primary paper and the exact named repository studies above
  are the fallback. No novelty claim follows from that search limitation.

## Advancement and limits

A completed positive query with exact signed output advances this extractor
check. A smaller variable count alone does not. If any chart is incomplete,
the whole query is incomplete. If both arms complete, this development control
still makes no speed claim. Full cold recovery, the production compact
quotient-product and oriented-S3 comparisons, admitted measurement, independent
replay and linear-algebra accounting remain the original research requirements.
