# CT100 — Chord capacity gate and rational cross-fiber positive control

**2026-11-14. Scientific standing [pcc:unverified].** Exact elementary algebraic obstruction to scaling CT99's *single chord*, six independently replayed finite-field toy fixtures and a complete rational order-five cross-characteristic positive control. No security speedup or novel group-law theorem is claimed.

## Sources and scope

Read AGENTS.md, Nov. 14 session log, root crypto PR #3877, CT81–94 PR #4074, CT95–99 PR #4102, and cryptanalysis public-target PR #1523 and Frobenius/Richelot PR #1511. The dedicated research-KB MCP and repo CLI are unavailable in this connector runtime; prior reports and GitHub search are fallback, not evidence of novelty. Primary classical sources are [Stanford on the chord-and-tangent group law](https://crypto.stanford.edu/pbc/notes/elliptic/group.html), [Brown's line/cubic intersection lemma](https://www.math.brown.edu/reschwar/M1540B/elliptic.pdf) and [Shoup's generic lower-bound paper](https://www.shoup.net/papers/dlbounds1.pdf). CT96 already studied the Galois tower; do not repeat its conditional field-degree results as a new discovery.

The inspected CT99 exact head passed its scoped math, validation and parity CI, but PCC prose lint failed on two lines of the PR body lacking scientific provenance annotations. Those two lines were corrected with [pcc:unverified] tags; a new run must confirm the correction before certification.

## Theorem: one affine chord supports at most three cyclic torsion x-classes

For E: Y²+Y=f(X)=X³−X²−10X−20 over any field with this nonsingular elliptic curve, substitute Y=lambda X+mu. The intersection abscissae are exactly the roots of

  f(X)−(lambda X+mu)²−(lambda X+mu).

This is monic cubic, hence has at most three distinct x-values (even in characteristic two); a vertical line has only one. For an odd prime ell and a point T of exact order ell, the nonzero cyclic subgroup has m=(ell−1)/2 distinct x-values x([j]T) for j=1,...,m: equal x means two points are inverses, so i is congruent to ±j modulo ell. Thus when ell≥11, m≥5>3 and **no single chord can supply an ordinate for every root of the cyclic subgroup's relative x-polynomial**.

The CT99 line goes through T, [2]T and −[3]T. For ell=7 its three x-classes are precisely 1,2,3, so the cubic is saturated. This is a special small-order coincidence, not a general orientation algorithm for ell≥11. Covering m classes by *arbitrary* affine lines requires at least ceil(m/3) lines; if every line passes through one fixed T, at least ceil((m−1)/2) are needed. These are **incidence / explicit-representation bounds**, not a lower bound on all algebraic point-recognition algorithms. Multichord and nonlinear representations remain possible.

## Independent six-fixture control

The same 11a1 model is specialized at six good primes. Python counts points by quadratic-character discriminants, computes double-and-add and runs source BSGS. Separately written JS counts each affine point directly, uses Euclidean modular inverses and repeated-addition / exhaustive-log controls.

| ell | source p | #E(Fp) | x-classes m | single chord hits | misses | min arbitrary lines |
|---:|---:|---:|---:|---:|---:|---:|
| 5 | 7 | 10 | 2 | 2 | 0 | 1 |
| 7 | 37 | 35 | 3 | 3 | 0 | 1 |
| 11 | 59 | 55 | 5 | 3 | 2 | 2 |
| 13 | 139 | 130 | 6 | 3 | 3 | 2 |
| 17 | 811 | 850 | 8 | 3 | 5 | 3 |
| 19 | 193 | 190 | 9 | 3 | 6 | 3 |

For the actual CT99 prime p=37 and T=(9,4), the chord slope is 27 and its intercept 20, matching CT99 exactly. At ell≥7 the oriented intersection scalar indices are 1,2,ell−3; at ell=5 the last intersection is tangent at [2]T. **All 66 nonidentity source DLP cases** passed BSGS in Python, independently confirmed by exhaustive JS. These are correctness tests, **not matched timed cryptanalytic benchmarks**.

Four local modes produced byte-identical output SHA-256 e102c79b5abbced8bce33df16c3538838be11e6ead453404755f03ef84c94c09.

## Second exact positive control: rational five-torsion from characteristic 7 to 2

On the same global curve, T=(5,−6) has rational order five, by direct exact additions:

  T=(5,−6); 2T=(16,60); 3T=(16,−61); 4T=(5,5); 5T=O.

The prime-to-characteristic good reductions at 7 and 2 give a complete group isomorphism between the selected order-five subgroups via the *same rational global point*. They have #E(F7)=10 and #E(F2)=5. The concrete public source to binary output table is:

| public F7 | target F2 |
|---|---|
| (5,1) | (1,0) |
| (2,4) | (0,0) |
| (2,2) | (0,1) |
| (5,5) | (1,1) |
| O | O |

