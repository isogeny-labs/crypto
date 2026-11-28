# Run72 verified result — authentic RHS certificate width K=2/3/4 (2026-11-22 UTC)

## Outcome

**PASS: 9/9 native-Rust complete-solvers on three deterministic 32-target cohorts.** GitHub Actions #45077314037: three separate seed jobs, each 41/41 original Rust library tests and successful K=2, K=3 and K=4 candidate executions. Every result recovered **32/32 genuine unknown-index public target logs**, independently rechecked by original-Rust scalar multiplication and the same-process BSGS reference. K=4 for the original seed exactly reproduced the frozen Run71 1,668,210 / 736 / 454 shift/hit/independent-pair stop and its 2,528,933-shift readout-only control.

**Observed winner in the three frozen jobs:** K=4 has the lowest cold candidate runtime, even after the cost of collecting more target certificates. This is an observed finite-cohort tradeoff, not a statistically established optimal K or a non-generic ECDLP breakthrough.

| Public target seed | K | Initial independent target-difference equations | Genuine public relation shifts | Certificate leaf probes | Cold candidate including setup and verified recovery (ms) | Same-process BSGS (ms) |
|---|---:|---:|---:|---:|---:|---:|
| A = 0x68b677a1d1 | 2 | 32 | 3,480,715 | 503,029 | 1,681.067 | 236.464 |
| A | 3 | 64 | 2,061,888 | 739,025 | 1,325.786 | 237.820 |
| A | 4 | 96 | **1,668,210** | 933,657 | **1,271.430** | 235.344 |
| B = 0x9d3a6f5c127 | 2 | 32 | 2,608,250 | 430,537 | 1,359.044 | 270.542 |
| B | 3 | 64 | 2,141,629 | 687,409 | 1,314.650 | 271.014 |
| B | 4 | 96 | **1,825,641** | 862,666 | **1,299.790** | 269.195 |
| C = 0x2c6478eb19d | 2 | 32 | 4,282,074 | 445,740 | 1,801.604 | 232.871 |
| C | 3 | 64 | 2,528,933 | 693,012 | 1,355.061 | 234.654 |
| C | 4 | 96 | **1,668,210** | 911,657 | **1,223.973** | 243.237 |

Cold candidate times were captured **before** readout-only baseline RREF and before executing the reference BSGS. They include the original 916,469-key generic table, the witness search, actual-RHS relation collection, modular linear algebra, and native group verification. Each BSGS total is setup_ms + online_32_ms for the same original public target cohort.

### Width cost / benefit

K=4 versus K=3: exact public relation shifts decrease by **19.09% (A), 14.75% (B), and 34.04% (C)**; charged cold candidate time decreases by **4.10%, 1.13%, and 9.67%** respectively. K=4 requires **194,632 / 175,257 / 218,645 more certificate leaf probes** than K=3 on A/B/C, so part of the relation savings is consumed by extra witness discovery. The 1.13% timing difference on B is too small to claim significance on one runner execution.

K=2 provides only 32 initial difference rows, and two of the three cohorts (A,C) reach exactly the same relation stop as their K2 readout-only control. K=4 starts with 96 genuine difference rows (rank 88 -> 184) and leaves the best early-stop opportunity after specialization against real signed-pair RHS relations. Rank additions at final stop can be smaller than 96 because relations become linearly dependent; do not count nominal rows as independent at final rank.

Across these frozen records, K=4 is still **4.83x to 5.40x slower** than same-process shared BSGS for 32 targets. This is a working complete GENERIC reference and cannot qualify as novel/non-generic index calculus.

## Mathematical control

For each public unknown Q, signed three-leaf certificate r gives Q+[j_r]G = sum_i c_(r,i) P_i. Subtract certificate 0 to cancel the unknown target and obtain a known-RHS, group-verified equation among factor-base logarithms:

sum_i (c_(r,i)-c_(0,i)) L_i = j_r-j_0 mod n.

K witnesses permit at most K-1 independent target-eliminating equations per Q; Run72 confirms exact initial ranks 32/64/96 on 32 targets at widths 2/3/4. The solver never receives the harness scalar before the post-solve verification. Its quotient row space is computed over subgroup prime n=2147446831; a target is counted recovered only when its affine form is determined and the original group check [k]G=Q succeeds.

