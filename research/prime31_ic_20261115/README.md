# Complete prime-field index-calculus baseline at p = 2^31 - 1 (2026-11-15)

**Standing [pcc:unverified]: producer-local**, not PCC signed or independently rerun on CI. This is an index-calculus **negative speed** result, not a faster-than-rho attack.

## Saved-data audit

The [native C++ audit](audit_recorded.cpp) checks the recorded public answers,
curve-order certificate and summary arithmetic. See the [scope report](AUDIT.md),
[PDF](AUDIT.pdf) and [raw audit output](audit_recorded.results.json).
Public-answer claim `PCC-20261115-63ae2a` pins the three distinct target
certificates; it remains [pcc:unverified]. This does not replay the solver,
reconstruct its relation matrices or validate its performance counters.

## Non-weak curve

The earlier smooth-order Mersenne-field curve is excluded. Here the ordinary curve is

```
p=2147483647; E:y^2=x^3+847514846*x+2087444724
G=(625037715,2100627251)
#E(F_p)=n=2147446831 (prime, cofactor 1)
```

The independent standard-library order certificate uses nonzero discriminant, prime n and p (trial division), G on E, [n]G=O, and 2n exceeding Hasse's upper bound. Therefore #E=n. It excludes j=0/1728 and embedding degree <=1000. This is a random-search ordinary curve, not a deliberately weak-order family, but 31 bits is not cryptographically strong.

## Complete algorithm

For each factor-base size B, construct the first B liftable x-coordinate curve points (with nonzero y). Build a signed two-point sum hash table from them, then sample *natural* known-scalars a, form R=[a]G and find a signed triple of factor-base points adding to R. Each positive gives a sparse linear equation for unknown factor-base logs modulo the prime n; failed calls are counted. Incremental elimination collects B independent equations, back-substitutes, and verifies every factor-base log by scalar multiplication. For public target Q with unknown d, search Q+tG until it has a triple decomposition; output d = (sum of three signed base logs) - t mod n, and explicitly verify [d]G=Q.

This is a Semaev-compatible **three-summand, x-restricted factor-base index-calculus pipeline implemented with group-arithmetic pair-table meet-in-the-middle**, not a new polynomial Gröbner oracle or a non-generic asymptotic speedup. A complete target solve follows after full relation rank; bounded relation-generation calls are explicitly censored if rank is not reached. No hidden scalar is used by the index-calculus solver.

## Cold measurements: three target seeds 20261114–20261116 for each B

| Solver | Factor-base size | Mean EC additions | Mean wall time |
|---|---:|---:|---:|
| BSGS, same EC arithmetic | - | 60,524 | 10–17 ms |
| 16-partition Floyd rho, same EC arithmetic | - | 183,586 | 7–27 ms |
| 3-summand index calculus | 384 | 20,477,630 | 4.128 s |
| 3-summand index calculus | 640 | 11,483,962 | 4.048 s |
| 3-summand index calculus | 1024 | 9,170,260 | 4.043 s |

All **nine** runs achieved full rank and correctly recovered/verified their targets on E(F_p). At B=1024 the ratio of mean group additions is 9,170,260 / 60,524 = **151.51x worse than BSGS** and 9,170,260 / 183,586 = **49.95x worse than basic rho**. The mean of per-target ratios differs (154.336x and 67.141x). A B=1024 run used ~90,240 KiB peak RSS. The rho control is **not** optimized negation-map rho, making the index-calculus deficit conservative. Measured operations include factor-base generation, signed-pair table, failed relation calls, linear algebra, verification of *every* base log, individual target recovery, and final verification. Curve selection and fixture input generation are shared public setup not counted. The initial B=96 / 10k-call pilot was censored at rank 6.

The source locally tested with GCC 14.2.0, `-O3 -std=c++17 -DNDEBUG`, SHA-256 **ecbacee0bab979e6106656d6ec51f4dbc01159c2723e9504d27d0983335f95d8**; raw nine-run JSONL SHA-256 **972fc034744449464a91c7cc47bf98de84082b91f5e561ab9001e23015c09106**. Separate Python arithmetic verified each recorded public target [d]G=Q. **The locally benchmarked C++ source, curve certificate, and raw nine-run JSONL have now been committed byte-for-byte to this branch**, confirmed against the original Git blob hashes (`ce3a7d5ef2da568606fd1147ca55a9b455584f55`, `6a2120f13e47e28ffd718ba7518a585c2bdeb3ee`, and `e7c9ceeefc8db7ec1aa5257f496ee03ee65bc902` respectively). `RESULTS.json` is also checked in. Treat the table as producer-local evidence pending exact-head CI and independent PCC replay. Never promote as validated leaderboard evidence.

## Mechanism and next experiments

For this restricted two-sum cache, construction requires O(B²) group operations and a trial O(B) checks. Under uniform signed-triple mixing, success is O(min(1,B³/n)); optimistic B-independent-relation collection therefore costs O(B²+n/B), with optimal exponent 2/3 rather than 1/2. This is *not* a lower bound on all index calculus: a genuine algebraic decomposition oracle must change the relation-cost scaling.

1. **Top priority:** sparse PKM m=8 on this same ordinary curve, using B=8,16,32 (log_B p roughly 10.3,7.75,6.2); extract natural nonplanted algebraic witnesses and their independent row-rank increments, including failures, memory, F4 and sign-lifting. The earlier m=2/3 tower data cannot establish m=8 behavior.
2. **Semaev S4:** substitute a certified algebraic rootfinder for the pair-table oracle on byte-identical point inputs; compare whole relation-finding cost, positive/negative incidence and final scalar recovery. Do not claim improvement from solving degree alone.
3. **Large-prime hypergraph:** permit one/two points outside the base, measure cycles, 2-core and independent relations, account for filtering and memory.
4. **Auxiliary covers and target-adaptive factor bases:** require explicit scalar-blind public maps, complete chain costs, and independent recovery on ordinary prime-order curves.

For PKM, simultaneously fit complete call exponent gamma and useful independent-output exponent delta. The optimistic necessary gate is gamma < 3+delta/2; beating an optimized rho on complete cold solving costs is still mandatory.

## Prior work

Internal: [#4077](https://github.com/isogeny-labs/crypto/pull/4077) (frozen protocol), [#4091](https://github.com/isogeny-labs/crypto/pull/4091) (tower), [#3877](https://github.com/isogeny-labs/crypto/pull/3877) (covers), [#4188](https://github.com/isogeny-labs/crypto/pull/4188). Literature: Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Semaev summation polynomials; Shoup, EUROCRYPT 1997 DOI 10.1007/3-540-69053-0_18. The research-KB, full checkout, Conductor, EKS Spot and independent PCC signer were not available in this runtime; no novelty or cryptographic-size claim. No new compute infrastructure, merge, or leaderboard promotion was performed.
