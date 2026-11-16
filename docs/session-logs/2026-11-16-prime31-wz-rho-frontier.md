# 2026-11-16 — Prime31 APS / exact repository WZ negation-rho benchmark

## What was done (commands, files, features)
- Queried GitHub for merged #4454 and #4451, prior #4391, `AGENTS.md`, latest prime31 24Q and 48Q raw reports, `src/cryptanalysis/ic_boundary.rs` and exact prior source CI. Read primary APS 2018, Wiener–Zuccherato, Bos–Kleinjung–Lenstra and Bernstein–Lange–Schwabe references. Shared research-KB MCP/local shell unavailable; bounded GitHub/publisher fallback.
- Verified frozen [CI 43974039411](https://github.com/isogeny-labs/crypto/actions/runs/43974039411) report: composed APS 74.433895ms, unfolded rho32 79.833260ms on 24 Q; 144/144 correct, engineering 5% threshold **failed**, paired APS/rho interval overlaps parity. No independently signed PCC.
- Discovered a reproducible repository state discrepancy: `fetch_file(ref=main)` returned HTTP 404 for `research/prime31_ic_20261115/native/{Cargo.toml,lib.rs,main.rs,examples/prime31_aps_one_relation.rs}`, while the same exact paths returned real contents at the merged immutable ref `1df714645aa683cf975bc76fcacb9842cb4780fd`. Other prime31 data/workflows remain visible on main. Did not overwrite or blindly restore concurrently changed code.
- Created branch `research/prime31-wz-rho-frontier-20261116`, prospectively committed `research/prime31_ic_20261115/native/WZ_FRONTIER_PROTOCOL_20261116.md` at `31397c644e838952ad6ff3597ab7cc766bacd601` before code.
- Added native Rust root example `examples/prime31_wz_rho_frontier.rs` (`770cc4c8f60a98d91ebda7c331be35f625b7b4e5`): calls **exact** `ic_boundary::rho_reference_negation` and `rho_walk_with_policy`/Brent on identical public Q, with caller-only final scalar replay; counts and retains failed/capped attempts, setup, walks, field operations, full cold wall, and deterministic A/A trajectories. No Python ECDLP solver.
- Added `.github/workflows/prime31-wz-rho-frontier.yml` (`255e0e13e838248a93a63fd50ce2d99f2401530e`) to compile native APS from immutable already-merged ref, run exact 24Q APS, then run root WZ on the same JSON on the same Ubuntu 24.04 runner; upload both full raw files even on failure. Pinned Rust 1.100.0.

## Decisions made (and why)
- Chose strongest outstanding *generic reference measurement* rather than replaying censored m6/F4 or fabricating an S9 breakthrough. The WZ function is actual existing repository code with look-ahead and short-cycle escape, plus exact v2 Brent policy; this is not a port or substitute.
- Comparator accepts earlier frozen 24Q APS cohort because its source is known and reproducible; describes it as an engineering development panel, **not** a fresh held-out scientific gate. A complete successful run would be same-host sequential producer evidence, not cross-host fastest-external validation.
- Reuse the immutable merged native source pin to avoid races with the suspicious main-native-file 404 and avoid a potentially harmful code restoration.
- Preserve 5% previous engineering gate failure, uncertainty crossing parity, capped rho attempts and no claim of novel non-generic/subgeneric prime-field ECDLP.

## Open threads (unfinished, pick up next time)
- Inspect exact Actions verdict, compile/test output, full case JSON and WZ methods. Repair if source-specific compile or runtime fails; independently replay returned scalars and recompute bootstrap against existing APS on same Q.
- Obtain isolated CPU and 96 *new* public Q with blinded test ordering, compare to fastest external rho and published APS/PKM. Signed PCC and cost accounting before any leaderboard claim.
- Build original-ideal-certified target-conditioned S9 f-from-R B4/B8 extractor without hidden four-sum enumeration; test known Dickson31 negatives, infinity and signed fibre, then B31 full rank32 recovery.

## Errors (what went wrong, how it was handled)
- This execution environment has neither local Rust/Cargo nor GitHub clone/EKS access; Rust source is committed for GitHub Actions, not locally compiled. No fabricated benchmarks.
- Shared canonical `docs/session-logs/2026-11-16.md` exceeds the connector's readable inline content and was empty with nonempty blob SHA; used this scoped log rather than overwriting concurrent sessions.
- Repository main native source paths gave 404 despite existing at merged immutable SHA; pinned exact source and recorded the discrepancy rather than claiming those files have been permanently deleted.