This is **not a scheme morphism between characteristic 7 and characteristic 2**. The abstract map is reduction at 2 composed with the inverse of reduction at 7 on the *fixed global rational five-torsion subgroup*. No input scalar is passed to the lookup. **Its cold setup stores the entire subgroup, so it is simultaneously a full discrete-log table, not an efficient attack.**

Two independent implementations check the five rational points, both finite curve orders, all **25 ordered group laws** over Q, F7 and F2, four public source and four target DLPs (Python BSGS, JS brute force), and all five source→binary lookup results. Four local modes produced byte-identical JSON SHA-256 8fd9c76473f73b96747a1419bde2474b12291b615796b0683ea374e92a23abc4.

## The rational-torsion shortcut cannot scale to large primes over Q

Mazur's rational torsion classification says that a prime-order torsion **point** on any elliptic curve over Q can only have order 2, 3, 5 or 7. See [University of Michigan course on Mazur's theorem](https://websites.umich.edu/~asnowden/teaching/2013/679/) and the [explicit torsion classification](https://planetmath.org/mazurstheoremontorsionofellipticcurves). Thus the order-five *rational-point* common lift above cannot be scaled to ell>=11 while keeping its global torsion point rational. The next non-rational ell=11 experiment necessarily requires a nontrivial extension for the point. This restriction is about **rational torsion points**; it does not exclude rational prime-degree isogenies, higher-degree number fields, or computationally useful implicit torsion representations. It is a classical theorem used as a gate, not a novel result. [pcc:unverified].

## Cold-cost boundaries

| Route | Charged cold setup | Online work | Verified cost standing |
|---|---|---|---|
| Source BSGS | O(sqrt(ell)) source group table | O(sqrt(ell)) group operations | 66 correct toy cases; no matched wall timing |
| Rational ell=5 transfer | All five global subgroup elements, four nonzero table entries | Encoded-point table lookup | Full toy proof, no advantage |
| CT97–99 ell=7 global transfer | Relative cubic, degree-8 subgroup field, degree-24 x and degree-48 oriented fields, root representations | Source-root match, binary F_(2^24) specialization | Exact toy coordinate transport, no cold timing |
| Dense higher-ell relative-root approach | m=(ell−1)/2 explicit x roots; conditional full-image x-field degree (ell²−1)/2 as in CT96 | Multichord or nonlinear ordinate required | Representation bottleneck only, no generic hardness theorem |

A fair complete-cost comparison still requires fixed public inputs, frozen number-field construction and embedding costs, arithmetic precision and coefficient heights, target logarithm recovery and fastest matched rho/IC references on the same host, with independently verified output. No EKS paid job or leaderboard promotion was authorized.

## Reproduction

From repository root, set D=research/cross_characteristic_global_torsion_20261112/ct100 and run each of the following programs in normal and optimized modes, comparing the complete one-line JSON outputs:

    python3 -B "$D/ct100_chord_capacity.py" > /tmp/ct100-py.json
    python3 -O -B "$D/ct100_chord_capacity.py" > /tmp/ct100-pyopt.json
    node "$D/ct100_chord_capacity_independent.js" > /tmp/ct100-js.json
    node --no-opt "$D/ct100_chord_capacity_independent.js" > /tmp/ct100-jsopt.json
    cmp /tmp/ct100-py.json /tmp/ct100-pyopt.json
    cmp /tmp/ct100-py.json /tmp/ct100-js.json
    cmp /tmp/ct100-py.json /tmp/ct100-jsopt.json

    python3 -B "$D/ct100_rational_torsion.py" > /tmp/ct100-rpy.json
    python3 -O -B "$D/ct100_rational_torsion.py" > /tmp/ct100-rpyopt.json
    node "$D/ct100_rational_torsion_independent.js" > /tmp/ct100-rjs.json
    node --no-opt "$D/ct100_rational_torsion_independent.js" > /tmp/ct100-rjsopt.json
    cmp /tmp/ct100-rpy.json /tmp/ct100-rpyopt.json
    cmp /tmp/ct100-rpy.json /tmp/ct100-rjs.json
    cmp /tmp/ct100-rpy.json /tmp/ct100-rjsopt.json

These were locally executed on independently written producer programs; exact committed-file GitHub CI and independent signed PCC verification are different gates. [pcc:unverified].

## Next decisive gate

Extend selective **global** algebraic x-root lifting and *oriented* quadratic extension recognition to ell=11 at source p=59, **without** a table of scalar multiples or a multiplication rational map encoding the input scalar. Reject single-chord universality before building expensive degree-60/120 field representations. Freeze a full cold-cost protocol, then compare source BSGS/rho and admitted index calculus when compute budget and environment are actually available. Do not claim scaled transfer without a same-input, independently verified end-to-end result.
