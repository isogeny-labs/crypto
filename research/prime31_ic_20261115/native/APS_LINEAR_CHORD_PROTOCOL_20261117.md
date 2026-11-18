# EXP-R31-APS-LINCHORD — prospective protocol (2026-11-17 UTC)

**Status at freeze: planned, not measured. [pcc:unverified]** Freeze this specification *before* changing any Rust solver implementation or seeing this experiment's timings. This is a development experiment on the existing public ordinary cofactor-one prime-field ECDLP instance, **not** a new asymptotic or non-generic attack.

## Source, hypothesis, exact arithmetic

Reuse `research/prime31_ic_20261115/native/examples/prime31_aps_one_relation.rs` (and its existing `native/lib.rs`, Cargo.lock, public point walks, labels, packed third-list x lookup and complete unknown-Q recovery). Curve `p=2147483647, a=847514846, b=2087444724, #E=n=2147446831, h=1, G=(625037715,2100627251)`; do not substitute a weak curve.

For two public *different-x* labelled points `P=(x,u)`, `T=(z,v)` on the same Weierstrass curve, the unordered finite S3 third-coordinate roots are

```
r_plus  = ((u+v)/(x-z))^2 - x - z  (mod p)
r_minus = ((u-v)/(x-z))^2 - x - z  (mod p).
```

The incumbent already uses `(u +/- v)^2 / (x-z)^2`, paid via one batched *squared-denominator* inversion per first-list row; the candidate instead batch-inverts unsquared differences and squares the two scaled numerators. This avoids one finite-field denominator square per pair, retaining the exact same S3 relation support. The identity is classical chord/tangent arithmetic. This is a **generic-pair-enumeration engineering improvement to the published APS one-relation index-calculus method**, not novel non-generic extraction. The global x-unique source excludes x=z across partitions; reject x=z rather than silently ignore tangent/infinity fibres.

## Prespecified evaluation

- Candidate is a compile-time-specialized `LINEAR=true` option in the existing Rust factored S3 oracle. `LINEAR=false` must preserve the incumbent semantics; only denominator selection and two root evaluations change. No algorithm change, source distribution change, altered target distribution, precomputation shared across public targets, or hidden target scalars in solver.
- Fresh deterministic caller-only target seed `0x4348_4f52_444c_3234` (CHORDL24), no reuse of previous `0x434f_4d50_4150_5331` composed panel. Exactly 24 distinct public `Q=[k]G`, with `k>1`, and `k` used only for separate post-solve replay. Same input `Q` in all arms.
- Five complete cold arms, per target in rotating order: (A) incumbent window4/public-y/packed APS, (B) candidate linear-chord APS, (B') candidate repeat, (C) existing unfolded rho32, (D) existing unfolded rho64. Existing profile `[384,384,16384]`, 6 rounds maximum for each APS call, existing rho seeds with `0x4348_4f52_4452_3234 XOR i*0xd1b5_4a32_d192_ed03`, 2,000,000-step rho cap. Every unsuccessful attempt and every setup/verification call is charged to complete cold runtime.
- Hard-correctness gates: 24/24 logarithms on each APS and rho arm (120/120), explicit `[log]G=Q` and caller-known-answer comparison, same round count, labelled seed, duplicate/identity counter, signed relation, third-index and pair-search count for A/B/B'. Both root families sorted/deduplicated. Reject discrepant or missing outcomes, and retain failures.
- Candidate development speed gate: B/A total cold wall `<=0.95`, B faster on >=18/24 paired public targets, and |B/B' - 1| <=5%. Report each constituent raw observation and output even if the gate fails. This is not a statistical guarantee. Future decisive claim requires independent cross-host evaluation, WZ rho and external PKM/APS/mature IC frontier comparisons, independent arithmetic replay and signed PCC; neither unfolded comparator is assumed optimal.
- Record per arm source setup, S3 oracle time, pair count, field inversion count, public square roots, full cold totals, AA drift, and immutable source commit SHA. Use GitHub Actions Rust 1.100.0, `--locked --release`, scoped native tests, source-specific push workflow, raw JSON artifact. No Python ECDLP code or paid external hardware.

## Prior-work/negative-lead receipt

Read existing `APS_S3_BUFFER_ABBA_RESULT_20261117.md` (reuse scratch only was 1.00167x *slower* in an ABBA test, PR #4484); `APS_COMPOSED_WIN4_PUBLICY_24Q_RESULT_20261116.md` (74.434ms versus rho32 79.833ms, confidence interval spans parity, PR #4454); merged #4391, #4451 and #4466 (RR9 predicates and signed norm without nonenumerative Q-to-f extraction); #4228/#4349 selected-root/quotient; #4373 censored F4; crypto-autoresearcher #3301/#3305 negative-ledger/errata; crypto #4108/#4363; cryptanalysis #1536; WZ work #4468 blocked by unrelated pinned `src/cryptanalysis/sat.rs` parse defects. Literature: Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), DOI 10.1016/j.ffa.2018.01.009; Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1. These papers establish prime-field IC prior art; no novelty claim. Research KB tool unavailable in this runtime; GitHub and publisher-primary fallback only.

**Separate novel-algorithm gate:** A real non-generic improvement requires target-conditioned RR9/S9 polynomial extraction from **public R** without explicit support enumeration; first B4/B8 solution-fibre equality (including no-solution controls), then full B31 unknown scalar plus priced rootfinding. This chord proposal does not substitute for that gate.
