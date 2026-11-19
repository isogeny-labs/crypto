# Prime31 W8 full-fibre multiplicity experiment — source-verified result (2026-11-19)

**Scope:** finite-group generic grading control only. This is not an efficient public-only RR9/F4 index-calculus extractor, and it is not a cheaper complete prime-field ECDLP solver. Source-bound tests passed; independent signed PCC validation remains outstanding.

## Frozen context and source

- Original **ordinary, prime-order, cofactor-one** curve over F_(2^31-1):
  p=2147483647; E: y²=x³+847514846x+2087444724;
  #E(F_p)=n=2147446831 prime; G=(625037715,2100627251).
- Original 31 rational-x point factor base and overlapping four 8-point windows from #4728/#4807; no deliberately weak replacement curve.
- [Protocol committed before results](W8_FULL_FIBRE_PROTOCOL_20261119.md), source first frozen as [commit 50028ed](https://github.com/isogeny-labs/crypto/commit/50028eda452cd198d48c268d2a35f44e96d39627); wholly new held-out SplitMix64 seed 0x5748385f48455432, exactly 256 public Q points per window. Scalar k used to construct Q=[k]G **never enters the grader**.
- Code reuses pinned original \`prime31-algebraic-ic\` Rust at 1df714645aa683cf975bc76fcacb9842cb4780fd (the new example is overlaid; arithmetic not reimplemented).
- [Source-specific GitHub Actions run #44533134056](https://github.com/isogeny-labs/crypto/actions/runs/44533134056), source head c099bbb821472986805ebb427c2987a9284cfca1: **PASS**. Original library 40/40 tests; focused 3/3; 1,024 public target checks for *each of four arms*; 16/16 independent Python affine signed-witness certificates; independently recomputed all four 1,024-entry histograms, witness-square totals, and 4×4 paired discordance matrix. [Raw ZIP artifact #14534683345](https://github.com/isogeny-labs/crypto/actions/runs/44533134056/artifacts/14534683345), ZIP SHA256 \`e7288b76fa24754995b7dbf0bfdf9f6121beefd1c0e82af3d11a9002c9ff92fa\`.
- Raw \`w8-full-fibre.json\` SHA256: \`5725afe43a35d6150a61748329a11a790de59cbf655b92d773327ad4b0af43f0\`; independent replay JSON \`f69050798b8960c0b316c12c3d8dcd6ef48885d5b0bc00cf7fb81b84936e78e6\`.

## Exact enumerative theorem behind the measurement

For an eight-point window and eight fixed positive integer weights w_j, each slot independently selects an index i_j in {0,...,7} and a sign eps_j∈{±1}. Let the first four-slot multiset be L and the last four-slot multiset H, with signed EC group-point count functions l(P), h(P) (including every word multiplicity). For public target R, the **complete** eight-slot witness count is exactly

\[
m(R)=\sum_{P\in E(\mathbb F_p)} l(P)\,h(R-P).
\]

The Rust program actually computes the sum over the sorted support of L with an EC group lookup for each complement, and independently checks a signed witness by original EC addition whenever \`m(R)>0\`. Since each half contains exactly 16^4=65,536 words, summing m(R) over *all* n group elements yields exactly 16^8=4,294,967,296, **for every weight schedule**, hence the exact uniform-target population mean is \`2.0000342890910905\`. This population identity does NOT mean all schedules have the same hit probability, variance or extraction cost. Concentration of witness multiplicity drives misses.

This is a **generic four-plus-four finite-group grader**, not a relation oracle admissible for a non-generic index-calculus speed claim.

## Frozen four-arm results on the new 1,024 public targets

| Schedule | Weights | Hits / 1,024 | Total signed ordered witnesses | Mean m(R) | Sample-panel second central moment (1/N) | Largest m(R) |
|---|---|---:|---:|---:|---:|---:|
| Historical OLD | 1,3,5,7,11,13,17,19 | **795 (77.64%)** | 2,116 | 2.0664 | **4.8140** | **28** |
| POW2 | 1,2,4,8,16,32,64,128 | **877 (85.64%)** | 2,131 | 2.0811 | 2.2307 | 8 |
| NEAR127 | 1,2,4,8,16,32,64,127 | **873 (85.25%)** | 2,018 | 1.9707 | 2.0363 | 8 |
| TERNARY | 1,3,9,27,81,243,729,2187 | **892 (87.11%)** | 2,056 | 2.0078 | **1.9550** | 7 |

Every outcome above is a full **exact count** of ordered signed eight-slot witnesses per public R, not a first-hit probe. All four arms used the *same* 1,024 public targets, with deterministic arm order rotated across windows. The 1/N variance is descriptive (not an unbiased, cross-curve estimate).

Window-by-window hits:

| Window | OLD | POW2 | NEAR127 | TERNARY |
|---|---:|---:|---:|---:|
| W0 (indices 0–7) | 200 | 216 | 216 | 224 |
| W1 (8–15) | 196 | 225 | 216 | 223 |
| W2 (16–23) | 200 | 220 | 222 | 223 |
| W3 (23–30) | 199 | 216 | 219 | 222 |
| **Total** | **795** | **877** | **873** | **892** |

Exact pooled histogram bins m(R) = 0, 1, 2, 3, 4–7, 8–15, 16+:

| OLD | POW2 | NEAR127 | TERNARY |
|---|---|---|---|
| 229 / 268 / 223 / 117 / 167 / 18 / 2 | 147 / 260 / 253 / 187 / 176 / 1 / 0 | 151 / 266 / 289 / 178 / 138 / 2 / 0 | 132 / 292 / 259 / 184 / 157 / 0 / 0 |

**Interpretation:** OLD is overdispersed, with large 16+ witness tails; improving its four-slot formal uniqueness is associated with more uniformly distributed witnesses and better target coverage. The total **mean witness counts** are all close to the exact population mean ≈2, but the hit probabilities are different. This is direct empirical support for multiplicity concentration as an explanatory mechanism; it does not isolate which algebraic collisions cause it or prove distributional behavior on another curve.

## Primary falsification and paired comparisons

- **NEAR127 did not improve on POW2:** 873 vs 877 hits, **−4/1024 (−0.39 percentage points)**. Discordants: 128 targets were NEAR127-only and 132 were POW2-only. Descriptive exact paired binomial/McNemar p≈0.852. In W1, NEAR127 has 65,496 left unique four-slot sums and 65,536 right, versus POW2's 65,496 and 65,496. Thus eliminating the 40 right-half collisions does **not** improve coverage at this sample size. Treat the simple “break scale-16 symmetry” hypothesis as a negative lead for yield, not as an algebraic-extractor obstruction.
- **POW2 improves on OLD on a second deterministic held-out panel:** 877 vs 795, +82/1024 (+8.01pp), discordants 194 POW2-only and 112 OLD-only; paired descriptive exact p≈3.21×10⁻⁶. Earlier separate panel (#4807) was 902 vs 772 (+130/1024). Different draw, same curve and fixed factor bases; not evidence of across-curve scaling.
- **TERNARY has highest observed hit yield**: 892 vs POW2 877 (+15/1024, +1.46pp; 125 TERNARY-only, 110 POW2-only; descriptive p≈0.361), and 892 vs OLD 795 (+97/1024, +9.47pp; descriptive p≈3.32×10⁻⁸). These are not controlled complete-solver timings. TERNARY was already established as a **generic grader** 1.2617× cheaper than OLD in #4770, not as a non-generic attack.

The Poisson(λ=16^8/n≈2.000034) independent-word heuristic predicts hit probability ≈86.47%, approximately consistent with POW2/NEAR127/TERNARY but not OLD. It is *not* a theorem about these structured, overlapping weighted sums.

## Measurement accounting and limits

The 4-arm full-fibre command completed in approximately 53.45 seconds on the source-bound GitHub runner (including four windows, all four arms, building half maps and testing all 4,096 arm-target combinations). All probes are full scans: **no positive-target early exit**. Per-window full-probe durations range approximately 2.93–3.30 seconds for each schedule (see raw JSON); these durations measure generic-grader enumeration and cannot be compared to the 170.113ms/1024 cached BSGS historical source from a different run. Counters include original EC additions and field operations in the grader and witness replay, but not Rust hashing/allocation costs in arithmetic units. These are not F4 extraction timings, not rank-31 relation generation, not complete ECDLP recovery, and no research leaderboard promotion is warranted.

## Checked earlier leads and literature

- #4807: original binary-hit held-out and exact S+[16]S collision result.
- #4805, #4722: formal full-/half-weight signature analysis.
- #4770: complete generic-grader ternary W8 rank-31/1024 pipeline on this original curve (not non-generic).
- #4728: original four-window rank31/1024 complete generic-table pipeline.
- #4692: current nonenumerative shared-coefficient RR9/F4 extractor owner; **do not implement a competing lane**.
- #4803: unweighted B8 formal support obstruction; #4779: full-rank exterior compression gate; #4801: all-sign interpolation rank obstruction.
- #4793: original-curve full-solver cost threshold; #4532: historical ~170.113ms/1,024 shared cached BSGS on different runner.
- Negative SA-04/SA-18 determinant/norm leads in \`crypto-autoresearcher\` previously recorded; no claim this covers every active PR.
- Published context: Petit–Kosters–Messeng, PKC 2016 (DOI 10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala, Finite Fields and Their Applications 51 (2018) (DOI 10.1016/j.ffa.2018.01.009); C. Diem, *On the discrete logarithm problem in elliptic curves*, prime-field relation-finding barrier. Semaev 2004 summation-polynomial framework. **No originality claim** for the general sumset convolution identity, generic MITM, or standard collision statistics. Research KB MCP/wrapper unavailable in this runtime; GitHub sources and published material reviewed instead.

## Next three tests (concrete gates, in order)

1. **Build a public-only nonenumerative algebraic extractor in the existing #4692 lane.** For a 4+4 signed-weight W8 target, compile a four-slot weighted RR9 membership constraint, then couple two halves through an x-only four-doubling chain and a final Semaev f3 constraint. Require indexed four-slot supports to be genuine liftable EC points; an x-only root may correspond to a point over F_(p²), not an F_p factor-base witness. For power-of-two, U+[16]V=R; use equations D(x_i)x_(i+1)=N(x_i), i=0,1,2,3, with D(x)=4(x³+ax+b), N(x)=x⁴−2ax²−8bx+a². On this prime-order odd group D has no zero at any F_p x (no rational 2-torsion), so denominator-clearing introduces no *F_p*-rational denominator-zero branch. For a general x-only auxiliary variable the signed x-lift check remains essential. The final x(U),x([16]V),x(R) relation is f3=0; replay signs, and reject any roots not in the actual factor base. **This is a proposed architecture, not yet a benchmarked F4 speedup**. The standard duplication rational identity was separately checked in Python on 128 known source-curve points × four doublings (512 equality checks); not a Lean proof or CI-signed benchmark.
2. **Controlled B8 extractor gate:** run frozen 46-positive/18-negative public-target corpus, compare explicit four-chain vs expanded degree-256 form, measured F4 degree, memory, multiply-adds, wall clock, censored timeouts, certificate correctness, and **zero** generic half-sum/hash-table accesses. Compare OLD/POW2/TERNARY as separate algebraic systems. Failure to extract witnesses at low cost closes the weight-only route regardless of generic hit yield. Only #4692 should own this implementation to avoid duplicate efforts.
3. **Complete solver gate:** integrate qualifying extractor into original Rust rank-31, 1,024-target pipeline; record full cold and amortized costs, rejected queries, LA, logarithm recovery and all scalar verifications, and rerun fastest cached BSGS, best matched IC and optimized rho **same binary/runner**. The 170.113ms historical BSGS batch is a sensitivity reference, not a same-runner bound or a non-generic solver win.

**Scientific standing:** source-bound component checks passed; measured W8 multiplicity findings are meaningful but do not establish an improved whole-solver non-generic Prime31 ECDLP result. Pending: independent signed PCC and algebraic extractor with complete cost measurement.
