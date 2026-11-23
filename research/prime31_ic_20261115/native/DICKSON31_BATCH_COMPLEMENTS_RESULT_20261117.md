# Prime31 Dickson31 batch-complement relation control — measured result

**2026-11-17 | Rust 1.100.0 GitHub-hosted producer result [pcc:unverified].** PR #4618 stacked on #4615, source control in `examples/prime31_s9_b31_half_sum_oracle.rs`. Exact frozen protocol: [DICKSON31_BATCH_COMPLEMENTS_PROTOCOL_20261117.md](DICKSON31_BATCH_COMPLEMENTS_PROTOCOL_20261117.md). Durable source-specific data: [DICKSON31_BATCH_COMPLEMENTS_RAW_20261117.json](DICKSON31_BATCH_COMPLEMENTS_RAW_20261117.json).

## Result: a faster *existing* IC relation-control kernel (not a new decomposition algorithm)

On the real ordinary prime-order curve `y²=x³+847514846x+2087444724` over `Fp=2147483647`, `n=2147446831`, `G=(625037715,2100627251)`, keep the **identical Dickson depth-six B31** signed four-sum index from #4615 (618,885 entries). The same 96 fresh public Q plus two historical a=0 miss controls, using public shifts a=0..8, give **109 exact membership attempts: 98 positive and 11 exhaustive negative**, unchanged by the optimization. First successful shift counts: a=0:88, a=1:9 (includes two legacy controls), a=2:1; all 98 signed eight-leaf witness tokens and all 18,049,427 original-ordered probe counts match exactly between legacy and optimized arms. Every positive witness is replayed using original Rust curve addition. Native 40/40 library and 1/1 focused example tests passed on the pinned source in each run.

### Exact-source observations (sequential A then B on a GitHub-hosted runner, not a randomized cross-host benchmark)

