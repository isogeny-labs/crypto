# Prime31 weighted M6: exact uniform-query throughput ceiling and direction change

**Date:** 2026-11-18 UTC. **PR:** [#4722](https://github.com/isogeny-labs/crypto/pull/4722). **Scope:** exact combinatorial theorem plus original-curve Rust support-control implementation; not a new nonenumerative solver and not a newly timed complete ECDLP run. Producer-side evidence `[pcc:unverified]`.

## Fixed original-curve instance and evidence provenance

- Original ordinary cofactor-one curve: `p=2147483647`, `a=847514846`, `b=2087444724`, certified prime group order `n=2147446831`, generator `(625037715,2100627251)`.
- Reuse the **existing native Rust** `prime31_algebraic_ic::{Arithmetic,check_curve,PRIME31}`. Exactly the first 31 rational x-coordinate points with canonical positive ordinate, and the established weighted positive slots `w=(1,3,5,7,11,13)`.
- Archive to compare, **not measurements made by this PR**: [#4714 any-witness M6](PRIME31_M6_ANY_WITNESS_RESULT_20261118.md) recorded 31 independent rows after 87 training queries and 1,024 verified public logarithms after 3,143 online relation queries (full runtime 4,274.8/4,286.1 ms on its own CI runner). The prior parity-M6 same-runner [frontier comparison](https://github.com/isogeny-labs/crypto/pull/4714) lost to optimized shared BSGS by 20.85–23.32x. Those **different CI-run absolute times must not be divided** to claim a new paired ratio.
- Related active research: [#4692 RR9/F4 public-residual extraction](https://github.com/isogeny-labs/crypto/pull/4692); [#4717 exact radical/multiplicity support strata](https://github.com/isogeny-labs/crypto/pull/4717); [#4720 table-free signed-pair algebraic projector](https://github.com/isogeny-labs/crypto/pull/4720); [#4698 parity-network asymptotic claim](https://github.com/isogeny-labs/crypto/pull/4698); [earlier M4 arity/coverage census](M4_NATURAL_YIELD_ARITY_GATE_20261115.md); [y-ranked original-curve negative control](https://github.com/isogeny-labs/crypto/pull/4652). None should be duplicated as a fresh invention.

## Exact theorem: six fixed weighted slots have a 0.4133 uniform hit ceiling

Let `F={P_1,...,P_31}` be the original positive base. For arbitrary weights `w_1,...,w_6` (including the fixed six), define the multiplicity function on the actual elliptic-curve group `G=E(F_p)`:

```
X(R) = #{(i_1,...,i_6) in {1,...,31}^6 :
          w_1 P_{i_1} + ... + w_6 P_{i_6} = R }.
```

Every ordered word has precisely **one** group image, so the counting identity

```
sum_{R in G} X(R) = 31^6 = 887503681
```

is exact, **regardless of scalar weights, point collisions or sign cancellations**. If a fresh residual `R` is uniformly distributed in this group, then

```
E[X(R)] = 887503681 / 2147446831 = 0.4132831920158493...
Pr[X(R) > 0] <= E[X(R)] = 0.4132831920158493...
```

The inequality is Markov's inequality for a nonnegative integer. It concerns **membership in the same six-slot support** and is independent of how efficiently membership is computed. A smarter extractor cannot raise this uniform hit probability above the capacity of its mathematical support without changing the support, factor base or query distribution.

If residual trials are *independent uniform draws* and each success gives at most one usable relation/target answer, the expected waiting work is at least `k/0.413283...` queries for `k` successes. The minimum integer fixed trial budget for its *expected number* of hits to reach `k` is `ceil(k*n/31^6)`. This does **not** imply a deterministic lower bound for a particular seeded cohort, nor does it constrain adaptive target-conditioned queries or guarantee linear independence of accepted relations.

### Original Prime31 diagnostic panel

| Quantity | Exact result / historically recorded count | Proper interpretation |
|---|---:|---|
| Ordered six-slot words | 887,503,681 | Exact potential words, before group collisions |
| Uniform residual hit probability | at most **41.3283192%** | Rigorous for the same fixed six-slot support |
| Weighted slots 1,3,5: distinct three-sums | **29,791 / 29,791** | Complete generic diagnostic census, no collisions |
| Weighted slots 7,11,13: distinct three-sums | **29,791 / 29,791** | Same |
| Idealized uniform trials for 31 *expected* hits | **75.0091** (integer budget ≥76) | No independence/rank guarantee |
| Archived M6 training | ≥31 independent rows / 87 trials | Empirical rank-derived minimum positive fraction ≥35.63% |
| Idealized uniform trials for 1,024 *expected* hits | **2,477.7199** (integer budget ≥2,478) | Hypothetical iid sampler only |
| Archived M6 online | 1,024 solved / 3,143 queries | Empirical 32.58% target-query success |
| Ceiling / archived minimum training yield | **1.159859×** | Throughput headroom *diagnostic*, not a measured speedup bound |
| Ceiling / archived online yield | **1.268505×** | Throughput headroom *diagnostic*, not a measured speedup bound |

The two 29,791-support counts are a **new exact curve-specific calculation**, made with the existing Rust group operations and independently reproduced in an integer-field Python checker. The full 31^6 sum image was **not** enumerated and may have cross-half collisions. The Rust census is a correctness/collision diagnostic only; it explicitly performs generic 3-sum additions and must never be substituted for the required algebraic extractor.

**Scientific interpretation:** With the current B31/6-weight architecture, pure positive-hit-rate tuning has limited room if the archived trials approximate fresh uniform draws. A further constant-factor hash, first-witness or inversion improvement can still be worthwhile engineering, but the large measured gap to the fastest shared BSGS will require much cheaper per-query arithmetic or a genuinely new relation-extraction mechanism. This is an engineering prioritization, **not** an impossibility proof against other prime-field index-calculus algorithms.

## Reproducibility and falsifiers

The new [Rust gate](examples/prime31_m6_yield_ceiling.rs) calls the pinned original Rust certificate, reconstructs the unchanged 31-point base, computes both weighted three-slot supports, asserts the exact word count and performs all fraction comparisons with `u128` cross multiplication. The independent [Python verifier](verify_prime31_m6_yield_ceiling.py) reimplements finite-field EC addition, scalar multiplication, canonical basis selection and both 29,791-sum support sets; it compares Rust output if supplied. It does not use any precomputed point-sum table as an ECDLP oracle.

Reproduce:

```sh
cargo +1.100.0 test --release --locked --manifest-path research/prime31_ic_20261115/native/Cargo.toml --lib
cargo +1.100.0 test --release --locked --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m6_yield_ceiling
cargo +1.100.0 run  --release --locked --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m6_yield_ceiling > prime31-m6-yield.json
python3 research/prime31_ic_20261115/native/verify_prime31_m6_yield_ceiling.py prime31-m6-yield.json
```

No Rust compiler or full repo checkout was present in the editing runtime. **The source-specific GitHub Actions run [#44330826755](https://github.com/isogeny-labs/crypto/actions/runs/44330826755) PASSED** at pinned original Rust source `1df714645aa683cf975bc76fcacb9842cb4780fd` (PR candidate head `fb4bb0dd5682753278caebfb1a86800d5c1184bd`, Rust 1.100.0), including **40/40 original Rust library tests, 2/2 new exact ceiling tests, source-produced JSON**, and separately implemented Python BigInt original-curve replay of both 29,791-sum supports, three point-order controls, exact fractions and expected-call budgets. Raw [JSON artifact #14439481917](https://github.com/isogeny-labs/crypto/actions/runs/44330826755/artifacts/14439481917), GitHub-provided SHA256 ZIP digest `e03737964eeb396d22affdfd654ed814589d62ca0b6ddc5cf54d07bc91ade683` (not independently rehashed). This is **producer-side reproducibility**, not an independently signed scientific PCC; signed PCC and any non-generic solver speed comparison remain pending.

## Priority queue: one changed mechanism per experiment

1. **Nonenumerative public-Q RR9/F4, B4/B8:** Use #4692 and #4717, with exact positive/negative/vertical and multiplicity controls. The independent #4720 polynomial pair screen may serve only as a final pair leaf. Discovery must not build 3+3, 4+4 or a generic two-sum enumeration table in disguise; assert forbidden operation counters zero. A timeout is undetermined, not a negative relation.
2. **B31 naturally discovered algebraic relations:** Feed only witnessed, signed, never pre-seeded public residuals into the unchanged Rust rank-31 and 1,024-log pipeline. Charge all failed queries, F4/quotient/radical preprocessing, exceptional branches, linear algebra and verification. Source and target logs must stay outside the relation oracle.
3. **Cost-per-query gate rather than more M6 yield tuning:** For the existing M6 control, benchmark 64/256/1024/4096 batch sizes and exact positive/negative early-exit counts on one frozen corpus, distinguishing an engineering speedup from a non-generic attack. Compare to the actual strongest cached/cold shared BSGS and optimized negation-map rho on identical hardware and 1,024 public targets; include the mixed 4+4 IC reference.
4. **Asymptotic audit lane (separate):** Review #4698 against the October 2026 Bochum algorithm and [Anthropic's graded crossing constructions](https://github.com/anthropics/formal-math), particularly explicit factor access and practical recursion thresholds. For b=31, a subexponential asymptotic bound by itself is no computational speed result.
5. **Structural generalization:** For any measured candidate that beats the frontier here, repeat on independently generated ordinary, prime-order curves; avoid selecting specially weak curves or conflating amortized batches with a single cold log.

## Prior-art / accessible-source boundary

Primary context: [Petit–Kosters–Messeng (PKC 2016)](https://doi.org/10.1007/978-3-662-49387-8_1); [Amadori–Pintore–Sala (2018)](https://doi.org/10.1016/j.ffa.2018.01.009); [Diem's index-calculus work](https://arxiv.org/abs/math/0606607); [Bochum 2026 preprint](https://github.com/bochum-crypto/subexponential-ecdl). These provide index-calculus methods and distinct asymptotic settings, **not** a new empirical Prime31 speed claim. The research-KB MCP/CLI, Conductor and EKS control were not available; repository PRs and file sources and public paper descriptions are the bounded fallback. No exhaustive novelty claim or independent PCC status is inferred.
