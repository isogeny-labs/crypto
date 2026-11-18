# Prime31: public-ordinate S3 reuse — exact-source completed Rust result

2026-11-16 UTC. Status: **120/120 same-host completed calls, 72/72 APS unknown-log recoveries, 18,432 modular square-root exponentiations eliminated, 5%-whole-call engineering gate FAILED**. [pcc:unverified]

## Prospective design and source-specific evidence

The protocol [APS_PUBLIC_ORDINATE_PROTOCOL_20261116.md](APS_PUBLIC_ORDINATE_PROTOCOL_20261116.md) was frozen at commit `62814d37cc5013c41c51fdcc7405618f8382b9d0` **before** the opt-in implementation `4bef06625135b14547650d4ae760f720756d574c` or the workflow edit `b65013f6ffe5324d4a8d87698ef00de4848f0b18`. Reused the same existing Rust `prime31_aps_one_relation.rs` and field arithmetic, no new solver and no Python relation discovery. The candidate obtains the **already available validated rational ordinate** `y` for each affine public labelled walk point instead of exponentiating a new square root of `x³+ax+b`. It verifies `y²=f(x)` for every input, keeps both exact S3 quadratic roots and their sorted order, and leaves signed candidate replay unchanged. Algebraically the roots `((u±v)/(x-z))²-x-z` depend only on the **unordered signs** of `u` and `v`; choosing source `y` instead of an arbitrary square root swaps them at most. This is an engineering gain to **published APS (2018) one-relation index calculus**, whose explicit S3 pair enumeration remains generic meet-in-the-middle equivalent.

