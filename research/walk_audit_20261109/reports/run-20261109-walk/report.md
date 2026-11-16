---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Exact walk boundaries and censuses

Lean checks actual PMF/group counterexamples to dropping fixed-support
conditions and to a printed in-degree variance sentence. All four frozen
native calls pass: 3,930 mixing supports and 82,034 fixed hash maps, each
population repeated twice with identical bytes. The walk project gate
passes 21 declarations, 0 failing, 0 trusted-native exceptions. New claims
PCC-20261109-13b8fc and PCC-20261109-a34842 remain unsigned. [pcc:unverified]

![Complete finite mixing populations: uniform iid increments, start0, TV threshold1/4. Points are support means; whiskers are the full finite support range, not confidence intervals. Both repetitions agree;0censored.](mixing.pdf){width=98%}

All distinct supports of sizes 2,3,4,6 where possible, over primes 5,7,11,13.
Native exact counts compare sparse recurrence to dense matrix multiplication
at every step, checking mass and TV contraction. The first threshold crossing
and previous exact TV are retained. Fixed cap 4p²; no censored support or
extension occurred. For two increments every support gives steps 5,9,23,32
at those four primes. For six increments, complete ranges are 1 at p7,
2 at p11, and 2–3 at p13. These are exact finite values, not asymptotic fits.
[pcc:unverified]

Lean constructs the actual distribution from uniform histories
(Fin t→Fin4), proves its product probability, and maps the histories to
their sum in Z/p. The four increments are distinct when p≥4. After t
steps an arc of at most 3t+1 residues has mass1. Its uniform mass is at
most(3t+1)/p. When 6t+2≤p this violates the event characterization of
quarter-TV mixing. The closed prime witness p101,t10 has event mass1
versus uniform31/101. No assumed transition table is used. [pcc:unverified]

Hildebrand1994 Theorem1 averages over uniformly chosen distinct supports,
with fixed positive weights and sufficiently large prime modulus. Its
Theorem2 and arc-support discussion explicitly permit slower fixed
supports. The Lean witness corrects an all-support reading and does not
refute the qualified expectation theorem. Full proof of that averaged
asymptotic result remains open. [pcc:unverified]

Flow: 4 planned, attempted and completed; 0 not-completed, interrupted,
resumed or excluded. One local shared host under the approved checkpoint
exception, one thread, zero paid spend, no clock metric. Native source
8c376a41, preregistration commit d801653d preceded affected calls; protocol
hash prefix dfd90b02047a. Export and every referenced file hash validate.
Held-out preflight is disclosed and excluded from these four rows.
[pcc:unverified]

\clearpage

![Exact collision means relative to the finite uniform-mapping baseline. Both support strata were frozen before data. The dashed heuristic factor uses its squared value r/(r−1). Finite normalization differs from the asymptotic sqrt(pi*p/2). No nonzero full-support table exists at p3,r3.](collisions.pdf){width=98%}

For each of 80 supports (sizes2,3; primes3,5,7), enumerate every label
function, giving 82,034 maps. Start0 and iterate one fixed hash map.
Visited-array tail/cycle detection agrees with Floyd for every map; all
first repeats obey the proved pigeonhole bound p. Exact first-repeat
histograms, in-degree moments and rational means are retained. Uniform
labels describe the ensemble; individual partition sizes need not match.
[pcc:unverified]

| p | r | All-support mean | No-zero mean | Uniform-mapping mean |
|---:|---:|---:|---:|---:|
| 3 | 2 | 2 | 5/2 | 17/9 |
| 3 | 3 | 17/9 | absent | 17/9 |
| 5 | 2 | 23/8 | 7/2 | 1569/625 |
| 5 | 3 | 1073/405 | 89/27 | 1569/625 |
| 7 | 2 | 823/224 | 87/20 | 355081/117649 |
| 7 | 3 | 28289/8505 | 4882/1215 | 355081/117649 |

Zero increments can create immediate self-loops. Both all-support and
no-zero strata are therefore shown. This census does not test nonuniform
p_i or condition on empirical partition sizes. Its finite ratios do not
decide a limiting collision constant. Full support at p3,r3 has histogram
9/12/6 and mean17/9; Lean separately proves that full-support uniform hash
tables induce uniform endofunctions by an actual function-space equivalence.
[pcc:unverified]

Galbraith ch14 version2.0, bookp297, prints r/(r−1) as the in-degree
variance. Lean checks all actual functions Z/3→Fin2, transition x+1+hash(x),
with two distinct nonzero increments. Hash-space cardinality8 and normalized
mean1 are proved; the exact variance is1/2, unequal to the printed2 at r2.
The native census's in-degree check agrees with( r−1)/r on all 80supports.
This refutes that literal variance sentence in this model; it does not
refute the heuristic collision penalty, which uses reciprocal variance.
The primary PDF page was visually checked. [pcc:unverified]

Sources: Hildebrand DOI10.1007/BF01199265, Theorems1–2; Galbraith ch14
§14.2.5/p297 (PDF SHA256 prefix ab59d8013db8). Full source hashes/host/build:
../../provenance.json. Kernel source: ../../lean/WalkBoundary.lean.
Raw distributions/histograms: ../../evidence/runs. Frozen native source:
../../native/census.cpp. Canonical record: ../../protocol.json and
../../export.json. Both new PCC packets pass full local checks but remain
unsigned; no independent-host or signed promotion is asserted.
[pcc:unverified]
