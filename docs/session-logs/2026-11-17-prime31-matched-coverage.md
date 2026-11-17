# 2026-11-17 — Prime31 B31 matched signed-eight-leaf coverage null control

## What was done (commands, files, features)

- Checked `AGENTS.md`, `docs/ACTIVE.md`, newest completed original-prime31 log `docs/session-logs/2026-11-17-prime31-complete-rank1024.md`, source and raw 1024-Q transcript from #4623. Inspected currently active #4596 (matched IC/BSGS), #4604 (D9 negative certificates), #4624 (48-curve synthetic Dickson comparison), #4617 (synthetic coverage), #4594 portfolio, #4615/#4618 generic signed oracle, and PKM/APS/Kudo literature. KB MCP/CLI unavailable in this connector runtime; existing repo and primary-source fallback only.
- Confirmed that first 256 frozen public targets in complete Rust transcript have 237 a=0 hits; all 1024 had 951. The formal signed-net support cardinality is N=5,654,193,417, N/n=2.6329841; the naive independent-bin occupancy heuristic predicts 237.60/256 and 950.41/1024. This is an analytic **heuristic**, not a causal explanation or proof for a fixed base.
- Registered task in `docs/ACTIVE.md`, created branch `automation/prime31-b31-coverage-null-control-20261117` from #4623 exact head `e4d5a8ba`, opened draft PR #4626 stacked on unmerged #4623, committed frozen prospective `B31_MATCHED_COVERAGE_NULL_PROTOCOL_20261117.md` before touching Rust experiment code.

## Decisions made (and why)

- Use original native Rust group, factor-base generator, 4A index and 512-way complement arithmetic; match target Q at first public shift with one Dickson, one first-x and two fixed-seed random B31 bases. This falsifies a proposed *unique special-base coverage* advantage without duplicating #4624's synthetic 48-curve study.
- Explicitly distinguish the exact pairwise-collision expectation for 31 **independent uniform abstract scalars** from the independent-bin occupancy heuristic (correlated signatures) and from actual Dickson B31.
- Use fixed 256-Q subset and seeds, original-curve witness checks, full negative exhaustion, counts, and source-specific pinned CI. No new solver or cryptanalytic novelty claim.

## Open threads (unfinished, pick up next time)

- Implement native Rust matched coverage test and exact-schema output on branch, pin workflow to original immutable native source, execute exact-source CI, preserve raw output, audit deterministic counts and matched base differences.
- Record observations and update PR #4626; no merges or PCC/leaderboard promotion. Continue true public-only sign-independent B31 RR9/S9 or K961/S5 extraction in separate owner lane after the coverage audit.

## Errors (what went wrong, how it was handled)

- Local container has no rustc/cargo; native measurement must run through GitHub Actions on exact committed Rust source. No result is claimed before source-specific CI.