| Workflow run | Legacy attempt time | Batch attempt time | Observed kernel ratio |
|---|---:|---:|---:|
| [44179136645](https://github.com/isogeny-labs/crypto/actions/runs/44179136645), source `bec8c11` | 5,944.999 ms | 1,337.701 ms | **4.44×** |
| [44179154585](https://github.com/isogeny-labs/crypto/actions/runs/44179154585), source `bec8c11` | 5,824.300 ms | 1,271.526 ms | **4.58×** |
| [44179525293](https://github.com/isogeny-labs/crypto/actions/runs/44179525293), source `f062125` (extra accounting) | 6,916.734 ms | 1,070.154 ms | **6.46×** |

The finalized source also reports **351.360 ms** for curve validation + real Dickson factor-base construction + 4A index construction, of which the index itself took **348.800 ms** (678,993 native additions). Common public residual generation for 109 attempts took **0.0318 ms** as instrumented. With these **measured common costs**, the finalized source reports **7,268.126 ms** for original setup + residuals + original oracle and **1,421.546 ms** for the new oracle (observed **5.11×** on that same run). This is still *not* a fully charged ECDLP solver: neither modular rank collection nor unknown-scalar recovery was performed here. Target construction and outer test overhead are also not included in those phase sums.

Operation counts (identical across all three source-specific runs) independently explain the change:

| Full 109-query workload | Baseline affine | Montgomery window-512 affine |
|---|---:|---:|
| Field inversions (including native replay) | 18,049,990 | **35,979** (=35,307 batched + 672 native) |
| Field multiplications (including inversion exponentiation, native replay) | 1,155,199,372 | **110,651,079** (=110,608,059 custom + 43,020 native) |
| Four-sum complement candidates processed | 18,049,427 before first hit/neg exhaust | 18,075,831 including all final-window extra work |

Thus inversions fell by approximately **501.7×**, multiplications by **10.44×**, despite 26,404 extra lanes computed in successful terminal windows. The 512-way windows use prefix products, one inverse per nonempty window, and reverse unwinding; each affine result computes `R + (-L)`. Identical-x inputs fall back to the original native group law, identities use direct negation/return. In natural 109 attempts equal-x fallbacks were zero; separate focused controls test equal-x and identity branches explicitly. The new method is **Montgomery's classical batch inversion**, *not* an original cryptanalytic technique.

### Validation and limits

- Exact source/test pinned to original Rust native arithmetic `1df714645aa683cf975bc76fcacb9842cb4780fd`, original indexed code from #4615; Rust 1.100.0, GitHub-hosted Ubuntu 24.04. Original library 40/40 and one added matched test 1/1. Source-specific final run **passed** [#44179525293](https://github.com/isogeny-labs/crypto/actions/runs/44179525293).
- [Final raw artifact](https://github.com/isogeny-labs/crypto/actions/runs/44179525293/artifacts/14370304612) GitHub metadata digest `sha256:5a0a07ef6129eeb2b2c2b5c479bf264c37a7b9d9f36652c5e49808eaf6ce1b19` (not separately downloaded/rehashed); all JSON fields and source/run provenance retained in durable raw JSON. Two earlier artifacts: #14371955713 and #14370533953.
- Sequential order is not randomized: wall numbers vary across hosted executions; ratio is a diagnostic, not a portable calibrated speedup. `HashMap` capacity 917,504 entries is reported, **not** RSS or peak bytes. The candidate reuses the same generic 4A hash index and retains its memory cost.
- This demonstrates no sign-free algebraic extraction, no new relation rank, no B31 unknown-logarithm recovery, no measured full 1,024-Q solver advantage, no novel non-generic exponent. All scientific measurements remain producer-only [pcc:unverified], not signed PCC.

## Audited leads and next independent solver experiments

**P0 — Non-generic extractor:** use the genuine target-conditioned RR9/S9 or `K961(U_1)..K961(U_4), S5(...,x_R)` circuit on public residuals and the 31 Dickson x-coordinates, without enumerating 4A or signed supports. Require one natural known-positive R and off-support negatives on the original curve, repeated/cancelled leaves, explicit original-curve signed lift, and cost bound for all failed queries. Existing B8 D9 #4604 and seven-quadratic split #4581 show why positive witnesses, not just negative rank certificates, are mandatory.

**P1 — Lower-arity algebraic gate:** separately probe *already studied* depth-9 Dickson B256 m=3 algebraic S4 against the native generic triple-bank baseline in `M3_PROJECTIVE_ROW_AUDIT.md`. Earlier producer result at 4,051,363 generic signed templates had rank 255/255 but 3,765 raw collisions reduced to just 305 distinct projective rows; do not recast those generic enumerations as new algebraic extraction. The next run must yield **new nonenumerated** public-target conditional roots and meaningful independent modular rank at bounded full F4 cost.

**P2 — Complete solver admission:** only after P0/P1 produces a bona fide non-generic relation generator, use existing Rust modular LA and individual-log recovery on fixed **1,024 unknown-to-worker Q** and independently verify each `[k]G=Q`. Cost *every* failed transform, base construction, quotient/F4, all relations and rank, linear algebra, individual logarithms, peak process memory and scalar verification. Compare same public corpus to strongest shared BSGS/rho and published prime-field IC, not merely a hand-selected planted target. #4596 owns the pre-existing 1,024-Q full-cost comparison; reuse its frozen corpus and avoid duplicate paid runs. Historical producer #4532 mixed IC 2,221.474 ms vs shared BSGS 170.113 ms for 1,024 targets.

**Deprioritize** further optimization of B8 sign-mask enumeration (#4556/#4575/#4612) and seven-of-eight split quotient (#4581): their structural generic equivalence is established on their finite fixtures. The current batching improvement is valuable for engineering controls, but **does not resolve** the non-generic extraction bottleneck.

## Source review and novelty boundary

- Current repo audits: #4615 parent, #4596 complete-solver comparator, #4604 D9, #4614 3-sum, #4594 portfolio, #4581 seven-quotient, #4612 B8 rarity, `research/prime31_ic_20261115/native/M3_PROJECTIVE_ROW_AUDIT.md`.
- Existing batch-inversion practice: `research/notes/ecc2k130/compact_s3_batch_20260929/RESULT.md`, `research/lead_sweep_20261110/LEAD-RHO-batched-inversion-shared-pm/PROTOCOL.md`, `research/prime31_ic_20261115/native/APS_JOINT_SHAMIR_BATCH_RESULT_20261116.md`.
- Primary prime-field IC references: Petit–Kosters–Messeng (PKC 2016, DOI `10.1007/978-3-662-49387-8_1`) and Amadori–Pintore–Sala (FFA 2018, DOI `10.1016/j.ffa.2018.01.009`). Batch inverses are known from Montgomery (1987), as covered in Brent–Zimmermann, *Modern Computer Arithmetic* §2.5.1.
- Research-KB/Conductor unavailable in connector-only session; this is a bounded repository + primary-publication audit, **not** proof of novelty or absence of prior art.

**Decision:** retain a well-tested ~4.4–6.5× faster *generic-equivalent* membership control for IC engineering; invest next in algebraic sign-free extraction and full rank/unknown-log benchmarks rather than calling this a complete solver win. [pcc:unverified]
