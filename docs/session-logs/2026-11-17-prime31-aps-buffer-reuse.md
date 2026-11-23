# 2026-11-17 — complete prime31 Rust APS scratch-array optimization

## What was done (commands, files, features)
- Queried GitHub live PR #4466 (merged), #4468 (merged), #4454 (merged), #4460/#4462 (merged), and code search for newer WZ updates. Re-read `AGENTS.md` on `main`, today's shared `docs/session-logs/2026-11-17.md` (ongoing separate F4, proof, production work), historical APS and RR9 experiment reports and exact source. The default branch has been subject to sparse merges, so no broad restoration or unrelated edits.
- Reviewed primary PKM 2016 (DOI 10.1007/978-3-662-49387-8_1), APS 2018 (DOI 10.1016/j.ffa.2018.01.009) and the repository's merged experiment history #4391, #4451, #4454, #4466, #4228/#4349, #4373, #4410; cryptanalysis #1536. The shared KB MCP and wrapper cannot be executed in this GitHub-connector runtime; no novelty claim. `conductor check` unavailable; narrowed write scope to feature branch and three new experiment paths plus the one existing native Rust example.
- Inspected newly failed WZ exact-source Actions #43994897230, job `133744418258`: it stops at a *different preexisting damaged pinned Rust file*, `src/cryptanalysis/sat.rs`, with multiple unclosed delimiters, e.g. an actual missing closure after `storage_snapshot_due` and after `storage_sampling_is_limited_to_boundaries_and_ten_thousand_conflicts`. The `trace_solve_phase` function is spliced into another function; not safe to apply a one-brace repair. No WZ unknown-target solve or timing was produced. Do not conflate with prior rho results.
- Created feature branch `research/prime31-aps-reusable-s3-buffers-20261117`. **Protocol frozen before code**: `APS_REUSABLE_S3_BUFFER_PROTOCOL_20261117.md`, commit `2c98f0b0068f1423dd01085e95f257f3ecc86f6b`. Scope: exact per-round S3 transcript preservation and ABBA 24 fresh public targets per panel with the old and modified complete Rust solver.
- Changed *only* the lifetime of `denoms`, `prefix`, `inverses` within the existing `algebraic_mode_factorized_index_variant_y`: three scratch buffers are allocated once per oracle call instead of once per left-row; clear/overwrite on each row. Group and finite-field formulas, original witness/source labels, exact S3 query order and public y logic are not changed. Commit `caba082308d0dfe5da642241a84eca6a8564febe`.
- New **Rust**, not Python, `prime31_aps_buffer_audit.rs` checks 4×144 complete calls, public Q and caller-only scalar coherence, every APS per-round transcript key, and predeclared old/new whole-cold prices. Commit `0db183c87c0c37e1ecfaedf4a4954f53397c06ab`.
- New `.github/workflows/prime31-aps-s3-buffer-reuse.yml` makes byte-identical immutable APS copies, overlays just the candidate file and auditor into one copy, replaces the *same* frozen target seed only in temporary benchmark checkouts, preserves 4 raw JSON runs plus source pins, tests both native Rust programs and the audit, and reports (not forces) a 5% improvement gate. Commit `0090447b2008148c761cb1b55604fa1a2c941ffd`.
- No local Rust compiler/network to clone the repository in this runtime, so compiling, 576 complete-call correctness validation and timing are **pending source-specific GitHub Actions**; nothing measured is claimed for this new code.

## Decisions made (and why)
- Choose allocation reuse as a narrow, reversible whole-cold IC engineering hypothesis; pair enumeration still qualifies as the generic-decomposition track and no asymptotic/non-generic breakthrough is asserted.
- Do **not** patch corrupted pinned SAT file or silently discard missing WZ controls. A clean isolated exact-source comparator is a separate task.
- Treat 5% speed gate, paired 18/24 wins per ABBA block, and A/A drift ≤5% as frozen. A speed regression or null is a valid outcome; no reroll after observations.

## Open threads (unfinished, pick up next time)
- Inspect exact CI for this feature branch/PR; if compilation fails, fix source errors before benchmarking, preserve negative attempts. Recheck raw public Q, all 576 scalar-point recoveries and 4× APS signed relation transcripts; recompute timing independently from raw artifacts.
- If gate fails, record the null and focus on **target-conditioned RR9 factor polynomial extraction** with certified B4/B8 solutions, then B31 unknown-Q rank recovery.
- For any fast-solver headline: 96 fresh fully cold hidden-scalar targets, no excluded failures, strongest correctly compiling tuned WZ/Brent and optimized rho comparator, published index-calculus frontier on same inputs/host, independent signed PCC.

## Errors (what went wrong, how it was handled)
- Native Rust toolchain unavailable in this runtime; controlled CI rather than fabricated local test is prescribed.
- GitHub's older WZ root build failed again on severely malformed pinned SAT source; reported it as infrastructure source corruption, not a mathematical failure.


## Additional cross-repository lead-atlas review (after code/protocol freeze, before timing results)

