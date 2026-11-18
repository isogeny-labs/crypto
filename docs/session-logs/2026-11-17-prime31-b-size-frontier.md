# 2026-11-17 — actual-prime31 B27/B29/B31 fully charged sensitivity

## What was done (commands, files, features)
- Read `AGENTS.md`, `docs/ACTIVE.md`, parent #4626 session log and primary native Rust 4A oracle and complete rank→1024-Q test; checked previous #4623/#4626, contemporaneous #4596/#4604 and cross-repo reviewed leads. Checked primary PKM 2016 and APS 2018 bibliographic sources. `research-kb` not offered by connected tools and repository CLI unavailable in this runtime.
- Created branch `automation/prime31-b27-b29-complete-sensitivity-20261117` stacked on open #4626, committed one-line task record in ACTIVE.md first, opened draft PR #4629, and attached its number in a separate commit.
- Committed prospective B27/B29/B31 experiment protocol before editing Rust. No new performance or solve claim at this stage. [pcc:unverified]

## Decisions made (and why)
- Reuse the actual Rust original-curve arithmetic, 4A index, batched complement, modular rank learning and frozen public targets. Vary **prefix base size**, not Dickson construction depth, so the only intended independent variable is B.
- Demand 256/256 complete blinded scalar recovery and rank B/B with all failed shifts and charged fallback, not a coverage-only optimism signal. Treat all 4A code as generic-equivalent, not novel index calculus.

## Open threads (unfinished, pick up next time)
- Insert a focused test in the original Rust source; add source-specific pinned CI; run, capture and independently read back raw evidence, evaluate full cost; update PR and handoff. Then attack sign-independent algebraic residual decomposition on the real B31 base.

## Errors (what went wrong, how it was handled)
- Native Rust/cargo and KB CLI are not installed in the connected container. GitHub Actions source-specific Rust runner will be necessary; no unexecuted test is presented as passed.

## Exact-source outcome and handoff — 2026-11-18 00:22 UTC

### What was done (commands, files, features)
- Implemented `complete_b31_1024_tests::actual_prime31_b27_b29_b31_256q_complete_cost` in the original Rust B31 4A+4A oracle source, using prefix lengths B=27/29/31 of the unchanged actual depth-six Dickson base, unchanged 512-way batched complement and rank/BSGS fallback modules, original `p=2147483647, n=2147446831` curve arithmetic.
- Added exact-source CI `.github/workflows/prime31-b27-b29-b31-full-cost.yml`: original arithmetic pinned to `1df714645aa683cf975bc76fcacb9842cb4780fd`, candidate Rust source `22a31ed2529ccbacf3d213985ecad69f639e6f22`, Rust 1.100.0, frozen 256 public targets shared across all panels.
- Both [push CI #44207041891](https://github.com/isogeny-labs/crypto/actions/runs/44207041891) and [PR CI #44207050180](https://github.com/isogeny-labs/crypto/actions/runs/44207050180) passed original native library tests 40/40 and focused integration 1/1. Artifacts #14384426906 and #14384715871 published; their GitHub-reported ZIP digests are in the result report (not independently rehashed). Parsed full original Rust JSON from both separate GitHub Actions job logs; all 768 Q/log/witness records, 87 factor-base logs and all collected modular rows match exactly across source-specific runs.
- Committed full first-run [raw JSON](../research/prime31_ic_20261115/native/B27_B29_B31_FULL_COST_RAW_20261117.json) and detailed [report](../research/prime31_ic_20261115/native/B27_B29_B31_FULL_COST_RESULT_20261117.md). Original signed eight-sum 4A index and its verification remain charged; no new algebraic extractor or genuinely non-generic speed claim. [pcc:unverified]
- A separately written producer-side BigInt affine elliptic-curve implementation rechecked all 87 factor-base logs, 768 recovered unknown-target logs, 768 signed eight-point witnesses, source relation row RHS values, generator order and 256 public Q distinctness across three panels. This is a producer-written arithmetic cross-check, **not signed independent PCC**.
- On first run full charged B27/B29/B31 times = 6,556.280 / 4,434.263 / 3,152.266ms; on repeat run 6,921.234 / 4,922.840 / 3,471.880ms. Basis ranks 27/27,29/29,31/31 and unknown logs 256/256 each. B27/29/31 first-shift hits = 147/195/237; all target attempts 460/337/276 and failed exhaustive shifts 204/81/20. No generic BSGS fallback on this frozen cohort.

### Decisions made (and why)
- For the fixed, preregistered original Dickson factor-base prefixes, B31 has the best complete charged runtime in both source-specific runner runs: saved table setup for smaller B loses to much more repeated exhaustive scans and additional residual shifts. Do not change B31 baseline to B27 or B29 solely based on smaller factor-base dimension.
- This establishes a bounded *generic-equivalent* Rust full-cost engineering comparison, not a non-generic index-calculus speedup. The independent-bin signed-net occupancy approximation matches monotone coverage only heuristically, not as a theorem about these correlated group sums. [pcc:unverified]
- Review-only [Kaarelson fast-ecdlp](https://github.com/kaarelkaarelson/fast-ecdlp) (2026 counting-engine asymptotic claim) is a theory watchlist, not a practical prime31 measured improvement; own reported crossover far beyond prime31 and current run did not verify its Lean/paper proof.

### Open threads (unfinished, pick up next time)
- Primary: genuinely sign-independent **public-residual-only B31 RR9/S9 algebraic coefficient/roots orientation extraction**, using existing Rust F4, quotient and rootfinding without any 4A or signed-support enumeration. First original B8 positive and two original-ideal negative certificates, then Dickson B31 repeated/exceptional fibres and public-only naturally obtained relations.
- Alternative: public-R-conditioned K961/S5 elimination whose measured complexity is below generic 4A enumeration. Feed surviving authentic nonenumerative relations into existing Rust full-rank and 256/1024 unknown-log pipeline; compare calibrated full-cost and peak memory with optimized same-advice BSGS/rho and published IC. Separate lanes #4596 matched full-cost and #4604 D9 sparse unit certificates are already owned; do not duplicate.
- Independent CI-main PCC signed validation and isolated same-runner full solver benchmark remain outstanding. Parent #4626 and its ancestors unmerged; keep #4629 stacked and unmerged, ready for review once final check-in verified.

### Errors (what went wrong, how it was handled)
- Native local Rust tooling and shared research-KB MCP/CLI unavailable here, so tests were executed and measured via two exact-source GitHub Actions jobs; no locally measured wall claims. No Rust compile/test failures in either source-specific job. Hosted runner fixed panel order and performance variability limit generalization.
