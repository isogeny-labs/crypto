---
title: "The ECBench ECDLP Performance Frontier"
subtitle: "End-to-end boundary, implementation census, bottlenecks, and the next isolated campaign"
author: "ECBench frontier audit"
date: "31 October 2026"
lang: en-US
geometry: margin=0.72in
fontsize: 10pt
colorlinks: true
linkcolor: NavyBlue
urlcolor: RoyalBlue
toc: true
toc-depth: 1
numbersections: true
header-includes:
  - |
    ```{=latex}
    \usepackage{booktabs}
    \usepackage{longtable}
    \usepackage{microtype}
    \usepackage{fancyhdr}
    \pagestyle{fancy}
    \fancyhf{}
    \lhead{ECBench ECDLP frontier}
    \rhead{2026-10-31}
    \cfoot{\thepage}
    ```
---

\newpage

# Executive answer

The repository does **not** have one global ECDLP performance frontier. It has
several frontiers indexed by curve family, target law, advice model, accounting
unit, and evidence class. Once those domains are kept separate, the boundary is
clear:

1. **Generic prime, random-binary, and Koblitz elliptic curves remain
   rho-led.** No admitted cold, one-target index-calculus result is below its
   strongest applicable matched generic reference. The closest current
   repository markers are 2.28× rho for inherited F4 on one tiny random-binary
   cell, 2.59× for the prime residual collector at 36 bits, and 3.43× on the
   admitted Koblitz ladder. These are different studies and units, so they are
   boundary markers, not entries in one sortable table.
2. **One restricted curve family is IC-led.** The plane-quartic attack on the
   special GHS-weak family $E_0/\mathbb F_{q^3}$, with
   $\#E_0(\mathbb F_{q^3})=4\ell$, is below matched rho in its own cold,
   one-target comparison. The older B0 arm crosses below rho at $q=4051$;
   the N4-v2 double-large-prime arm reaches 0.00523× negation-map rho at
   $q=2{,}097{,}091$. This is a real restricted-family result. It is **not** a
   generic-curve, standard-curve, prime-order $E/\mathbb F_{q^3}$, or
   ECC2K-130 result.
3. **The best newly validated factor-base change is engineering progress, not a
   frontier crossing.** On the 20-bit prime holdout, the selected low-x base
   grows from $N=32$ to $N=45$ and costs 0.836, 0.817, and 0.746 of the
   incumbent at native-operation prices $w=0,0.1,1$. It still costs 7.95,
   8.24, and 9.62 times matched `rho.negation`. The result was reproduced on a
   second L2 EKS host, but covers one curve and gives no exponent.
4. **F5 and F6 improve local solver work without improving E2E ECDLP cost.** F5
   uses 0.641–0.647 of matrix F4's word work at degree 13. F6 uses roughly
   0.77–0.82 of inherited-F4 word work and shortens the relation phase by
   1.7–1.9×, but its charged closure/addition work makes its whole-pipeline
   lower-bound cost 64.6–1,352.6× inherited F4 and 399–4,883× rho on the tested
   cells.
5. **The optimized rho comparison is materially stronger at the implementation
   level.** The trajectory-identical signed-Frobenius rho path uses 0.802155 of
   the same-binary legacy Callgrind instructions, a 1.246642× speedup, while
   counted GAE and all outcomes remain identical. Its native wall improvement
   remains unpromoted because no L2 wall session completed. The new campaign
   therefore uses accounting-v2 rho, a same-target A/A rho control, and also
   measures BSGS and kangaroo where they can be finite-size leaders.

