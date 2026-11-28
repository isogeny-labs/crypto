# Three pair sums: the remaining batch intersection

The selected pair-sum divisor has exactly `M=B²=5,987,809` distinct
x-classes, both before and after `[7]` projection. Each nonzero projected
x-class represents two signed subgroup points, so the signed pair-sum set
has `2M` elements here. A six-term relation can be sought as
`A+B+C=T`, with `A,B,C` drawn from that set, followed by a check that
the recovered base columns are distinct. This is a precise **three-sum
problem on an algebraically generated set**, not a solved index-calculus
step. The factor-base and pair-sum construction already have substantial
prior art; see [Petit et al., PKC 2016, §3.3](https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf)
and [Semaev 2004](https://eprint.iacr.org/2004/031.pdf).

## Random-set baseline

If signed projected pair sums behave as uniform independent points of the
prime-order subgroup `r=329406144237565159`, a precomputed hash of all
`2M` points accepts a sampled ordered pair `(A,B)` with probability about
`2M/r=3.64×10⁻¹¹`, because `T−A−B` must lie in the set. One hit needs
about `r/(2M)=27,506,400,441` two-pair trials, **4,594 times** the
entire `M` budget. Roughly `B=2447` useful rows would require about
`B·r/(2M)=6.73×10¹³` trials, **11.24 million times** that budget,
before independence, rank, index overlap, target descents, and sparse
linear algebra. These are random-model expectations, not lower bounds
against an algebraic batch method. A relation can have several pairings;
that changes constants, not the exponent in this baseline.

The fixed-ratio quartic locator offers `O(B·polylog p)` expected work for
one pair-sum query with `O(B)` setup. It can be composed with fixed base
terms, but the repository already documents `O(B)` triple-target checks
after `O(B²)` pair-hash setup in
[pair_sum_quotient.md](pair_sum_quotient.md). Neither version locates
three-pair intersections in `Õ(B²)` *total* work. A direct pair hash
uses `Θ(M)` memory and the random two-pair trial route costs
`Θ(r/M)=Θ(p^(2/3))` for one hit in a family with `M=Θ(p^(1/3))`.

## What would be a new piece

A candidate batch operator must use the algebraic structure of the
ratio-quartic images to find enough **verified, independent** three-pair
relations and both target descents with `Õ(M)` combined preprocessing,
queries, witness extraction, memory, and sparse solve. A low-degree
description of each pair fiber is insufficient if the operator still
scans `Θ(r/M)` pair trials, expands `Θ(B³)` triple sums, or creates a
`Θ(M²)` convolution table. The exact collision-free pair census removes
one proposed shortcut, while the `t=−1` elliptic quotient supplies a
small, testable exceptional family rather than enough rows on its own.
