# EXP-R31-APS-BUFFER — preregistered ABBA result (2026-11-17)

**Outcome: negative engineering gate on the exact tested candidate.** A source-specific Ubuntu 24.04 / Rust 1.100.0 producer CI experiment completed and recovered all 576 logarithms in the 24-target × 6-arm × 4-panel ABBA fixture; the existing S3 oracle's three batch-inversion scratch buffers were changed to reusable allocations. Old/new candidate arithmetic and signed S3 transcripts were equal. **The candidate was ~0.17% slower**, not 5% faster. This is a measured *null* for the prescribed one-shot single-target APS ECDLP run, not a new cryptanalytic attack. All statements [pcc:unverified].

## Source and evidence
- [Preregistered protocol](APS_REUSABLE_S3_BUFFER_PROTOCOL_20261117.md): commit `2c98f0b0068f1423dd01085e95f257f3ecc86f6b`, before the candidate edit.
- Narrow Rust source change: `caba082308d0dfe5da642241a84eca6a8564febe` to `native/examples/prime31_aps_one_relation.rs`. Exact scientific baseline `1df714645aa683cf975bc76fcacb9842cb4780fd` (merged #4454), same two full pinned worktrees differing solely by candidate example before fresh target-seed injection.
- Source-specific [successful CI #44003116725](https://github.com/isogeny-labs/crypto/actions/runs/44003116725), job [133771812187](https://github.com/isogeny-labs/crypto/actions/runs/44003116725/job/133771812187), executed the frozen old/new/new/old order.
- [Raw artifact #14293546662](https://github.com/isogeny-labs/crypto/actions/runs/44003116725/artifacts/14293546662) contains four complete solver JSON outputs, Rust audit JSON, and source SHA manifest.
- In this *first* source-specific run, both pinned native worktrees passed 40/40 library and 22/22 example tests; the separate (then-current) Rust audit example passed 2/2. Auditor checked all 576 scalar labels and old/new per-round S3/solution transcript equivalence, but **did not independently compute `[k]G=Q`**. A later pre-measurement-hardening source commit `0cc827ce...` adds independent standalone Rust affine group-law replay, and has its own separately triggered CI; do not call this historical run the hardened auditor.

## Primary fully cold matched measurements

Each panel contains **the identical 24 public Q** on the actual ordinary cofactor-one `p=2147483647` curve, `n=2147446831`; candidate and original both execute the existing published APS `S3` one-relation solver. Each of four panels additionally runs four APS and two rho arms for completeness/correctness; the primary cost is the composed APS arm's full cold call, not isolated oracle time.

| Chronological ABBA pair | Old complete APS total, ns | Reuse candidate complete APS total, ns | Candidate/old | Candidate paired target wins | Frozen ≤0.95 + ≥18/24 gate |
|:--|--:|--:|--:|--:|:--|
| old1 → new1 | 81,295,975 | 81,434,728 | **1.00170676** | 9/24 | **FAIL** |
| new2 → old2 | 81,238,127 | 81,371,380 | **1.00164028** | 13/24 | **FAIL** |
| Combined, descriptive only | **162,534,102** | **162,806,108** | **1.00167353** | — | **FAIL** |

- Old arm repeat aggregate drift: `0.00071208` (0.0712%); candidate repeat aggregate drift: `0.00077850` (0.0779%).
- Candidate internal composed-versus-AA drift: `0.00274328` (0.2743%) for first pair and `0.00128960` (0.1290%) for second; both below 5% A/A threshold.
- Thus candidate is systematically slightly slower under this producer runner; **no new faster baseline** should be registered. The effect is much smaller than the frozen 5% improvement target.
- No paired confidence interval or cross-host isolation was established by this job. Do not interpret 0.17% as a portable performance penalty or a statistically established universal regression.
- Most important: this is generic-decomposition **index-calculus** engineering based on Amadori–Pintore–Sala (2018), not non-generic extraction. No strongest optimized Wiener–Zuccherato rho or external PKM/APS speed comparison came from this run.

## Follow-up decision / further leads
1. **Stop this exact allocation-only lead.** The target cost is field algebra, not the measured short-lived scratch allocations. Do not rerun unchanged with new favorable seeds.
2. Prioritize target-conditioned RR9/S9 function extraction without enumerating eight-point multisets, using the merged #4466 Hermite/signed x-norm certificate. On blinded public residual `R`, solve `F(-R)=0` and the factor-base divisibility condition for `N_F/(X-x_R)`, certify solution *fibres* on exhaustive B4/B8 negative/positive controls first; charge Gröbner/rootfinding costs before B31.
3. **Unblock the strongest-rho control separately and safely**: the historical pinned root `src/cryptanalysis/sat.rs` has multiple unclosed/misnested delimiters (Actions #43994897230), not a one-brace nuisance. Build an isolated source-identical Rust WZ comparator rather than mutate a stack of unrelated broken modules. It yielded zero comparable WZ timings.
4. Respect cross-repository negative evidence: 41-lead heat map ([autoresearcher #3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301)), Semaev-alternatives errata ([#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305)), single-target IC vs tuned rho ([crypto #4108](https://github.com/isogeny-labs/crypto/pull/4108)), and multitarget IC ([crypto #4363](https://github.com/isogeny-labs/crypto/pull/4363)). Do not treat bounded toy failures as proofs of general impossibility.

**PCC:** no signed independent verdict. This result remains in the experiment directory, not the IC leaderboard.
