# Prime31: same-runner 1024 unknown logs — reusable mixed-base IC versus exact original native rho32

**Run date:** 2026-11-17 UTC. **Status: complete original-curve Rust solver win against two explicitly tested native rho32 controls, only at the *multi-target*, same-runner, producer-development scope**. This does **not** demonstrate a novel non-generic algorithm, asymptotic improvement, or superiority to the strongest tuned external WZ / shared-precomputed generic BSGS / best mature external index calculus. [pcc:unverified], no independently signed PCC.

## Decisive source-specific verification

- [GitHub Actions #44025539872](https://github.com/isogeny-labs/crypto/actions/runs/44025539872) **SUCCESS**, exact source SHA `533c6fd40d3bf36e3fb559255128f317e70d1aef`, pinned native Rust 1.100.0. The unchanged native `prime31_aps_one_relation.rs` rho/Jacobian arithmetic was reused **byte-for-byte**; workflow only renames its old unused main, then appends the [new wrapper fragment](prime31_amortized_rho_1024q_suffix.rsfrag). It executes the [existing Rust reusable IC](../native/examples/prime31_mixed_base_amortized_ic.rs) on 1024 public Q on the same GitHub runner; both rho arms use **those identical 1024 public Q**, exact original Rust affine/Jacobian code, 2m step cap and 512-step watchdog.
- **40/40 Rust library tests, 3/3 new IC example tests; all 1024/1024 IC, all 1024/1024 unfolded rho32, all 1024/1024 folded negation rho32; 64/64 unfolded rho32 A/A**. No capped or incomplete target in any measured arm.
- [Raw full matched results / original signed relations artifact #14302284807](https://github.com/isogeny-labs/crypto/actions/runs/44025539872/artifacts/14302284807). GitHub ZIP digest **`sha256:a720f5f0533d384ee7526455453fae5e5dfb39e129914c924a2e3649ca098f93`**, independently recomputed from downloaded exact artifact. [Frozen comparison protocol](MIXED_AMORTIZED_MATCHED_RHO_PROTOCOL_20261117.md) committed **before** new comparator code and timings.

## Fully charged single-runner wall-time comparison, same 1024 Q

| Component | Complete cases | Charged cold total |
|---|---:|---:|
| Mixed-base index calculus: **setup**, two tables + 61 base logs + point checks | 61/61 basis | **1655.699 ms** |
| Mixed-base index calculus: all **target descents**, including point checks | 1024/1024 | **1029.098 ms** |
| **Index calculus full setup + all 1024 solved logs** | **1024/1024** | **2684.798 ms** |
| Existing optimized-batch 32-lane **unfolded rho**, original Rust | **1024/1024** | **3017.608 ms** |
| Existing optimized-batch 32-lane **negation folded rho**, original Rust | **1024/1024** | **5625.174 ms** |

- Full-batch ratio **IC / fastest of these two native rho arms = 0.8897104**, or **11.03% lower total wall time**, charging all table construction, linear algebra, failed work (none), source original-curve verification and every target. 1024 independent logarithms were actually completed, not just a relation oracle.
- IC / folded rho32 = 0.4772826; the native folded arm is slower on these inputs, **not** assumed to be the fastest possible negation rho implementation. The comparison to unfolded is the conservative one.
- Per-target descent only (excluding IC setup) beats unfolded rho32 on **913/1024** Q and folded on **979/1024** Q, but this statistic alone does **not** price setup.
- The old 24-Q source-specific IC run #44024919023 separately found 2763.095 ms setup+1024, demonstrating same order of magnitude; neither run is isolated or independently PCC signed. Its 2763 ms should not be substituted into this source-matched numerator.

### Independent original-curve arithmetic readback

The downloaded matched ZIP was independently parsed by a separate Python **auditor only**, not an alternative ECDLP solver. It reconstructed both original factor bases (including Dickson depth-six seed **3**) and recomputed with independent affine `F_p` arithmetic:

- **61/61 factor-base scalar points** `[ell_i]G=P_i`;
- **61/61 Q-independent setup signed group relations** and exact modular rank-row equations;
- **1024/1024** target signed 4+4 witnesses and public residual equations `aG+bQ=R`;
- **1024/1024** IC scalar-point checks, **1024/1024** unfolded rho scalar-point checks, **1024/1024** folded rho checks and **64/64** A/A rho checks;
- **1024** target residual attempts, **zero** misses and **zero** detected inconsistent rows. Every caller k was unique.

A **25,000-replicate paired-target bootstrap**, resampling the same 1024 Q with original charged **fixed setup** and all paired target solve times, gives:

| Complete-cost ratio | Point ratio | 95% paired-target percentile interval |
|---|---:|---:|
| IC / original native unfolded rho32 | 0.88971 | **[0.85818, 0.92276]** |
| IC / original native folded rho32 | 0.47728 | [0.45971, 0.49584] |

The rho32 64-case A/A repeat differed by **0.445%** in aggregate from first executions, a source-specific timing stability check. These percentile intervals account for *target sampling only*, **not** CI-host variability, CPU contention, multiple runs, or frozen-setup randomness.

## Mathematical/novelty interpretation

The exact algorithm is a full **index-calculus-derived multi-target solver**: solve 61 factor-base logarithms using verified original-curve relations, then for each unknown public `Q` recover `k=b^{-1}(Σ c_i ell_i-a) mod n` from one certified signed mixed factor-base relation `aG+bQ=Σc_iP_i`. The existing half-table oracle explicitly enumerates generic sums and is **generic meet-in-the-middle-equivalent**. Therefore:

1. **Yes**, this experiment measured a **cheaper fully complete prime-field ECDLP *batch*** relative to the two **specific native rho32 arms measured on the same 1024 Q** at the development-run scope.
2. **No**, it has **not** established lower asymptotic complexity, a genuinely non-generic polynomial extractor, faster single cold ECDLP, superiority over **optimized rho as a class**, or a shared-precomputation BSGS frontier. It requires **1655.699 ms setup for the first target**, far more than a cold rho log. The gain appears only after amortizing across many Q on one curve and at some additional memory cost (peak RSS not captured).
3. Before a frontier claim, test precomputed BSGS (which also amortizes generically), isolated CPU same-Q A/B crossover and independent signed PCC; then implement genuinely **nonenumerative RR9/S9 extraction** from public R, revisiting B4/B8 exact original-ideal solution fibres without support enumeration.

## Prior-lead audit and reproducibility

Prior #4503/#4506 reused rather than rewritten; #4505 mixed formal lattice support is not a runtime proof; #4454/#4486 native APS/rho controls, #4466 RR9 given-leaf certificates, #4228/#4349 quotient/subresultants; autoresearcher #3301/#3305 SA campaigns (Vieta, m6/m8 F4, norm-lattice and ancillary negatives) checked to avoid duplicates. Published PKM 2016 DOI 10.1007/978-3-662-49387-8_1 and APS 2018 DOI 10.1016/j.ffa.2018.01.009 remain prior art. The separately pinned root WZ source #43994897230 is still a **build failure before solving**, and cannot be compared. No paid compute, no platform leaderboard promotion, no merge, and no independent signed PCC asserted.

**Next priority:** precomputed generic BSGS head-to-head on same 1024 public Q, then strongest corrected WZ / external IC, followed by actual RR9/S9 nonenumerative unknown-R polynomial extraction. The amortized native-rho advantage is a valid **measured restricted benchmark**, not the stronger cryptanalytic breakthrough target.