## Source and experimental reproducibility

- Source-tested commit: 783d2a928a548d48be9f2e0e6e7be9b0cf60141b, source-specific workflow https://github.com/isogeny-labs/crypto/actions/runs/45077314037. Job IDs: A 137377318768, B 137377318720, C 137377318378.
- Original Rust examples: research/prime31_ic_20261115/native/examples/prime31_run72_k2_rhs_width.rs, prime31_run72_k3_rhs_width.rs, prime31_run72_k4_rhs_width.rs. All source-include the Run69 original native EC arithmetic / base construction and reuse Run71's true-RHS signed DSU and exact quotient solver. No new EC implementation.
- Preregistered protocol (committed before source): RUN72_WIDTH_SWEEP_PROTOCOL.md; frozen 9-row metrics, source blobs, proof gates, limitations: RUN72_RESULTS.json.
- Exact three target cohorts each have 32 independently derived deterministic xorshift public points, with common cohort across K choices. These are **three cohorts**, not 96 independent benchmark trials. Each width runs sequentially on the same GitHub runner for its cohort. Two original Run71 runs on the same original seed were previously performed; timing differences across Actions jobs are not matched-host performance effects.
- K=4 Run71 controlled exact replay checks pass; all other widths and seeds validate all 32 public target logarithms and RHS group identities. No failed/censored cells in the nine frozen registrations.

## Prior leads and new experiment queue

1. **Primary gate: genuine unknown-index public-only algebraic extraction**, not generic enumeration. Run the frozen 46-positive/18-negative corpus through RR9/Krylov PR #4822 and Horner/F3 PR #4812, separately checking B8 coefficient PR #4919 and selector/F4 PR #4692. Record every failed root/timeout/OOM, full point-sign lifting, independently group-checked extracted roots, exact memory and extraction time. Only after this succeeds connect its verified certificates to the original-Rust K4 difference-row engine.
2. Evaluate **adaptive K** after a source-verified algebraic extractor exists. Compare K=3,4,5,6 and early stopping after the marginal quotient-rank gain becomes small; cost full witness probes, not just relation shifts. Additional certificates must be root-recovered from public equations; otherwise this is only generic calibration.
3. Cold T=1 and shared T=32/128/1024 complete log recovery on same hardware against strongest existing prime-field index-calculus frontier, memory-matched shared BSGS and optimized negation-map rho. Report genuine independent relations, rank, full setup, unsuccessful attempts, root-solving, LA, verification, RSS and wall-time distributions. The historical 1024-target BSGS result from PR #4532 is a necessary strong frontier comparator, not a cross-run speedup denominator.
4. Avoid merely re-encoding the enormous generic signed pair table as a dense algebraic polynomial: the accompanying RUN72_UNIVARIATE_PAIR_SUPPORT_BARRIER.md proves that any nonzero univariate polynomial vanishing on every finite x-coordinate of the frozen B677 signed pair set must have degree **at least 458,234**. This is a degree-only observation, not an arithmetic complexity lower bound.

### Prior art and status qualifications

The one-relation prime-field Semaev approach predates the lab's experiments: Amadori–Pintore–Sala (2018), https://doi.org/10.1016/j.ffa.2018.01.009. Hybrid Gröbner acceleration/symmetry and limitations are discussed by Kudo–Yokota–Takahashi–Yasuda (2018), https://kyushu-u.elsevierpure.com/en/publications/acceleration-of-index-calculus-for-solving-ecdlp-over-prime-field/. FASS symmetric S3 in PR #4921 is currently a protocol, not a verified solver, and PR #4809 covers quadratic S3. Prior Run68/69/70/70B/71 negative controls are retained without relabeling their generic tables as algebraic extraction. Shared research-KB MCP and local cargo were unavailable here; direct source and primary publication records were reviewed.

**Current verdict:** The native-Rust K2/K3/K4 optimization sweep is reproducible and checked in; K4 is the best of these widths on all three frozen cohorts. A cheaper complete **non-generic/index-calculus** prime-field ECDLP solver is **not yet established**. The blocker is blind unknown-index algebraic relation extraction, not scalar recovery or verification. PR #4901 remains draft and PCC independent verification remains outstanding.
