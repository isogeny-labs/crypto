# Prime31 complete cold APS versus BSGS protocol — frozen before implementation

Date: 2026-11-16. Source/branch: `research/prime31-aps-walk-source-20261116`, open PR #4391.
Standing: protocol only, [pcc:unverified]; **no measurement claimed by this file**.

## Research question and classification

Does the **already completed** Rust APS2018-derived one-relation S3 factorized-coordinate solver actually beat an independently implemented, strong batched-Jacobian baby-step/giant-step baseline under matched cold costs on the registered *ordinary cofactor-one* prime31 curve? This is a falsifiable comparator, **not** a new non-generic ECDLP algorithm. A loss or failure must be kept. Do not substitute older unmatched BSGS numbers.

The curve, base point, field, group order and factor-base profile are unchanged:
`p=2147483647`, `n=2147446831`, `E:y²=x³+847514846*x+2087444724`, `G=(625037715,2100627251)`, APS labelled partitions `[384,384,16384]` with six rounds. BSGS has `m=ceil(sqrt(n))` and **all** baby-table generation, storage, giant walks, normalization, lookup and final replay charged for **each** independent target.

## Prospective fixed experiment

- A fresh 24-target public-Q panel is produced by caller-only Rust `SplitMix64` already used in the source, seed `0x4247_5353_434f_4c44`, nonzero k, duplicate point/k excluded, target Q = [k]G. The k is held outside both solver routines and used only by the caller verifier.
- Three arms per Q: APS factorized S3; BSGS using existing Rust mixed-Jacobian/Montgomery batch normalization, **no** target answer oracle; BSGS independent cold A/A repeat. Rotate call orders over 3 permutations to reduce sequencing bias.
- All 72 recoveries must be nonzero, exactly match caller known k, and pass an independent `[k]G=Q` replay. BSGS must also handle zero baby (identity) and zero giant (exact multiple of m), with final scalar check. Record initial/source, complete all-round, verification, and table time; do not retain only the winning arm or drop misses.
- Output machine-readable per-target JSON: Q, caller-answer used **only after** solving, method, result, complete in-process cold ns, APS per-round breakdown and BSGS baby/giant counts. Record total cold cost and pairing per arm, A/A drift. No accuracy or speed claim before source-exact Rust CI is observed.
- Do not promote an IC leaderboard row or claim rho/security/fastest external CAS comparisons. This is a generic BSGS comparator, NOT optimized rho; independent PCC and a matched optimized rho remain separate gates.
- Stop/failure gate: if any Rust assertion or source-specific run fails, record it accurately; repair and rerun before drawing conclusions. No paid EKS required; focused existing `prime31-s4-split-20261116.yml` Actions Rust 1.100 release gate is the measurement backend.

## Prior sources and searched paths

- Existing prime31 Rust implementation: `research/prime31_ic_20261115/native/examples/prime31_aps_one_relation.rs`, source-dependent complete cold S3 factorized result, #4391.
- Earlier `research/prime31_ic_20261115/BATCHED_INDEX_CALCULUS_FOLLOWUP.md` reported earlier BSGS outperforming an **older** IC variant by about 100×, and explicitly lacked an optimized rho and matched current APS control; this protocol asks a narrower changed-source question.
- `research/prime31_ic_20261115/native/M8_SYMMETRIC_ROOT_RESULT_20261116.md`: B8 given-f validator **does not extract target-conditioned f**. `#4373`: censored F4; `#4228` and `#4349`: smaller certified-quotient machinery, incomplete B31 extraction; do not repeat prior pair enumeration as an algebraic discovery.
- `isogeny-labs/crypto-autoresearcher/experiments/semaev-alternatives-20261105/SUMMARY.md`: SA-02, SA-14b, SA-15b and related negatives; no unsupported novelty claims.
- APS (Amadori, Pintore, Sala, FFA **2018**, DOI `10.1016/j.ffa.2018.01.009`); PKM (Petit, Kosters, Messeng, PKC **2016**, DOI `10.1007/978-3-662-49387-8_1`). BSGS is the classical Shanks generic method. The repository KB MCP, Conductor and local Rust toolchain are unavailable in this connector-only runtime; this is a bounded committed-GitHub and publisher source review.

## Next scientific gate, independent of this comparator

Original-ideal-certified target-conditioned 3+3+2 symmetric S9/Vieta root extraction at B4/B8, then B31 with no scan over C(38,8)=48,903,492 multisets, full unknown target scalar recovery and exact completed CAS/PKM/APS/rho controls. Fail or censor rather than claim a complete non-generic solver without rational roots and a successful target replay.
