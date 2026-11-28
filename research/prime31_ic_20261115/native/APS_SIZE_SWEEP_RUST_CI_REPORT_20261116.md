# Prime31 complete Rust APS size sweep and confirmatory next gate (2026-11-16)

**Standing:** source-exact development GitHub Actions, [pcc:unverified], not an independently replayed/isolated comparison and not a non-generic ECDLP attack. All implementation and measurements are Rust. Original run: [Actions 43792410607](https://github.com/isogeny-labs/crypto/actions/runs/43792410607), job 133053626486, source `72f5c9557cf2d32256183fbda5e127d1714a8196` (merged as [#4374](https://github.com/isogeny-labs/crypto/pull/4374)). Raw per-target output [APS_SIZE_SWEEP_RUST_CI_20261116.json](APS_SIZE_SWEEP_RUST_CI_20261116.json).

## Complete same-source development result

The published APS one-relation Rust solver receives only a public target Q, constructs random labelled `aG+bQ` points in three partitions, discovers an S3 zero via finite field roots, recovers the unknown logarithm from a signed relation and independently replays it. This particular implementation enumerates pair-root support and is **generic-MITM equivalent**, despite being an index-calculus formulation. The 12 heldout k values only enter a caller-side verifier **after** the solve. Each arm used the identical 12 public Q but changed T and registered maximum attempts. The source contains no hidden generic fallback.

| Points per partition T | Heldout logs correct | Charged total cold wall | Average per Q | Quadratic root queries | Total attempts |
|---:|---:|---:|---:|---:|---:|
| 768 | 12/12 | 3,030.833 ms | 252.569 ms | 6,645,504 | 18 |
| **1024** | **12/12** | **2,314.613 ms** | **192.884 ms** | 4,707,328 | 12 |
| 1536 (previous control) | 12/12 | 2,734.196 ms | 227.850 ms | 4,368,384 | 12 |
| 2048 | 12/12 | 2,490.056 ms | 207.505 ms | 1,839,104 | 12 |
| 3072 | 12/12 | 3,421.787 ms | 285.149 ms | 1,668,096 | 12 |

**The T=1024 full cold total was 1.1813× lower than T=1536 in this one ordered same-runner observation**, equivalent to 15.35% less paid wall time across these twelve complete solves. Crucial limitation: T=1024 was faster on **7/12 individual targets**, slower on five. Sequential fixed configuration order, random target difficulty, host interference and limited sample size mean the aggregate advantage is **not yet a validated or robust best-size result**. T=2048 reduces root queries substantially but pays more for constructing the 3T source points; T=768 incurs six extra rounds in this cohort. No stage-only timing is substituted for completed scalar recovery. This is an engineering **complete IC-control** tuning observation, not a new finite-field complexity exponent or fastest external PKM/APS comparison.

### Separate rational-quotient algebraic result

The *same source-exact Rust CI* also passed all 40 shared library tests and the m8 rational quotient example, including the **new** source assertion `deg gcd(K,K')=0` for full B31 `K(U)=prod_(i<=j) S3(x_i,x_j,U)` of degree 961, together with 465 off-diagonal quadratics, 31 diagonal linear factors and 961 signed affine x root checks. The degree-zero gcd confirms the finite coordinate support polynomial is squarefree on this exact ordinary cofactor-one p=2^31-1 curve. Since all roots are rational, the algebra is split and reduced. This is a mathematically useful **certified-by-reexecuted-arithmetic fixture property** but not a target-conditioned m8 solution or shortcut: explicitly enumerating these pair roots is generic MITM.

## New confirmatory Rust experiment (source checked in; result pending)

Flag `--size-confirm` uses 24 new deterministic, previously unused public Q values; each receives **three** complete cold runs: T=1024, T=1536, and a same-T=1536 A/A repeat. Alternating call order `A-B-B` and `B-B-A` controls some order dependence; exact same public Q and deterministic source labels are used per arm. Every miss/retry, source construction, signed extraction and verifier is charged. The known target scalar is never passed to either solver; an invalid completed scalar is a hard test failure. Max rounds 8 for T1024 and 3 for T1536, as in the frozen exploratory sweep. A/A provides a same-host noise diagnostic, **not** runner isolation, an independently signed PCC or a pre-agreed statistical threshold.

Reproduce from repository root:

    cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation
    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation -- --size-confirm

The existing Rust GitHub Actions prime31 workflow includes this new step. This report does not claim it has run or that the T=1024 win persists.

## Lead assessment and stop/go

Primary prior work: [PKM 2016](https://doi.org/10.1007/978-3-662-49387-8_1) and [APS 2018](https://doi.org/10.1016/j.ffa.2018.01.009), as well as #4188 m4 algebraic quotient, merged #4373 m8 S3 chain (censored at F4 cap), merged #4228 and open [#4349](https://github.com/isogeny-labs/crypto/pull/4349) selected-root/compact certificates. #4349's latest exploratory L0 measured compact **slower than optimized native F4** on the registered six-variable pilot, despite 2.43–2.99x full-cost gain over *expanded certificates*. These results cannot be extrapolated to wide-prime m8.

- **Engineering control:** confirm or falsify T1024 savings on fresh paired heldouts with A/A, record complete costs regardless of result, and compare mature external APS/PKM implementations before any fastest claim.
- **Non-generic mathematical goal:** develop a complete target-conditioned selected-root/subresultant oracle for the four K-root / S5 identity, without enumerating 961 roots or 961^4 coordinate combinations. Start with small-B original-ideal/root-equivalence, exceptional infinity/cancellation and signed full-point verification. Require actual natural B31 extracted relations and independent rank or one-relation recovery before comparing completed full cold time.
- **Discard as new IC advance:** treating split K roots as a group-sum lookup, B31 m4 factorized stage alone, swapping S4 for chained S3 with identical censored ideal, or quoting uncompleted F4 internal milliseconds against a completed scalar solve.

The research-KB/Conductor/EKS route was unavailable here; GitHub exact sources and published primary literature were used as bounded fallback. No new leaderboard row or independent security claim, and no paid compute.


## Fresh 24-target confirmatory result — **T=1024 rejected as default**

[Exact-source Rust Actions 43793061150](https://github.com/isogeny-labs/crypto/actions/runs/43793061150), job 133055634090, source `baba1a495bcbe5807a98098ff0dcf65133459dde`, completed **all 72 complete cold unknown-log solves** (24 new Q × T1024, T1536, and repeated T1536 A/A). Every returned scalar matched the caller's withheld known answer, and every configured run charged all internal setup and attempts. Individual caller-side known scalars, attempt counts, ordered calls and full raw stdout are frozen in [APS_SIZE_CONFIRM_RUST_CI_20261116.json](APS_SIZE_CONFIRM_RUST_CI_20261116.json).

| Rust arm on same 24 fresh public Q | Correct unknown logs | Charged total cold wall | Average / Q |
|---|---:|---:|---:|
| T=1024, max 8 rounds | 24/24 | **5,175.152 ms** | 215.631 ms |
| **T=1536**, max 3 rounds | **24/24** | **3,920.747 ms** | **163.365 ms** |
| T=1536 A/A repeat | 24/24 | 3,913.221 ms | 163.051 ms |

**The fresh result reverses the initial twelve-target tuning observation:** T1536 was 1.31994× faster *in total cold cost* than T1024, and faster for **15 of 24 individual targets**, with 9 favoring T1024. The repeat T1536 total differed by about **0.192%** from its first pass (ratio 0.998081), supporting that this aggregate reversal is not solely a gross fixed-order A/A disturbance. However that observation does not eliminate process/host variability, input heterogeneity or seed dependence. Alternating A-B-B vs B-B-A order reduces one source of bias but is still a single nonisolated GitHub development runner, [pcc:unverified]. The experiment did **not** compare a mature external APS implementation or establish a security advance.

**Decision / stop-go:** preserve **T1536 as the default** and do not promote the exploratory T1024 aggregate advantage. This is a measured negative for the proposed partition-size change and useful calibration for future complete algebraic index-calculus controls. No benchmark leaderboard update is justified as a validated large speedup. Next genuine non-generic scientific gate remains a target-conditioned, original-ideal-certified S5/K quotient oracle on ordinary prime31 without expanding a generic support catalogue. No completed m8 ECDLP or superiority over the published prime-field IC frontier is claimed.
