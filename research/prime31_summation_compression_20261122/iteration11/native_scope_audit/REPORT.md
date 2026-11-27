# Native four-summand evidence scope audit — 2026-11-24

[pcc:unverified] Read-only audit of existing evidence; no new experiment.

- #5142 at c301874200e5f55ad848d8d9ab1bd9c9225250cd, NATIVE.md blob ed79cd29f5ce0f40a7a7c2081fa81620cb3f9ee8: historical Rust/msolve discovery on F127 and F8191 only. Distinct-image arm 4/4 on F127, 4/4 empty controls at F8191 selector16, 5/5 undetermined at selector32 including one planted positive. Quadratic arm 4/4 timeouts on F127. F127 recovery used seed 781815; no observed independent replicate panel, prospective manifest or paired uncertainty. Not M31 evidence; no smaller-field substitution.
- #5136 at 46af4bd43bdda3defd2ad198ed1cb0fcd133959b reports 16 supplied B509 M31 quartets with Jacobian rank one to three. Standalone checker, not a coupled solver or discovery. PR body evidence only.
- #5120 at 573d704563ed1375bd4cfe2fd8b771f46861c6a1 reports 70 passing component tests, distinct-four RR5 streaming adapter, not full-root or natural B677 extraction. Do not duplicate its source or ledger reconciliation.
- #4955 closed at 48b34c2d14b617f662c655f6f35dd86c5463587f already provides representation expectations and source-bound B677 fixture reconstruction; expected multiplicity is not hit probability.
- #5128 at 9f3aec6264e7a91f81ff8ea81be8bc83ec458f53 owns current-root qualification. #4977/#4979 merged; #5131 remains draft. Main/lane ACTIVE and journals inspected, not merged.

```text
Small-field native discovery -> historical chart correctness
M31 supplied quartets        -> local Jacobian correctness
M31 frozen chain guard       -> supplied-certificate verification
Blind M31 discovery          -> not qualified by these results
```

The frozen original guard (examples/prime31_rational_chain_guard.rs, blob 7f991783bd99d4f95ec4df1a94b50e79be693b51) checks supplied chains, depth-six terminal 933677148, full fibre 64 and rational B31, ghost, identity-prefix and balanced-tree exceptions, and 4^3+4^4 ordered fixtures on a four-element signed alphabet. These 320 fixtures are not 320 independently sampled public targets; the sign reference is conditional on supplied leaf x coordinates. No new guard execution.

## Blind-panel interpretation

Conditional on the earlier written cancellation-reduced support cap 618884 nonidentity signed targets and r=2147446831, the uniform nonidentity target hit probability is at most h=618884/(r-1)=0.0002881952611604358. For n independent draws with replacement, P(any hit)<=1-(1-h)^n: 1.43085% at 50, 25.04159% at 1000, 94.39980% at 10000. This is a ceiling, not predicted or measured yield. Even complete discovery may return no positive in a 50-target panel with probability at least 98.56915%. Zero hits in 50 independent draws gives one-sided 95% exact binomial upper limit 1-0.05^(1/50)=5.81551%, much larger than the support cap. NIST reference: https://itl.nist.gov/div898/software/dataplot/refman2/auxillar/exacbino.htm .

Keep positive correctness fixtures separate from blind-panel inference; timeout, cap and incomplete chart/output are undetermined, never certified empty. No new trial, curve operations, native tests, timing or paid computation. Seed manifest not applicable to deterministic arithmetic and read-only audit. No new formal proof or independent attestation. Native worker dispatch, KB/Conductor unavailable; coordinator parallelized source reads. No Space write or merge. Preserve prior receipts and both divergent journal histories. Reuse existing qualification work, exact discovery remains open.

Impact score: 3/10 — source-linked validation-scope correction and conditional blind-panel interpretation; no new native qualification or speedup.
