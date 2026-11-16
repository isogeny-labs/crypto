# Packed x-index prime31: 96 completed Rust solves, prospectively registered 10% gate fails

2026-11-16 UTC. **Source-specific producer/development observation [pcc:unverified]**, not a certified speed record or a new non-generic ECDLP algorithm.

## Source and executed experiment

- Existing PR [#4391](https://github.com/isogeny-labs/crypto/pull/4391), source commit `f6060840cf081a875b846a90575f557e1dc58668`, before this evidence/report update.
- [Preregistered protocol](APS_PACKED_X_INDEX_PROTOCOL_20261116.md) was committed before implementation and execution; it explicitly requires **at least 10%** lower cold cost versus compact HashMap as well as 18/24 paired wins, 96/96 correct logs, exact relation transcripts and repeat drift no more than 5%.
- Reused existing **Rust** `examples/prime31_aps_one_relation.rs` and its `PackedXIndex` linear-probing power-of-two array, compared with the previous `HashMap<u64,usize>`. Independent Rust Jacobian BSGS is the fourth cold arm; the caller-only known scalar never enters root selection.
- [Source-specific passing GitHub Actions run 43890868003](https://github.com/isogeny-labs/crypto/actions/runs/43890868003), job `133381372752`, Rust 1.100 release. Example tests plus `--packed-x-index-confirm` completed. The original emitted JSON, with GitHub timestamp prefixes stripped and JSON formatting normalized but fields preserved, is [frozen here](APS_PACKED_X_INDEX_CI_20261116.json).

The fixed ordinary cofactor-one curve is **not weakened**:
`p=2147483647=2^31-1`;
`E: y^2=x^3+847514846*x+2087444724`;
prime `#E=n=2147446831`, `G=(625037715,2100627251)`.
The existing public APS partitions are `[384,384,16384]`. Each of 24 prospectively seeded public targets runs four fully cold arms, paying public labelled-walk construction, index build, every attempted S3 query, verification, and the independent cold 46,340-baby BSGS table. This is **complete unknown-log recovery**, not a microbenchmark of hash probes.

## Matched raw outcome: **negative preregistered 10% gate**

| Source-exact Rust solver, same 24 Q | Verified complete logs | Aggregate cold |
| --- | ---: | ---: |
| Compact third-list `HashMap<u64,usize>` APS (control) | 24/24 | 117.215002 ms |
| Packed `Vec<u64>` open-addressed APS (candidate) | **24/24** | **110.408323 ms** |
| Packed APS A/A repeat | 24/24 | 109.905153 ms |
| Independent fresh-table Jacobian BSGS | 24/24 | **108.326084 ms** |

The packed array reduced complete APS cost **5.8070%** (ratio `0.9419299673`), winning **24/24** paired Q relative to HashMap; its A/A drift was **0.4557%** and it preserved **586,752 S3 pair quadratics** and the original signed first-hit relation in every compared arm. But the preregistered improvement threshold was at least **10%**, so **the registered engineering gate FAILS**. Packed APS also remained **1.9222% slower** than matched fully cold BSGS (ratio `1.019221954`), with only **13/24** per-target wins over BSGS. These are same-host, unisolated developer CI timings; no independent PCC or reproducibility interval.

## Independent in-session audit (not an independently signed PCC)

Parsed the frozen **24 × four-arm** original Rust JSON from job stdout. Separately recomputed all four aggregate timings, the three win counts, repeated-run drift and the 10% decision. A separately written finite-field affine point-addition/scalar-multiplication implementation, not the Rust solver or its known-answer control, reconstructed **all 24 public `[k]G=Q` points** over the given curve. All 96 returned k values equal the caller-only controls, and **72/72** signed-relation congruences satisfy `alpha+beta*k ≡ 0 (mod n)`; candidate and control round counts, S3 counters, selected signed indices, signs and coefficients agree exactly. **No discrepancies.** This is an independent *in-session* arithmetic cross-check but has no trusted PCC signature, isolated reproduction or external library comparison, so its status is still **[pcc:unverified]**.

## Existing-lead audit, before choosing another experiment

- **Already measured within #4391:** walking labelled points, Mersenne field multiplication, asymmetric partition `384/384/16384`, x-fibre roots, batched Jacobian source, factored S3 chord coordinates, and compact HashMap index. They remain published-APS-derived **generic pair-MITM-equivalent** source improvements, not a newly discovered non-generic asymptotic algorithm. Today's matched BSGS is still cheaper by 1.9222%.
- **Negative fixed arity:** #4391 two-anchor m6 has at most **3.7407%** target support and censored F4 natural-root calls. #4373 unanchored m8 tower F4 calls hit dense caps; do not relabel a cap as a negative algebraic root. #4383 and the post-Jacobian profile sweep reject repeating prior partition-size retunes without changed mechanisms.
- **Scientific algebraic path:** [m8 rational bridge](M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md) constructs `H` of degree 31 and pair-support `K` of degree 961; #4228/#4349 are the existing certified quotient/selected-root Rust work. The [symmetric exactness report](M8_SYMMETRIC_ROOT_RESULT_20261116.md) has exhaustive B4/B8 **given-f** agreement, but not unknown-target f extraction. Avoid a `961^4` pair-coordinate tensor or enumeration of `C(38,8)=48,903,492` B31 f multisets: those are generic support scans.
- **Prior-art/negative atlas:** [crypto-autoresearcher Semaev alternatives summary](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/experiments/semaev-alternatives-20261105/SUMMARY.md) and [errata](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/experiments/semaev-alternatives-20261105/ERRATA.md) reject rerunning unchanged SA-02 Vieta-conic, SA-14b PKM F4 degree, SA-15b m4 nonsuccess, and SA-24c genus-2 covers. Primary prior art: Petit–Kosters–Messeng [PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1), Amadori–Pintore–Sala [FFA 2018](https://doi.org/10.1016/j.ffa.2018.01.009), and the prime-field limitations discussed in [JMC complexity analysis](https://doi.org/10.1515/jmc-2019-0029).
- **KB receipt:** Research-KB MCP and `scripts/research-kb` wrapper/Conductor unavailable in this GitHub connector + no local clone/toolchain runtime. This is a *bounded* repo/atlas and literature review, not an exhaustive novelty clearance.

## Next registered-work proposals (unexecuted; do not count as wins)

1. **Highest priority, genuinely algebraic:** reuse Rust `prime31_m8_quotient_bridge.rs` and #4228/#4349 to build a **target-conditioned** quotient/eliminant `K(U_i)=0` for four pair abscissae, plus `S5(U1,U2,U3,U4,x_Q)=0`, over B4 then B8. Verify *all* rational signed roots against the independent exhaustive validator; handle diagonal repetition, cancellation/infinity, nonradical components, nonadmitted x and lifted y. For B4, `deg K=16` and full four-block tensor already has `16^4=65,536` entries; for B8 `deg K=64`, `64^4=16,777,216`; for B31 `961^4=852,891,037,441`. The critical gate is **nonenumerative selected-root extraction with an original-ideal certificate**. A symbolic prefilter followed by all-support enumeration is a stop, not a success. Require naturally selected candidate f without giving the algorithm f, Q's scalar, or group-sum joins.
2. **Optional complete Rust cost control:** adaptive third-list construction instead of eagerly allocating 16,384 labelled points. Pre-register a distinct 24-Q panel; guarantee deterministic source-extension and identical first-hit S3 roots; compare fully cold costs against current packed, compact and BSGS including every unsuccessful extension. Merely adjusting index size or hiding source time is not an admissible win. At present no benchmark supports this idea.
3. **Scientific acceptance:** complete B31 unknown-target signed root extraction, enough independent useful relations or a validated one-relation scalar recovery, full cold 12+ public Q, all failed calls charged, comparisons with native APS/PKM, mature external CAS and optimized rho on same host; independent replay / signed PCC before a leaderboard claim. The >generic threshold is not assumed.

**Decision:** freeze this 96/96 measured implementation result and its **failed 10% preregistered gate**. Do **not** promote a speed ranking or claim a novel non-generic cheaper solver. Continue the algebraic S9 extractor rather than recycle rejected small-system/lattice variants.
