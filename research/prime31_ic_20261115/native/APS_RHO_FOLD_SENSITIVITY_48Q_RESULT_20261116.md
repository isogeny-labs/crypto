# Prime31: 48 fresh Q × 5 matched complete solvers — fold sensitivity resolves the rho-control problem

**Evidence standing:** GitHub Actions producer-development, [pcc:unverified]. Complete, correctly checked scalar recovery is reported at **source-specific Rust CI** scope. The strongest available general-purpose solver, independently signed scientific cost validation, and a new non-generic ECDLP attack are not established.

## Frozen experimental provenance

- Prospective [48-Q protocol](APS_RHO_FOLD_SENSITIVITY_48Q_PROTOCOL_20261116.md) committed at `92b58e047985cbc4249e51831c3879a7f513030b` **before** this follow-up Rust code or measurement.
- Rust code `4d7e9c0c9702131c71ecc27e381ff607f644992d`, focused workflow source commit [`aed18cf91172a74d76bcab2a7530aa656dc61ee2`](https://github.com/isogeny-labs/crypto/commit/aed18cf91172a74d76bcab2a7530aa656dc61ee2).
- **[Exact-source GitHub Actions run 43921120382](https://github.com/isogeny-labs/crypto/actions/runs/43921120382)**, job `133488352756`: **SUCCESS**, pinned Rust 1.100.0 release on Ubuntu 24.04, existing and new focused tests passed. New benchmark `--rho-fold-sensitivity-48q` emitted complete original Rust JSON.
- [All **240** original cold-call records](APS_RHO_FOLD_SENSITIVITY_48Q_EXACT_CI_20261116.json), committed at `4e2b047f9db5b625aec1cefb09650d73b81b499b`, with only Actions timestamp wrappers removed.
- Curve: `p=2147483647=2^31-1`, `E:y²=x³+847514846x+2087444724`, `n=2147446831` prime, cofactor one, original `PRIME31.g`. Frozen caller-only target seed `0x52484f5f4e455734`, no repeated target scalar among 48 and **zero** scalar overlap with the previous 24-target unfolded panel. Arms rotated through all five chronological positions to mitigate temporal skew. Rho seed base `0x52484f5f43523031`, cap 2m charged walk steps.

## Matched whole-solver results

| Fully cold, same host, same 48 public Q | Correct recovered unknown logs | Total full cold ms | APS faster per target |
|---|---:|---:|---:|
| APS published S3 one-relation IC, [384,384,16384], joint-Jacobian public source | **48/48** | **109.734280** | — |
| 32-lane / DP4 **negation-folded** rho, 512-step watchdog | **48/48** | **213.126270** | 37/48 |
| 32-lane / DP4 **unfolded** rho, signed-x collision certification | **48/48** | **101.453752** | 21/48 |
| 64-lane / DP5 **unfolded** rho, signed-x collision certification | **48/48** | **99.473873** | 23/48 |
| Unfolded 32-lane deterministic A/A repeat | **48/48** | **101.399922** | — |

All **240/240 completed** and recovered the caller-known scalar; each solver independently replayed `[k]G=Q`. The raw report independently recomputed all 240 per-call nanosecond sums, 240 recovered scalars, the 48 APS signed relations `alpha+beta*k=0(mod n)`, success counts and paired wins with **zero discrepancies**. The 32-lane unfolded A/A total drift is **0.0531%** (same seeds, independent cold calls).

**Cost ratios on this panel:**

- APS / folded32 **0.514879**: the APS method is **1.942× faster** than this particular cycle-prone folded comparator.
- APS / unfolded32 **1.081619**: unfolded rho32 is **7.55% cheaper** in aggregate than APS here.
- APS / unfolded64 **1.103147**: unfolded rho64 is **9.35% cheaper** in aggregate than APS here.
- Folded32 / unfolded32 **2.100723**: the unfolded map is **2.10× faster** than the matched folded implementation. The folded map restarted on **1,627** stagnant walks while both unfolded configurations had **zero** watchdog restarts. Folded32 charged **3,605,184** walk steps versus unfolded32 **2,796,384** and unfolded64 **2,966,464**.

**Uncertainty and scoped inference:** A deterministic 20,000 paired-target bootstrap resamples the 48 case pairs and compares the ratio of summed wall times; approximate 95% bounds:

| Ratio | Observed | Bootstrap 95% interval | Interpretation |
|---|---:|---:|---|
| APS / folded32 | 0.515 | 0.421–0.633 | APS substantially cheaper than *this* folded implementation |
| APS / unfolded32 | 1.082 | 0.878–1.336 | **Overlaps 1: no statistically clear APS/rho ordering** |
| APS / unfolded64 | 1.103 | 0.925–1.312 | **Overlaps 1: no statistically clear APS/rho ordering** |
| Folded32 / unfolded32 | 2.101 | 1.682–2.594 | Severe actual overhead from the tested folded implementation |

The bootstrap resamples cases from one unisolated developer-host run and does not estimate cross-machine noise or independent source validity. A/A repeat preserves deterministic collision choices and cannot establish success-rate statistics. Sample medians (ms/call): APS 1.865; folded32 3.778; unfolded32 2.035; unfolded64 1.949; unfolded32 A/A 2.017.

## What the result actually changes

The previous [fresh 24-Q folded watchdog study](APS_VS_NEG_RHO_WATCHDOG_RESULT_20261116.md) showed APS 64.742ms against folded rho32 125.262ms and rho64 140.041ms. Those were complete valid timings against the stated implementations, **not** a universal optimized-rho advantage. The separately preregistered [fresh 24-Q unfolded follow-up](APS_VS_UNFOLDED_RHO_RESULT_20261116.md) first showed 96/96 correct complete recoveries with an aggregate unfolded advantage over APS, but an especially wide paired confidence interval. This new **direct folded/unfolded/APS comparison on 48 identical fresh targets** eliminates cross-panel comparisons and confirms that the folded watchdog comparator can materially exaggerate APS's apparent advantage.

The mathematical reason is plausible, not proved causal by a single ablation: the negation map with a fixed 16-jump additive walk can enter fruitless cycles requiring escape. Real-world caveats and practical limits for negation-map acceleration are prior art, especially Bos/Kleinjung/Lenstra ANTS 2010 (DOI 10.1007/978-3-642-14518-6_9), Wiener–Zuccherato SAC 1998 (DOI 10.1007/3-540-48892-8_15), and this repository's `research/rho_walk_leads_20261014`. The unfolded control still uses the same batch inversion and exact signed x-class collision reconstruction; it did not gain by receiving the verifier's secret.

## Lead audit and next experiment gates

1. **Best generic comparator:** next run a *same-Q* Rust implementation of `src/cryptanalysis/ic_boundary.rs::rho.negation` with Wiener–Zuccherato look-ahead, tuned jump counts and cycle escape; include unfolded32/64, the APS incumbent, A/A, and real CPU isolation. Avoid claiming this 16-jump unfolded control is fastest external rho.
2. **APS engineering:** current proven-complete published IC-derived one-relation source still has classical labelled points, factorized S3 field roots, packed source x-table and 64-point joint-Jacobian batches. Optimization passes in #4391 are real implementation improvements against their frozen previous APS arms, separate from any unestablished generic frontier victory. Any next APS optimization must beat the new stronger unfolded control by **full cold cost** on unseen Q.
3. **Genuinely non-generic lead:** original-target conditioned symmetric S9/K–S5 or block-Vieta elimination for an eight-leaf B31 factor base. Reuse #4228 merged compact quotient and #4349 open exact Rust selected-root/FGLM tests and the exhaustive B8 given-`f` signed root validator. Build rational *f-from-Q* without enumerating 48,903,492 B31 candidate multisets; test B4/B8 positive/negative/infinity/repeated roots and original-ideal equality first. Ordered m8 F4/anchored m6 F4 are previously censored/limited; fixed-anchor m6 support <=3.7407208%, so do not repeat unchanged.
4. **Prior alternative lanes:** auxiliary/Random-base timing in isogeny-labs/cryptanalysis #1536 still lacks transport/execution and independent validation; Vieta #4410 is separate. Published PKM 2016 (DOI 10.1007/978-3-662-49387-8_1), APS 2018 (DOI 10.1016/j.ffa.2018.01.009) already establish the algebraic framework. Evaluate completed mature external PKM/APS/F4/CAS over exactly the same field, curve, Q, host, and cold-cost contract.

**Limitations:** GitHub Actions CI producer evidence [pcc:unverified]; no signed independent PCC, isolated-frequency host, EKS Spot transport, full external PKM/APS/CAS ranking, or claim of non-generic/subgeneric scaling. KB MCP and local repo wrapper were unavailable; literature checks used repository source and primary published papers. No policy waiver, paid compute, merge, or leaderboard edit.
