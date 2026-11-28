# Prime31 S9 eight-summand orientation-fibre audit and CI repair (2026-11-16)

Status: exact combinatorial derivation and Rust source checked in; new source-specific execution **not yet admitted**. [pcc:unverified]

## What changed

1. **Fixed a real regression in the exhaustive B31 multiplicity harness.** Actions [43957151566](https://github.com/isogeny-labs/crypto/actions/runs/43957151566) finished **failure**, not pending: 40/40 reused native library tests passed, but `tests::b4_curve_roots_replay_and_zero_control` panicked with `index out of bounds: len is 8 but index is 8` at line 70. The implementation stores each short B4/B8 net signature as a *B31-wide* array and indexed a leaf for zero coefficients beyond the shorter base. Commit `d394d110beb2a5a03e27a6104b8f2e796d75c200` skips zero coefficients before touching the signed point slice. This is a unit-test indexing repair, **not** a mathematical counterexample or a successful benchmark. Retain the original failed run; require a successful exact-head rerun and exhaustive natural/negative replay before accepting a multiplicity distribution.
2. Added a separately preregistered **Rust-only x-polynomial/orientation fibre experiment**, using the project's `Arithmetic`, `Base::build`, and exact PRIME31 curve. Protocol: [S9_ORIENTATION_FIBRES_PROTOCOL_20261116.md](S9_ORIENTATION_FIBRES_PROTOCOL_20261116.md). Implementation: [prime31_s9_orientation_fibres.rs](examples/prime31_s9_orientation_fibres.rs). Workflow: [prime31-s9-orientation-fibres.yml](../../../.github/workflows/prime31-s9-orientation-fibres.yml). The executable independently checks two generating-function double counts; on the actual Dickson31 base it replays 256 sign choices of one fixed squarefree degree-eight x polynomial and nine sign choices of a fixed eighth-power polynomial, including cancellation to the identity. The Rust tests are **submitted but not yet reported green**; the exact combinatorial values below are derived separately.

## Exact arithmetic distinction — why it is a necessary gate

Fix `B` distinct rational x-coordinates with signed lifts `P_i`. An eight-leaf signed word has plus/minus counts `a_i,b_i>=0`, `sum(a_i+b_i)=8`. Its **net integer signature** is `d_i=a_i-b_i`, its x-only degree-eight polynomial has factor multiplicities `m_i=a_i+b_i`, and its group point is `Q=sum d_i P_i`.

For any fixed net signature `d` with `||d||_1=k in {0,2,4,6,8}`, write `m_i=|d_i|+2c_i` with `sum_i c_i=(8-k)/2=t`. Hence the exact number of **distinct x-multiset polynomials per net d** is `binom(B+t-1,t)`, irrespective of which coordinates support d. Conversely, for a fixed x polynomial with multiplicities m, the exact number of distinct **orientation net vectors** is `product_i(m_i+1)`. Neither quantity is the number of distinct group points: different d can share Q on an elliptic curve.

Both double counts must equal the exact number of unordered eight-token multisets over 2B signed leaves:

`sum_d binom(B+(8-||d||_1)/2-1, (8-||d||_1)/2) = sum_{m_1+...+m_B=8} product_i(m_i+1) = binom(2B+7,8).`

| B | Distinct net signatures, ||d||<=8 even | Degree-eight x-multiset polynomials | Oriented signed eight-token multisets | Mean orientations per x polynomial |
|---:|---:|---:|---:|---:|
| 4 | 2,241 | 165 | 6,435 | 39.000 |
| 8 | 187,137 | 6,435 | 490,314 | 76.195 |
| **31** | **5,654,193,417** | **48,903,492** | **8,361,453,672** | **170.979** |

For B31, a net signature of reduced length 8 has exactly one x-only f; reduced lengths 6/4/2/0 admit respectively **31 / 496 / 5,456 / 46,376** x-only f. In particular, the zero-net / identity branch has 46,376 distinct cancellation polynomials. The existing B31 point-sum study's mean **2.633 net signatures per group target** is *not* the number of x-polynomial roots or signed orientations; the exact relationships above prevent that mistake. The B4/B8 `d->Q` injectivity observed in the previous separate source-bound support census must not be extrapolated to B31.

A target-conditioned S9 algorithm must therefore **return and verify f AND its sign/ordinate branch against full Q**, reject repeated-root and infinity shortcuts that change the original solution set, and distinguish `Q` from `-Q` (they share x_Q). Its original-ideal equivalence check must not collapse 46,376 cancellation f at Q=O into one witness or mistake many oriented words for many independent relations. [pcc:unverified]

## Prior leads and performance gate

- **Published prime-field IC**: Petit--Kosters--Messeng (PKC 2016, DOI 10.1007/978-3-662-49387-8_1); Amadori--Pintore--Sala (FFA 2018, DOI 10.1016/j.ffa.2018.01.009). Existing APS S3 explicit pair scanning is a measured Rust engineering baseline but remains generic MITM-equivalent. Do not rebrand it as new non-generic complexity.
- **Current algebraic branch**: [crypto #4228](https://github.com/isogeny-labs/crypto/pull/4228) and [#4349](https://github.com/isogeny-labs/crypto/pull/4349) provide selected-root/quotient components, but not a completed B31 f-from-Q S9 extractor. Depth-six Dickson fibre is the *actual* factor base; the first-x31 alternative is only a matched control.
- **Negative searches already recorded**: SA-02 Vieta-conic, SA-14b tower F4, anchored m6 F4 capped with only <=3.7407% six-leaf natural support; prior chained m8 F4 censored. Do not repeat unchanged F4 parametrizations or count a censored call as nonrepresentability.
- **Strongest measured complete control on same 48-Q panel** (producer, unisolated [pcc:unverified]): APS 109.734280 ms, unfolded rho64 99.473873 ms; paired uncertainty overlaps 1. Repository's separately tuned WZ rho and mature external PKM/APS/F4/CAS remain missing. The new orientation experiment is a **preprocessing correctness diagnostic**, not a full ECDLP solve or speed gain.

## Next concrete scientific experiment

Pass the actual **Dickson B4/B8 blinded constructed-positive Q** through the existing #4228/#4349 selected-root interface. Return `(f,signs)` without accessing a 4A table, scalar labels, or enumerating all x multisets. Independently compare the complete f family against the formula above for targets with norm 8,6,4,2,0; use exact B4/B8 negative/infinity controls and the two *retrospective* Dickson B31 negatives `(854640086,1374782344)` and `(1146958692,202234633)`. Require original-ideal membership + complete sign replay before promoting. Once extraction exists, obtain rank32 full ECDLP recoveries over p=2^31-1 and time **complete cold** calls against tuned WZ rho and the external index-calculus frontier. A fixed-f sign check, generic 4+4 table, or a large orientation count is not this extractor.

**Evidence limits:** GitHub connector access only, no local rustc/cargo, EKS or Research-KB MCP/wrapper; repo AGENTS.md read; prior source, logs and published PKM/APS records reviewed. New Rust source and workflow are checked in; independent PCC and exact-head CI outstanding. Do not edit leaderboard or claim a faster complete solver.
