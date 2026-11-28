# Prime31 four-distinct-leaf representation density and real solver-cost experiment

**Date:** 2026-11-23 UTC. **Standing:** exact integer combinatorial derivation and a new original-Rust diagnostic source; the diagnostic PASSED 2/2 source-bound original Rust 1.100.0 tests in GitHub Actions run #45111925963; the frozen B677 fixture identity and final table were executed. No non-generic extraction, cold solve or timing advantage is claimed. [pcc:unverified]

## Unchanged curve and genuine source provenance

- E / F_2147483647: y²=x³+847514846x+2087444724, generator G=(625037715,2100627251), subgroup n=2147446831, reported cofactor one.
- Existing Rust `prime31_algebraic_ic::{Arithmetic,Base,PRIME31}` and the **same** B509 Dickson + 168 public Y7 leaves (`RUN66_Y7_EXTRA168.csv`) reconstruct B677. No curve engine, base, signed relation or parser is replaced.
- The new source `research/prime31_ic_20261115/native/examples/prime31_m4_representation_density.rs` verifies the actual field, base membership, on-curve and x-distinctness checks, then evaluates exact coefficient-vector counts using u128. This is a **combinatorial prescreen**, NOT an index-calculus oracle; its output passed an exact-source native Rust 1.100.0 GitHub Actions development correctness gate (2/2 tests, frozen 168-leaf fixture hash check).

## Exact identity (not a probabilistic model)

Let F consist of B fixed distinct finite points, no pairs P and -P and no 2-torsion. For every public target Q drawn **uniformly** from the prime-order group of size n, consider all signed coefficient vectors on four **distinct** base indices. Each choice maps to exactly one group element. Double counting the (vector,target) incidences therefore proves the unconditional identity

`E_Q[# distinct-index four-signed coefficient vectors mapping to Q] = 16*C(B,4)/n.`

It does not require random base points, independent group sums, an assumption about F4, or an elliptic-curve weakness. For a separate even-length <=4 enumeration including cancellations, the known exact vector count from the November-15 m4 support gate is

`M4(B)=1+4B+16*C(B,2)+24*C(B,3)+16*C(B,4).`

Exactly similarly, `E_Q[# even-parity, l1 <=4 signed coefficient vectors mapping to Q]=M4(B)/n`. A count of **vectors** is not a count of distinct target points, chart solutions, independent rank or solver successes. For natural uniform Q, the exact success probability is bounded above by `min(1,M4(B)/n)`, but once this upper bound exceeds one it is vacuous. A Poisson coverage approximation would require an unverified mixing assumption and is deliberately NOT included as an observed value.

| Public factor-base size B | Exact four-distinct signed coefficient vectors | Exact uniform-Q expected four-distinct representations | Expected all parity-compatible <=4 representations |
| ---: | ---: | ---: | ---: |
| 31 | 503,440 | 0.000234437 | 0.000288196 |
| 128 | 170,688,000 | 0.0794842 | 0.0833602 |
| 192 | 877,927,680 | 0.408824 | 0.421939 |
| 224 | 1,633,830,016 | 0.760824 | 0.781667 |
| 256 | 2,796,682,240 | **1.302329** | 1.333458 |
| 320 | 6,860,184,320 | **3.194577** | 3.255423 |
| 384 | 14,270,102,016 | 6.645148 | 6.750345 |
| **677** | **138,805,851,600** | **64.637620** | **65.214735** |

For B=8 the expected number of distinct-index four-signed representations is only **5.22e-7** per uniformly sampled target. Thus the prior planted B4 and selected B8 positive controls are valuable semantic checks but cannot establish natural blind yield or comparative cost. At B677, the much larger *average* output multiplicity means one public query may yield multiple independent rows; charging only binary hit/miss discards information. Conversely those output rows may be dependent or clustered on a tiny set of Q. We must measure the **full distribution**, not use the average to assert nearly universal coverage.

[Editable diagram](FOUR_LEAF_DENSITY.svg) is a *mathematical expectation graph*, not benchmark data.

## Reused measurements / previous leads (checked before choosing this work)