[Successful GitHub Actions #43971747803](https://github.com/isogeny-labs/crypto/actions/runs/43971747803), job `133665367717`, source head `b65013f6ffe5324d4a8d87698ef00de4848f0b18` (Rust 1.100 release, Ubuntu 24.04, producer CI) passed 40/40 native library tests, 2/2 joint/window tests, **1/1 focused new public-y/S3 equality test**, and **both** five-arm 24-Q complete-cold studies. The [raw second-panel ZIP artifact, ID 14280255418](https://github.com/isogeny-labs/crypto/actions/runs/43971747803/artifacts/14280255418) contains all 120 public-target/solver records including failed-round and signed-witness details. The prior first-panel ZIP in the same run is artifact 14279625695. All 120/120 second-panel calls completed and recovered the exact caller-held scalar. The duplicate old window4 panel still passed its own checks but is a different CI host/run than the previous exact-source 1.2445% panel; don't substitute cross-panel ratios.

## Five-arm matched fully cold cost, 24 independent public Q

Exact curve: `p=2^31-1=2147483647`, `E:y²=x³+847514846x+2087444724`, prime `n=2147446831`, cofactor one, actual repository generator. Fixed previously unseen `target_seed=59334150534f5231`; every Q is public, scalar k held only by the verifier. APS partitions `[384,384,16384]`, six-round cap, rootfinding, cold setup, retries, signed verification and replay paid.

| Complete existing Rust solver | Correct public unknown logs | Total cold wall |
|---|---:|---:|
| Current window4 APS, independent y=sqrt(f(x)) | 24/24 | **51.643590 ms** |
| **Window4 APS, checked public ordinate y** | **24/24** | **50.551033 ms** |
| New window4 public-y APS A/A repeat | 24/24 | 50.168149 ms |
| Unfolded 32-lane rho, 32 jumps | 24/24 | **50.504898 ms** |
| Unfolded 32-lane rho, 64 jumps | 24/24 | 54.834757 ms |

- **Candidate/old APS = 0.9788442864**, a **2.1156% reduction** in aggregate cold runtime. Candidate wins **23/24 paired public targets**. This is a scoped producer-development stage improvement, **below preregistered ≥5% threshold**, so **development gate FAIL**; we do not move the threshold.
- S3 oracle cumulative time: old `28.251591 ms`, new `27.553425 ms` (2.4712% lower). Across all 24 calls old solver made exactly **18,432** S3 sqrt exponentiations (768 per Q); new solver **0**; both still check all public y via finite-field squaring.
- New A/A repeat `50.168149 ms`, drift `0.7632%` (passes ≤5%).
- Candidate versus faster included unfolded rho32: **ratio 1.0009134758** (APS **0.0913% slower** in aggregate), **12/24 paired APS wins**; versus rho64 wins 15/24. Rho profiles and collision randomness make these ratios inconclusive, and the separate tuned Wiener–Zuccherato rho, external fastest generic rho, and independent PCC are **not** included. This is **not a measured faster-than-best-rho outcome**.
- Deterministic 10,000-resample 24-target paired bootstrap descriptive 95% percentile intervals from archived per-Q cold measurements (seed 0x5933434f4e463136): candidate/old **[0.95308,1.01676]**, candidate/rho32 **[0.73174,1.35915]**, candidate/rho64 **[0.74028,1.15908]**. The window4 versus old interval crosses parity, so even the modest observed whole-call gain is not statistically settled across this sample.

### Independent archived-JSON checks (not a signed PCC)

A separate Python-standard-library verifier (used **only after** Rust completed the solves) decoded the raw ZIP. It implemented elliptic-curve point addition/scalar multiplication separately and recomputed:
- All **120/120** complete logged scalar-point equations `[k]G=Q` on the original curve;
- All **72/72** completed APS signed equations `alpha+beta*k ≡0 mod n`;
- All **384/384** old-vs-candidate source/solver transcript invariants (24 targets × two candidate arms × 8 fields, including identical signed witnesses and pair counts);
- All five wall-ns sums, S3 query sums, cold source sums, 18,432/0 sqrt counts, 23/12/15 paired wins and total ratios; no discrepancies.

A *different* prior 24Q window4 experiment remains frozen as [APS_WINDOW4_PUBLIC_JUMPS_RESULT_20261116.md](APS_WINDOW4_PUBLIC_JUMPS_RESULT_20261116.md), with 120/120 solves but an earlier preregistered 5% cold gate FAIL (66.708464ms versus incumbent 67.549090ms). Retain both independent negative gate outcomes. Independent full environment verification and CI-main keyring-signed PCC standing are unavailable: **[pcc:unverified]**.

## Lead assessment and next genuine scientific experiment

**Stop treating small public-source arithmetic tuning as a likely ECDLP breakthrough.** The fully cold generic frontier has not been decisively beaten even at this small ordinary prime31 size. The strongest structural candidate remains **blinded target-conditioned S9/K961-S5 factor polynomial and signed-point root extraction** from a nonzero random labelled residual `R=aG+bQ`, reusing existing Rust quotient/root infrastructure from merged #4228/#4349. Require original-ideal equivalence and rational signed branches; no 4+4 generic group-table join or exhaustive B31's 48,903,492 x-polynomial scan. First use blinded B4/B8 constructed-positive controls and certified negatives; then actual Dickson31 natural-positive and two explicit negative targets. Use the #4451 conditional D4 injectivity / zero-fibre obstruction as a mandatory falsifier for a spurious nontrivial `R=O` relation. Only after genuine f-from-R extraction attempt rank32/full unknown-scalar solve, charging every quotient build, failed query, root proof, linear algebra and verification, with same-host mature external IC and tuned WZ rho comparisons.

Sources reviewed: original PKM 2016 DOI 10.1007/978-3-662-49387-8_1; APS 2018 DOI 10.1016/j.ffa.2018.01.009; current repo `research/prime_one_shot_20261029/README.md`, `research/norm_branch_rank_pilot_20261115/README.md`, `research/prime31_ic_20261115/native/M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md`, merged #4391, #4228/#4349 and live #4451. The research KB and Conductor/EKS were not available in this GitHub connector runtime; search was bounded and cannot establish novelty by absence.

**No novel non-generic or cheaper-than-fastest-rho complete prime-field ECDLP solver has been established here.** This result is scoped, executable improvement and an exact S3 optimization result within established index-calculus methodology, with negative gates retained and follow-up requirements explicit.
