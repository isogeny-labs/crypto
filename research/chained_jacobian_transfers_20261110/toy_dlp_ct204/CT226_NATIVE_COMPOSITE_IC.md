# CT226 — native composite-order genus-4 toy index-calculus cold integration

2026-11-19 UTC · [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715) · **[pcc:unverified]**

## Scope and hypothesis

This is the first *native Rust integration test* in this CT204 lane to execute an anchor-free index-calculus relation solve for the **full composite-order** genus-4 Jacobian, with the elliptic source subgroup preserved inside it. It is an implementation/test of the narrow F101 toy construction rather than a new general solver or a qualified prime-field attack. It does not replace the original authenticated 695/701-record Jacobian fixture.

Model: `C/F101: y²=u⁹+u³+46 → E/F101: y²=x³+x+46`, source `G=(2,37)` of prime order 109, public known-answer `Q=(64,19)=[37]G` (37 used for post-solve validation only). CT221 established `#J_C(F101)=115,593,846=109·102·10,397`; CT225 excluded maps into only E0 or B, not index calculus on the full J.

**Hypothesis to test:** The producer's anchor-free CT219 relation schedule can be independently reproduced with the repository's native `FpPoly` and `MumfordDivisorP`/Cantor library, and its *fully cold* producer wall time should be compared to a fresh direct source-elliptic solver. Expected CT219 deterministic finite witness: 54 signed rational factor-base variables, 1,421 trial candidates, 62 smooth degree-4 relations, rank 54/54. A discrepancy is a test failure, not a reason to patch the baseline or redefine a fixture.

## What is implemented

[Test source](../../../../tests/ct226_native_genus4_transferred_ic.rs) `tests/ct226_native_genus4_transferred_ic.rs`, checked in at `eacafdbce6d66f6da12381af3b817f45c2b73fcc`:

- Actual prime-field genus-4 curve validation, reduced Mumford pullbacks for generator and target, Cantor compositions and subgroup-order checks in native Rust, not Python emulation.
- Deterministic F101 enumeration of the signed 54-point factor base and lexicographic 2/3/4-point offsets from the first eight curve points. Offset-substituted relation equations over F109: `sum(log of degree-4 support) - sum(log of offset support) = 3k`. The offset term cannot be silently dropped.
- Modular elimination, rank test, factor-base log recovery, per-row residual checks and independent source EC multiplication checks; target coset search and a public `[recovered]G==Q` test.
- Three *alternating-order*, fresh setup/relations/linear-algebra/recovery/verification elapsed-time pairs against freshly built native direct elliptic BSGS. Includes native Cantor addition counters and polynomial field-add/mul/square/inversion tallies; emits three structured `CT226_NATIVE_PAIR_JSON` records.
- Fail-closed expectations for `(rank, relations, trials)=(54,62,1421)`. The code does **not** use a known factor-base generator-log anchor, and does **not** solve an ambient order-115593846 DLP; it solves a target in the specified source 109-primary subgroup of the full composite-order Jacobian.

The benchmark is an intentionally scoped *lower bound on the cost* of a fully qualified transferred solver: the current timed candidate does not charge a general divisor norm projector, a trusted-vs-untrusted memory audit, or authenticated 695/701-relation evaluation. The direct comparator is a simple inline BSGS reference, **not the strongest matched optimized rho/BSGS/IC baseline**. The target and generator are a public known-answer test, not blinded attack targets. Counting both full cold construction and correctness checks makes this a genuine finite toy regression but **cannot qualify a performance advantage**, even if a reported timing ratio were favorable. Peak resident memory, isolation, cross-host CI, PCC and uncertainty bounds are unmeasured.

## Reproducibility and CI

```sh
cargo test --locked --release --test ct226_native_genus4_transferred_ic -- --nocapture
```

[Boun-ded exact-head workflow](https://github.com/isogeny-labs/crypto/blob/research/ct204-odd-cover-toy-dlp-20261118/.github/workflows/ct205-toy-transfer.yml) updated in `95d374fe1e31214b5eff586866404a1a3dc53216`. At time of first report: [CT226 hosted run #44467973031](https://github.com/isogeny-labs/crypto/actions/runs/44467973031), source `95d374fe`: **independent-field-oracle job completed success**, replaying CT225 and all prior mathematical controls. The **native Rust test job is still in progress**, so no native rank/timing/test verdict exists yet. Later commits here do not by themselves certify the new head. Local container lacks an installed Rust compiler; no unexecuted test is described as having passed.

## Scientific and resource disposition

The earlier Python-only complete F101 CT219 experiment was a ~10,542× direct BSGS slowdown and must remain a negative. CT215 native genus-2 BSGS transfer showed ~61.7× slowdown. This work makes native same-host composite-J IC testable with physical clocks and native primitive counters but **does not yet establish either a native performance ratio or an advantage**. If the native run fails, fix the concrete defect before using its numbers; if it passes and is slower, retain the negative and switch to a higher-expected-value route.

Original 695/701-record genuine model data are still S3-403/authorization and approved-EKS-budget blocked, preventing required `C169`, `C13a`, `C13b`, `D`, norm-point reuse and strongest end-to-end rho/BSGS/IC comparisons. Do not synthesize a replacement and call it original. No paid EKS/Spot compute, protected merge, PCC promotion or speedup is authorized or claimed.

**Next measurable gate:** inspect exact-head CT226 native CI and three raw pairs; if passed, independently recompute phase sums and ratios and compare native output to the CT219 frozen counts. Then add qualified same-host native optimized-rho/strong-IC and RSS baselines under explicit bounded cost; prioritize original norm-point reuse only upon authenticated data access and budget approval.