![Boundary markers. A ratio is meaningful only inside its row's exact domain and unit.](figures/boundary-map.png){width=100%}

This report is an evidence synthesis at source commit
`48c8a4767d2d1eed6a7b379f74410bc729239380`. It does not present blocked jobs
as measurements. The exact current-source EKS campaign is prepared, but no
authenticated AWS/Iso Lab route is available in this workspace; consequently,
there is no fresh 2026-10-31 wall-time leaderboard.

## Boundary by domain

| Domain | Current finite boundary | Leader | Evidence reading |
|:--|:--|:--|:--|
| Prime-field generic ECDLP | closest recent IC marker 2.59× matched rho; P20 validated base row 7.95–9.62× | generic algorithms | rho remains lower; BSGS can beat rho on several toy bitop cells |
| Random-binary generic ECDLP | inherited F4 2.28× rho on the tiny m=19 diagnostic; admitted ladder 10.6–38.2× | generic algorithms | no IC win |
| Koblitz generic ECDLP | admitted cold one-target IC 3.43–11.5× matched rho | signed-Frobenius generic walk | no IC win |
| Special GHS-weak $E_0/\mathbb F_{q^3}$ | B0 0.735× at q=4051; N4-v2 DL 0.00523× negation rho at q=2,097,091 | plane-quartic IC | restricted-family IC win |
| Hyperelliptic Jacobians | genus-4 toy row 0.114× rho in bitop/v1 | Jacobian IC | not an elliptic-curve result |
| Standard cryptographic curves / ECC2K-130 | no complete break or scaling evidence | generic security boundary | unresolved by experiment; no claim of a break |

\newpage

# What “the frontier” formally means

## Domain key

A benchmark row is assigned to the exact domain

$$
D=(\mathcal C, G, \mathcal T, A, k, U, P, E),
$$

where $\mathcal C$ is the curve family and representation, $G$ is the
subgroup and cofactor treatment, $\mathcal T$ is the target distribution,
$A$ is reusable advice or preprocessing, $k$ is the number of targets
charged together, $U$ is the accounting unit, $P$ is the native-operation
pricing rule, and $E$ is the evidence class. Two observations from different
domain keys do not form a speed ratio. In particular:

- prime GAE, binary word XORs, SAT conflicts, FES Gray-code vector XORs, and
  native wall seconds are not interchangeable;
- a one-target cold run and a retained-setup online run are different objects;
- a transfer to a special weak curve family is not evidence about the original
  generic family unless an explicit, subgroup-preserving, costed map is part of
  the run;
- L0 shared-host wall time cannot displace L2 isolated wall evidence.

## Cost vector and dominance

For an admitted observation $x$ in a fixed domain, retain the vector

$$
C(x)=(W_0,W_{0.1},W_1,T_{L2},M,A,S,F),
$$

where $W_w$ is counted work with each unpriced native operation charged at
$w\in\{0,0.1,1\}$ GAE, $T_{L2}$ is isolated wall time, $M$ is peak memory,
$A$ is reusable advice, $S$ is success/censoring, and $F$ is the retained
failure set. Observation (x) dominates (y) only if both have the same domain
key, (x) is no worse on every required axis, and it is strictly better on at
least one axis. The frontier is the non-dominated set.

Unknown quantities remain unknown. They are never inserted as zero. A smaller
backend-local counter can move a component frontier without moving the ECDLP
frontier.

## Admission gate

The refreshed ranking requires all of the following:

- exact curve, subgroup, target, factor-base, binary, and algorithm identity;
- cold one-target cost as the primary object;
- strongest applicable rho reference with the same usable symmetry, plus a
  same-target rho A/A control;
- verified scalar recovery and replay of every measured record;
- at least eight public targets and at least five measured rounds;
- at least four sizes before fitting or describing scaling;
- failures, timeouts, work limits, and censored pairs retained;
- the $w=0,0.1,1$ pricing band whenever native operations are not fully
  converted;
- L2 or better isolation for a wall-time claim;
- no cross-ranking of backend-native units.

The full rules and machine-readable design are in [PROTOCOL.md](PROTOCOL.md)
and [protocol.json](protocol.json). The protocol validator reports zero
findings, but the document remains `draft` until the exact current-source ECR
digest is bound and committed before execution.

# Implementation census

The release binary exposes **74 top-level method IDs**:

| Family | Count |
|:--|--:|
| Index calculus and algebraic/transfer candidates | 31 |
| Rho and transfer references | 32 |
| BSGS | 7 |
| Kangaroo | 2 |
| Pair-claw | 2 |

The algebraic pipeline separately exposes **11 nested solver backends**:
`f4-f2`, Buchberger, matrix F4, matrix F5, inherited F4, Crossbred, XL,
SAT-CDCL, scalar FES, wide FES, and exhaustive search. These are not eleven
additional top-level ECDLP methods; they are solver choices inside a complete
pipeline.

Every ID and backend has an explicit disposition in [COVERAGE.md](COVERAGE.md)
and [coverage.json](coverage.json): 13 current-primary methods, 10 diagnostic
methods, 21 legacy controls, 6 pricing-sensitivity methods, and 24
domain-specialized methods. “Legacy” does not mean incorrect, and “blocked”
does not mean slow; it records why the method is not in the fresh primary
campaign.

The exact native registry is preserved in [method-registry.json](method-registry.json).
This prevents “all implementations” from silently meaning only the arms that
appeared in an older report.

# Generic prime-field frontier

## Generic algorithms are the finite-size leaders

On the 16–40-bit prime `bitop/v1` ladder, rho, BSGS, kangaroo, the pair-table
IC rule, and the m=3 root-finding rule are measured on public synthetic
prime-order curves. BSGS is below the current rho implementation on many toy
cells (for example 0.558× at 16 bits), while rho approaches the better scaling
boundary. The IC pair rule ranges from 6.23× to 78.3× rho; root-finding ranges
from 10.1× to 69.7× through 28 bits.

![Prime finite-size ladder. BSGS is included so the rho reference is not mistaken for the absolute finite-size leader.](figures/prime-bitop-ladder.png){width=100%}

This is why the prepared campaign includes `bsgs.negation_v2` and
`kangaroo.vow_v2` beside `rho.negation_v2`. Rho remains the asymptotic generic
reference; it is not assumed to win every tiny finite instance.

## Factor-base selection

The 2026-10-29 study jointly selected the low-x base size and exact pair oracle
on training curves, then decided each size on a fresh holdout curve. The pair
oracle stayed fixed; only (N) changed.

| Bits | Incumbent → selected $N$ | selected/incumbent at $w=0/0.1/1$ | selected/rho at $w=0/0.1/1$ | Decision |
|--:|:--|:--|:--|:--|
| 20 | 32 → 45 | 0.818 / 0.798 / 0.726 on frozen holdout; 0.836 / 0.817 / 0.746 on second host | 7.95 / 8.24 / 9.62 on second host | promote N=45 engineering baseline |
| 24 | 162 → 115 | 0.902 / 0.940 / 1.126 | 12.44 / 12.92 / 15.19 | pricing relabelling; keep N=162 |
| 28 | 407 → 288 | 0.929 / 0.974 / 1.193 | 20.25 / 20.98 / 24.33 | pricing relabelling; keep N=407 |
| 32 | 1024 → 1024 | 1 / 1 / 1 | 33.30 / 33.18 / 32.61 | null selection |

For P20, the second host admitted all 2,016 executions at L2, verified all
2,016, and matched 1,008/1,008 original records in every counted and
band-priced field. The independent validation is filed in
[`VALIDATION.json`](../prime_base_oracle_tuning_20261029/VALIDATION.json). The
leaderboard's stale “no validation exists” sentence was corrected at the
generator and generated Markdown/HTML/JSON surfaces.

The operational interpretation is narrow: increasing (N) helps the 20-bit
pipeline because the improved relation probability pays back the larger base
and table over that finite envelope. At 24 and 28 bits the decision changes
with the native-operation price, so those candidates do not dominate the
incumbent.

## Root finding: useful algorithm, wrong asymptotic label if overclaimed

`rootfind-subtract` and the m=3 root-finding arm are exact generic
table/search procedures. They can be optimized and ranked E2E, but they do not
receive the factor-base equations inside an algebraic system. They therefore
belong to the **generic decomposition track**, bounded by the square-root
frontier, rather than serving as evidence for a sub-square-root algebraic
attack. The repository's 16–28-bit measurements remain 10.1–69.7× rho in
`bitop/v1`.

The older root-finder replay audit also records an open regression: only
72/168 current-binary replays were identical on 2026-10-19. Affected historical
stage counters remain blocked from promotion. The fresh campaign measures the
current implementation from new records rather than inheriting those counters.

## Best recent prime marker and the missing algebraic baseline

The progress ledger's closest recent prime IC marker is the residual collector
at 36 bits, 2.59× `rho.negation` in cold one-target (S). It is closer than the
P20 factor-base study, but it is not a completed prime Gröbner-basis attack.

There is still no native prime-field ECBench arm that places the factor-base
constraint inside a Semaev system and then runs a complete F4/F5/root-extraction
pipeline against matched rho. Prime PKM/tower and GLV studies contain useful
component counts, but their units and systems do not form that missing E2E
baseline. This is the largest algorithmic coverage hole in the prime program.

## Standalone GPU explicit relations

The RTX PRO 6000 study completes both fixed public 62-bit fixtures, with cold
times 2,139.88 and 2,187.94 seconds in its V11 panel. Certificates replay and
corruption controls pass. It remains **unranked** here because it is a
standalone GPU implementation, its clocks are L0, and there is no same-fixture
ECBench rho arm. It establishes an operational capability boundary, not a
generic or matched-rho frontier.

# Binary and Koblitz frontiers

## Random binary

The admitted random-binary ladder remains 10.6–38.2× matched negation rho. In
the newer F4/F5/F6 cross-family diagnostic, inherited F4 is closer on the tiny
cells: 5.89× at m=13 and 2.28× at m=19. Two sizes do not establish an exponent,
and the study's (S) is a lower bound because backend-native work is unpriced.

The key result is negative but useful: Koblitz-specific structure does not
automatically improve the decomposer if the factor base is folded only by
negation. The same solver ordering and similar local ratios appear on Koblitz
and random-binary systems.

## Koblitz

The admitted cold, one-target Koblitz table covers subgroup sizes around
(2^{36.6})–(2^{47.2}). IC/rho ranges from 3.43× to 11.5×. Setup accounts
for roughly 98.7–99.6% of the cold IC pipeline on the central rows, so an
online-only view can look attractive while the primary cold object remains
rho-led.

The correct Koblitz reference uses signed Frobenius and negation. The canonical
medium-tier counted boundary is 1.089× its floor, with interval
[0.927, 1.261], across six sizes and 144 runs. Pair-claw is a useful
memory/time frontier and reached 1.85× matched rho on one earlier cell, but did
not displace the walk.

The factor-base symmetry lesson is asymmetric:

- folding a base or pair table can reduce stored representatives and setup;
- the matched rho reference must receive the same usable automorphism group;
- a symmetry that helps only a representation stage is not an ECDLP speedup;
- orbit naming, canonicalization, and retained advice must be charged to both
  arms consistently.

# The restricted GHS-weak exception

The repository's only clear IC-led elliptic-curve domain is the special
GHS-weak family $E_0/\mathbb F_{q^3}$ with
$\#E_0(\mathbb F_{q^3})=4\ell$. The comparison is a whole cold one-target
pipeline in $\mathbb F_q$-multiplication equivalents, with no subgroup
enumeration hidden outside the interval.

![The GHS-weak curve family has its own, separate frontier.](figures/ghs-weak-frontier.png){width=92%}

The B0 arm crosses below plain rho at $q=4051$, where its ratio is 0.735
[0.662, 0.816] at $w=0$, and falls to 0.122 [0.0987, 0.157] by
$q=262111$. N4-v2 adds double large primes, structured Gaussian elimination,
and block Lanczos; its DL arm reaches 0.00456× plain rho and 0.00523×
negation-map rho at $q=2{,}097{,}091$ in the recorded counted unit.

This result must travel with four restrictions:

1. the curves are intentionally GHS-weak and are not generic curves;
2. they are not generic prime-order $E/\mathbb F_{q^3}$ instances;
3. no standard cryptographic curve is in this family;
4. the result cannot be transported to another curve without an explicit,
   subgroup-preserving, costed map.

The correct synthesis is therefore not “IC never beats rho” and not “ECDLP is
broken.” It is: **IC beats rho on one special weak family; generic ECC remains
rho-led in the measured envelope.**

# F4, F5, F6, SAT, WDSAT, and FES

## F4/F5/F6 result

The cross-family experiment uses eight targets in each of K13, R13, K19, and
R19. Matrix F4/F5 at m=19 hit the retained 1,800-second timeout. At m=13:

| Cell | Matrix F4 word XORs | Matrix F5 word XORs | F5/F4 | Inherited F4 word XORs | F6 word XORs |
|:--|--:|--:|--:|--:|--:|
| K13 | 2.245e9 | 1.454e9 | 0.6465 | 1.112e7 | 9.10e6 |
| R13 | 2.325e9 | 1.495e9 | 0.6414 | 8.66e6 | 7.99e6 |
| K19 | timeout | timeout | unknown | 6.70e8 | 5.19e8 |
| R19 | timeout | timeout | unknown | 7.15e8 | 5.62e8 |

F5's signature criterion removes about a third of matrix F4's elimination
work on the same tree. Inherited F4 is roughly 199–222× cheaper than matrix F4
on the m=13 systems. F6 further reduces word work and relation wall time, but
performs exact factor-base point additions for its closure, which dominate the
common priced pipeline.

![Backend-local savings and whole-pipeline cost move in opposite directions.](figures/solver-tradeoff.png){width=100%}

The frontier rule follows directly: F5 and F6 are component leaders in their
native scopes, while inherited F4 remains the best of these four in the common
E2E lower-bound (S). None beats rho.

## SAT-CDCL

SAT-CDCL is live in the nested registry and supports CNF-expanded parity and
native-XOR parity. Existing n=17 records show why conflict counts cannot be a
universal unit: native parity reports thousands of conflicts while CNF parity
reports more than a million on related systems, with different clauses,
auxiliaries, and encodings. A conflict reduction is a solver diagnostic unless
CNF construction, XOR handling, relation yield, verification, and the whole
pipeline share a common priced boundary.

The new solver-backend spec pins encoding and budget and runs SAT beside the
same curve, factor base, target seeds, rho reference, and A/A control. Until
that L2 session completes, SAT is implemented but not a ranked E2E leader.

## WDSAT

WDSAT is a historical external path rather than a current native ECBench
method ID. XG/Gaussian-elimination mode can cut its own conflict count by tens
or hundreds of times, but “conflict” changes meaning with solver, encoding,
and XG mode. More importantly, a retained audit found a false-UNSAT result on
one of sixteen decomposable targets in the unfixed path. The historical WDSAT
evidence is therefore useful for regression and solver research, not for
promotion to the current E2E frontier.

## FES and Gray code

`fes-f2` uses scalar Gray-code enumeration; `fes-f2-wide` evaluates multiple
subcubes through AVX2/AVX-512 and is the strongest exhaustive reference where
its equation limit applies. Gray-code word XORs and vector XORs are native
units, not GAE. Past component studies show real implementation gains, but a
universal gate was not cleared on every cell. The fresh all-backend spec keeps
both implementations, their timeouts, and their native counters, while ranking
only common E2E outcomes.

## Crossbred, XL, Buchberger, and exhaustive controls

These backends remain important controls:

- Buchberger checks that matrix techniques are not winning by an incorrect
  algebraic shortcut;
- Crossbred tests hybrid specialization/elimination;
- XL provides a degree-lifted linearization baseline;
- exhaustive search supplies the correctness and small-system floor.

Their native counter definitions differ, so no report should divide one by
another and call the quotient a speedup. The common comparison starts at the
same ECDLP target and ends only after the recovered scalar verifies.

# Rho comparison: current optimized boundary

The optimized `rho.signed_frobenius_strong` path preserves the exact walk,
recovered scalar, counters, and GAE while reducing implementation instructions:

| Measurement | Result |
|:--|--:|
| Optimized / same-binary legacy Callgrind instructions | 0.802155 |
| Optimized / archived pre-change instructions | 0.804582 |
| Speedup over same-binary legacy | 1.246642× |
| Canonicalization share | 53.91% → 42.48% |
| Verified executions | 864 / 864 |
| Measured replays identical | 720 / 720 |
| GAE optimized / legacy | 1.0000 [1.0000, 1.0000] |
| Native L2 wall verdict | unknown |

The implementation win comes from paired coordinate maps, fixed common byte
shapes, and shared coefficient reduction. It does not change the mathematical
rho operations boundary. The old L0 wall ratio is not promoted.

For the new campaign:

- prime uses `rho.negation_v2` and a byte-identical A/A arm;
- Koblitz uses `rho.signed_frobenius_strong_v2`, also with A/A;
- BSGS and kangaroo are measured on prime cells to reveal finite-size leaders;
- every symmetry available to an IC arm is audited for the reference;
- setup, distinguished-point policy, table state, restarts, and verification
  stay inside the declared interval.

# Bottleneck map

| Bottleneck | Evidence now | Why it holds the frontier | Required next gate |
|:--|:--|:--|:--|
| Relation probability / trials | prime and binary IC gaps grow across several ladders | solver improvements do not change the probability of a useful decomposition | algebraic factor-base constraint inside the system; four-size E2E sweep |
| Factor-base and pair-table setup | 98.7–99.6% of central cold Koblitz rows; $O(|F|^2)$ pair materialization | online-only improvements disappear when one target pays setup | cold primary result; equal-advice online secondary result |
| F6 closure additions | 64.6–1,352.6× inherited F4 in lower-bound S | local word-XOR savings are overwhelmed by exact group work | price closure and solver in one common unit; change closure algorithm, not label |
| Matrix F4/F5 scale | m=19 matrix arms time out at 1,800 s | symbolic preprocessing and matrix construction explode before useful scaling | memory/degree receipts and retained timeout curve |
| Solver-counter mismatch | XORs, conflicts, vector steps, multiply-adds | cross-solver ratios lack semantic equality | E2E wall or complete common pricing; native counters remain diagnostics |
| Linear algebra | large stage-only speedups, small E2E changes in several pipelines | relation collection and setup dominate | measure phase share before further LA work |
| Rho reference quality | 19.8% instruction cut; toy BSGS sometimes below rho | a weak reference creates false IC wins | current accounting-v2 rho, A/A, BSGS/kangaroo finite-size controls |
| Prime algebraic coverage | no complete native prime F4/F5 factor-base-constrained arm | generic root finding cannot answer the algebraic claim | implement and validate a prime algebraic reference arm |
| Evidence portability | mixed L0/L1 historical wall, one-curve studies | host effects and curve effects are confounded | L2 second host, at least 8 targets, 5 rounds, and 4 sizes |
| GPU integration | verified 62-bit standalone result, no matched rho | capability result cannot enter the ECBench Pareto set | native adapter, same fixtures, matched reference, isolated clocks |

The dominant pattern is that improvements are arriving after the point that
currently controls total cost. Fast root extraction does not fix relation
probability; fast linear algebra does not fix pair-table setup; fewer F6 XORs
do not fix closure additions. The next useful optimization should be selected
from the measured whole-pipeline phase share, not from the largest isolated
microbenchmark speedup.

# Curve-by-curve interpretation

## Toy curves

Toy curves are valuable for exhaustive controls, replay, seed-law tests,
solver parity, and detecting fixed overhead. They do not establish a security
claim. At 16–26 prime bits, BSGS can beat rho in `bitop/v1`; at binary m=13,
fixed setup can make the “strong” parallel rho variant worse than simple
negation rho. The report therefore retains multiple generic controls.

## Medium experimental curves

The repository's most informative generic boundary comes from ladders with
enough sizes to reveal whether a fixed overhead is fading or a gap is growing.
Prime pair/rootfinding costs move away from rho through 40/28 bits. Koblitz
one-target IC remains several times rho through roughly 47 subgroup bits. The
62-bit GPU fixture is operationally larger but is not a matched frontier row.

## Special transfers and covers

Isogeny, quartic-cover, GHS-cover, three-leads genus-2, and hyperelliptic paths
must each retain the map, subgroup image, target transport, and inverse
recovery cost. A cheaper DLP on the destination object helps only when those
obligations are explicit and cheaper end to end. The GHS-weak family is a
direct restricted problem class, not a free transfer from arbitrary curves.

## Standard curves and ECC2K-130

No standard curve in the registry has a completed IC solve that changes its
security boundary. The repository's ECC2K-130 material is a feasibility and
solver program, not a demonstrated break. Claims about standard curves remain
bounded by generic algorithms and external cryptanalytic knowledge, not by
extrapolating toy ECBench fits.

# Frozen fresh campaign

The prepared campaign contains nine independent L2 jobs. Each has eight
targets, one warmup, six measured rounds with balanced arm order, replay-all,
failure retention, and the three-point pricing band.

| Spec | Domain/size | Principal candidate arms | Plan ID |
|:--|:--|:--|:--|
| `prime-p20.json` | prime 20 | rho-v2, N45/N32 pair IC, rootfind m2/m3, BSGS, kangaroo, A/A | `ECS1h3320a6f156a5` |
| `prime-p24.json` | prime 24 | rho-v2, N162/N115 pair IC, rootfind m2/m3, BSGS, kangaroo, A/A | `ECS1h8d2f84ff0bd1` |
| `prime-p28.json` | prime 28 | rho-v2, N407/N288 pair IC, rootfind m2/m3, BSGS, kangaroo, A/A | `ECS1h280a82a43148` |
| `prime-p32.json` | prime 32 | rho-v2, N1024 pair IC, rootfind m2/m3, BSGS, kangaroo, A/A | `ECS1h483e365694a1` |
| `koblitz-n13.json` | Koblitz 13 | strong/signed/negation rho, rootfind, inherited F4, F6, claw, A/A | `ECS1h5134645433fe` |
| `koblitz-n19.json` | Koblitz 19 | same | `ECS1h729b865269f9` |
| `koblitz-n23.json` | Koblitz 23 | same | `ECS1h1a916886cdf0` |
| `koblitz-n31.json` | Koblitz 31 | same; work limits/timeouts retained | `ECS1h0ac2f6395870` |
| `solver-backends-k17.json` | binary solver cell 17 | all 11 nested backends, strong rho, A/A | `ECS1h052a2276d7a5` |

All specs pass `ecbench plan --json`. They are listed in
[execution/manifest.json](execution/manifest.json). The locally built release
binary has SHA-256
`91f5895637501c6de257bbaf3afc9cbc2c0f5e964964d3dc4fc3bf78a427d96b`.

Execution is intentionally blocked. This workspace has no AWS identity, no
AWS CLI, no callable Iso Lab connector, and an invalid GitHub token. The only
available pinned EKS image contains an older source revision, so using it would
not measure this report's commit. The exact blocker is in
[execution/BLOCKER.md](execution/BLOCKER.md). Local wall timing is not silently
substituted.

Before the first warmup, the executor must:

1. build the exact commit into the trusted static runtime image;
2. bind the immutable ECR digest and binary SHA-256 in the protocol;
3. validate and commit the frozen identity amendment;
4. run each size as a fresh job, without CRIU-restored timed rows;
5. upload raw records, host capsule, replay receipt, A/A analysis, and hashes;
6. retain every timeout, failure, and unattempted cell;
7. regenerate the Pareto tables and this report from the new records.

# Conclusions and research priorities

The performance frontier is now explicit:

- **Generic ECC:** rho/BSGS/kangaroo remain the leaders; no generic IC crossing
  is admitted.
- **Restricted GHS-weak ECC:** plane-quartic IC is the leader and must remain
  labelled as a special-family result.
- **Prime engineering:** P20 $N=45$ is the validated factor-base baseline,
  still roughly 8–10× rho.
- **Algebraic solvers:** F5 and F6 win component metrics; inherited F4 wins the
  tested common E2E lower-bound comparison; rho wins the ECDLP comparison.
- **SAT/WDSAT/FES:** implemented or historically explored, but not yet on one
  common, validated E2E frontier; WDSAT also carries a retained correctness
  failure in an old path.
- **Root finding:** valuable generic decomposition machinery, not evidence for
  an algebraic sub-square-root attack.
- **Rho:** instruction-level optimization is real and verified; isolated native
  wall promotion is still open.

The highest-value next work is therefore:

1. execute the already frozen L2 campaign on the exact source;
2. implement the missing prime factor-base-constrained algebraic arm;
3. attack relation yield and cold setup before further optimizing stages that
   are already a small share;
4. integrate the GPU relation engine with a matched ECBench reference;
5. preserve the GHS-weak win as a separate domain rather than using it to
   headline generic curves.

# Reproduction and evidence index

The report is generated only from committed data and prior frozen evidence.
Presentation data are in [frontier_snapshot.json](frontier_snapshot.json), and
[`render_figures.py`](render_figures.py) renders all four figures. Important
sources are:

- [canonical IC leaderboard](../../docs/ic/LEADERBOARD.md);
- [ECBench contract](../../docs/ecbench/README.md);
- [backend-frontier contract](../../docs/ecbench/BACKEND_FRONTIER.md);
- [F4/F5/F6 result](../f4_f5_f6_cross_family_20261011/RESULT.md);
- [rho optimization](../ecbench_rho_optimized_20261014/README.md);
- [prime factor-base second-host replay](../prime_base_oracle_tuning_20261029/second-host/README.md);
- [GPU explicit-relations result](../explicit_relations_e2e_20261030/RESULTS.md);
- [literature and prior-evidence ledger](LITERATURE.md);
- [live implementation coverage](COVERAGE.md);
- [campaign protocol](PROTOCOL.md).

Primary mathematical references used to set the boundary include Shoup's
generic-group lower bound (EUROCRYPT 1997,
[DOI 10.1007/3-540-69053-0_18](https://doi.org/10.1007/3-540-69053-0_18)),
Faugère's F4
([DOI 10.1016/S0022-4049(99)00005-5](https://doi.org/10.1016/S0022-4049(99)00005-5))
and F5
([DOI 10.1145/780506.780516](https://doi.org/10.1145/780506.780516)),
Semaev's summation-polynomial work
([IACR ePrint 2004/031](https://eprint.iacr.org/2004/031)), and the
Faugère–Perret–Petit–Renault binary-field IC analysis
([DOI 10.1007/978-3-642-29011-4_4](https://doi.org/10.1007/978-3-642-29011-4_4)).

No sentence in this report converts a missing measurement into a loss, a local
counter into an E2E speedup, a special-family result into a generic break, or a
draft EKS plan into fresh benchmark data.
