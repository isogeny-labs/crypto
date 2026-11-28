# CT327 — bounded F103 negation-canonical rho control (prospective)

2026-11-23 UTC. **[pcc:unverified]**. This protocol is committed before the new Rust implementation or new timing. Scope is the original fixed public Q73 toy transferred IC from CT238/241, **not** the 695/701 authenticated norm fixture.

## Ownership and immutable input

Use the existing `tests/ct238_native_cubic_norm_compression.rs` at original Git blob `4c8f7a03c7859483f329a5d819747842d2d94a5f`; append **only** a nested `ct327` test module within the existing `ct241` module. Do not edit `ct241::cold`, its frozen output schema, its pair measurements, the CT241 15-row panel blob `61d8aa3f86ef5672faf09c9fc638e5fe9594f429`, the CT326 auditor, original 695/701 fixture, cryptanalysis #1523 or norm-reuse owner code. Parent state: merged root [#3877](https://github.com/isogeny-labs/crypto/pull/3877), merged [#4215](https://github.com/isogeny-labs/crypto/pull/4215), [#4715](https://github.com/isogeny-labs/crypto/pull/4715), [#4964](https://github.com/isogeny-labs/crypto/pull/4964), and the persistent [queue](https://github.com/isogeny-labs/crypto/issues/3904).

The source `E/F103:y²=x³+x+24`, `r=109`, `G=(1,51)`, `Q73=(42,71)`, and genus-four `C:v²=u⁹+u³+24` remain unchanged.

## New public, bounded control

- A new Rust **diagnostic** Pollard rho with negation canonicalization (`y<=p-y`), 16 explicit public linear-combination jumps per restart, a finite table of canonical group points, deterministic seeded restarts and all walks capped at 64 restarts × 4r iterations. Before using a collision, solve `x=(a-a_0)/(b_0-b) mod 109` only for a nonzero denominator. Every returned scalar must independently reproduce the public Q. Count source elliptic additions (including jump setup, scalar initialization and final verification), negation/canonicalization, walks, restart count and resident table slots; count and time setup, walk and verification without excluding retries.
- Public-target correctness panel: all 109 `Q=[k]G`, k=0..108, independently comparing `[x]G=Q`. The scalars only appear as *post-solve* known-answer controls, never an oracle during rho. Reject wrong scalars and a malformed subgroup target. Mutate a deliberately forged recovered scalar to verify the independent final check rejects it. No random seed is obtained from a target log.
- A genuinely paired *same-process* three-round panel for **B22** using the unchanged `ct241::cold(22)`, unchanged `direct_bsgs`, and new rho on **the same public Q73**, with round-dependent order `IC_RHO_BSGS`, `BSGS_IC_RHO`, `RHO_BSGS_IC`. Each run cold-starts the respective arm; no cache precomputed for rho between rounds. Report all three baseline and candidate nanoseconds and direct ratios; all relation/L.A./setup/verification timings are those of the unchanged original cold implementation. If any solver fails to return and verify 73, fail the entire run. Record process-wide VmHWM as a nonisolated diagnostic, not per-arm peak RSS.
- No universal optimized-rho supremacy assertion: this simple toy diagnostic may be slower than the mature project `negation_remeasure::rho_full` or other rho implementations. Keep direct BSGS as a required independent reference; strongest matched rho and prime-field IC at the *target sizes* remain an admission gate.
- No timed run may compete with another job reservation or exceed shared CPU/budget capacity; request only one sequential, bounded, existing GitHub-hosted test. **No EKS/S3/VLT1 paid/privileged work**. If native CI is unavailable or shared root fails, mark source as unexecuted rather than reporting a timing result.

## Planned commands and acceptance

```sh
cargo test --locked --release --test ct238_native_cubic_norm_compression ct327 -- --test-threads=1 --nocapture
```

Original root Cargo build may be broken by concurrent changes. Capture exact head SHA, build status and full failed logs. Require compiler success, complete 109/109 rho correctness, negative controls, exactly three Q73 triplets with correct scalar and all phases billed. CPU/RSS/CI and PCC remain provisional unless separately authenticated; do not infer independent verification from the producer's tests.

A resulting cold ratio <1 is a negative result for this exact fixed instance, not a theorem about transfer feasibility or larger p. A ratio >1 is only a provisional finite finding until independently replayed with matched optimized rho, BSGS, incumbent IC, confidence interval, full cost, memory and signed PCC. Failure triggers the next non-overlapping transfer/collector lead rather than repeating CT241.

## Mathematical boundaries

For a degree-d cover `c:C→E`, `c_*c^*=[d]` gives injectivity on a prime r-subgroup if gcd(r,d)=1. The F103 degree-3 cover has r=109. The nonzero transferred DLP remains the E-isotypic component; the presence of a genus-four Jacobian alone does not lower generic rho's \(\sqrt{r}\) work. CT224/225 already exclude complementary-factor geometric maps **for their distinct F101 model**, not this F103 instance. Cross-characteristic algebraic transfer is not asserted: unital field homomorphisms between fields of distinct characteristics do not exist; this leaves algorithms outside algebraic-variety morphisms unclassified.

No new theorem, native benchmark, source-file compilation or performance advantage is claimed by this preregistration. [pcc:unverified]
