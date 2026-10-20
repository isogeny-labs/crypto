# Five ECDLP follow-ups: executed evidence and remaining work

This is a follow-up to the five leads in
[the lead register](../new_leads_20261019/README.md). Each result below names
the operation that was actually run. The two genus-2 changes have full-cohort
execution and replay. Mixed fibers have end-to-end pair-table, exhaustive
summation-polynomial and native F4 DLP controls, but no external msolve
comparison. The cubic route has one complete mapped source-to-answer run;
the genus-3 odd-kernel escape remains open. All tested curves, points and scalars are public
synthetic or published fixtures.

## Cubic odd-torsion transfer

For [Tian's published cubic example](https://arxiv.org/html/2012.07173),
the native [kernel checker](native/tian_cubic_kernel.rs) constructs
\(\mathbf F_{523^3}\) and its quadratic tower, checks irreducibility, verifies
all nine listed point components have order three on their conjugate
elliptic curves, and enumerates all 27 kernel elements. The three generators
are stable under Frobenius with rows \((0,1,0),(0,0,1),(2,0,0)\). Product
Weil pairings evaluated with two auxiliary choices equal one for every
generator pair; the subgroup is maximal isotropic. A point of prime order
143,069,629 and the Hasse interval certify the source group order, so the
prime source subgroup
has trivial intersection with the 3-kernel. The
[machine receipt](correctness/tian-cubic-kernel-v1.json) includes the
witness, component pairings and explicit false flags for quotient and
transfer execution.

I transcribed the six rational functions for Tian's published source map
from the original TeX with a [SHA-checked extractor](extract_tian_map.py)
into a [coefficient fixture](fixtures/tian-cubic-map-v1.json). The native
[map gate](native/tian_cubic_map_gate.rs) evaluates them on 100 successive
multiples of the published source point and on its published 1987 multiple.
Every denominator is nonzero; both coordinate cubics are separable; and
multiplication by the quartic in their nine-dimensional coordinate algebra
has rank six, showing that precisely three geometric pairs lie on the
quartic. A one-unit corruption of the first coefficient fails this check.
The 101 source points and their coefficients are
[retained](correctness/tian-cubic-map-gate-v2.jsonl). This validates the
published divisor cut on these inputs. The trace and DLP checks are below.

The [quartic checker](../../src/bin/tian_quartic_smoothness.rs) derives a unit
gradient ideal in each affine chart for the copied published \(\mathbf F_{523}\)
quartic; a deliberately singular quartic fails the same check. This is
geometric smoothness of the written equation, not proof that it is the
quotient of the checked kernel. A new native
[evaluation-space Jacobian implementation](native/tian_quartic_km.rs) checks
the published destination classes \(S,T\), \(T=1987S\), and \(rS=0\) using
the genus-3 quartic group law. The known 1987 is used for this certificate;
this is not an attack answer. An independently optimized
[source rho control](correctness/tian-cubic-rho-v2.jsonl) recovers and
source-verifies 1987 on five seeds. Its cubic-field inversion charges one
base-field inversion via the norm formula.

The [arithmetic gate](results/tian-km-arithmetic-gate-summary.json) prices
the destination group law against those complete rho controls. At
\(M+0.5S+10I\), one verified destination doubling alone costs 13,318,526
base-field multiplication equivalents, or **14.04×** the five-seed mean
source rho cost of 948,415. Across the 12 arithmetic prices, that one
doubling spans **0.997–16.46×** mean rho. The deliberately known-scalar
\(1987S\) multiplication costs **212.42×** mean rho at the representative
price, with a **15.13–249.03×** price band. These are arithmetic diagnostics,
not transfer DLP ratios.

A separate native [bounded destination search](native/tian_quartic_km.rs)
received the published \(S,T\) classes but not their scalar. It recovered
**1987** after 1,987 tested multiples and checked the answer on the source
curve by recomputing \(1987P=Q\)
([receipt](correctness/tian-km-linear-dlp-v1.json)). Its cold cost, including
quartic setup and source verification, is 49,265,606,844 base-field
multiplications, 20,468 squarings and 3,275,161 inversions. At the same
representative price this is **51,979.77×** the mean optimized source-rho
cost; the [native 12-price grid](results/tian-km-linear-dlp-price-grid.csv)
spans **3,710.83–60,937.36×**. These are cold field-operation costs;
native logical work is unpriced on both arms. This is a verified DLP on
*published destination classes* for one unusually small scalar. The later
mapped index-calculus run is the source-transfer comparison.

The new native [line-sieve solver](native/tian_quartic_km.rs) executes an
index-calculus DLP on the same *published destination classes*. It enumerates
522 affine rational quartic points and 11,116 fully split rational lines.
The line divisor relations have rank 521 modulo the stated prime order,
leaving one factor-base log scale. A degree-three rational-divisor descent
splits (S) immediately and (6T) after five failed target attempts.
Each successful descent is checked again by constructing the rational
divisor's Jacobian class and comparing it to the negative input class.
Solving the two resulting equations recovers **1987** without that scalar as
a solver input. Both the destination group law and recomputation of
(1987P=Q) on the source curve verify the answer
([raw receipt](correctness/tian-quartic-line-sieve-dlp-v1.json)).

The [native cold price grid](results/tian-quartic-line-sieve-price-grid.csv)
includes quartic enumeration, all line relations, modular elimination,
factor-base nullspace recovery, failed and successful descents, and source
and destination verification. At (M+0.5S+10I+0.1A), with one modular
addition priced at 0.1 multiplication, the candidate costs **606,292,001.4**
base-field multiplication equivalents against **1,079,600.82** for the mean
of five source-rho solves, a **561.59× loss**. Across the 36 (S,I,A)
prices it loses in every cell; the ratio spans **40.85–684.92×**.
The scalar-field and curve-field M/S/I/addition vectors are in the
[analysis](results/tian-quartic-line-sieve-summary.json). Allocation,
comparisons and control flow are unpriced, so this is an operation-cost
comparison, not a qualified wall-speed result. This initial run received
the published (S,T) directly, so its cost excludes source mapping.

For the mapped run, the native evaluator converts each cubic divisor cut
into nine base-field linear constraints for its Frobenius trace. A cubic
with exact ninefold contact at the chosen point at infinity has the
[verified residual divisor](correctness/tian-osculating9-gate-v1.json)
given by \(x^3+337x^2+392x+60=0\) and
\(y=188+405x+180x^2\). Adding that residual to the trace produces a
degree-twelve divisor in the Jacobian model. The
[class gate](correctness/tian-cubic-trace-map-gate-v1.json) checks that all
100 successive source multiples map to the matching multiples of the
published S, and that the published Q maps to T: **101/101** comparisons
pass. The [P/Q class receipt](correctness/tian-cubic-trace-map-pq-v1.json)
stores the actual mapped classes used by the solver. Q's published
coordinates are the map input; 1987 is used only for the independent
known-answer checks.

The [mapped line-sieve run](correctness/tian-quartic-line-sieve-mapped-dlp-v1.json)
consumes those computed classes, recovers **1987** from the line relations
and descents, and verifies \(1987P=Q\) on the source as well as the
destination relation. Its [cold 36-price analysis](results/tian-quartic-line-sieve-mapped-summary.json)
charges the six rational-function evaluations and trace-constraint
construction for P and Q, the trace classes and checks, the complete
line-sieve solve, and final verification. At \(M+0.5S+10I+0.1A\), it costs
**697,039,875.2** multiplication equivalents against **1,079,600.82** for
mean source rho: a **645.65× loss**. It loses in all 36 cells, spanning
**47.14–787.55×**. The [grid](results/tian-quartic-line-sieve-mapped-price-grid.csv)
retains every price. The counted implementation also pays for the published
S,T comparison as a map check. This establishes an end-to-end result for this single
published fixture; an independent reconstruction of the quotient from the
checked 3-kernel, exceptional-input handling and growing-size evidence
remain [open](../../problems/odd-isogeny-quartic-transfer-execution.md).
The [replay hashes](correctness/tian-cubic-mapped-pipeline-replay.sha256)
bind byte-identical repeats of the formula, P/Q trace and mapped DLP stages.

## Genus-2 target-useful relations

The [frozen protocol](protocols/genus2-choice2-v1.md) adds one seeded
two-choice collector method to the original exact-order cohort: 21 curves,
60 targets each, sizes 16 through 40. Each continuation evaluates and charges
both prospective jumps, inserts both valid relations, and then follows the
candidate favored by a component/cycle heuristic. The signed elimination or
a charged group identity is still required to recover the target scalar.
This is one heuristic candidate, not a proof of optimal rank targeting.

The seven generated [specifications](specs/genus2-choice2-v1/) retain the
original curve/target identities and include the incumbent v2 collector,
source plain/negation rho and destination Jacobian rho. Analysis and replay
receipts cover all 6,300 arm records and 1,260 paired one-target workloads.
Every arm recovered and source-verified its scalar, and all 6,300 independent
replays reproduced the original records with zero audit problems
([audit summary](results/replay-audit-summary.json)). The
[native reanalysis price grid](results/genus2-choice2-native-v3-price-grid.csv) retains all 12 arithmetic
prices and three native-operation charges, with per-curve as well as
per-size ratios. At the representative
\(M+0.5S+10I\) price and 0.1 \(M\) per declared native logical operation:

| Source order bits | Two-choice / incumbent | Two-choice / source negation rho |
| ---: | ---: | ---: |
| 16 | 0.943697 | 3.812897 |
| 20 | 0.972590 | 3.824903 |
| 24 | 0.981876 | 4.229300 |
| 28 | 0.988278 | 4.373698 |
| 32 | 1.018553 | 4.366459 |
| 36 | 1.027152 | 4.694925 |
| 40 | 0.993748 | 4.444705 |
| **All 1,260** | **1.000857** | **4.483247** |

The ratios divide sums of complete, cold, one-target operation costs for
paired targets. The cohort's total is 4,881,369,517 source-addition
equivalents for two-choice, 4,877,191,974 for incumbent and 1,088,802,370
for source negation rho at that price. Across the 21 individual curves, the
two-choice/incumbent ratio spans 0.933237 to 1.102391. Without any native
logical-operation charge, the two-choice/incumbent ratio is 0.990528 at the same arithmetic
price; over the entire frozen price/native grid it ranges from 0.989481 to
1.006720. The candidate processed 221,270,948 divisors against 223,343,235
for incumbent. It solved at the first graph cycle on 23 targets, against
16 for incumbent; 1,237 and 1,244 respectively required later work.
The [native cohort summary](results/genus2-choice2-native-v3-summary.json)
includes replay checks, session digests and cost bands; the retained
[original summary](results/genus2-choice2-summary.json) gives cycle counts
and logical memory peaks.
These mixed, price-sensitive results do not establish a robust full-cohort
improvement or a rho crossover. Operation counts are L0 evidence; they do
not qualify a wall-speed claim.

The dated [operation-cost report source](reports/choice2-operation-costs-20261019/report.tex),
[SVG graph](reports/choice2-operation-costs-20261019/choice2-operation-cost-ratios.svg),
[150 dpi PNG graph](reports/choice2-operation-costs-20261019/choice2-operation-cost-ratios.png),
and [renderer source](reports/choice2-operation-costs-20261019/render.py)
reproduce the fixed-price, per-size and pooled ratios from the frozen price grid.
The browsing copy is in
[`benchmarks/20261019-five-leads-choice2-costs/`](../../benchmarks/20261019-five-leads-choice2-costs/README.md).
The [inspected PDF](reports/choice2-operation-costs-20261019/report.pdf)
was rendered from the equivalent Typst source after the offline TeX build
encountered a missing `tuenc.def` cache file; the
[build note](reports/choice2-operation-costs-20261019/PDF_BUILD_BLOCKER.md)
retains that history.
No timing or performance-history entry is derived from these operation counts.

The next collector test is an explicit target-coefficient rank score from
the current signed linear system before branch selection. It must charge
both provisional eliminations and every rejected relation, preserve the
same target identities and caps, and be compared with this two-choice arm
as well as incumbent and source rho. The component score tested here is
only a proxy for that algebraic dependency.

## Projective genus-2 arithmetic

The [model gate](native/genus2_model_gate.rs) applies
\(x=L u-a_4/(5L),\;y=L^3 v\) to all 42 recorded Scholten quintics
from the field-size and exact-order fixtures. Each transformed equation
is monic and has no \(u^4\) term, the model required by the published
[complete projective group law](https://link.springer.com/article/10.1007/s13389-024-00351-7).
The gate validates 100 transferred divisor samples and 100 generator-addition
commutation checks per model: 4,200 of each.
The [receipt](correctness/genus2-model-gate-v1.jsonl) records coefficients
and samples. This establishes applicability of the equation form. A new
[fraction-free CRT numerator chart](../../src/cryptanalysis/three_leads_followup.rs)
implements ordinary degree-two divisor additions with one normalization
inversion and falls back to complete affine Cantor on exceptional inputs.
It is not the complete projective law from the cited paper.

The [frozen chart protocol](protocols/genus2-projective-chart-v1.md) uses the
same 21 exact-order curves and all 1,260 original targets as the two-choice
collector. The corrected cohort completed **3,780 source-verified arm
records** (projective chart, unchanged affine collector, matched source
negation rho), with no failure or exhaustion. The native
[analysis](results/genus2-projective-summary.json) checked that all
unchanged affine/rho counters exactly match the archived cohort and retained
all 21 curves and 60 targets each. The seven
[replay receipts](replay_projective_chart_v2/) independently reproduce every
record. A first developmental archive was excluded because it added
zero-valued chart counters and a detail label to the unchanged affine arm;
the corrected run is the only priced projective archive.

At \(M+0.5S+10I\) and 0.1\(M\) per declared native logical operation, the
pooled projective/affine ratio is **0.642714×**, while
projective/source-rho is **2.881445×**. Its complete one-target cost is
3,137,323,980 source-addition equivalents, versus 4,881,369,517 for affine
and 1,088,802,370 for rho. Over the full 12×3 price/native grid, chart over
affine spans **0.5471–0.9760×** and chart over rho spans
**1.3868–15.1219×**. The chart beats affine in all 36 pooled cells, every
size's 36 cells, and every curve's 36 cells; it loses to rho in every one of
those cells. The [price grid](results/genus2-projective-price-grid.csv)
includes all prices, sizes and curves. There were 221,529,716 ordinary chart
additions and 483,273 exceptional fallbacks; the relation collector processed
the same 221,270,948 divisors as affine. These are L0 operation counts,
without a qualified wall-speed or external-implementation comparison.

## Odd-kernel exits from Smith-negative inputs

The published \(\mathbf F_{257}\) genus-3 positive control from
[Tian's odd-isogeny construction](https://arxiv.org/html/2007.03172)
was partially replayed: a rational divisor \(T_1\) validates and
\(3T_1=0\) in the repo's hyperelliptic arithmetic
([receipt](correctness/tian-genus3-f257-t1-result.json)).
The copied rational quartic passes the gradient-ideal smoothness check.
Its [extension-field kernel gate](correctness/tian-genus3-f257-kernel-gate-v1.json)
now verifies the defining polynomial for \(\mathbf F_{257^6}\) is
irreducible, all three published generators satisfy \(3T_i=0\), their
27 linear combinations are distinct, and Frobenius sends the three
generators to subgroup coordinates \((1,0,0),(1,2,2),(2,1,0)\).
Thus the listed subgroup is rational as a set. Its Weil-pairing isotropy,
quotient-from-kernel construction and induced general Jacobian map have
not been checked.

The original TeX of the same paper has SHA-256
`46092fc37bc6bac7e0f42c67e6764150557f63ac4292da86d739980268d3eaae`.
Its printed \(\mathbf C_1\) numerator says `103x+207x`; both of its
explicit mapped divisor examples require `103+207x`. The
[extractor](extract_tian_genus3_map.py) applies only this recorded
emendation. The [point-map gate](correctness/tian-genus3-f257-map-gate-v1.json)
reproduces both published cubic pairs exactly and enumerates every affine
\(\mathbf F_{257}\)-point: 251 source points, no denominator zeros, 249
degree-three intersection cuts (quartic multiplication rank six), and two
exceptional rank-five cuts at (84,35) and (105,92). Corrupting one printed
coefficient gives rank nine. Independently, the repo's Jacobian arithmetic
checks the paper's source relation \(86241[(2,7)-\infty]=[(121,5)-\infty]\)
([receipt](correctness/tian-genus3-f257-p1-scalar-result.json)). These
point-cut checks alone do not establish the induced Jacobian homomorphism;
the two rank-five cases require special divisor handling.

For the two ordinary published points, the [destination transfer
gate](correctness/tian-genus3-f257-transfer-gate-v1.json) constructs their
quartic Jacobian classes and checks the 86,241 scalar relation. A separate
[mapped line sieve](correctness/tian-genus3-f257-line-sieve-mapped-dlp-v1.json)
evaluates the six corrected map functions on the source inputs, constructs
both destination classes, collects 2,458 fully split lines over a 250-point
factor base and obtains rank 249 modulo 172,553. It recovers 86,241 from
the relation kernel and two rational-divisor descents, lifts over the
factor-four source subgroup cofactor, and checks the destination and the
independent source-scalar [receipt](correctness/tian-genus3-f257-p1-scalar-result.json).
The published scalar is used only after solving. This is a complete mapped
DLP on **one published F257 pair**, not a Smith-negative holdout result.

The independently counted [source rho runs](correctness/tian-genus3-f257-source-rho-v1.json)
first certify that the source point has order \(690212=4\cdot172553\),
then recover and source-check 86,241 on five seeds with a genus-3 Cantor
law. A negation-normalized birthday-walk control also solves on five seeds;
the conventional walk has lower mean priced cost at the representative
price. The [36-cell cost analysis](results/tian-genus3-f257-line-sieve-cost-summary.json)
includes quartic setup and mapping, curve and scalar-field line relations,
descent, full-order lift, destination check and source check. Its cold
count vector is \((519482988 M,36928 S,35740 I,490114430 A)\). At
\(M+0.5S+10I+0.1A\), the mapped line sieve costs 568,870,295 equivalents
against 704,665.7 for the faster mean source rho: **807.29× slower**.
It loses to rho in **36/36** pricing cells (47.40–1004.66×). Even charging
the rho side for source-order discovery leaves a 433.74× loss at the
representative price and losses in every cell.

A [bounded mapped BSGS control](correctness/tian-genus3-f257-split-bsgs-twist0.json)
also recovers the scalar without it as solver input, after 512 baby and
168 giant steps, using canonical rational-divisor fingerprints. Directly
hashing evaluation-space matrices [exhausts its fixed interval](correctness/tian-genus3-f257-direct-matrix-hash-control-v1.json)
because equivalent Jacobian classes can have different matrices; the
successful collision explicitly has unequal matrices. The line sieve wins
against the corrected mapped BSGS in 36/36 price cells, but both lose to
source rho. Counts omit memory, hashing and control flow and are not
qualified wall-speed measurements. The extension-field kernel generators,
quotient-from-kernel proof, exceptional point handling and 20-input
Smith-negative odd-kernel screen remain open.

A [20-input holdout](fixtures/smith-negative-holdout-v1.json) was frozen
before odd-kernel testing from the prior \(p=2^{31}-1\) Smith census:
ten with no tractable two-torsion kernel and ten with a trigonal candidate
but no isotropic two-kernel. The
[materialization replay](correctness/smith-negative-materialized-v1.jsonl)
regenerates every degree-eight polynomial and checks the archived verdicts.
No odd torsion or odd-kernel exit was tested on these inputs. The held-out
coverage question remains open with the quotient/map work above.

An exact [rational-root and model-conversion gate](correctness/smith-odd-model-gate-v1.json)
finds 15 of the 20 frozen curves have a rational branch point. For each
such curve it constructs a monic degree-seven one-infinity equation using
`z=1/(f'(r)(x-r))` and `w=z^4 y`, checks the root and derivative, and checks
the polynomial identity at six nonzero coordinates. A second
[native arithmetic gate](correctness/smith-odd-model-native-gate-v1.json)
maps one independently selected rational point on each of those 15 curves,
validates its divisor with the checked Jacobian CLI, and verifies
`[3]D+[5]D=[8]D` and `[p]D+D=[p+1]D`. It also checks all 12 rational images of old infinity
points and rejects 15 deliberately corrupted point classes. Five curves
have no rational branch point and still
require a two-infinity or extension-field adapter. These 15 arithmetic
checks do not test 3- or 5-torsion: the requested odd-kernel screen,
quotient, mapped holdout DLP and cost comparison all remain unexecuted.

## Mixed-fiber PKM systems

The [small oracle protocol](protocols/mixed-fiber-oracle-v1.md) freezes three
prime fields, three curves per field and 20 targets per curve. For each
target, the native pair-sum oracle enumerates the complete three-summand
solution set for homogeneous, mixed and matched random fibers, then compares
it with an independent exhaustive triple enumeration. All 540 oracle records
agree ([receipt](correctness/mixed-fiber-census-v1.jsonl)).
The retained historical [cost summary](results/mixed-fiber-summary.json),
assembled with the earlier Python analysis script, reports cold pair-table
work, including fiber selection. At the declared
\(M+0.75S+10I\) price, mixed/homogeneous cost is 1.0191, 1.0052 and
1.0108 for \(p=97,193,257\), respectively; mixed/random cost is
0.8650, 0.8876 and 0.8952. These are **pair-table oracle ratios**.
They are not costs for heterogeneous polynomial solving or a DLP.

A subsequent [native pair-table DLP](native/mixed_fiber_dlp.rs) selected nine
eligible prime-order curves (three per field) and 20 distinct exact-order
targets per curve. These are new curves because some of the original oracle
curves have order five and cannot supply 20 distinct nonzero targets. The
solver received each target point without its planted scalar, built and
charged its fibers and pair table, accumulated modular logarithm rows, solved
for the scalar, and checked every result by recomputing \(kP\). All 540
candidate records and their matched source-rho controls verified; the
[replay receipt](correctness/mixed-fiber-dlp-v2-replay.sha256) binds a
byte-identical rerun. At \(M+0.5S+10I\) with 0.1\(M\) per declared native
logical operation, the pooled homogeneous, mixed and matched-random costs
are respectively **3.6435×, 3.2222× and 3.7180×** rho. Mixed fibers beat
homogeneous in this implementation, but all three lose to rho in every
pooled pricing cell. Mixed beats rho in 16/36 cells for the \(p=97\) slice and
none for \(p=193,257\). The
[full native price grid](results/mixed-fiber-dlp-native-v3-price-grid.csv) retains every
curve, price and loss. This measures the **pair-table oracle** variant; the
polynomial and F4 implementations are reported below.

A new native [Semaev (S_4) solver](native/mixed_fiber_polynomial_dlp.rs)
evaluates the full resultant polynomial over every prescribed three-fiber
tuple, enumerates every rational zero, tests all sign lifts, and inserts only
group-law-valid relations. It reran the same 180 target points on all three
arms: **540/540** candidate scalars and all matched rho controls verified on
the source; no run exhausted. The raw
[540-record receipt](correctness/mixed-fiber-polynomial-dlp-v1.jsonl) includes
3,783,011 polynomial evaluations, 21,849 polynomial roots and 165,669 sign
lifts, including failed lifts. A small-curve test checks every finite group
sum against the (S_3,S_4) formulas; the native analyzer matches all 540
targets, fibers and rho controls to the earlier pair-table cohort.

At (M+0.5S+10I) and 0.1(M) per declared native logical operation, the
[36-price analysis](results/mixed-fiber-polynomial-dlp-summary.json) gives
homogeneous, mixed and random **57.8840×, 32.2975× and 34.4846×** rho.
Each loses in all 36 pooled price cells. Against the prior pair-table solver
on identical targets, the corresponding costs are **15.8869×, 10.0233× and
9.2752×**; all 36 pooled cells lose for every arm. Mixed fibers beat
homogeneous at this price but lose to matched random at (p=97); the
[full grid](results/mixed-fiber-polynomial-dlp-price-grid.csv) retains
per-prime and per-curve wins and losses. This is an exhaustive polynomial
all-roots solver, **not F4 or msolve**. The fastest external Gröbner-system
comparison and larger-size evidence remain
[open](../../problems/mixed-fiber-pkm-full-solver.md).

The repository's [native F4 engine](../../src/bin/five_leads_mixed_f4.rs)
also solved all 540 target-coordinate systems with exact root-set agreement
against exhaustive enumeration and no timeout
([receipt](correctness/mixed-fiber-f4-target-cohort-v1.jsonl)). For a real
F4-backed DLP replay on the same 540 one-target arms, it then handled
**2,316 actual relation trials**: F4 solved 2,290 nonidentity right-hand
sides, while 26 identity right-hand sides needed no polynomial system.
Every F4 root set was checked against the exhaustive polynomial oracle
([query receipt](correctness/mixed-fiber-f4-dlp-oracle-v1.jsonl)).
The collector consumed those ordered F4 roots, sign-lifted them, recovered
**540/540** scalars and checked every (kP=Q) on the source
([DLP receipt](correctness/mixed-fiber-f4-dlp-v1.jsonl)). The native
[analyzer](native/mixed_fiber_f4_analyze.rs) matches every target, query,
relation count and answer to the exhaustive polynomial and pair-table
controls. There were zero F4 timeouts or root-set mismatches.

F4 reports **467,482,105,353** prime-field multiplications in row reduction
across the 540 DLP arms. Adding those to the collector's recorded
field-operation and declared native counts yields a **lower bound** on cold
cost. At (M+0.5S+10I) and 0.1(M) per native logical operation, the
homogeneous, mixed and random lower bounds are **160,899×, 232,110× and
239,616×** matched rho, respectively. All three lose in all 36 pooled
pricing cells and also lose to the pair-table arms in all 36. Within this
counted lower bound, mixed/homogeneous is **1.44258×** (36/36 losses),
while mixed/random is **0.96868×** (36/36 wins). The
[full grid](results/mixed-fiber-f4-dlp-lower-bound-price-grid.csv) includes
the per-prime and per-curve rows; the
[summary](results/mixed-fiber-f4-dlp-lower-bound-summary.json) includes
cross-arm comparisons. F4 symbolic preprocessing,
substitution arithmetic, allocation and control flow are not in its
`field_ops` counter; the recorded **356.1 s** sum of F4 internal wall
times is unqualified on a busy host and is not a rho wall-time ratio.
Thus the lower bound suffices to establish a loss to rho on these small
systems, but does not establish a complete F4 cost or external-tool ranking.

The [results report PDF](reports/results-20261020/report.pdf) contains the
quantitative graphs and links the seven individual projective
[run reports](reports/) to their raw sessions. The corresponding SVG and PNG
figures are beside the PDF. The
[benchmark browsing copy](../../benchmarks/20261020-five-leads-followup-operations/)
contains the PDFs and figures; source records and native analyzers remain
in this study directory. The canonical index-calculus scoreboard was checked
and left unchanged because neither new solver beat source rho; the separate
cubic gate and untested odd-kernel route are not eligible for an end-to-end
scoreboard entry.

## Reproduction and qualification

The exact source and protocol are under this directory. Native fixture tests
pass with:

```sh
cargo test --release --offline --locked --manifest-path research/five_leads_e2e_20261019/native/Cargo.toml
cargo test --release --offline --locked --lib
```

The native crate passed 13 tests
([log](correctness/native-tests-v3.log)); the full root library passed 4,418
tests with 118 ignored and zero failures
([log](correctness/root-lib-tests-v2.log)). The cohort's ecbench audit and
independent replay receipts are under `replay/` and
`replay_projective_chart_v2/`; full raw sessions are preserved in archives.
No L2/A/A isolated timing or fastest external implementation comparison was
run. This report therefore claims no hardware-speed record.
