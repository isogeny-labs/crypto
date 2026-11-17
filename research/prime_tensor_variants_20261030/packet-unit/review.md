# Read-only review of the finite ternary packet unit

2026-10-30 UTC. Reviewer `/root/modular_curve_sources/unit_review`.
Read the frozen protocol, native `packet_unit.cpp`, initial retained
records and AppendixD.4.2-D.4.5 of the supplied primary text.
No edits, independent execution, scientific timing, contraction code
or theorem certification were performed in this review.

Reviewed snapshot:

    8dbecf76c52fe626b76411d2a143d69f631215babf44f7be1871a7842c63ac01  PROTOCOL.md
    e3f17795ffb9ac013e11628c2ee40e9acf33afed562691733a1e41e5e304abbd  packet_unit.cpp

## Verdict

No blocking correctness or frozen-protocol coverage defect was found
in the reviewed fixed core packet unit. The code checks the supplied
ternary gadget rather than silently treating the primary binary
construction as a proved ternary theorem.

The initial run records report the full fixed panel passing. That is
an observation of retained output, not an independent replay by this
reviewer. The scope qualifications below remain material.

## Action, locality and orbit checks

The implemented S(a,b)=(-a-b,a) and inverse(b,-a-b) agree with the
protocol. Component1 receives S, component2 receives S^-1. The
inverse-U flag reverses both directions correctly. Distinct levels
operate on disjoint four-entry blocks; order3, inverse and commutation
are actually tested over the full fixed class panels.

For nonzero integer difference y-x, the least nonzero base-three
digit is correctly computed by repeatedly dividing by3 and reducing
the remaining quotient modulo3 with a nonnegative residue. The chosen
component is1or2 accordingly. The diagonal is counted but skipped
before any finite valuation is assigned to zero.

For component1 the row sequence under S is a,-a-b,b and the column
sequence is b,a,-a-b; a digit increment1 gives the required equality.
For component2, S^-1 gives the corresponding equality for increment2.
This also explains the finite checks for differences with carries.

In characteristic other than3 the only fixed pair under S is(0,0).
Each nonzero four-entry level therefore contributes an orbit of size3;
independence between levels gives3^k. The full source checks orbit
size, closure, equal slot multiplicities and disjointness, not merely
the expected total orbit count.

## Matching graph and padding coverage

The source constructs precisely the fixed full-class matching panel:
one fresh row and column vertex per edge, all class loads H, and
L-slot packets generated from orbit representatives. Each class has
L/|O| visits per packet; the copy count H|O|/L is checked integral.

Every assigned edge receives its explicit class/copy ID. A bounded
visited array detects duplicate or omitted IDs; the per-ID row and
column degree arrays verify the constructed matching endpoints have
degree1. Class totals, complete edge coverage and packet totals are
checked. Marginals are accumulated from the labels of the actually
assigned ports and compared to M/r on both sides, rather than only
reported from a symbolic formula.

The fixed graph records are consistent with the protocol:

| r,a | Darr | H | M | Packets |
| --- | ---: | ---: | ---: | ---: |
| 2,1 |16|33|528|176|
| 2,2 |256|513|131328|14592|
| 5,1 |625|1251|781875|260625|

The total913731 edges and275393 packets match those three rows.

## Tiles and negative controls

For each selected level/component the columns in a tile have a fixed
base-three digit at that level. The code checks that their column
group is constant and then enumerates every row slot's distinct
crossing group. It permits up to two targets. The explicit
F5, gamma=(0,0,1,0) packet genuinely has row groups0,4,1 and
source group1, so the recorded targets0and4 reject a binary-style
single-cross-target predicate.

The wrong-second-direction control generates both correct and
corrupted packets, then calls the actual locality predicate on both.
It is not just a hard-coded arithmetic expectation. The field3
guard is exercised and the nonzero fixed pair(1,1) is exhibited
independently by the raw action. Nonmultiple padding is rejected
by the same divisibility predicate used for packet construction.

The F67 panel exhausts all4489 ordered pairs, verifies both action
directions and all six off-diagonal slot comparisons per pair.
The target-size counts[1,198,4290] are consistent with the triple
(a,b,-a-b): one all-equal triple, three disjoint nonzero equality
conditions with66 solutions each, and4290 distinct triples.

## Scope that must remain explicit

1. **Only two load inequalities are met.** The matching has Delta=1.
   It satisfies M>=2Darr^2 Delta and M>=L Darr, as tested, but does
   not satisfy the primary third condition2L<=Delta. Consequently
   this is not a test of all assumptions of the original packet
   lemma or its whole recursive theorem. A final README/report
   should state this directly, as the review task already does.
2. **The ternary label space is different.** The primary binary
   construction has Darr=r^(2a), whereas this supplied gadget has
   four coordinates per level and Darr=r^(4a). Finite success does
   not transfer the primary asymptotic parameter estimates.
3. **F67 is a single-pair panel.** It does not enumerate all paired
   four-entry F67 classes or construct their full matching graphs.
   The complete class panels are exactly F2 at a1,a2 and F5 at a1.
4. **Two targets require distinct target modes in a later proof.**
   The packet regression proves that a blanket one-cross-target
   assertion fails on an actual generated packet. It does not by
   itself establish the proposed18M/D mode bound,3-adic coefficient
   valuations, precision recovery, child contractions or recursion.
   Those require their separate mathematical hypotheses and checks.
5. **These are bounded exact-integer operations.** They fit the
   fixed panels. No generic large-field arithmetic, performance
   scaling, asymptotic exponent or sub-exponential algorithm is
   verified by this implementation.

## Retained output observed

The first strict build and run receipts record exit0. The retained
summary reports897 complete classes,251 orbits,26505 slot pairs,
22278 nonzero-difference locality checks,9990 tiles(including1800
two-target tiles),44 marginal rows,4489 F67 pairs and26934 F67
locality checks. All four declared negative controls report
rejection. The output streams are flushed/closed and checked, so
the earlier Hermite unit's minor write-state robustness concern
does not recur in this source.

These observations and source checks support the declared finite
development contract only. No optional mode addendum was reviewed
or executed as part of this core packet review.