### What was done
- Inspected merged [crypto-autoresearcher #3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301): the 2026-11-14 prime-field IC heat map covered 41 leads; 39 lead dispositions changed. It retains 14 proved/cost-obstructed, 17 exhausted *only at tested scale*, five blocked, two unexamined and one generic pair-table constant lead. Do not convert toy nulls into mathematical impossibility.
- Inspected merged [crypto-autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305): 28 preregistered Semaev-alternatives experiments and ERRATA, SA-23/24/25 limitations, lead-atlas fold, and preregistration rules. Target-conditioned RR9 extraction must explicitly check these before a novelty claim.
- Inspected merged [crypto #4363](https://github.com/isogeny-labs/crypto/pull/4363): memory-optimized multitarget IC was 0.69–0.76× incumbent IC within the study; after jointly tuning rho, median IC/rho was 0.914–0.996 for tested 2^32/T=1024–16384 but every cell had a counterexample above parity, and 1.088–1.230 at 2^36. These are producer-level and multi-target, not evidence of a cheaper single cold prime31 solver; repeat on matched fronts.
- Inspected merged [crypto #4108](https://github.com/isogeny-labs/crypto/pull/4108): shift-fixed symmetric IC improved incumbent IC ≈2.8–3× at 28–34 bits but still lost to fastest matched rho, by 6× to 88× across sizes. This is an IC-baseline success, not a rho success.
- Inspected merged [cryptanalysis #1529](https://github.com/isogeny-labs/cryptanalysis/pull/1529): relation storage CSR-like encoding reduced ≈784 B to ≈68 B at 48 bits, with exact replay controls; a *different C* implementation and a memory-only result, not a directly reusable Rust oracle. Avoid claiming novelty for packed relation storage.

### Decisions made
- The new S3 buffer reuse belongs to the **generic-decomposition index-calculus** engineering track and must beat the *exact named APS incumbent*, not retroactively present a speedup over the repository's multi-target IC or exterior WZ rho.
- Prior atlas records establish extensive earlier exploration, but no proof that target-conditioned RR9 norm extraction is impossible. The next non-generic gate is a solver *given only public residual R*, against B4/B8 exhaustive fibres including repeated leaves and negative targets, with original-ideal certification. No extra GH workflow/target seeds changed by this review.

### Open threads
- Retrieve full atlas evidence and semaev campaign's archival reports before implementing any allegedly novel RR9 elimination; this quick GitHub PR audit is not an exhaustive KB review.

### Errors
- No additional execution; this was read-only cross-repository triage and does not revise a frozen experiment or result.


## Pre-measurement correctness hardening (separate source commit)

### What was done
- Hardened the Rust ABBA auditor to independently reconstruct every `Q=[k]G` equation using a separately written affine Weierstrass group law (`u128` modular products, Fermat inversion) rather than trusting the original APS source/caller-only known labels. The standalone checker is not the solver and runs *after* all four timed solver panels. Test controls cover generator on-curve, identity, double, opposite points and `[n]G=O`. Commit `0cc827ce70424f521eb4b93bf8481dbd06f4561a`.
- Sanity-checked the public curve constants outside GitHub Actions; (G) lies on the curve, ([n]G=O), and ([2]G=(1923055775,307731458)). This is a producer-side arithmetic check, [pcc:unverified]; exact Rust build/tests remain pending.

### Decisions made
- Require independent recomputation of all 576 recorded recovered scalars (4 panels × 24 targets × 6 complete solvers) against original-curve group arithmetic and reject any mismatch. The frozen target seed, experiment comparison design, cohort and oracle implementation are unmodified.

### Open threads
- Inspect newly triggered CI on the hardened source head and preserve earlier source-specific run as pre-hardening evidence. Advance to performance reporting only if every correctness gate passes.

### Errors
- No new runtime benchmark or performance data yet.


## Result closure — first source-specific ABBA panel (2026-11-17 02:32 UTC)

### What was done
- Exact [Actions #44003116725](https://github.com/isogeny-labs/crypto/actions/runs/44003116725) passed: original and candidate 40/40 native and 22/22 APS example Rust tests; old 2/2 Rust audit tests. Four cold panels old/new/new/old each solved 24 Q × 6 arms; 576/576 returned the caller-supplied known scalar and old/new signed S3 transcripts matched. Raw artifact [#14293546662](https://github.com/isogeny-labs/crypto/actions/runs/44003116725/artifacts/14293546662) stored.
- Auditor's two predeclared old/new result ratios were **1.0017067635 (9/24 paired target wins)** and **1.0016402766 (13/24 wins)**, each missing the ≤0.95 plus ≥18/24 speed gate. Combined descriptive candidate/original ratio **1.0016735319**; A/A and between-repeat drifts were below 0.3%. **No APS speedup** at this scope. All are [pcc:unverified], no isolated host or strongest external comparison.
- Created [full null report](../../research/prime31_ic_20261115/native/APS_S3_BUFFER_ABBA_RESULT_20261117.md) on PR branch, commit `4ec36a2fa3586ca57d85297d246ec02b90cb144b`.

### Decisions made
- Retire this unchanged scratch-buffer-only optimization as a zero-gain lead. Do not reroll the cohort. Only the separately predeclared source-identity, proof-certified RR9 relation extractor is worth further algebraic exploration.
- This historical CI auditor did not separately recompute `[k]G=Q`, although the original native APS Rust source did its own caller replay. The later standalone affine Rust auditor commit `0cc827ce` is separately checking new independent group law and requires its own passing CI before claiming external-style arithmetic readback.

### Open threads
- Check hardened source-specific CI, then update evidence/PR body with its result and distinguish it from historical run. Keep strongest-WZ control blocked until exact Rust dependency closure is repaired.

### Errors
- Scientific null, not a CI failure: candidate adds ~0.17% time at this resolution. Auto-merge through GitHub connector failed because it inferred squash merging while repository disallows squash; leave PR open pending checks/review.
