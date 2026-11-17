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
