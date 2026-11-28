# EXP-R31-MIXED-4P4-IC — complete two-target prime31 Rust result (2026-11-17 UTC)

**Result scope:** both new unknown-scalar public targets recovered with the native Rust *mixed-base index-calculus-derived* 4+4 solver; 62/62 independent rank in each, 61/61 factor-base logs checked in each, and 124/124 original-curve signed relations. The entire pipeline is **much more expensive in these development observations** than the historical single-target APS baseline, so the cheap-ECDLP objective is **not met**. The 4+4 tables are generic MITM-equivalent, not a new non-generic S9 extractor. **Producer-side [pcc:unverified].**

## Frozen evidence and source provenance

- Original prospective method: [MIXED_4P4_IC_PROTOCOL_20261117.md](MIXED_4P4_IC_PROTOCOL_20261117.md), commit `89a9dab6961589d4fc92eafdf26664a32485882d` before code, tests or timing. No retuned seed or speed threshold.
- Existing Rust source reused: `PRIME31`, `Arithmetic`, `Base::build(PRIME31,6)`, frozen native dependency source SHA `1df714645aa683cf975bc76fcacb9842cb4780fd`. New solver example `research/prime31_ic_20261115/native/examples/prime31_mixed_base_ic.rs` SHA-256 `c8a79b0edee9791ae42cd132cfebd867dfaa2cc0a1bf9cbac223247aa3a4efe5`. Released Rust 1.100, GitHub-hosted Ubuntu 24.04.
- [GitHub Actions run 44017666839](https://github.com/isogeny-labs/crypto/actions/runs/44017666839) **SUCCESS**, with native 40/40 library tests and 2/2 mixed-example tests, then a full two-target Rust execution and hard JSON schema gates. Artifact [14299004221](https://github.com/isogeny-labs/crypto/actions/runs/44017666839/artifacts/14299004221) contains complete 63,968-byte raw JSON (all 124 signed rows, labels, group coordinates and costs) and source hashes. **Raw JSON SHA-256** `029f8948aef3cdaa14e1ae664b7c54d3ce083f0603c0e55b2787aea31d52c53c`.
- [Merged implementation PR #4503](https://github.com/isogeny-labs/crypto/pull/4503) and [follow-on mixed formal-support theorem PR #4505](https://github.com/isogeny-labs/crypto/pull/4505).
- Independent *producer-side* supplementary check reconstructed the actual first-x and Dickson31 points via separately coded Python 3 affine elliptic-curve arithmetic, re-derived the common point `(3,166645833)`, independently replayed all **124/124** signed relations `sum leaves=aG+bQ`, recomputed all **124 modular row coefficients**, independently Gaussian-eliminated each 62×63 system and recovered both logged scalars with all 61 basis logs satisfying `[log]G=P`. The reproduction was conducted outside the native solver, **not** independently signed PCC, nor a production Python ECDLP algorithm.

## Complete cold producer observations

Same curve as preregistered: `p=2^31-1`, `a=847514846`, `b=2087444724`, `n=2147446831`, cofactor one, `G=(625037715,2100627251)`. Every cost includes two fresh four-sum tables, original curve witnesses, all attempts, signed row assembly and the complete 62-variable elimination, and final 62 scalar-point validations; tables are rebuilt from scratch for each Q.

| Case | Caller-known k, not passed to solver | Public Q | Rank | Attempts / misses | Half-table probes | Table/setup ns | Complete cold ns |
|---|---:|---|---:|---:|---:|---:|---:|
| 0 | 2062758563 | (872133043, 510239525) | 62/62 | 62 / 0 | 179502 | 2051902033 | **2115457523** |
| 1 | 651689658 | (1841259055, 779429965) | 62/62 | 62 / 0 | 232380 | 2042133375 | **2122593909** |
| **Totals** | — | — | **124 independent rows** | **124 / 0** | **411882** | **4094035408** | **4238051432** |

Each of the 31+31 point bases had exactly **618,885** distinct four-net signatures, no within-base extra point collisions and exactly **one shared affine point**, which is correctly deduplicated. Thus the basis contains **61 unique point columns plus 1 unknown Q column**. Each run reached complete rank with zero duplicate/dependent rows or failed mixed-group membership residuals. This is an instance observation (two natural Q), not a general full-rank theorem or group coverage proof.

**Cold total = 2.115 and 2.123 seconds per Q**. ~**96.6%** of the two complete totals is charged to building/replaying both four-sum tables, leaving 63.6 and 80.5 ms after that point. Mean **3,322 half-point probes per accepted signed relation**. This is not a better-cost method than the existing 24-Q single-target APS one-relation baseline (historical 81.681 ms / 24 ≈ 3.403 ms per Q), even ignoring strongest WZ rho. Those timings come from different source-specific CI runs and are **not a statistically valid same-host paired speed ratio**; do not report them as a calibrated slowdown factor or match them against an external frontier. The magnitude of the gap makes another identical 4+4 full-cold trial low priority.

## New mathematical consequence and lead triage

For two B31 bases sharing one point, the separate exact counting derivation (see `MIXED_FORMAL_SUPPORT_THEOREM_20261117.md`) gives **381,434,284,809** distinct *formal* signed 4+4 net vectors, compared with 5,654,193,417 homogeneous eight-net vectors and 383,018,643,225 ordered half-pairs. Only ~0.414% of formal mixed half-pairs duplicate another mixed net. This explains why mixed-base relations are abundant *at small dimension* and predicts a useful general-purpose correctness control. It **does not** prove curve-image coverage, a non-generic algorithm, or lower ECDLP cost.

**Disposition:** the present mixed MITM-control collector passes its *two-case completeness* gate but fails the **cheap-complete-solver goal in the available development cost accounting**. Replaying more 2-second cold cases alone will not close the cost gap. The top mathematical track remains `R=aG+bQ` target-conditioned **RR9/S9 extraction of `F=U+yV` and the eight-root polynomial from R** without half-sum enumeration. Reuse existing Hermite/norm certificates from #4466 and Rust quotient/subresultant rootfinding from #4228/#4349, first verify exact blinded positive/negative B4/B8 fibres and Dickson31 negatives, then a full B31 rank solver against WZ rho and mature index calculus. Benchmark the *fastest* matched reference and count polynomial elimination, failures, rank, root extraction and cold setup.

Other previously rejected or pending leads: allocation reuse #4484 (no gain), linear chord #4486 (0.825% unqualified), anchored m6/m8 F4 #4373 (censored), Vieta #4410 (no complete advantage), autoresearcher #3301/#3305, cryptanalysis #1536; tuned WZ workflow #43994897230 was blocked by an unrelated `sat.rs` unclosed delimiter, so there is still **no strongest rho measurement**. Literature and classical prior art: APS 2018 DOI 10.1016/j.ffa.2018.01.009 and PKM 2016 DOI 10.1007/978-3-662-49387-8_1.

## Separate exact-support Rust CI closure (2026-11-17 UTC)

The follow-up free-integer-lattice theorem [#4505](https://github.com/isogeny-labs/crypto/pull/4505) also passed [exact-source Rust Actions #44018093813](https://github.com/isogeny-labs/crypto/actions/runs/44018093813), including the recomputed **381,434,284,809** mixed formal signed 4+4 net count, 383,018,643,225 ordered half pairs, and brute-force B2/B3/B4 one-overlap versus disjoint small-base controls. [Raw theorem artifact #14298884847](https://github.com/isogeny-labs/crypto/actions/runs/44018093813/artifacts/14298884847). This is a checked combinatorial component; the formal-net count is **not** a speedup or group coverage proof. Both CI runs remain producer-side [pcc:unverified].

## Standing and next experiment

All results are producer tests and replay, **[pcc:unverified]** with no CI-main signed execution verdict. This is an honest completed experimental rung, not a claimed world-record or non-generic ECDLP advantage. The next preregistered Rust run should avoid the two 618,885-entry generic tables entirely and test target-conditioned algebraic extraction; only after that succeeds should a 96-Q full cold performance panel be scheduled. Preserve raw artifacts (currently 30-day Actions retention) for independent validation before expiration.