1. [November-15 exact m4 support and generic B31 census](../prime31_ic_20261115/native/M4_NATURAL_YIELD_ARITY_GATE_20261115.md) reached precisely 618,885 of 618,885 possible vector targets at B31 in a generic enumeration. It did **not** measure natural B677 quartic roots or cold index calculus.
2. [Run66 original-Rust three-sum B677 support](../prime31_f4_admission_20261121/RUN66_THREESUM_RESULT.md): 711/4096 independent SHA-256 public targets had a generic signed triple (17.36%). This is m3, not m4. Its previously reported four-term capacity ceiling was not a four-term yield measurement.
3. [Run71 authentic RHS and difference rows](../prime31_f4_admission_20261121/RUN71_DIFFERENCE_RHS_RESULT.md): 32/32 completed target logs and 96 genuine difference rows, but a **generic 916,469-key pair dictionary** and the measured 1,074.583/1,274.441 ms cold solver versus 200.234/236.674 ms BSGS were not an admissible non-generic advantage.
4. [Run75 cold cost floor](../prime31_f4_admission_20261121/RUN75_COLD_PREORACLE_FLOOR_20261123.md) proves the existing setup+witness cost exceeds matched 32-target BSGS before relation work in all nine **existing K2/3/4 architecture** cells. This does not bound a replaced RR5 architecture.
5. [Merged RR5 #4942/#4949 and active #4953](https://github.com/isogeny-labs/crypto/pull/4953): B8 compact monic quotient yielded one signed positive and an empty generator control with reported 19 native and eight example checks; B677 root extraction and the complete cold comparison remain open. This task does **not** duplicate or modify its extraction code.
6. [Prime31 hybrid x/y projection](../prime31_ic_20261115/native/XY_COORDINATE_PROJECTION_PILOT_20261115.md) found non-injective y fibers and extra log unknowns; B-size optimization must preserve the original x-base semantics. Existing RR9, Horner/F3, FASS, Dickson S3 and pair-spectrum PRs remain independent active lanes.
7. Primary published prior art: [Petit-Kosters-Messeng, PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1); [Faugere et al., EUROCRYPT 2014](https://doi.org/10.1007/978-3-642-55220-5_3); [Amadori-Pintore-Sala, FFA 51 (2018)](https://doi.org/10.1016/j.ffa.2018.01.009). This coefficient count is not a novel index-calculus algorithm. Shared research-KB MCP was unavailable; the bounded source review is not an exhaustive novelty search.

## Frozen next original-Rust experiment: base-size versus **independent rank per fully charged query**

**P0 admission before paid runs:** Native original-Rust diagnostic gate now PASSED 2/2 at [Actions #45111925963](https://github.com/isogeny-labs/crypto/actions/runs/45111925963), on source commit `10414848c449dba47f8367fc8e38ec5418aec106` (the later PDF adds no Rust changes); original fixture SHA-256 checked, B677 row `677,138805851600,140045175925,64.6376197055,65.2147349603`. This is a correctness pass only. Confirm current #4953 B8 complete positive/empty statuses from source-bound CI before transplanting any comparison.

**Arms:** public-only compact RR5 monic quotient extractor and incumbent best RR9/F3 nonenumerative extractor, each at B=224,256,320,677 using the existing registered original-Rust B677 points and subgroup. Define B224/256/320 as deterministic prefixes of the original B509 Dickson list; B677 is the original 509+168 hybrid, **not** the same random ensemble. Recompute preexisting *valid* free homogeneous relation rank at each B; do not transplant all 88 rows unchanged. The ordinary cofactor-one curve and generator remain identical throughout.

**Frozen probes:** reuse the already preregistered blind **46 positive + 18 negative** oracle panel from #4692 as a semantic control, but never use oracle sign/answer labels during extraction. Add a disjoint, frozen SHA-256-hash-to-curve cohort of 128 public Q without disclosing scalar logs to the candidate. Keep failure/timeout cases and all public shift k identities. Do not replace these with planted witnesses. Natural-target group support may be checked separately by a *generic grader* but must never feed candidate input or be timed as the candidate.

**Raw per-query record:** SHA and source/fixture hashes; B, Q, algorithm, chart; attempted/completed/censored; module prep, F4 pairs/degree/matrix width, root factoring, lifts, exceptions, allocation/RSS, field ops, group additions, wall and deadline; every distinct checked coefficient vector, public signed group identity and true RHS; duplicate and new-rank counts before/after genuine free and difference rows; full-target recovery [d]G=Q verified by a separate original group-law path. Each failed or refused query is billed. Distinguish `Empty` only with a complete proof, never from degree exhaustion or missing univariate.

**Direct falsifier:** At B677, if the independently group-checked number of four-distinct signed coefficient vectors on fresh uniformly hashed Q has an average far below the exact uniform expectation 64.637620, examine the cohort's uniformity, base cardinality, distinct-root chart exclusions, and solver incompleteness. Do not call an incomplete extractor's low witness count a counterexample to the incidence theorem. If B224 or B256 produces cheaper exact roots and *more useful rank per millisecond* than B677, retain the original baseline and stage a complete cold comparison; if all arms stall, record that rather than silently assuming smaller B helps.

**End-to-end acceptance:** recover all requested actual public target logs through authentic-RHS independent rank using original Rust, including cold setup and all failed probes. Compare identical source-pinned target cohorts on the same isolated host against the strongest reproducible existing index-calculus solver, tuned `prime_fast` negation-map Pollard rho, and shared BSGS diagnostic. Primary metric cold **one-target** full cost; T=32/128/1024 only separately labelled amortization. Require A/A, at least five measured rounds for an admitted hardware speed comparison, independent replay and signed PCC; repeat on two additional ordinary prime-order curves before claiming broader prime-field performance.

**Current result:** Exact combinatorial prescreen, B677 original Rust factor-base reconstruction and 2/2 native correctness tests passed; **no new solver throughput, rank or scalar-recovery measurement** was made in this run. B677 root extraction and its cost remain the decisive missing gates. The 2-page PDF draft passed structural xref/object checks but could not be rasterized from this connector environment, so it was **removed from the PR** pending the required visual inspection. Markdown, editable SVG and CSV remain the reviewable source. [pcc:unverified]
