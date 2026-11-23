# Prime31 exact WZ rho: immutable-source dependency diagnosis (2026-11-16)

Latest exact-source [Actions run 43981242805](https://github.com/isogeny-labs/crypto/actions/runs/43981242805) failed **before all solver arms executed**. The test step tried to build the root Rust package from the immutable merged #4454 full checkout. Its `crates/hashed-ecdl-tensor/src/curve.rs` and `partial.rs` contain eight `crate::metrics::{group,scalar,phase}` references; the same pinned checkout includes real `src/metrics.rs` (8,121 bytes) but `src/lib.rs` fails to declare it. Rust E0433: `could not find metrics in the crate root` (eight locations). This is a compile defect in a **non-benchmark dependency**, not an observed slowdown, failed discrete log, or result from Wiener–Zuccherato rho.

**Scoped repair**: the benchmark workflow, after copying the new WZ runner into the immutable full checkout, verifies that the real hashed metrics module exists and uses a single line to expose **the original** `pub mod metrics;` in its own transient checkout only, if missing. It does **not stub metric checks or modify the APS or WZ arithmetic**. Pin remains `1df714645aa683cf975bc76fcacb9842cb4780fd`; source-specific run required. New run may reveal further preexisting root build defects; preserve all.

Existing scientific limitations: current APS one-relation S3 pair oracle is generic MITM-equivalent, not genuinely non-generic; new root WZ uses counted affine arithmetic, while existing 32/64-lane native rho uses batched Jacobian arithmetic; even a WZ loss would not establish strongest optimized-rho superiority. Prior producer 24-Q fully cold APS 74.433895 ms vs native unfolded rho32 79.833260 ms has uncertainty crossing parity. The current WZ check has **zero** new complete recoveries and **zero** new measured timings. [pcc:unverified]

Prior evidence: merged crypto #4391/#4454 and #4460/#4462, native Rust source + pinned [protocol](../../research/prime31_ic_20261115/native/WZ_FRONTIER_PROTOCOL_20261116.md), [failed CI job](https://github.com/isogeny-labs/crypto/actions/runs/43981242805/jobs/133697961251). The 2026-11-16 main branch has missing native modules and major unrelated workflow failures; do not blindly restore large trees or infer algorithm regressions.

Next: require successful Rust compile and 24/24 for all three WZ arms before any comparison, then independently replay all returned scalars and paired bootstrap with full cold costs; if root source remains damaged, isolate just the actual WZ functions in a minimal reusable Rust source-only crate *without editing their arithmetic*.


## 2026-11-17 follow-up: exact next blocking syntax defect (preregistered repair)

Source-specific [Actions #43987323713](https://github.com/isogeny-labs/crypto/actions/runs/43987323713), job [133718727051](https://github.com/isogeny-labs/crypto/actions/runs/43987323713/job/133718727051), failed **during root Rust compilation**, before either APS or WZ arm. The preceding `hashed-ecdl-tensor` module E0433 failure was removed: it compiled with only a preexisting unused-function warning, so the prior patch did reach the intended dependency. The next blocking error is `src/cryptanalysis/koblitz_bench.rs:3427:3` unclosed delimiter for `mod tests`. The immutable pinned file at `1df714645aa683cf975bc76fcacb9842cb4780fd` has this exact text following the `s3_link_roots_match_the_field_scan` test:

```rust
        assert!(two > 0 && one > 0 && none > 0, "{two} {one} {none}");
    /// The Artin–Schreier chain walk counts exactly what the scan counts.
    #[test]
    fn fast_chained_count_matches_the_reference() {
```

There is **one missing closing brace** between that assertion and the following test's documentation comment. Proposed strictly scoped fix: in the transient pinned build checkout only, assert the full exact anchor occurs once and insert `    }` plus a blank line. If the anchor has changed, fail closed. No benchmark code, walk logic, APS arithmetic, target input or previous fixed source pin may change. Source-specific compilation and full 24-Q correctness/timing must be rerun; the last two failed runs establish **zero** measured WZ timings. If another root-file parse defect appears, report it independently rather than treating build failures as failed DLPs. [pcc:unverified]
