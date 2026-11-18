# Actual-prime31 B27/B29/B31 complete 256-logarithm cost sensitivity (2026-11-17 UTC)

**Producer source-specific Rust and independent producer-written arithmetic replay: [pcc:unverified].**
PR [#4629](https://github.com/isogeny-labs/crypto/pull/4629), stacked on #4626/#4623. [Prospective protocol](B27_B29_B31_FULL_COST_PROTOCOL_20261117.md) was committed before changing Rust; [complete raw records](B27_B29_B31_FULL_COST_RAW_20261117.json) contain 3×256 public Q/log/witness transcripts, all 3 factor bases, public relation rows and recovered logs, with two exact-source CI timing sets. **No sign-independent algebraic extractor is used, no new cryptanalytic exponent or cheaper non-generic ECDLP solver is claimed.**

## Decisive result on the actual curve

The original existing Rust complete pipeline (signed eight-leaf 4A+4A HashMap → learned modular log basis → 256 blinded unknown Q scalars) succeeded at every B, but **B31 won both charged full-cost runs despite having the largest 4A table**. Smaller bases saved table construction and columns, yet lost more time to costly unsuccessful complement scans and extra residual shifts.

| Original depth-six Dickson **prefix** | B27 | B29 | B31 |
| --- | ---: | ---: | ---: |
| Rank learned, columns | 27 / 27 | 29 / 29 | 31 / 31 |
| Signed four-net group points | 356,725 | 474,325 | 618,885 |
| First-shift positive public Q / 256 | 147 | 195 | **237** |
| All nine-shift attempts across Q | 460 | 337 | **276** |
| Exact exhaustive failed Q shifts | 204 | 81 | **20** |
| Full logs solved & checked | 256/256 | 256/256 | 256/256 |
| Generic fallback logarithms | 0 | 0 | 0 |
| Public basis sources until full rank | 27 | 29 | 31 |
| Basis/control membership attempts | 75 | 59 | **48** |
| Charged 256-log solver, push run | 6,556.280 ms | 4,434.263 ms | **3,152.266 ms** |
| Charged 256-log solver, PR run | 6,921.234 ms | 4,922.840 ms | **3,471.880 ms** |

In the push run B31's full cost was 0.481× B27 and 0.711× B29; the independent PR run gave 0.502× and 0.705×, respectively. Both jobs used the same exact source, pinned original native crate and arithmetic, lockfile and Rust 1.100.0, and **identical outputs for all 768 target logs and witnesses and the 87 factor-base log entries**. The fixed B27→B29→B31 panel order and hosted runner variability limit wall-time comparisons; these are repeat diagnostics, not independently calibrated fastest-solver claims.

## Why this happened

The exact count of formal signed net vectors of at most eight leaves with even parity, allowing cancellations, is

`N_8(B)=1+Σ_{k∈{2,4,6,8}}Σ_{j=1}^{min(k,B)} 2^j C(B,j)C(k−1,j−1)`.

For B=27/29/31 it equals 1,897,865,641 / 3,336,583,401 / 5,654,193,417. The exact four-net counts 356,725 / 474,325 / 618,885 agree with the measured collision-free 4A group-point sets on these fixed prefixes. **The counts do not make the eight-net group images injective.**

The *nonrigorous independent-bin approximation*, `1−exp(−N_8(B)/n)` for subgroup order `n=2,147,446,831`, predicts first-shift coverage 58.68% / 78.85% / 92.81%. The original observed values 57.42% / 76.17% / 92.58% have the same monotonicity, but the 8A images are highly dependent, so this is **not** an exact coverage theorem or a causal proof. Actual failed-shift counts, charged inv/multiply counters and source-specific raw traces—not the bin heuristic—support the observed cost ordering.

Across B27/B29/B31, the original public relation oracle made 460 / 337 / 276 target attempts (including 204 / 81 / 20 failed exhaustive scans) to finish the identical 256 Q, so shrinking the 4A index while decreasing target coverage raised **total** target work. B31 also needed only 48 rank/control membership attempts versus 75 and 59 for smaller bases, even though its rank target was larger. Other subselections of 27 or 29 points might differ: this result holds for these preregistered prefixes only.

## Source-specific evidence and arithmetic readback

- Exact working Rust example: `examples/prime31_s9_b31_half_sum_oracle.rs`, new `complete_b31_1024_tests::actual_prime31_b27_b29_b31_256q_complete_cost`; no new group-law implementation, no secret-scalar oracle; `FourSet::build`, `recover_batched`, original modular rank/Rust 1.100, full-group fallback reused.
- Original source arithmetic from immutable `1df714645aa683cf975bc76fcacb9842cb4780fd`; candidate source `22a31ed2529ccbacf3d213985ecad69f639e6f22`. [Push CI #44207041891](https://github.com/isogeny-labs/crypto/actions/runs/44207041891) and [PR CI #44207050180](https://github.com/isogeny-labs/crypto/actions/runs/44207050180) both **passed** 40 original native library tests and 1 focused integration test. The raw artifacts are [#14384426906](https://github.com/isogeny-labs/crypto/actions/runs/44207041891/artifacts/14384426906) and [#14384715871](https://github.com/isogeny-labs/crypto/actions/runs/44207050180/artifacts/14384715871); GitHub-reported ZIP SHA256 are `b9d109a29d95152ea01e89b64df8c0af066fcc39b23e179e5946bb49f4420f4b` and `b7c962dd7f9c5fe1dc20f60402440732dcc90fdc6442a2a75f1ae3f535995961` (ZIPs not independently downloaded/rehashed). The [durable raw JSON](B27_B29_B31_FULL_COST_RAW_20261117.json) includes both timing runs and the full first-run transcript.
- Producer-written independent BigInt group arithmetic check replayed **all 87 factor-base logarithms**, **768 recovered target discrete logs**, and **768 signed eight-point witness identities** with a separately written short Weierstrass law, and recomputed each authentic row's modular right-hand side; all passed. Confirmed exactly **256 distinct public Q shared across the panels**, unchanged known scalar→Q mapping, all input coordinates on `y²=x³+847514846x+2087444724 (mod 2147483647)`, generator order `2147446831`. This is an arithmetic cross-check by the producing run, **not** an independent signed PCC verifier.
- Both CI runs' 87 factor-base log values, 768 complete target records, 3 rank-row sets and first-shift masks are bit-for-bit equal after excluding timing/counters. All 16 extra independent public source checks per panel were represented (no missing controls), all 256 target results independently checked against caller-only known answers and original scalar group law. The experiment has *not* demonstrated new public-target algebraic relation extraction.

## Prior work and novelty boundaries

Read the frozen actual-prime31 complete B31 reference [#4623](https://github.com/isogeny-labs/crypto/pull/4623) (1,024 logs; fully charged 7,658.159ms), original Dickson-versus-generic matched bases [#4626](https://github.com/isogeny-labs/crypto/pull/4626) (237 vs 244/237/237 first-shift hits), incremental public shift [#4615](https://github.com/isogeny-labs/crypto/pull/4615), Montgomery batch [#4618](https://github.com/isogeny-labs/crypto/pull/4618), separate matched full IC vs shared-BSGS owner [#4596](https://github.com/isogeny-labs/crypto/pull/4596), sparse D9 negative unit certificates [#4604](https://github.com/isogeny-labs/crypto/pull/4604), synthetic 48-curve comparison [#4624](https://github.com/isogeny-labs/crypto/pull/4624), portfolio [#4594](https://github.com/isogeny-labs/crypto/pull/4594), cross-repo [autoresearcher #3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301) / [#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305) and [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536). These carry the earlier B8 RR9 barycentric, seven-equation quotient, Macaulay degree-5/6/9, signed-branch enumeration, generic 4A no-gos and next quotient/F4/rootfinding leads.

Established published prime-field IC approaches: Petit–Kosters–Messeng (PKC 2016, doi:10.1007/978-3-662-49387-8_1) and Amadori–Pintore–Sala (FFA 2018, doi:10.1016/j.ffa.2018.01.009). Newly noted 2026 *unindependently validated* asymptotic/theoretical claim: Kaarelson, [fast-ecdlp](https://github.com/kaarelkaarelson/fast-ecdlp), describes a counting-engine-based p^{2/m+o(1)} attack; its own version-2 README says provable crossover requires log2(log2(p)) at least 6.292·10^7, astronomically outside prime31. It is not a practical candidate or a substitute for our full-cost benchmark. Its public Lean `Challenge.lean` deliberately contains `sorry`, while separate solution modules claim proofs; this run did **not** build/audit its Lean project or establish the paper-only ECDLP theorem.

Shared `research-kb` MCP server/CLI was unavailable in this connector runtime, so prior-art review is bounded, not novelty clearance. The historical shared-precomputation **170.113ms/1,024-Q BSGS** benchmark [#4532](https://github.com/isogeny-labs/crypto/pull/4532) is a different-run **different-size** reference only; do not divide its time by these 256-Q cases to make a claimed speed ratio.

## Next experiment queue (nonduplication)

1. **Required novelty gate: public-residual-only B31 sign-independent RR9/S9 extraction**, use existing original Rust quotient/F4/rootfinding, no `FourSet`, precomputed 4A group table or supplied leaf support. Recover an original-curve signed eight-point witness for B8 positive; prove negative original-ideal unit certificates for two frozen negatives; then move to B31 natural positives, off-support negatives, repeated roots, y=0 and infinity charts. Record all algebraic setup, failures, degree and memory. Source-owned D9 lane #4604; coordinate with it rather than duplicating.
2. **Target-conditioned K961/S5 sparse resultant alternative**: construct the 31×31 pair-coordinate polynomial with original Rust arithmetic; measure whether a public-R-conditioned squarefree quotient/factor/root step can recover full signed leaf witnesses *without* visiting 4A or enumerating 961^4 branches. First establish complexity and memory gates, only implement if bounded. The 4A lookup can remain a **test oracle**, not appear in the candidate cost or as a hidden adviser.
3. **Complete cost**: only after a truly nonenumerative extractor passes exact small and exceptional controls, feed its actual relations to the known Rust rank→256/1024 unknown-log pipeline; remeasure all setup, misses, linear algebra, target retries, peak RSS and fallback on the same isolated runner against shared-advice BSGS, optimized rho and a published prime-field IC baseline, with independent PCC. Retain this B31 generic-equivalent branch solely as a reproducible correctness control.

**Decision:** For the existing generic-equivalent 4A Rust mechanism and these fixed prefixes, do not downsize B31 to B27/B29. The measured two-run full-cost ordering favors B31. The separate algebraic nonenumeration target remains unresolved.
