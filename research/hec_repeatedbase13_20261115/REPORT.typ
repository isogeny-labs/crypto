#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Multiplicity-two fixed-base cancellation
Genus 3 · 15 November 2026
#image("flow.svg",width:100%)

The previously rejected F97Fermatquartic now admits the tested section construction. O=(4,24)has tangent y=22x+33 and residual x²+11x+6=(x−43)², representing D=2Qwith Q=(43,9). Divisibility constraints include multiplicity and produce eight sections.

== Why first-order cancellation fails
On a squarefree effective cubic containing Qonce, all numerator values and first along-curve derivatives vanish atQ. Cancelling only one copy leaves a zero triple-section vector. Removing the admission guard alone would be incorrect.

The new formula takes twoNewtonsteps modulo u³, checks the exact curve residual, finds the common numerator gcd with u², divides the full fixed factor, then reduces modulo u. The registered test cancels both copies. Independent along-curve second derivatives give the quotient limit atQ. All120tensor identities pass; the corrected projected vector [84,10,30,13,73,77,5,73]is nonzero. First-order-only output is zero.

== Separately frozen regression
V1sourcec6478e372b41/checkpoint20261116T005110Z passes F97once. V2sourceddc7f2fbeab6/checkpoint20261116T005254Z checks the same candidate formula over all four fixed primes:
#table(columns:3,inset:6pt,[Field],[Points],[Fixed base / outcome],
 [97],[44],[Double rational point / pass],
 [257],[252],[Split two-point / pass],
 [509],[520],[Split two-point / pass],
 [1009],[1100],[Split two-point / pass])

Each passes RRdimension8,multiplicationrank18,diagonalrank112,exact coefficient and corruption checks,480determinant/480ordinarytensor controls,120base-cancellation checks and canonical verification. All statuses0and stderr empty. No scientific rerun or dropped field; earlier failures remain preserved in the prior fieldchart study.

== Formal precision and scope
Precision.lean checks: if g≠0, u²=g·v and g·qa−g·qb=u³·t, then qa−qb is divisible by u. This supplies the conditional quotient-precision argument. Lean4.19/mathlib pinned environment, axioms propext/Quot.sound; no sorry/customaxiom. Hensel convergence, geometry and implementation are not formalized.

This is a known local multiplicity correction, without novelty or speedup evidence. Repeated effective support remains excluded. New-prime non-split cancellation, complete atlas, subgroup centering, source isogenies and new-field DLP are untested. Extra lifting may cost more. Genus two/four and the mathematical tenfold objective remain open; the earlier failed charged comparison is unchanged.

Protocols, source, locks, hashes, both raw executions and proof output accompany RESULTS.md.
